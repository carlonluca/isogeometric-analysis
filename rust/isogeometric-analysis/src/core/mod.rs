/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.11.01
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

pub use self::size::Size;
pub use self::point::IntPoint;
pub use self::point::RealPoint;
pub use self::point::RealPoint1d;
pub use self::point::RealPoint2d;
pub use self::point::RealPoint3d;
pub use self::point::Point;
pub use self::range::IntRange;
pub use self::equatable::Equatable;
pub use self::matrix::RectMatrix;
pub use self::matrix::RowVector;
pub use self::matrix::ColVector;
pub use self::matrix::MatElement;
pub use self::factorial::fact;
pub use self::utils::measure_time;
pub use self::evaluate::Evaluatable;
pub use self::evaluate::Evaluator;
pub use self::color::HslProvider;
mod size;
mod point;
mod range;
mod equatable;
mod matrix;
mod factorial;
mod utils;
mod evaluate;
mod color;
