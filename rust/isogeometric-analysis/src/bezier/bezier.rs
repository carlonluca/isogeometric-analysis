/**
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
use crate::core::Point;
use crate::core::RealPoint;
use crate::core::Evaluatable;
use num::traits::Pow;
use unroll::unroll_for_loops;

///
/// Implements Bezier curves and surfaces.
/// 
pub struct BezierCurve {
    pub p: Vec<RealPoint>
}

impl Evaluatable<f64, f64> for BezierCurve {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi is intended in the
    /// parametric space.
    /// 
    fn evaluate(&self, xi: &RealPoint) -> RealPoint {
        self.evaluate_direct(xi)
    }
}

impl BezierCurve {
    ///
    /// Computes the value of the Bezier curve in xi using the direct algorithm. This
    /// technique is not numerically stable.
    ///
    #[unroll_for_loops]
    pub fn evaluate_direct(&self, xi: &RealPoint) -> RealPoint {
        let n = self.p.len();
        let mut p = RealPoint::point3d(0f64, 0f64, 0f64);
        for i in 0..n {
            let bernstein = Bernstein::create((n - 1) as u32, i as u32).unwrap();
            let bout = bernstein.evaluate(&xi).x();
            p = p + self.p[i]*bout;
        }

        return p;
    }

    ///
    /// Computes the value of the Bezier curve in xi using the De Casteljau's algorithm.
    ///
    #[unroll_for_loops]
    pub fn evaluate_de_casteljau(&self, xi: &RealPoint) -> RealPoint {
        let n = self.p.len() - 1;
        let mut q = Vec::<RealPoint>::new();
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

///
/// Represents a Bernstein basis polynomial.
/// 
pub struct Bernstein {
    n: u32,
    i: u32
}

impl Evaluatable<f64, f64> for Bernstein {
    ///
    /// Evaluates the function.
    /// 
    fn evaluate(&self, xi: &RealPoint) -> RealPoint {
        let num = (fact(self.n) as f64)*xi.x().pow(self.i as f64)*(1f64 - xi.x()).pow((self.n - self.i) as f64);
        let den = (fact(self.i)*fact(self.n - self.i)) as f64;
        return Point::point1d(num/den);
    }
}

impl Bernstein {
    ///
    /// Creates a bernstein basis polynomial.
    /// 
    pub fn create(n: u32, i: u32) -> Option<Bernstein> {
        if i > n {
            log::warn!("Index cannot be grater than degree");
            return None;
        }
        Some(Bernstein { n: n, i: i })
    }
}
