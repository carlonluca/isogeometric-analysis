/**
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
use super::RectMatrix;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

///
/// Represents a point.
///
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Point<T: MatElement> {
    pub x: T,
    pub y: T,
    pub z: T,
    dim: u8
}

impl<T: MatElement> Point<T> {
    ///
    /// Creates a point on a straight line.
    /// 
    pub fn point1d(x: T) -> Point<T> {
        Point {
            x: x,
            y: T::zero(),
            z: T::zero(),
            dim: 1
        }
    }

    ///
    /// Creates a point in the 2D space.
    /// 
    pub fn point2d(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y,
            z: T::zero(),
            dim: 2
        }
    }

    ///
    /// Creates a point in the 3D space.
    /// 
    pub fn point3d(x: T, y: T, z: T) -> Point<T> {
        Point {
            x: x,
            y: y,
            z: z,
            dim: 3
        }
    }

    ///
    /// Builds a point from a matrix.
    /// 
    pub fn from_matrix(m: RectMatrix<T>) -> Point<T> {
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
    }

    ///
    /// Returns the x coord if it exists or zero.
    ///
    pub fn x(&self) -> T {
        self.x.clone()
    }

    ///
    /// Returns the y coord if it exists or zero.
    ///
    pub fn y(&self) -> T {
        self.y.clone()
    }

    ///
    /// Returns the z coord if it exists or zero.
    ///
    pub fn z(&self) -> T {
        self.z.clone()
    }

    ///
    /// Returns the dimension of the space in which the point is included.
    /// 
    pub fn dim(&self) -> u8 {
        self.dim
    }

    ///
    /// Returns the idx-th element or zero.
    ///
    pub fn value(&self, idx: u8) -> T {
        return if self.dim > idx {
            match idx {
                0 => self.x(),
                1 => self.y(),
                2 => self.z(),
                _ => T::zero()
            }
        }
        else {
            T::zero()
        }
    }
}

impl<T: MatElement> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: MatElement> Add for Point<T> {
    type Output = Self;

    ///
    /// Adds another matrix.
    ///
    fn add(self, other: Self) -> Self {
        if self.dim != other.dim { panic!() }
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            dim: self.dim
        }
    }
}

impl<T: MatElement> Sub for Point<T> {
    type Output = Self;

    ///
    /// Subtracts another matrix.
    /// 
    fn sub(self, other: Self) -> Self::Output {
        if self.dim != other.dim { panic!() }
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            dim: self.dim
        }
    }
}

impl<T: MatElement> Mul<T> for Point<T> {
    type Output = Self;

    ///
    /// Multiplication by a scalar.
    /// 
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x*scalar.clone(),
            y: self.y*scalar.clone(),
            z: self.z*scalar.clone(),
            dim: self.dim
        }
    }
}

///
/// Represents a point with integer coords.
/// 
pub type IntPoint = Point<i32>;

///
/// Represents a point in ℝ^3.
/// 
pub type RealPoint = Point<f64>;

#[cfg(test)]
mod tests {
    use crate::core::Point;

    #[test]
    fn test_eq() {
        assert_eq!(Point::point2d(6, 5), Point::point2d(6, 5));
        assert_eq!(Point::point2d(56.7, 12.3), Point::point2d(56.7, 12.3));
    }
}
