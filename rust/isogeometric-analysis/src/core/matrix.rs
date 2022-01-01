/*
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
use std::ops::{MulAssign, AddAssign, SubAssign};
use std::clone::Clone;
use std::fmt::Display;
use super::size::Size;
use super::point::IntPoint;
use num::traits::{Signed};
use log;

pub trait MatElement: Signed + Clone + MulAssign + AddAssign + SubAssign + PartialOrd + Display + Copy {}
impl<T> MatElement for T where T: Signed + Clone + MulAssign + AddAssign + SubAssign + PartialOrd + Display + Copy { }

#[derive(Debug)]
#[derive(Eq)]
pub struct RectMatrix<T: MatElement> {
    data: Array2D<T>
}

impl<T: MatElement> PartialEq for RectMatrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: MatElement> Add for RectMatrix<T> {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        return self.mult_add(&other, T::one());
    }
}

impl<T: MatElement> Sub for RectMatrix<T> {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        return self.mult_add(&other, T::one().neg());
    }
}

impl<T: MatElement> Mul<T> for RectMatrix<T> {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    /// 
    fn mul(self, scalar: T) -> Self::Output {
        let mut ret = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                ret.data[(i, j)] *= scalar.clone();
            }
        }
        return ret;
    }
}

impl<T: MatElement> Mul<RectMatrix<T>> for RectMatrix<T> {
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
                let mut e = T::zero();
                for p in 0..self.cols() {
                    e += self.data[(i, p)].clone()*other.data[(p, j)].clone();
                }
                res.data[(i, j)] = e;
            }
        }

        return res;
    }
}

impl<T: MatElement> Clone for RectMatrix<T> {
    ///
    /// Clones this matrix.
    /// 
    fn clone(&self) -> Self {
        RectMatrix {
            data: self.data.clone()
        }
    }
}

impl<T: MatElement> RectMatrix<T> {
    ///
    /// Constructs a matrix from data. Ownership is transferred.
    /// 
    pub fn from_array(data: Array2D<T>) -> RectMatrix<T> {
        RectMatrix { data: data }
    }

    ///
    /// Constructs a matrix from data. Ownership is not transferred and
    /// the array is copied.
    /// 
    pub fn from_array_ref(data: &Array2D<T>) -> RectMatrix<T> {
        RectMatrix { data: data.clone() }
    }

    ///
    /// Builds a matrix from an array of vectors.
    /// 
    pub fn from_vec(data: &[Vec<T>]) -> RectMatrix<T> {
        RectMatrix { data: Array2D::from_rows(data) }
    }

    ///
    /// Returns the value.
    /// 
    pub fn value(&self, row: usize, col: usize) -> T {
        self.data[(row, col)].clone()
    }

    ///
    /// Sets a value.
    /// 
    pub fn set_value(&mut self, row: usize, col: usize, value: T) {
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
    pub fn row(&self, i: usize) -> RowVector<T> {
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
    pub fn col(&self, i: usize) -> ColVector<T> {
        // TODO: Optimize
        ColVector::from_vec(&self.data.as_columns()[i])
    }
    
    ///
    /// Returns the max value in column.
    /// 
    pub fn max_col(&self, j: usize) -> T {
        let mut max: Option<T> = None;
        for i in 0..self.rows() {
            match &max {
                None => max = Some(self.data[(i, j)].clone()),
                Some(v) => if v < &self.data[(i, j)] { max = Some(self.data[(i, j)].clone()); }
            }
        }
        match max {
            None => panic!(),
            Some(v) => v
        }
    }

    ///
    /// Prints the matrix.
    /// 
    pub fn print(&self) {
        if self.rows() == 0 || self.cols() == 0 {
            log::info!("\n[ ]");
            return;
        }

        let mut ret = String::new();
        for i in 0..self.rows() {
            if self.rows() == 1 { ret += "["; }
            else if i == 0 { ret += "⎡"; }
            else if i == self.rows() - 1 { ret += "⎣"; }
            else { ret += "⎢" }
            for j in 0..self.cols() {
                let max = self.max_col(j);
                let size = max.to_string().chars().count() + 1;
                let this_item = self.data[(i, j)].to_string();
                let spaces = &size - this_item.chars().count();
                ret += &String::from_utf8(vec![b' '; spaces]).unwrap();
                ret += &this_item;
            }
            if self.rows() == 1 { ret += " ]"; }
            else if i == 0 { ret += " ⎤"; }
            else if i == self.rows() - 1 { ret += " ⎦"; }
            else { ret += " ⎥"; }
            ret += "\n";
        }

        log::info!("\n{}", ret);
    }

    ///
    /// Crops the matrix.
    /// 
    pub fn rect(&self, top_left: IntPoint<2>, bottom_right: IntPoint<2>) -> RectMatrix<T> {
        let cols = bottom_right.x() - top_left.x() + 1;
        let rows = bottom_right.y() - top_left.y() + 1;
        let mut mat = RectMatrix::zeros(rows as usize, cols as usize);
        for j in top_left.x().clone()..(bottom_right.x() + 1) {
            for i in top_left.y().clone()..(bottom_right.y() + 1) {
                mat.set_value((i - top_left.y()) as usize, (j - top_left.x()) as usize, self.value(i as usize, j as usize));
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
                res.data[(j, i)] = self.data[(i, j)].clone();
            }
        }
        res
    }

    ///
    /// Assigns a column.
    /// 
    pub fn assign_col(&mut self, col: usize, v: ColVector<T>) -> &RectMatrix<T> {
        if v.length() != self.rows() {
            panic!();
        }
        for i in 0..v.length() {
            self.data[(i, col)] = v.value(i);
        }
        self
    }

    ///
    /// Returns true iif the matrix is lower triangular.
    /// 
    pub fn is_lower_triangular(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 0..(self.rows() - 1) {
            for j in (i + 1)..self.cols() {
                if (self.data[(i, j)]) != T::zero() {
                    return false;
                }
            }
        }
        true
    }

    ///
    /// Returns true iif the matrix is upper triangular.
    /// 
    pub fn is_upper_triangular(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 1..(self.rows()) {
            for j in 0..(i) {
                if (self.data[(i, j)]) != T::zero() {
                    return false;
                }
            }
        }
        return true;
    }

    ///
    /// Returns true iif the matrix is square.
    /// 
    pub fn is_square(&self) -> bool {
        self.cols() == self.rows()
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
            data: Array2D::filled_with(T::zero(), rows, cols)
        }
    }

    ///
    /// Creates an identity matrix.
    ///
    pub fn identity(size: usize) -> Self {
        let mut zero = Self::zeros(size, size);
        for i in 0..size {
            for j in 0..size {
                zero.set_value(i, j, if i == j { T::one() } else { T::zero() });
            }
        }
        return zero;
    }

    // Private impl
    // ============
    ///
    /// Adds to another matrix for terms multiplied by a factor.
    /// 
    fn mult_add(&self, other: &RectMatrix<T>, fac: T) -> RectMatrix<T> {
        if self.size() != other.size() {
            panic!()
        }

        let mut output = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                output.data[(i, j)] += fac.clone()*other.value(i, j).clone();
            }
        }
        output
    }
}

impl RectMatrix<f64> {
    ///
    /// Rounds all the values in the matrix.
    /// 
    pub fn round(&mut self, decimals: i32) -> &RectMatrix<f64> {
        let ten = 10f64;
        let factor = ten.powi(decimals);
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[(i, j)] = (self.data[(i, j)]*factor).round()/factor;
            }
        }
        self
    }
}


///
/// Represents a row.
/// 
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
pub struct RowVector<T: MatElement> {
    pub matrix: RectMatrix<T>
}

impl<T: MatElement> RowVector<T> {
    ///
    /// Builds a RowVector from a vector.
    /// 
    pub fn from_vec(data: &[T]) -> RowVector<T> {
        RowVector {
            matrix: RectMatrix {
                data: Array2D::from_rows(&[data.to_vec()])
            }
        }
    }

    ///
    /// Creates a evenly spaced sequence of numbers.
    /// 
    pub fn evenly_spaced(a: &f64, b: &f64, count: &i64) -> RowVector<f64> {
        if count < &2i64 {
            let n: [f64; 0] = [];
            return RowVector::from_vec(&n);
        }
        let mut v: Vec<f64> = Vec::new();
        for i in 0i64..*count {
            v.push(a + (i as f64)*(b - a)/((*count - 1i64) as f64));
        }
        return RowVector::from_vec(&v);
    }

    ///
    /// Length of the row.
    /// 
    pub fn length(&self) -> usize {
        self.matrix.cols()
    }

    ///
    /// Value in row.
    /// 
    pub fn value(&self, j: usize) -> T {
        self.matrix.value(0, j).clone()
    }

    ///
    /// Clones data and returns a vector.
    /// 
    pub fn to_vec(&self) -> Vec<T> {
        self.matrix.data.as_rows()[0].clone()
    }
}

impl<T: MatElement> PartialEq for RowVector<T> {
    fn eq(&self, other: &Self) -> bool {
        other.matrix == self.matrix
    }
}

///
/// Represents a column.
/// 
#[derive(Debug)]
#[derive(Eq)]
pub struct ColVector<T: MatElement> {
    pub matrix: RectMatrix<T>
}

impl<T: MatElement> ColVector<T> {
    ///
    /// Builds a ColVector from a vector.
    /// 
    pub fn from_vec(data: &[T]) -> ColVector<T> {
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

    ///
    /// Height of the column.
    /// 
    pub fn length(&self) -> usize {
        self.height()
    }

    ///
    /// Value in row.
    /// 
    pub fn value(&self, i: usize) -> T {
        self.matrix.value(i, 0).clone()
    }
}

impl<T: MatElement> PartialEq for ColVector<T> {
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
        let m = RectMatrix::<f64>::identity(10);
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
        assert_eq!(m1.rect(
            IntPoint::<2>::point2d(1, 1),
            IntPoint::<2>::point2d(2, 2)),
            m2);
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
        let _m = RectMatrix::from_vec(&[
            vec![500f64, 6f64, 7f64],
            vec![1f64, 200f64, 3f64],
            vec![9f64, 8f64, 700000f64],
            vec![1f64, 1f64, 1f64]
        ]);
        //m.print();

        let _r = RowVector::from_vec(&[10f64, 200f64, 3000f64]);
        //r.matrix.print();

        let _c = ColVector::from_vec(&[10f64, 200f64, 3000f64]);
        //c.matrix.print();
    }

    #[test]
    fn test_assign_col() {
        let mut m = RectMatrix::from_vec(&[
            vec![5f64, 6f64, 7f64],
            vec![1f64, 2f64, 3f64],
            vec![9f64, 8f64, 7f64],
            vec![1f64, 1f64, 1f64]
        ]);
        m.assign_col(1, ColVector::from_vec(&vec![4f64, 4f64, 4f64, 4f64]));
        assert_eq!(m, RectMatrix::from_vec(&[
            vec![5f64, 4f64, 7f64],
            vec![1f64, 4f64, 3f64],
            vec![9f64, 4f64, 7f64],
            vec![1f64, 4f64, 1f64]
        ]));
    }

    #[test]
    fn test_round() {
        let mut m = RectMatrix::from_vec(&[
            vec![5.6666777f64, 6.7777f64, 7f64]
        ]);
        m.round(2);
        assert_eq!(m.value(0, 0), 5.67f64);
        assert_ne!(m.value(0, 0), 5.6666777f64);
        assert_eq!(m.value(0, 1), 6.78f64);
        assert_eq!(m.value(0, 2), 7.00f64);
    }

    #[test]
    fn test_low_upp_triangular() {
        let l = RectMatrix::from_vec(&[
            vec![1f64, 0f64, 0f64, 0f64],
            vec![1f64, 2f64, 0f64, 0f64],
            vec![1f64, 2f64, 3f64, 0f64],
            vec![1f64, 2f64, 3f64, 4f64]
        ]);
        assert_eq!(l.is_lower_triangular(), true);
        assert_eq!(l.is_upper_triangular(), false);
    }

    #[test]
    fn test_evenly_spaced() {
        let seq = RowVector::<f64>::evenly_spaced(&1f64, &4f64, &4i64);
        assert_eq!(seq.value(0), 1f64);
        assert_eq!(seq.value(1), 2f64);
        assert_eq!(seq.value(2), 3f64);
        assert_eq!(seq.value(3), 4f64);

        let seq = RowVector::<f64>::evenly_spaced(&-1f64, &-4f64, &4);
        assert_eq!(seq.value(0), -1f64);
        assert_eq!(seq.value(1), -2f64);
        assert_eq!(seq.value(2), -3f64);
        assert_eq!(seq.value(3), -4f64);

        let seq = RowVector::<f64>::evenly_spaced(&-4f64, &-1f64, &4);
        assert_eq!(seq.value(0), -4f64);
        assert_eq!(seq.value(1), -3f64);
        assert_eq!(seq.value(2), -2f64);
        assert_eq!(seq.value(3), -1f64);

        let seq = RowVector::<f64>::evenly_spaced(&-4.5f64, &-1.5f64, &5);
        assert_eq!(seq.value(0), -4.5f64);
        assert_eq!(seq.value(1), -3.75f64);
        assert_eq!(seq.value(2), -3f64);
        assert_eq!(seq.value(3), -2.25f64);
        assert_eq!(seq.value(4), -1.5f64);
    }
}
