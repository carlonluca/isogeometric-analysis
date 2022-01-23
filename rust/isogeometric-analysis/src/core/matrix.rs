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

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::{MulAssign, AddAssign, SubAssign};
use std::clone::Clone;
use std::fmt::Display;
use std::fmt::Debug;
use super::size::Size;
use num::traits::{Signed};
use float_cmp::ApproxEq;
use log;

pub trait MatElement: Signed + Clone + MulAssign + AddAssign + SubAssign + PartialOrd + Display + Copy + Debug {}
impl<T> MatElement for T where T: Signed + Clone + MulAssign + AddAssign + SubAssign + PartialOrd + Display + Copy + Debug { }

#[derive(Debug)]
#[derive(Eq)]
#[derive(Copy)]
pub struct RectMatrix<T: MatElement, const R: usize, const C: usize> {
    data: [[T; C]; R]
}

pub type RealRectMatrix<const R: usize, const C: usize> = RectMatrix<f64, R, C>;

impl<T: MatElement, const R: usize, const C: usize> PartialEq for RectMatrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: Copy + Default, F: MatElement + ApproxEq<Margin=M>, const R: usize, const C: usize> ApproxEq for RectMatrix<F, R, C> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if !self.data[i][j].approx_eq(other.data[i][j], margin) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T: MatElement, const R: usize, const C: usize> Add for RectMatrix<T, R, C> {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        return self.mult_add(&other, T::one());
    }
}

impl<T: MatElement, const R: usize, const C: usize> AddAssign for RectMatrix<T, R, C> {
    fn add_assign(&mut self, rhs: RectMatrix<T, R, C>) {
        if R != rhs.rows() || C != rhs.cols() { panic!() }
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}

impl<T: MatElement, const R: usize, const C: usize> Sub for RectMatrix<T, R, C> {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        return self.mult_add(&other, T::one().neg());
    }
}

impl<T: MatElement, const R: usize, const C: usize> SubAssign for RectMatrix<T, R, C> {
    fn sub_assign(&mut self, rhs: RectMatrix<T, R, C>) {
        if R != rhs.rows() || C != rhs.cols() { panic!() }
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}

impl<T: MatElement, const R: usize, const C: usize> Mul<T> for RectMatrix<T, R, C> {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    /// 
    fn mul(self, scalar: T) -> Self::Output {
        let mut ret = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                ret.data[i][j] *= scalar;
            }
        }
        return ret;
    }
}

impl<T: MatElement, const R: usize, const C: usize> MulAssign<T> for RectMatrix<T, R, C> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[i][j] *= rhs;
            }
        }
    }
}

impl<T: MatElement, const R1: usize, const C1: usize, const C2: usize> Mul<RectMatrix<T, C1, C2>> for RectMatrix<T, R1, C1> {
    type Output = RectMatrix<T, R1, C2>;

    ///
    /// Multiplication by another matrix.
    /// 
    fn mul(self, other: RectMatrix<T, C1, C2>) -> RectMatrix<T, R1, C2> {
        if self.cols() != other.rows() {
            panic!();
        }

        let mut res = RectMatrix::<T, R1, C2>::zeros();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let mut e = T::zero();
                for p in 0..self.cols() {
                    e += self.data[i][p].clone()*other.data[p][j].clone();
                }
                res.data[i][j] = e;
            }
        }

        return res;
    }
}

impl<T: MatElement, const R: usize, const C: usize> Clone for RectMatrix<T, R, C> {
    ///
    /// Clones this matrix.
    /// 
    fn clone(&self) -> RectMatrix<T, R, C> {
        RectMatrix {
            data: self.data.clone()
        }
    }
}

impl<T: MatElement, const R: usize, const C: usize> RectMatrix<T, R, C> {
    ///
    /// Create a new vector from data.
    /// 
    pub fn mat_from_vec(data: [[T; C]; R]) -> RectMatrix<T, R, C> {
        RectMatrix::<T, R, C> {
            data: data
        }
    }

