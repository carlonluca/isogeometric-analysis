/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.10.14
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

import { ColVector, Matrix2 } from "./matrix";

/**
 * Implements forward substitution in a system of form:
 * 
 *      Lx = b
 * 
 * @param L triangular matrix
 * @param b column
 */
export function forwardSub(L: Matrix2, b: ColVector): ColVector {
    let n = L.cols()
    let x = ColVector.zero(n)
    for (let i = 0; i < n; i++) {
        let bi = b.value(i)
        for (let j = 0; j < i; j++)
            bi -= L.value(i, j)*x.value(j)
        x.setValue(i, 0, bi/L.value(i, i))
    }
    return x
}
