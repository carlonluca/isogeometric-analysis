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
use crate::core::Evaluatable;
use num::traits::Pow;

///
/// Implements Bezier curves and surfaces.
/// 
pub struct BezierCurve {
    p: Vec<Point<f64>>
}

impl Evaluatable<f64, f64> for BezierCurve {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi is intended in the
    /// parametric space.
    /// 
    fn evaluate(&self, xi: Point<f64>) -> Point<f64> {
        let mut x = 0f64;
        let mut y = 0f64;
        let mut z = 0f64;
        let n = self.p.len();
        for i in 0..n {
            x = x + Bezier::bernstein((n - 1) as i32, i as i32, xi.x)*self.p[i].x;
            y = y + Bezier::bernstein((n - 1) as i32, i as i32, xi.x)*self.p[i].y;
            z = z + Bezier::bernstein((n - 1) as i32, i as i32, xi.x)*self.p[i].z;
        }

        return Point::point3d(x, y, z);
    }
}

pub struct Bezier {}

impl Bezier {
    ///
    /// Computes the i-th bernstein basis of degree n in xi.
    /// 
    pub fn bernstein(n: i32, i: i32, xi: f64) -> f64 {
        ((fact(n) as f64)*xi.pow(i as f64)*(1f64 - xi).pow((n - i) as f64))/((fact(i)*fact(n - i)) as f64)
    }
}
