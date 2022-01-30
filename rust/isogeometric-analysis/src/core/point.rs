/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.11.17
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

use super::MatElement;
use crate::core::RowVector;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use num::traits::{pow, Float};
use float_cmp::ApproxEq;
use unroll::unroll_for_loops;

///
/// Represents a point.
///
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Point<T: MatElement, const SIZE: usize> {
    data: RowVector<T, SIZE>
}

impl<T: MatElement, const SIZE: usize> Display for Point<T, SIZE> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.data)
    }
}

///
/// Represents a point in ℝ^3.
/// 
pub type RealPoint<const SIZE: usize> = Point<f64, SIZE>;

///
/// Represents a point with integer coords.
/// 
pub type IntPoint<const SIZE: usize> = Point<i32, SIZE>;

pub type RealPoint1d = RealPoint<1>;
pub type RealPoint2d = RealPoint<2>;
pub type RealPoint3d = RealPoint<3>;
pub type RealPoint4d = RealPoint<4>;

impl<T: MatElement, const SIZE: usize> Point<T, SIZE> {
    ///
    /// Creates a point on a straight line.
    ///
    #[inline(always)]
    pub fn point1d(x: T) -> Point<T, 1> {
        Point {
            data: RowVector::row_from_vec(&[x])
        }
    }

    ///
    /// Creates a point in the 2D space.
    ///
    #[inline(always)]
    pub fn point2d(x: T, y: T) -> Point<T, 2> {
        Point {
            data: RowVector::row_from_vec(&[x, y])
        }
    }

    ///
    /// Creates a point in the 3D space.
    /// 
    #[inline(always)]
    pub fn point3d(x: T, y: T, z: T) -> Point<T, 3> {
        Point {
            data: RowVector::row_from_vec(&[x, y, z])
        }
    }

    #[inline(always)]
    pub fn origin() -> Point<T, SIZE> {
        Point {
            data: RowVector::zeros()
        }
    }

    ///
    /// Returns the dimension of the space containing this point.
    ///
    #[inline(always)]
    pub fn dim(&self) -> usize {
        self.data.cols()
    }

    ///
    /// Sets all values to zero.
    ///
    #[inline(always)]
    pub fn reset(&mut self) {
        self.data.reset()
    }

    ///
    /// Builds a point from a matrix.
    /// 
    /*pub fn from_matrix(m: RectMatrix<T>) -> Point<T> {
        let x = m.value(0, 0);
        let y;
        let z;
        if m.cols() > 1 { y = m.value(0, 1); }
        else { y = T::zero(); }
        if m.cols() > 2 { z = m.value(0, 2); }
        else { z = T::zero() }
        Point {
            x: x,
            y: y,
            z: z,
            dim: m.cols() as u8
        }
    }*/

    ///
    /// Returns the idx-th element or zero.
    ///
    #[inline(always)]
    pub fn value(&self, idx: usize) -> T {
        return if self.dim() > idx as usize {
            self.data.value(0, idx)
        }
        else {
            T::zero()
        }
    }

    ///
    /// Sets the coordinate of the point.
    ///
    #[inline(always)]
    pub fn set_value(&mut self, idx: usize, val: T) -> &mut Self {
        if self.dim() > idx {
            self.data.set_value(0, idx, val);
        }
        self
    }

    ///
    /// Clones this point to another one.
    ///
    #[inline(always)]
    pub fn clone_to(&self, dest: &mut Point<T, SIZE>) {
        dest.data = self.data.clone();
    }
}

impl<T: MatElement, const SIZE: usize> Point<T, SIZE> {
    ///
    /// Converts this point to a corresponding point in homogeneous coordinates on the plane w.
    ///
    #[inline(always)]
    #[unroll_for_loops]
    pub fn to_homogeneous<const HOMSIZE: usize>(&self, w: T) -> Point<T, HOMSIZE> {
        let mut res = Point::<T, HOMSIZE>::origin();
        if res.dim() != self.dim() + 1 {
            panic!();
        }
        for i in 0..SIZE {
            res.set_value(i, self.value(i)*w);
        }
        res.set_value(SIZE, w);
        return res;
    }

    ///
    /// Converts this point to a corresponding point in cartesian coordinates.
    ///
    #[inline(always)]
    #[unroll_for_loops]
    pub fn to_cartesian<const CARTSIZE: usize>(&self) -> Point<T, CARTSIZE> {
        if self.value(self.dim() - 1) == T::zero() {
            panic!("Invalid plane")
        }
        let mut res = Point::<T, CARTSIZE>::origin();
        for i in 0..CARTSIZE {
            res.set_value(i, self.value(i)/self.value(SIZE - 1));
        }
        res
    }
}

impl<T: MatElement> Point<T, 1> {
    #[inline(always)]
    pub fn x(&self) -> T { self.value(0) }
    #[inline(always)]
    pub fn set_x(&mut self, x: T) { self.set_value(0, x); }
}

