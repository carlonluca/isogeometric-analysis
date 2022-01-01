/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.21
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

use colorsys::{Rgb, Hsl};

///
/// Structure to automatically compute colors for plots.
/// 
pub struct HslProvider {
    pub count: u32
}

impl HslProvider {
    ///
    /// Returns the color by rotating the HSL space of i*360/count degrees.
    /// 
    pub fn hex_color_for_index(&self, i: u32) -> String {
        let hsl = Hsl::new(360f64/(self.count as f64)*(i as f64), 100f64, 50f64, None);
        let rgb = Rgb::from(hsl);
        rgb.to_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::HslProvider;

    #[test]
    fn test() {
        let mut hsl = HslProvider { count: 3 };
        assert_eq!("#ff0000", hsl.hex_color_for_index(0));
        assert_eq!("#00ff00", hsl.hex_color_for_index(1));
        assert_eq!("#0000ff", hsl.hex_color_for_index(2));

        hsl = HslProvider { count: 15 };
        assert_eq!("#00ff66", hsl.hex_color_for_index(6));
    }
}
