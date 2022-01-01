/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.03
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

use crate::core::fact;
use crate::core::{RealPoint, RealPoint1d};
use crate::core::Evaluatable;
use num::traits::Pow;
use unroll::unroll_for_loops;

///
/// Represents a Bernstein basis polynomial.
/// 
/// # Example
/// 
/// ```rust
/// let b = Bernstein::create(5, 2).unwrap();
/// let (xpoints, ypoints) = Evaluator::<1, 1>::evaluate_r_to_r3(&b, &0f64, &1f64, &10000);
/// ```
/// 
pub struct Bernstein {
    n: u32,
    i: u32
}

impl Evaluatable<f64, f64, 1, 1> for Bernstein {
    ///
    /// Evaluates the function.
    /// 
    fn evaluate(&self, i: &RealPoint<1>) -> RealPoint<1> {
        let num = (fact(self.n) as f64)*i.x().pow(self.i as f64)*(1f64 - i.x()).pow((self.n - self.i) as f64);
        let den = (fact(self.i)*fact(self.n - self.i)) as f64;
        RealPoint::<1>::point1d(num/den)
    }

    ///
    /// Evaluate without creating a new object.
    /// 
    fn evaluate_fill(&self, input: &RealPoint1d, mut output: RealPoint1d) -> RealPoint1d {
        let num = (fact(self.n) as f64)*input.x().pow(self.i as f64)*(1f64 - input.x()).pow((self.n - self.i) as f64);
        let den = (fact(self.i)*fact(self.n - self.i)) as f64;
        output.set_x(num/den);
        output
    }
}

impl Bernstein {
    ///
    /// Creates the `i`-th bernstein basis polynomial of degree `n`.
    /// 
    pub fn create(n: u32, i: u32) -> Option<Bernstein> {
        if i > n {
            log::warn!("Index cannot be grater than degree");
            return None;
        }
        Some(Bernstein { n: n, i: i })
    }
}

///
/// Implements Bezier curves and surfaces.
/// 
pub struct BezierCurve<const SIZE: usize> {
    pub p: Vec<RealPoint<SIZE>>
}

impl<const SIZE: usize> Evaluatable<f64, f64, 1, SIZE> for BezierCurve<SIZE> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi is intended in the
    /// parametric space.
    /// 
    fn evaluate(&self, xi: &RealPoint<1>) -> RealPoint<SIZE> {
        let output = RealPoint::<SIZE>::origin();
        self.evaluate_direct(xi, output)
    }

    fn evaluate_fill(&self, input: &RealPoint1d, output: RealPoint<SIZE>) -> RealPoint<SIZE> {
        self.evaluate_direct(input, output)
    }
}

impl<const SIZE: usize> BezierCurve<SIZE> {
    ///
    /// Computes the value of the Bezier curve in xi using the direct algorithm. This
    /// technique is not numerically stable.
    ///
    #[unroll_for_loops]
    pub fn evaluate_direct(&self, input: &RealPoint<1>, mut output: RealPoint<SIZE>) -> RealPoint<SIZE> {
        let n = self.p.len();
        let tmp = RealPoint1d::origin();
        output.reset();
        for i in 0..n {
            let bernstein = Bernstein::create((n - 1) as u32, i as u32).unwrap();
            let bout = bernstein.evaluate_fill(&input, tmp).x();
            output += self.p[i]*bout;
        }

        return output;
    }

    ///
    /// Computes the value of the Bezier curve in xi using the De Casteljau's algorithm.
    ///
    #[unroll_for_loops]
    pub fn evaluate_de_casteljau(&self, xi: &RealPoint1d) -> RealPoint<SIZE> {
        let n = self.p.len() - 1;
        let mut q = Vec::<RealPoint<SIZE>>::new();
        for i in 0..(n + 1) {
            q.push(self.p[i]);
        }
        for k in 1..(n + 1) {
            for i in 0..(n - k + 1) {
                q[i] = q[i]*(1f64 - xi.x()) + q[i + 1]*xi.x();
            }
        }

        return q[0];
    }
}
