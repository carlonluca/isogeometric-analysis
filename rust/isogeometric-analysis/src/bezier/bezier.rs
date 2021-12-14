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

///
/// Implements Bezier curves and surfaces.
/// 
pub struct BezierCurve {
    p: Vec<RealPoint>
}

impl Evaluatable<f64, f64> for BezierCurve {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi is intended in the
    /// parametric space.
    /// 
    fn evaluate(&self, xi: &RealPoint) -> RealPoint {
        let mut x = 0f64;
        let mut y = 0f64;
        let mut z = 0f64;
        let n = self.p.len();
        for i in 0..n {
            let bernstein = Bernstein {
                n: n as i32 - 1i32,
                i: i as i32
            };
            x = x + bernstein.evaluate(&RealPoint::point1d(xi.x)).x*self.p[i].x;
            y = y + bernstein.evaluate(&RealPoint::point1d(xi.x)).x*self.p[i].y;
            z = z + bernstein.evaluate(&RealPoint::point1d(xi.x)).x*self.p[i].z;
        }

        return Point::point3d(x, y, z);
    }
}

pub struct Bernstein {
    pub n: i32,
    pub i: i32
}

impl Evaluatable<f64, f64> for Bernstein {
    fn evaluate(&self, xi: &RealPoint) -> RealPoint {
        let num = ((fact(self.n) as f64)*xi.x.pow(self.i as f64)*(1f64 - xi.x).pow((self.n - self.i) as f64));
        let den = ((fact(self.i)*fact(self.n - self.i)) as f64);
        return Point::point1d(num/den);
    }
}
