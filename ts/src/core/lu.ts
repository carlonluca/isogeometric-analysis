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

import { ColVector, Matrix2, RowVector } from "./matrix";
import { Range } from "./range";

/**
 * Data resulting from LU-decomposition.
 */
export class LUDecompData {
    constructor(lower: Matrix2, upper: Matrix2) {
        this.lower = lower
        this.upper = upper
    }

    public lower: Matrix2
    public upper: Matrix2
}

/**
 * Implements forward substitution in a system of form:
 * 
 *      Lx = b
 * 
 * @param L upper-triangular matrix
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

/**
 * Implements back substitution in a system of form:
 * 
 *      Ux = b
 * 
 * @param U lower-triangular matrix
 * @param b column
 * @returns 
 */
export function backwardSub(U: Matrix2, b: ColVector): ColVector {
    let n = U.cols()
    let x = ColVector.zero(n)
    for (let i = n - 1; i >= 0; i--) {
        let bi = b.value(i)
        for (let j = i + 1; j < n; j++)
            bi -= U.value(i, j)*x.value(j)
        x.setValue(i, 0, bi/U.value(i, i))
    }
    return x
}

export function luDecomp(A: Matrix2) {
    let n = A.cols()
    if (n == 1) {
        let L = new Matrix2([[1]])
        let U = A.clone()
        return new LUDecompData(L, U)
    }

    let A11 = A.value(0, 0)
    let A12 = A.row(0).row(0).right(1)
    let A21 = A.col(0).col(0).bottom(1)
    let A22 = A.mid(new Range(1, A.rows() - 1), new Range(1, A.cols() - 1))

    let L11 = 1
    let U11 = A11

    let L12 = RowVector.zero(n - 1)
    let U12 = A12.clone()

    let L21 = A21.clone().mult(1/U11)
    let U21 = ColVector.zero(n - 1)

    let S22 = A22.sub(A21.mult(1/A11).multMat(A12))
    let luData = luDecomp(S22)

    
}
