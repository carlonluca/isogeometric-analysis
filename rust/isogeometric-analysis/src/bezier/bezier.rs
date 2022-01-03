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
use crate::core::{RealPoint, RealPoint1d, RealPoint3d};
use crate::core::Evaluatable;
use num::traits::Pow;
use unroll::unroll_for_loops;
use array2d::Array2D;

///
/// Represents a Bernstein basis polynomial.
/// 
/// # Example
/// 
/// ```rust
/// use isogeometric_analysis::bezier::Bernstein;
/// use isogeometric_analysis::core::Evaluator;
/// let b = Bernstein::create(5, 2).unwrap();
/// let (xpoints, ypoints) = Evaluator::<1, 1>::evaluate_parametric(&b, &0f64, &1f64, &10000);
/// ```
/// 
pub struct Bernstein {
    n: u32,
    i: u32
}

impl Evaluatable<f64, f64, 1, 1> for Bernstein {
    ///
    /// Evaluate without creating a new object.
    /// 
    fn evaluate_fill<'a>(&self, input: &RealPoint1d, output: &'a mut RealPoint1d) -> &'a mut RealPoint1d {
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
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    /// 
    fn evaluate_fill<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        self.evaluate_direct(xi, output)
    }
}

impl<const SIZE: usize> BezierCurve<SIZE> {
    ///
    /// Computes the value of the Bezier curve in xi using the direct algorithm. This
    /// technique is not numerically stable.
    ///
    #[unroll_for_loops]
    pub fn evaluate_direct<'a>(&self, input: &RealPoint<1>, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        let n = self.p.len();
        let mut tmp = RealPoint1d::origin();
        output.reset();
        for i in 0..n {
            let bernstein = Bernstein::create((n - 1) as u32, i as u32).unwrap();
            let bout = bernstein.evaluate_fill(&input, &mut tmp).x();
            *output += self.p[i]*bout;
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

    ///
    /// Returns the degree of the Bezier curve.
    /// 
    pub fn degree(&self) -> u32 {
        (self.p.len() - 1) as u32
    }
}

///
/// Represents a Bezier surface.
/// 
pub struct BezierSurf {
    pub data: Array2D<RealPoint3d>
}

impl BezierSurf {
    ///
    /// Returns the degree on the Xi axis.
    /// 
    pub fn degree_xi(&self) -> u32 { (self.data.row_len() - 1) as u32 }

    ///
    /// Returns the degree on the Eta axis.
    /// 
    pub fn degree_eta(&self) -> u32 { (self.data.column_len() - 1) as u32 }
}


