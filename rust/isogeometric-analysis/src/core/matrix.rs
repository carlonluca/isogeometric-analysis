/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.11.02
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

use array2d::Array2D;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use super::size::Size;
use super::point::IntPoint;
use log;

#[derive(Debug)]
pub struct RectMatrix {
    data: Array2D<f64>
}

impl PartialEq for RectMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Add for RectMatrix {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        return self.mult_add(&other, 1f64);
    }
}

impl Sub for RectMatrix {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        return self.mult_add(&other, -1f64);
    }
}

impl Mul<f64> for RectMatrix {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    /// 
    fn mul(self, scalar: f64) -> Self {
        let mut ret = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                ret.data[(i, j)] *= scalar;
            }
        }
        return ret;
    }
}

impl Mul<RectMatrix> for RectMatrix {
    type Output = Self;

    ///
    /// Multiplication by another matrix.
    /// 
    fn mul(self, other: Self) -> Self {
        if self.cols() != other.rows() {
            panic!();
        }

        let mut res = RectMatrix::zeros(self.rows(), other.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let mut e: f64 = 0f64;
                for p in 0..self.cols() {
                    e += self.data[(i, p)]*other.data[(p, j)];
                }
                res.data[(i, j)] = e;
            }
        }

        return res;
    }
}

impl Clone for RectMatrix {
    ///
    /// Clones this matrix.
    /// 
    fn clone(&self) -> Self {
        RectMatrix {
            data: self.data.clone()
        }
    }
}

impl RectMatrix {
    ///
    /// Constructs a matrix from data. Ownership is transferred.
    /// 
    pub fn from_array(data: Array2D<f64>) -> RectMatrix {
        RectMatrix { data: data }
    }

    ///
    /// Constructs a matrix from data. Ownership is not transferred and
    /// the array is copied.
    /// 
    pub fn from_array_ref(data: &Array2D<f64>) -> RectMatrix {
        RectMatrix { data: data.clone() }
    }

    ///
    /// Builds a matrix from an array of vectors.
    /// 
    pub fn from_vec(data: &[Vec<f64>]) -> RectMatrix {
        RectMatrix { data: Array2D::from_rows(data) }
    }

    ///
    /// Returns the value.
    /// 
    pub fn value(&self, row: usize, col: usize) -> f64 {
        self.data[(row, col)]
    }