    ///
    /// Returns the value.
    /// 
    pub fn value(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    ///
    /// Sets a value.
    /// 
    pub fn set_value(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value
    }

    ///
    /// Returns the number of rows.
    /// 
    pub fn rows(&self) -> usize {
        R
    }

    ///
    /// Returns the i-th row.
    /// 
    pub fn row(&self, i: usize) -> RowVector<T, C> {
        RowVector::row_from_vec(&self.data[i])
    }

    ///
    /// Returns the i-th row to an array.
    /// 
    pub fn row_to_vec(&self, i: usize) -> [T; C] {
        self.data[i].clone()
    }

    ///
    /// Returns the j-th column as an array.
    ///
    pub fn col_to_vec(&self, j: usize) -> [T; R] {
        let mut col = [T::zero(); R];
        for i in 0..R {
            col[i] = self.data[i][j];
        }
        col
    }

    ///
    /// Returns the number of columns.
    /// 
    pub fn cols(&self) -> usize {
        C
    }

    ///
    /// Returns the i-th column.
    /// 
    pub fn col(&self, j: usize) -> ColVector<T, R> {
        // TODO: optimmize
        let mut data = [T::zero(); R];
        for i in 0..R {
            data[i] = self.data[i][j];
        }
        ColVector::<T, R>::col_from_vec(&data)
    }
    
    ///
    /// Returns the max value in column.
    /// 
    pub fn max_col(&self, j: usize) -> T {
        let mut max: Option<T> = None;
        for i in 0..self.rows() {
            match &max {
                None => max = Some(self.data[i][j]),
                Some(v) => if v < &self.data[i][j] { max = Some(self.data[i][j]); }
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
                let this_item = self.data[i][j].to_string();
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
    pub fn rect<const R2: usize, const C2: usize>(&self, top: usize, left: usize) -> RectMatrix<T, R2, C2> {
        let mut mat = RectMatrix::<T, R2, C2>::zeros();
        for j in left..(left + C2) {
            for i in top..(top + R2) {
                mat.data[i - top][j - left] = self.data[i][j];
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
    pub fn transposed(&self) -> RectMatrix<T, C, R> {
        let mut res = RectMatrix::<T, C, R>::zeros();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }

    ///
    /// Assigns a column.
    /// 
    pub fn assign_col(&mut self, col: usize, v: ColVector<T, R>) -> &RectMatrix<T, R, C> {
        if v.rows() != self.rows() {
            panic!();
        }
        for i in 0..v.rows() {
            self.data[i][col] = v.data[i][0];
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
                if self.data[i][j] != T::zero() {
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
                if self.data[i][j] != T::zero() {
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
    pub fn zeros() -> Self {
        RectMatrix::<T, R, C> {
            data: [[T::zero(); C]; R]
        }
    }

    ///
    /// Reset all values to zero.
    /// 
    pub fn reset(&mut self) {
        self.data = [[T::zero(); C]; R];
    }

    ///
    /// Creates an identity matrix.
    ///
    pub fn identity() -> Self {
        if R != C {
            panic!();
        }
        let mut zero = Self::zeros();
        for i in 0..R {
            for j in 0..C {
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
    fn mult_add(&self, other: &RectMatrix<T, R, C>, fac: T) -> RectMatrix<T, R, C> {
        if self.size() != other.size() {
            panic!()
        }

        let mut output = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                output.data[i][j] += fac.clone()*other.value(i, j);
            }
        }
        output
    }
}

impl<const R: usize, const C: usize> RectMatrix<f64, R, C> {
    ///
    /// Rounds all the values in the matrix.
    /// 
    pub fn round(&mut self, decimals: i32) -> &RectMatrix<f64, R, C> {
        let ten = 10f64;
        let factor = ten.powi(decimals);
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[i][j] = (self.data[i][j]*factor).round()/factor;
            }
        }
        self
    }
}


///
/// Represents a row.
/// 
pub type RowVector<T, const L: usize> = RectMatrix<T, 1, L>;

impl<T: MatElement, const L: usize> RowVector<T, L> {
    ///
    /// Builds a RowVector from a vector.
    /// 
    pub fn row_from_vec(data: &[T]) -> RowVector<T, L> {
        let mut mat = [[T::zero(); L]; 1];
        for i in 0..data.len() {
            mat[0][i] = data[i];
        }
        RowVector {
            data: mat
        }
    }

    ///
    /// Creates a evenly spaced sequence of numbers.
    /// 
    pub fn evenly_spaced(a: &f64, b: &f64) -> RowVector<f64, L> {
        if L < 2 {
            let n: [f64; 0] = [];
            return RowVector::row_from_vec(&n);
        }
        let mut v: Vec<f64> = Vec::new();
        for i in 0..L {
            v.push(a + (i as f64)*(b - a)/((L - 1) as f64));
        }
        return RowVector::row_from_vec(&v);
    }
}

///
/// Represents a column.
///
pub type ColVector<T, const L: usize> = RectMatrix<T, L, 1>;

impl<T: MatElement, const L: usize> ColVector<T, L> {
    ///
    /// Builds a ColVector from a vector.
    /// 
    pub fn col_from_vec(data: &[T]) -> ColVector<T, L> {
        let mut mat = [[T::zero(); 1]; L];
        for i in 0..data.len() {
            mat[i][0] = data[i];
        }
        ColVector {
            data: mat
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::RectMatrix;
    use crate::core::RealRectMatrix;
    use crate::core::RowVector;
    use crate::core::ColVector;

    #[test]
    fn test_get() {
        let m = RealRectMatrix::<2, 2>::mat_from_vec([
            [1f64, 2f64],
            [3f64, 4f64]
        ]);
        assert_eq!(m.value(0, 0), 1f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
        assert_eq!(m.row(0), RowVector::row_from_vec(&[1f64, 2f64]));
        assert_eq!(m.row(0), RowVector::row_from_vec(&[1f64, 2f64]));
        assert_eq!(m.row(1), RowVector::row_from_vec(&[3f64, 4f64]));
        assert_eq!(m.col(0), ColVector::col_from_vec(&[1f64, 3f64]));
        assert_eq!(m.col(1), ColVector::col_from_vec(&[2f64, 4f64]));
    }

    #[test]
    fn test_set() {
        let mut m = RealRectMatrix::<2, 2>::mat_from_vec([
            [1f64, 2f64],
            [3f64, 4f64]
        ]);

        m.set_value(0, 0, 15f64);

        assert_eq!(m.value(0, 0), 15f64);
        assert_eq!(m.value(0, 1), 2f64);
        assert_eq!(m.value(1, 0), 3f64);
        assert_eq!(m.value(1, 1), 4f64);
    }

    #[test]
    fn test_size() {
        let m = RealRectMatrix::<3, 2>::mat_from_vec([
            [1f64, 2f64],
            [3f64, 4f64],
            [5f64, 6f64]
        ]);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_add() {
        let m1 = RealRectMatrix::<1, 3>::mat_from_vec([
            [1f64, 2f64, 3f64]
        ]);
        let m2 = RealRectMatrix::<1, 3>::mat_from_vec([
            [1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, 1f64);
        let m4 = m1.clone() + m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, RealRectMatrix::<1, 3>::mat_from_vec([
            [2f64, 3f64, 4f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_sub() {
        let m1 = RealRectMatrix::<1, 3>::mat_from_vec([
            [1f64, 2f64, 3f64]
        ]);
        let m2 = RealRectMatrix::<1, 3>::mat_from_vec([
            [1f64, 1f64, 1f64]
        ]);
        let m3 = m1.mult_add(&m2, -1f64);
        let m4 = m1.clone() - m2.clone();
        assert_ne!(m2, m1);
        assert_eq!(m3, RealRectMatrix::<1, 3>::mat_from_vec([
            [0f64, 1f64, 2f64]
        ]));
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_zeros() {
        let m = RealRectMatrix::<5, 5>::zeros();
        assert_eq!(m, RectMatrix::mat_from_vec([
            [0f64, 0f64, 0f64, 0f64, 0f64],
            [0f64, 0f64, 0f64, 0f64, 0f64],
            [0f64, 0f64, 0f64, 0f64, 0f64],
            [0f64, 0f64, 0f64, 0f64, 0f64],
            [0f64, 0f64, 0f64, 0f64, 0f64]
        ]));
        assert_eq!(m.value(0, 0), 0f64);
        assert_eq!(m.value(1, 1), 0f64);
        assert_eq!(m.value(2, 2), 0f64);
        assert_eq!(m.value(3, 3), 0f64);
        assert_eq!(m.value(4, 4), 0f64);
    }

    #[test]
    fn test_identity() {
        let m = RealRectMatrix::<10, 10>::identity();
        for i in 0..(m.rows() - 1) {
            for j in 0..(m.cols() - 1) {
                assert_eq!(m.value(i, j), if i == j { 1f64 } else { 0f64 });
            }
        }
    }
    
    #[test]
    fn test_mult_scalar_1() {
        let m = RealRectMatrix::<3, 3>::mat_from_vec([
            [1f64, 2f64, 3f64],
            [4f64, 5f64, 6f64],
            [7f64, 8f64, 9f64]
        ]);
        let m2 = m*5f64;
        assert_eq!(m2, RealRectMatrix::<3, 3>::mat_from_vec([
            [5f64, 10f64, 15f64],
            [20f64, 25f64, 30f64],
            [35f64, 40f64, 45f64]
        ]));
    }

    #[test]
    fn test_mult_scalar_2() {
        let m = RealRectMatrix::<3, 3>::mat_from_vec([
            [5f64, 6f64, 7f64],
            [1f64, 2f64, 3f64],
            [9f64, 8f64, 7f64]
        ]);
        assert_eq!(m.clone()*9f64, RealRectMatrix::<3, 3>::mat_from_vec([
            [5f64*9f64, 6f64*9f64, 7f64*9f64],
            [1f64*9f64, 2f64*9f64, 3f64*9f64],
            [9f64*9f64, 8f64*9f64, 7f64*9f64]
        ]));
    }

    #[test]
    fn test_mult_scalar_3() {
        let m = RealRectMatrix::<3, 3>::mat_from_vec([
            [5f64*9f64, 6f64*9f64, 7f64*9f64],
            [1f64*9f64, 2f64*9f64, 3f64*9f64],
            [9f64*9f64, 8f64*9f64, 7f64*9f64]
        ]);
        assert_eq!(m*0f64, RealRectMatrix::<3, 3>::zeros());
    }

    #[test]
    fn test_mult_matrix() {
        let m1 = RealRectMatrix::<3, 3>::mat_from_vec([
            [5f64, 6f64, 7f64],
            [1f64, 2f64, 3f64],
            [9f64, 8f64, 7f64]
        ]);
        let m2 = RealRectMatrix::<3, 3>::mat_from_vec([
            [1f64, 2f64, 3f64],
            [4f64, 5f64, 6f64],
            [7f64, 8f64, 9f64]
        ]);
        let m3 = RealRectMatrix::<3, 3>::identity();
        let m4 = RealRectMatrix::<3, 3>::zeros();
        assert_eq!(m1.clone()*m1.clone(), RealRectMatrix::<3, 3>::mat_from_vec([
            [94f64, 98f64, 102f64],
            [34f64, 34f64, 34f64],
            [116f64, 126f64, 136f64]
        ]));
        assert_eq!(m1.clone()*m2.clone(), RealRectMatrix::<3, 3>::mat_from_vec([
            [78f64, 96f64, 114f64],
            [30f64, 36f64, 42f64],
            [90f64, 114f64, 138f64]
        ]));
        assert_eq!(m1.clone()*m3.clone(), m1);
        assert_eq!(m2.clone()*m3.clone(), m2);
        assert_eq!(m1.clone()*m4.clone(), m4);
    }

    #[test]
    fn test_transpose() {
        let m = RealRectMatrix::<3, 3>::mat_from_vec([
            [1f64, 2f64, 3f64],
            [4f64, 5f64, 6f64],
            [7f64, 8f64, 9f64]
        ]);
        assert_eq!(m, RealRectMatrix::<3, 3>::mat_from_vec([
            [1f64, 4f64, 7f64],
            [2f64, 5f64, 8f64],
            [3f64, 6f64, 9f64]
        ]).transposed());
    }

    #[test]
    fn test_vec() {
        let r = RowVector::<f64, 2>::row_from_vec(&[1f64, 2f64]);
        assert_eq!(r.cols(), 2);
        assert_eq!(r.is_row(), true);
        assert_eq!(r.transposed().is_col(), true);

        let c = ColVector::<f64, 2>::col_from_vec(&[2f64, 4f64]);
        assert_eq!(c.rows(), 2);
        assert_eq!(c.is_col(), true);
        assert_eq!(c.transposed().is_row(), true);
    }

    #[test]
    fn test_rect() {
        let m1 = RealRectMatrix::<4, 3>::mat_from_vec([
            [5f64, 6f64, 7f64],
            [1f64, 2f64, 3f64],
            [9f64, 8f64, 7f64],
            [1f64, 1f64, 1f64]
        ]);
        let m2 = RealRectMatrix::<2, 2>::mat_from_vec([
            [2f64, 3f64],
            [8f64, 7f64]
        ]);
        assert_eq!(m1.rect::<2, 2>(1, 1), m2);
    }

    #[test]
    fn test_max() {
        let m = RealRectMatrix::<4, 3>::mat_from_vec([
            [5f64, 6f64, 7f64],
            [1f64, 2f64, 3f64],
            [9f64, 8f64, 7f64],
            [1f64, 1f64, 1f64]
        ]);
        assert_eq!(m.max_col(0), 9f64);
        assert_eq!(m.max_col(1), 8f64);
        assert_eq!(m.max_col(2), 7f64);
    }

    #[test]
    fn test_print() {
        let _m = RealRectMatrix::<4, 3>::mat_from_vec([
            [500f64, 6f64, 7f64],
            [1f64, 200f64, 3f64],
            [9f64, 8f64, 700000f64],
            [1f64, 1f64, 1f64]
        ]);
        //m.print();

        let _r = RowVector::<f64, 3>::row_from_vec(&[10f64, 200f64, 3000f64]);
        //r.matrix.print();

        let _c = ColVector::<f64, 3>::col_from_vec(&[10f64, 200f64, 3000f64]);
        //c.matrix.print();
    }

    #[test]
    fn test_assign_col() {
        let mut m = RealRectMatrix::<4, 3>::mat_from_vec([
            [5f64, 6f64, 7f64],
            [1f64, 2f64, 3f64],
            [9f64, 8f64, 7f64],
            [1f64, 1f64, 1f64]
        ]);
        m.assign_col(1, ColVector::col_from_vec(&[4f64, 4f64, 4f64, 4f64]));
        assert_eq!(m, RealRectMatrix::<4, 3>::mat_from_vec([
            [5f64, 4f64, 7f64],
            [1f64, 4f64, 3f64],
            [9f64, 4f64, 7f64],
            [1f64, 4f64, 1f64]
        ]));
    }

    #[test]
    fn test_round() {
        let mut m = RealRectMatrix::<1, 3>::mat_from_vec([
            [5.6666777f64, 6.7777f64, 7f64]
        ]);
        m.round(2);
        assert_eq!(m.value(0, 0), 5.67f64);
        assert_ne!(m.value(0, 0), 5.6666777f64);
        assert_eq!(m.value(0, 1), 6.78f64);
        assert_eq!(m.value(0, 2), 7.00f64);
    }

    #[test]
    fn test_low_upp_triangular() {
        let l = RealRectMatrix::<4, 4>::mat_from_vec([
            [1f64, 0f64, 0f64, 0f64],
            [1f64, 2f64, 0f64, 0f64],
            [1f64, 2f64, 3f64, 0f64],
            [1f64, 2f64, 3f64, 4f64]
        ]);
        assert_eq!(l.is_lower_triangular(), true);
        assert_eq!(l.is_upper_triangular(), false);
    }

    #[test]
    fn test_evenly_spaced() {
        let seq = RowVector::<f64, 4>::evenly_spaced(&1f64, &4f64);
        assert_eq!(seq.value(0, 0), 1f64);
        assert_eq!(seq.value(0, 1), 2f64);
        assert_eq!(seq.value(0, 2), 3f64);
        assert_eq!(seq.value(0, 3), 4f64);

        let seq = RowVector::<f64, 4>::evenly_spaced(&-1f64, &-4f64);
        assert_eq!(seq.value(0, 0), -1f64);
        assert_eq!(seq.value(0, 1), -2f64);
        assert_eq!(seq.value(0, 2), -3f64);
        assert_eq!(seq.value(0, 3), -4f64);

        let seq = RowVector::<f64, 4>::evenly_spaced(&-4f64, &-1f64);
        assert_eq!(seq.value(0, 0), -4f64);
        assert_eq!(seq.value(0, 1), -3f64);
        assert_eq!(seq.value(0, 2), -2f64);
        assert_eq!(seq.value(0, 3), -1f64);

        let seq = RowVector::<f64, 5>::evenly_spaced(&-4.5f64, &-1.5f64);
        assert_eq!(seq.value(0, 0), -4.5f64);
        assert_eq!(seq.value(0, 1), -3.75f64);
        assert_eq!(seq.value(0, 2), -3f64);
        assert_eq!(seq.value(0, 3), -2.25f64);
        assert_eq!(seq.value(0, 4), -1.5f64);
    }
}
