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
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};

///
/// Represents a point.
///
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Point<T: MatElement, const SIZE: usize> {
    data: [T; SIZE]
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

impl<T: MatElement, const SIZE: usize> Point<T, SIZE> {
    ///
    /// Creates a point on a straight line.
    /// 
    pub fn point1d(x: T) -> Point<T, 1> {
        Point {
            data: [x]
        }
    }

    ///
    /// Creates a point in the 2D space.
    /// 
    pub fn point2d(x: T, y: T) -> Point<T, 2> {
        Point {
            data: [x, y]
        }
    }

    ///
    /// Creates a point in the 3D space.
    /// 
    pub fn point3d(x: T, y: T, z: T) -> Point<T, 3> {
        Point {
            data: [x, y, z]
        }
    }

    pub fn origin() -> Point<T, SIZE> {
        Point {
            data: [T::zero(); SIZE]
        }
    }

    ///
    /// Returns the dimension of the space containing this point.
    /// 
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    ///
    /// Sets all values to zero.
    /// 
    pub fn reset(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = T::zero();
        }
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
    pub fn value(&self, idx: u8) -> T {
        return if self.dim() > idx as usize {
            match idx {
                0 => self.data[0],
                1 => self.data[1],
                2 => self.data[2],
                _ => T::zero()
            }
        }
        else {
            T::zero()
        }
    }
}

impl<T: MatElement> Point<T, 1> {
    pub fn x(&self) -> T { self.data[0] }
    pub fn set_x(&mut self, x: T) { self.data[0] = x; }
}

impl<T: MatElement> Point<T, 2> {
    pub fn x(&self) -> T { self.data[0] }
    pub fn set_x(&mut self, x: T) { self.data[0] = x; }
    pub fn y(&self) -> T { self.data[1] }
    pub fn set_y(&mut self, y: T) { self.data[1] = y; }
}

impl<T: MatElement> Point<T, 3> {
    pub fn x(&self) -> T { self.data[0] }
    pub fn set_x(&mut self, x: T) { self.data[0] = x; }
    pub fn y(&self) -> T { self.data[1] }
    pub fn set_y(&mut self, y: T) { self.data[1] = y; }
    pub fn z(&self) -> T { self.data[2] }
    pub fn set_z(&mut self, z: T) { self.data[2] = z; }
}

impl<T: MatElement, const SIZE: usize> PartialEq for Point<T, SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: MatElement, const SIZE: usize> Add for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        if self.dim() != other.dim() { panic!() }
        let mut data = self.data.clone();
        for i in 0..(data.len()) {
            data[i] += other.data[i];
        }
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> AddAssign for Point<T, SIZE> {
    fn add_assign(&mut self, rhs: Point<T, SIZE>) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs.data[i];
        }
    }
}

impl<T: MatElement, const SIZE: usize> Sub for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        if self.dim() != other.dim() { panic!() }
        let mut data = self.data.clone();
        for i in 0..(data.len()) {
            data[i] -= other.data[i];
        }
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> SubAssign for Point<T, SIZE> {
    fn sub_assign(&mut self, rhs: Point<T, SIZE>) {
        for i in 0..self.data.len() {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: MatElement, const SIZE: usize> Mul<T> for Point<T, SIZE> {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    /// 
    fn mul(self, scalar: T) -> Self::Output {
        let mut data = self.data.clone();
        for i in 0..data.len() {
            data[i] *= scalar;
        }
        Self {
            data: data
        }
    }
}

impl<T: MatElement, const SIZE: usize> MulAssign<T> for Point<T, SIZE> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::RealPoint;
    use crate::core::IntPoint;

    #[test]
    fn test_eq() {
        assert_eq!(IntPoint::<2>::point2d(6, 5), IntPoint::<2>::point2d(6, 5));
        assert_eq!(RealPoint::<2>::point2d(56.7, 12.3), RealPoint::<2>::point2d(56.7, 12.3));
    }
}
