/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.11
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

use crate::core::RealPoint;
use crate::core::Point;
use crate::core::RowVector;
use crate::core::MatElement;

///
/// Generic interface for an evaluatable element.
/// 
pub trait Evaluatable<I: MatElement, O: MatElement> {
    fn evaluate(&self, p: &Point<I>) -> Point<O>;
}

///
/// This class is used to automate computations for curves.
/// 
pub struct Evaluator {}

impl Evaluator {
    ///
    /// Computes a geometric element for a sequence of points.
    /// 
    pub fn evaluate(element: &impl Evaluatable<f64, f64>, values: &Vec<RealPoint>) -> Vec<RealPoint> {
        let mut ret: Vec<RealPoint> = Vec::new();
        for p in values.iter() {
            ret.push(element.evaluate(p));
        }
        return ret;
    }

    ///
    /// Evalutes a geometric element for an interval of elements of R.
    /// 
    pub fn evaluate_r_to_r3(element: &impl Evaluatable<f64, f64>, from: &f64, to: &f64, count: &i64) -> (Vec<RealPoint>, Vec<RealPoint>) {
        let rv: RowVector<f64> = RowVector::<f64>::evenly_spaced(from, to, count);
        let values = rv.to_vec();
        let mut input: Vec<RealPoint> = Vec::new();
        for v in values {
            let p = Point::point1d(v);
            input.push(p);
        }

        let mut output: Vec<RealPoint> = Vec::new();
        for p in &input {
            output.push(element.evaluate(&p));
        }

        return (input, output);
    }
}
