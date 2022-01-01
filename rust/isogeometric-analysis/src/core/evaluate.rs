/*
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
pub trait Evaluatable<I: MatElement, O: MatElement, const DIMDOM: usize, const DIMCOD: usize> {
    ///
    /// Evaluates the function.
    /// 
    fn evaluate(&self, i: &Point<I, DIMDOM>) -> Point<O, DIMCOD>;

    ///
    /// Evaluates the function and assigns the values to an existing point
    /// object.
    /// 
    fn evaluate_fill(&self, i: &Point<I, DIMDOM>, o: Point<O, DIMCOD>) -> Point<O, DIMCOD>;
}

///
/// This class is used to automate computations for curves.
/// 
pub struct Evaluator<const DIMIN: usize, const DIMOUT: usize> {}

impl<const DIMIN: usize, const DIMOUT: usize> Evaluator<DIMIN, DIMOUT> {
    ///
    /// Computes a geometric element for a sequence of points.
    /// 
    pub fn evaluate(element: &impl Evaluatable<f64, f64, DIMIN, DIMOUT>, values: &Vec<RealPoint<DIMIN>>) -> Vec<RealPoint<DIMOUT>> {
        let mut ret: Vec<RealPoint<DIMOUT>> = Vec::new();
        for p in values.iter() {
            ret.push(element.evaluate(p));
        }
        return ret;
    }

    ///
    /// Evalutes a geometric element for an interval of elements of R.
    /// 
    pub fn evaluate_r_to_r3(element: &impl Evaluatable<f64, f64, 1, DIMOUT>, from: &f64, to: &f64, count: &i64) -> (Vec<RealPoint<1>>, Vec<RealPoint<DIMOUT>>) {
        let rv: RowVector<f64> = RowVector::<f64>::evenly_spaced(from, to, count);
        let values = rv.to_vec();
        let mut input: Vec<RealPoint<1>> = Vec::new();
        for v in values {
            let p = Point::<f64, 1>::point1d(v);
            input.push(p);
        }

        let mut output: Vec<RealPoint<DIMOUT>> = Vec::new();
        let tmp = RealPoint::<DIMOUT>::origin();
        for p in &input {
            output.push(element.evaluate_fill(&p, tmp));
        }

        return (input, output);
    }

    ///
    /// Rearranges coords in arrays.
    /// 
    pub fn split_coords<T: MatElement>(mapx: usize, x: &Vec<Point<T, DIMOUT>>, mapy: usize, y: &Vec<Point<T, DIMOUT>>, mapz: usize, z: &Vec<Point<T, DIMOUT>>) -> (Vec<T>, Vec<T>, Vec<T>) {
        let sizes = vec![x.len(), y.len(), z.len()];
        let count = sizes.iter().min().clone();
        match count {
            None => { return (vec![], vec![], vec![]) },
            Some(c) => {
                let mut xvalues = Vec::new();
                let mut yvalues = Vec::new();
                let mut zvalues = Vec::new();
                for i in 0..*c {
                    xvalues.push(x[i].value(mapx as u8));
                    yvalues.push(y[i].value(mapy as u8));
                    zvalues.push(z[i].value(mapz as u8));
                }

                return (xvalues, yvalues, zvalues);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Evaluator;
    use crate::core::RealPoint;

    #[test]
    fn test() {
        env_logger::init();
        let xvalues = vec![
            RealPoint::<1>::point1d(0f64),
            RealPoint::<1>::point1d(1f64),
            RealPoint::<1>::point1d(2f64)
        ];
        let yvalues = vec![
            RealPoint::<1>::point1d(3f64),
            RealPoint::<1>::point1d(4f64),
            RealPoint::<1>::point1d(5f64)
        ];
        let zvalues = vec![
            RealPoint::<1>::point1d(6f64),
            RealPoint::<1>::point1d(7f64),
            RealPoint::<1>::point1d(8f64)
        ];
        let (x, y, z) = Evaluator::<1, 1>::split_coords(0, &xvalues, 0, &yvalues, 0, &zvalues);
        assert_eq!(x.len(), 3);
        assert_eq!(y.len(), 3);
        assert_eq!(z.len(), 3);
        assert_eq!(x[0], 0f64);
        assert_eq!(x[1], 1f64);
        assert_eq!(x[2], 2f64);
        assert_eq!(y[0], 3f64);
        assert_eq!(y[1], 4f64);
        assert_eq!(y[2], 5f64);
        assert_eq!(z[0], 6f64);
        assert_eq!(z[1], 7f64);
        assert_eq!(z[2], 8f64);
    }
}
