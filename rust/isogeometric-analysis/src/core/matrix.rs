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

#[derive(Debug)]
pub struct Matrix2 {
    data: Array2D<f64>
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Add for Matrix2 {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        return self.mult_add(&other, 1f64);
    }
}

impl Sub for Matrix2 {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        return self.mult_add(&other, -1f64);
    }
}

impl Mul<f64> for Matrix2 {
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

impl Mul<Matrix2> for Matrix2 {
    type Output = Self;

    ///
    /// Multiplication by another matrix.
    /// 
    fn mul(self, other: Self) -> Self {
        if self.cols() != other.rows() {
            panic!();
        }

        let mut res = Matrix2::zeros(self.rows(), other.cols());
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

impl Clone for Matrix2 {
    ///
    /// Clones this matrix.
    /// 
    fn clone(&self) -> Self {
        Matrix2 {
            data: self.data.clone()
        }
    }
}

impl Matrix2 {
    ///
    /// Constructs a matrix from data. Ownership is transferred.
    /// 
    pub fn from_array(data: Array2D<f64>) -> Matrix2 {
        Matrix2 { data: data }
    }

    ///
    /// Constructs a matrix from data. Ownership is not transferred and
    /// the array is copied.
    /// 
    pub fn from_array_ref(data: &Array2D<f64>) -> Matrix2 {
        Matrix2 { data: data.clone() }
    }

    ///
    /// Builds a matrix from an array of vectors.
    /// 
    pub fn from_vec(data: &[Vec<f64>]) -> Matrix2 {
        Matrix2 { data: Array2D::from_rows(data) }
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
    /// Returns the number of columns.
    /// 
    pub fn cols(&self) -> usize {
        self.data.num_columns()
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
    fn mult_add(&self, other: &Matrix2, fac: f64) -> Matrix2 {
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

#[cfg(test)]
mod tests {
    use crate::core::Matrix2;

    #[test]
    fn test_get() {
        let m = Matrix2::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64]
        ]);
        assert_eq!(m.value(0, 0), 1f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
    }

    #[test]
    fn test_set() {
        let mut m = Matrix2::from_vec(&vec![
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
        let m = Matrix2::from_vec(&vec![
            vec![1f64, 2f64],
            vec![3f64, 4f64],
            vec![5f64, 6f64]
        ]);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix2::from_vec(&vec![
            vec![1f64, 2f64, 3f64]
        ]);
        let m2 = Matrix2::from_vec(&vec![
            vec![1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, 1f64);
        let m4 = m1.clone() + m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, Matrix2::from_vec(&vec![
            vec![2f64, 3f64, 4f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_sub() {
        let m1 = Matrix2::from_vec(&vec![
            vec![1f64, 2f64, 3f64]
        ]);
        let m2 = Matrix2::from_vec(&vec![
            vec![1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, -1f64);
        let m4 = m1.clone() - m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, Matrix2::from_vec(&vec![
            vec![0f64, 1f64, 2f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_zeros() {
        let m = Matrix2::zeros(5, 5);
        assert_eq!(m, Matrix2::from_vec(&[
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
        let m = Matrix2::identity(10);
        for i in 0..(m.rows() - 1) {
            for j in 0..(m.cols() - 1) {
                assert_eq!(m.value(i, j), if i == j { 1f64 } else { 0f64 });
            }
        }
    }
    
    #[test]
    fn test_mult_scalar() {
        let m = Matrix2::from_vec(&[
            vec![1f64, 2f64, 3f64],
            vec![4f64, 5f64, 6f64],
            vec![7f64, 8f64, 9f64]
        ]);
        let m2 = m*5f64;
        assert_eq!(m2, Matrix2::from_vec(&[
            vec![5f64, 10f64, 15f64],
            vec![20f64, 25f64, 30f64],
            vec![35f64, 40f64, 45f64]
        ]));
    }

    #[test]
    fn test_mult_matrix() {
        let m1 = Matrix2::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64]
        ]);
        let m2 = Matrix2::from_vec(&[
            vec![1f64, 2f64, 3f64],
            vec![4f64, 5f64, 6f64],
            vec![7f64, 8f64, 9f64]
        ]);
        let m3 = Matrix2::identity(3);
        let m4 = Matrix2::zeros(3, 3);
        assert_eq!(m1.clone()*m1.clone(), Matrix2::from_vec(&[
            vec![94f64, 98f64, 102f64],
            vec![34f64, 34f64, 34f64],
            vec![116f64, 126f64, 136f64]
        ]));
        assert_eq!(m1.clone()*m2.clone(), Matrix2::from_vec(&[
            vec![78f64, 96f64, 114f64],
            vec![30f64, 36f64, 42f64],
            vec![90f64, 114f64, 138f64]
        ]));
        assert_eq!(m1.clone()*m3.clone(), m1);
        assert_eq!(m2.clone()*m3.clone(), m2);
        assert_eq!(m1.clone()*m4.clone(), m4);
    }
}
