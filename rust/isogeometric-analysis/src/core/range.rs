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

 ///
 /// Represents linear range.
 /// 
pub struct IntRange {
    pub a: i32,
    pub b: i32
}

impl IntRange {
    ///
    /// Size of the closed range.
    /// 
    pub fn size_closed(&self) -> i32 { self.b - self.a + 1}
}

#[cfg(test)]
mod tests {
    use crate::core::IntRange;

    #[test]
    fn test_size() {
        assert_eq!(IntRange {
            a: 0,
            b: 0
        }.size_closed(), 1);
        assert_eq!(IntRange {
            a: 0,
            b: 6
        }.size_closed(), 7)
    }
}