    ///
    /// Sets a value.
    /// 
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.data[(row, col)] = value
    }

    ///
    /// Returns the number of rows.
    /// 
    pub fn rows(&self) -> usize {
        self.data.num_rows()
    }

    ///
    /// Returns the i-th row.
    /// 
    pub fn row(&self, i: usize) -> RowVector {
        // TODO: Optimize
        RowVector::from_vec(&self.data.as_rows()[i])
    }

    ///
    /// Returns the number of columns.
    /// 
    pub fn cols(&self) -> usize {
        self.data.num_columns()
    }

    ///
    /// Returns the i-th column.
    /// 
    pub fn col(&self, i: usize) -> ColVector {
        // TODO: Optimize
        ColVector::from_vec(&self.data.as_columns()[i])
    }
    
    ///
    /// Returns the max value in column.
    /// 
    pub fn max_col(&self, j: usize) -> f64 {
        let mut max = f64::MIN;
        for i in 0..self.rows() {
            if max < self.data[(i, j)] {
                max = self.data[(i, j)];
            }
        }
        max
    }

    ///
    /// Prints the matrix.
    /// 
    pub fn print(&self) {
        let mut ret = String::new();
        for i in 0..self.rows() {
            if i == 0 { ret += "⎡"; }
            else if i == self.rows() - 1 { ret += "⎣"; }
            else { ret += "⎢" }
            for j in 0..self.cols() {
                let max = self.max_col(j);
                let size = max.to_string().chars().count() + 1;
                let thisItem = self.data[(i, j)].to_string();
                let spaces = &size - thisItem.chars().count();
                ret += &String::from_utf8(vec![b' '; spaces]).unwrap();
                ret += &thisItem;
            }
            if i == 0 { ret += " ⎤"; }
            else if i == self.rows() - 1 { ret += " ⎦"; }
            else { ret += " ⎥" }
            ret += "\n";
        }

        log::info!("\n{}", ret);
    }

    ///
    /// Crops the matrix.
    /// 
    pub fn rect(&self, top_left: IntPoint, bottom_right: IntPoint) -> RectMatrix {
        let cols = bottom_right.x - top_left.x + 1;
        let rows = bottom_right.y - top_left.y + 1;
        let mut mat = RectMatrix::zeros(rows as usize, cols as usize);
        for j in top_left.x..(bottom_right.x + 1) {
            for i in top_left.y..(bottom_right.y + 1) {
                mat.set_value((i - top_left.y) as usize, (j - top_left.x) as usize, self.value(i as usize, j as usize));
            }
        }
        mat
    }

    ///
    /// Returns the size of the matrix.
    /// 
    pub fn size(&self) -> Size {
        Size {
            width: self.cols(),
            height: self.rows()
        }
    }

    ///
    /// Transpose this matrix and returns a new transposed instance.
    /// 
    pub fn transposed(&self) -> Self {
        let mut res = RectMatrix::zeros(self.cols(), self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                res.data[(j, i)] = self.data[(i, j)];
            }
        }
        res
    }

    ///
    /// Returns true iif this is a row.
    /// 
    pub fn is_row(&self) -> bool {
        self.cols() >= 1 && self.rows() == 1
    }

    ///
    /// Returns true iif this a col.
    /// 
    pub fn is_col(&self) -> bool {
        self.cols() == 1 && self.rows() >= 1
    }

    ///
    /// Creates a matrix filled with zeros.
    ///
    pub fn zeros(rows: usize, cols: usize) -> Self {
        if rows <= 0 || cols <= 0 {
            panic!()
        }

        Self {
            data: Array2D::filled_with(0f64, rows, cols)
        }
    }

    ///
    /// Creates an identity matrix.
    ///
    pub fn identity(size: usize) -> Self {
        let mut zero = Self::zeros(size, size);
        for i in 0..size {
            for j in 0..size {
                zero.set_value(i, j, if i == j { 1f64 } else { 0f64 });
            }
        }
        return zero;
    }

    // Private impl
    // ============
    ///
    /// Adds to another matrix for terms multiplied by a factor.
    /// 
    fn mult_add(&self, other: &RectMatrix, fac: f64) -> RectMatrix {
        if self.size() != other.size() {
            panic!()
        }

        let mut output = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                output.data[(i, j)] += fac*other.value(i, j);
            }
        }
        output
    }
}

///
/// Represents a row.
/// 
#[derive(Debug)]
pub struct RowVector {
    pub matrix: RectMatrix
}

impl RowVector {
    ///
    /// Builds a RowVector from a vector.
    /// 
    pub fn from_vec(data: &[f64]) -> RowVector {
        RowVector {
            matrix: RectMatrix {
                data: Array2D::from_rows(&[data.to_vec()])
            }
        }
    }

    ///
    /// Length of the row.
    /// 
    pub fn length(&self) -> usize {
        self.matrix.cols()
    }
}

impl PartialEq for RowVector {
    fn eq(&self, other: &Self) -> bool {
        other.matrix == self.matrix
    }
}

///
/// Represents a column.
/// 
#[derive(Debug)]
pub struct ColVector {
    pub matrix: RectMatrix
}

impl ColVector {
    ///
    /// Builds a ColVector from a vector.
    /// 
    pub fn from_vec(data: &[f64]) -> ColVector {
        ColVector {
            matrix: RectMatrix {
                data: Array2D::from_columns(&[data.to_vec()])
            }
        }
    }

