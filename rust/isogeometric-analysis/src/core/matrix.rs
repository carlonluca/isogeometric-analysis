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
}
