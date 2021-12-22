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

use super::RowVector;
use super::MatElement;
use super::RectMatrix;

///
/// Represents a point.
///
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
pub struct Point<T: MatElement> {
    pub vector: RowVector<T>
}

impl<T: MatElement> Point<T> {
    ///
    /// Creates a point on a straight line.
    /// 
    pub fn point1d(x: T) -> Point<T> {
        Point {
            vector: RowVector::from_vec(&[x])
        }
    }

    ///
    /// Creates a point in the 2D space.
    /// 
    pub fn point2d(x: T, y: T) -> Point<T> {
        Point {
            vector: RowVector::from_vec(&[x, y])
        }
    }

    ///
    /// Creates a point in the 3D space.
    /// 
    pub fn point3d(x: T, y: T, z: T) -> Point<T> {
        Point {
            vector: RowVector::from_vec(&[x, y, z])
        }
    }

    ///
    /// Builds a point from a matrix.
    /// 
    pub fn from_matrix(m: RectMatrix<T>) -> Point<T> {
        Point {
            vector: m.row(0)
        }
    }

    ///
    /// Returns the x coord if it exists or zero.
    ///
    pub fn x(&self) -> T {
        self.element_if_exists(0)
    }

    ///
    /// Returns the y coord if it exists or zero.
    ///
    pub fn y(&self) -> T {
        self.element_if_exists(1)
    }

    ///
    /// Returns the z coord if it exists or zero.
    ///
    pub fn z(&self) -> T {
        self.element_if_exists(2)
    }

    ///
    /// Returns the dimension of the space in which the point is included.
    /// 
    pub fn dim(&self) -> usize {
        self.vector.length()
    }

    ///
    /// Returns the idx-th element or zero.
    ///
    fn element_if_exists(&self, idx: usize) -> T {
        return if self.vector.length() > idx {
            self.vector.value(idx)
        }
        else {
            T::zero()
        }
    }
}

impl<T: MatElement> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
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
