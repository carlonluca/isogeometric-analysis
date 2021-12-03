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

use num::traits::Zero;

///
/// Represents a point.
///
pub struct Point<T: Zero> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Zero> Point<T> {
    ///
    /// Creates a 2D point.
    /// 
    pub fn point2d(x: T, y: T) -> Point<T> {
        return Point {
            x: x,
            y: y,
            z: T::zero()
        };
    }
}

///
/// Represents a point with integer coords.
/// 
pub type IntPoint = Point<i32>;
