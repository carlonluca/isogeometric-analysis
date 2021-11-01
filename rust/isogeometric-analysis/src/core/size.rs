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
 /// Size represents a size with width and height.
 /// 
pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Size {
    ///
    /// Determines if width or height or both are zero.
    /// 
    pub fn is_empty(&self) -> bool {
        if self.width <= 0 || self.height <= 0 { true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Size;

    #[test]
    fn test_empty() {
        assert_eq!(Size {
            width: 0,
            height: 0
        }.is_empty(), true);
    }
}