impl<T: MatElement> Point<T, 2> {
    #[inline(always)]
    pub fn x(&self) -> T { self.value(0) }
    #[inline(always)]
    pub fn set_x(&mut self, x: T) { self.set_value(0, x); }
    #[inline(always)]
    pub fn y(&self) -> T { self.value(1) }
    #[inline(always)]
    pub fn set_y(&mut self, y: T) { self.set_value(1, y); }
}

impl<T: MatElement> Point<T, 3> {
    #[inline(always)]
    pub fn x(&self) -> T { self.value(0) }
    #[inline(always)]
    pub fn set_x(&mut self, x: T) { self.set_value(0, x); }
    #[inline(always)]
    pub fn y(&self) -> T { self.value(1) }
    #[inline(always)]
    pub fn set_y(&mut self, y: T) { self.set_value(1, y); }
    #[inline(always)]
    pub fn z(&self) -> T { self.value(2) }
    #[inline(always)]
    pub fn set_z(&mut self, z: T) { self.set_value(2, z); }
}

impl<T: MatElement, const SIZE: usize> PartialEq for Point<T, SIZE> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: MatElement, const SIZE: usize> Add for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        if self.dim() != other.dim() { panic!() }
        let data = self.data + other.data;
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> AddAssign for Point<T, SIZE> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Point<T, SIZE>) {
        if self.dim() != rhs.dim() { panic!() }
        self.data += rhs.data;
    }
}

impl<T: MatElement, const SIZE: usize> Sub for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    ///
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        if self.dim() != other.dim() { panic!() }
        let data = self.data - other.data;
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> SubAssign for Point<T, SIZE> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Point<T, SIZE>) {
        if self.dim() != rhs.dim() { panic!() }
        self.data -= rhs.data
    }
}

impl<T: MatElement, const SIZE: usize> Mul<T> for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    ///
    #[inline(always)]
    fn mul(self, scalar: T) -> Self::Output {
        let data = self.data*scalar;
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> MulAssign<T> for Point<T, SIZE> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        self.data *= rhs;
    }
}

impl<M: Copy + Default, F: MatElement + ApproxEq<Margin=M>, const SIZE: usize> ApproxEq for Point<F, SIZE> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        self.data.approx_eq(other.data, margin)
    }
}

impl<T: Float + MatElement, const SIZE: usize> Point<T, SIZE> {
    ///
    /// Returns the euclidean disance between the two points d(self, p2).
    ///
    #[inline(always)]
    #[unroll_for_loops]
    pub fn dist(&self, p2: &Point<T, SIZE>) -> T {
        let mut sum = T::zero();
        for i in 0..SIZE {
            sum += pow::<T>(self.value(i) - p2.value(i), 2);
        }
        sum.sqrt()
    }
}

#[inline(always)]
pub fn p1(x: f64) -> Point<f64, 1> {
    Point { data: RowVector::row_from_vec(&[x]) }
}

#[inline(always)]
pub fn p2(x: f64, y: f64) -> Point<f64, 2> {
    Point { data: RowVector::row_from_vec(&[x, y]) }
}

#[inline(always)]
pub fn p3(x: f64, y: f64, z: f64) -> Point<f64, 3> {
    Point { data: RowVector::row_from_vec(&[x, y, z]) }
}

#[cfg(test)]
mod tests {
    use crate::core::{RealPoint, RealPoint1d, RealPoint2d, RealPoint3d};
    use crate::core::IntPoint;
    use float_cmp::approx_eq;

    #[test]
    fn test_eq() {
        assert_eq!(IntPoint::<2>::point2d(6, 5), IntPoint::<2>::point2d(6, 5));
        assert_eq!(RealPoint::<2>::point2d(56.7, 12.3), RealPoint::<2>::point2d(56.7, 12.3));
        assert_eq!(RealPoint2d::point2d(1.0, 2.0).to_homogeneous(1.1), RealPoint3d::point3d(1.1, 2.2, 1.1));
        assert_eq!(RealPoint2d::point2d(1.0, 2.0).to_homogeneous::<3>(1.1).to_cartesian(), RealPoint2d::point2d(1.0, 2.0));
        assert!(approx_eq!(RealPoint2d, RealPoint2d::point2d(2.0/2.0, 2.0), RealPoint2d::point2d(1.0, 2.0)));
        assert!(approx_eq!(f64, RealPoint1d::point1d(5.).dist(&RealPoint1d::point1d(3.)), 2.));
        assert!(approx_eq!(f64, RealPoint1d::point1d(3.).dist(&RealPoint1d::point1d(5.)), 2.));
        assert!(approx_eq!(f64, RealPoint3d::point3d(7., 4., 3.).dist(&RealPoint3d::point3d(17., 6., 2.)), 105f64.sqrt()));
    }
}
