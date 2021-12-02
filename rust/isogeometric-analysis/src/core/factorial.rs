/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.02
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
/// Computes the factorial.
/// 
pub fn fact(n: i32) -> i64 {
    if n < 0 {
        panic!()
    }
    if n == 0 || n == 1 {
        return 1i64;
    }
    return (n as i64)*fact(n - 1);
}

#[cfg(test)]
mod tests {
    use crate::core::fact;
    use crate::core::measure_time;

    #[test]
    fn test() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(4), 24);
        assert_eq!(fact(5), 120);
        measure_time("fact(6)", || assert_eq!(fact(6), 720));
        measure_time("fact(7)", || assert_eq!(fact(7), 5040));
        measure_time("fact(8)", || assert_eq!(fact(8), 40320));
        measure_time("fact(9)", || assert_eq!(fact(9), 362880));
        measure_time("fact(20)", || assert_eq!(fact(20), 2432902008176640000));
    }
}