    ///
    /// Height of the column.
    /// 
    pub fn height(&self) -> usize {
        self.matrix.rows()
    }
}

impl PartialEq for ColVector {
    fn eq(&self, other: &Self) -> bool {
        other.matrix == self.matrix
    }
}

#[cfg(test)]
mod tests {
    use crate::core::RectMatrix;
    use crate::core::RowVector;
    use crate::core::ColVector;
    use crate::core::IntPoint;

    #[test]
    fn test_get() {
        env_logger::init();
        let m = RectMatrix::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64]
        ]);
        assert_eq!(m.value(0, 0), 1f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
        assert_eq!(m.row(0).matrix, RowVector::from_vec(&[1f64, 2f64]).matrix);
        assert_eq!(m.row(0), RowVector::from_vec(&[1f64, 2f64]));
        assert_eq!(m.row(1), RowVector::from_vec(&[3f64, 4f64]));
        assert_eq!(m.col(0), ColVector::from_vec(&[1f64, 3f64]));
        assert_eq!(m.col(1), ColVector::from_vec(&[2f64, 4f64]));
    }

    #[test]
    fn test_set() {
        let mut m = RectMatrix::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64]
        ]);

        m.set_value(0, 0, 15f64);

        assert_eq!(m.value(0, 0), 15f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
    }

    #[test]
    fn test_size() {
        let m = RectMatrix::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64],
            vec![5f64, 6f64]
        ]);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_add() {
        let m1 = RectMatrix::from_vec(&vec![
            vec![1f64, 2f64, 3f64]
        ]);
        let m2 = RectMatrix::from_vec(&vec![
            vec![1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, 1f64);
        let m4 = m1.clone() + m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, RectMatrix::from_vec(&vec![
            vec![2f64, 3f64, 4f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_sub() {
        let m1 = RectMatrix::from_vec(&vec![
            vec![1f64, 2f64, 3f64]
        ]);
        let m2 = RectMatrix::from_vec(&vec![
            vec![1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, -1f64);
        let m4 = m1.clone() - m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, RectMatrix::from_vec(&vec![
            vec![0f64, 1f64, 2f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_zeros() {
        let m = RectMatrix::zeros(5, 5);
        assert_eq!(m, RectMatrix::from_vec(&[
            vec![0f64, 0f64, 0f64, 0f64, 0f64],
            vec![0f64, 0f64, 0f64, 0f64, 0f64],
            vec![0f64, 0f64, 0f64, 0f64, 0f64],
            vec![0f64, 0f64, 0f64, 0f64, 0f64],
            vec![0f64, 0f64, 0f64, 0f64, 0f64]
        ]));
        assert_eq!(m.value(0, 0), 0f64);
        assert_eq!(m.value(1, 1), 0f64);
        assert_eq!(m.value(2, 2), 0f64);
        assert_eq!(m.value(3, 3), 0f64);
        assert_eq!(m.value(4, 4), 0f64);
    }

    #[test]
    fn test_identity() {
        let m = RectMatrix::identity(10);
        for i in 0..(m.rows() - 1) {
            for j in 0..(m.cols() - 1) {
                assert_eq!(m.value(i, j), if i == j { 1f64 } else { 0f64 });
            }
        }
    }
    
    #[test]
    fn test_mult_scalar_1() {
        let m = RectMatrix::from_vec(&[
            vec![1f64, 2f64, 3f64],
            vec![4f64, 5f64, 6f64],
            vec![7f64, 8f64, 9f64]
        ]);
        let m2 = m*5f64;
        assert_eq!(m2, RectMatrix::from_vec(&[
            vec![5f64, 10f64, 15f64],
            vec![20f64, 25f64, 30f64],
            vec![35f64, 40f64, 45f64]
        ]));
    }

    #[test]
    fn test_mult_scalar_2() {
        let m = RectMatrix::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64]
        ]);
        assert_eq!(m.clone()*9f64, RectMatrix::from_vec(&[
            vec![5f64*9f64, 6f64*9f64, 7f64*9f64],
            vec![1f64*9f64, 2f64*9f64, 3f64*9f64],
            vec![9f64*9f64, 8f64*9f64, 7f64*9f64]
        ]));
    }

    #[test]
    fn test_mult_scalar_3() {
        let m = RectMatrix::from_vec(&[
            vec![5f64*9f64, 6f64*9f64, 7f64*9f64],
            vec![1f64*9f64, 2f64*9f64, 3f64*9f64],
            vec![9f64*9f64, 8f64*9f64, 7f64*9f64]
        ]);
        assert_eq!(m*0f64, RectMatrix::zeros(3, 3));
    }

    #[test]
    fn test_mult_matrix() {
        let m1 = RectMatrix::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64]
        ]);
        let m2 = RectMatrix::from_vec(&[
            vec![1f64, 2f64, 3f64],
            vec![4f64, 5f64, 6f64],
            vec![7f64, 8f64, 9f64]
        ]);
        let m3 = RectMatrix::identity(3);
        let m4 = RectMatrix::zeros(3, 3);
        assert_eq!(m1.clone()*m1.clone(), RectMatrix::from_vec(&[
            vec![94f64, 98f64, 102f64],
            vec![34f64, 34f64, 34f64],
            vec![116f64, 126f64, 136f64]
        ]));
        assert_eq!(m1.clone()*m2.clone(), RectMatrix::from_vec(&[
            vec![78f64, 96f64, 114f64],
            vec![30f64, 36f64, 42f64],
            vec![90f64, 114f64, 138f64]
        ]));
        assert_eq!(m1.clone()*m3.clone(), m1);
        assert_eq!(m2.clone()*m3.clone(), m2);
        assert_eq!(m1.clone()*m4.clone(), m4);
    }

    #[test]
    fn test_transpose() {
        let m = RectMatrix::from_vec(&[
            vec![1f64, 2f64, 3f64],
            vec![4f64, 5f64, 6f64],
            vec![7f64, 8f64, 9f64]
        ]);
        assert_eq!(m, RectMatrix::from_vec(&[
            vec![1f64, 4f64, 7f64],
            vec![2f64, 5f64, 8f64],
            vec![3f64, 6f64, 9f64]
        ]).transposed());
    }

    #[test]
    fn test_vec() {
        let r = RowVector::from_vec(&[1f64, 2f64]);
        assert_eq!(r.length(), 2);
        assert_eq!(r.matrix.is_row(), true);
        assert_eq!(r.matrix.transposed().is_col(), true);

        let c = ColVector::from_vec(&[2f64, 4f64]);
        assert_eq!(c.height(), 2);
        assert_eq!(c.matrix.is_col(), true);
        assert_eq!(c.matrix.transposed().is_row(), true);
    }

    #[test]
    fn test_rect() {
        let m1 = RectMatrix::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64],
            vec![1f64, 1f64, 1f64]
        ]);
        let m2 = RectMatrix::from_vec(&[
            vec![2f64, 3f64],
            vec![8f64, 7f64]
        ]);
        assert_eq!(m1.rect(IntPoint {
            x: 1,
            y: 1
        }, IntPoint {
            x: 2,
            y: 2
        }), m2);
    }

    #[test]
    fn test_max() {
        let m = RectMatrix::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64],
            vec![1f64, 1f64, 1f64]
        ]);
        assert_eq!(m.max_col(0), 9f64);
        assert_eq!(m.max_col(1), 8f64);
        assert_eq!(m.max_col(2), 7f64);
    }

    #[test]
    fn test_print() {
        let m = RectMatrix::from_vec(&[
            vec![500f64, 6f64, 7f64],
            vec![1f64, 200f64, 3f64],
            vec![9f64, 8f64, 700000f64],
            vec![1f64, 1f64, 1f64]
        ]);
        m.print();
    }
}
