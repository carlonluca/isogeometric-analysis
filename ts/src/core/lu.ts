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
 * Data resulting from LUP-decomposition.
 */
export class LUPDecompData extends LUDecompData {
    constructor(lower: Matrix2, upper: Matrix2, perm: Matrix2) {
        super(lower, upper)
        this.perm = perm
    }

    public perm: Matrix2
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

/**
 * Implements LUP decomposition, without any pivoting.
 * 
 * @param A 
 * @returns 
 */
export function lupDecomp(A: Matrix2): LUPDecompData {
    let n = A.cols()
    if (n == 1) {
        let L = new Matrix2([[1]])
        let U = A.clone()
        let P = new Matrix2([[1]])
        return new LUPDecompData(L, U, P)
    }

    let i = A.col(0).indexMax()
    let A_bar = Matrix2.zero(A.rows(), A.cols())
    for  (let j = 0; j < A_bar.cols(); j++)
        A_bar.setValue(0, j, A.value(i, j))
    for (let k = 0; k < i; k++)
        for (let j = 0; j < A_bar.cols(); j++)
            A_bar.setValue(k + 1, j, A.value(k, j))
    for (let k = i + 1; k < A.rows(); k++)
        for (let j = 0; j < A_bar.cols(); j++)
            A_bar.setValue(k, j, A.value(k, j))
    
    let A_bar11 = A_bar.value(0, 0)
    let A_bar12 = A_bar.row(0).clone().row(0).right(1)
    let A_bar21 = A_bar.col(0).clone().col(0).bottom(1)
    let A_bar22 = A_bar.mid(new Range(1, A.rows() - 1), new Range(1, A.cols() - 1))

    let S22 = A_bar22.sub(A_bar21.clone().multMat(A_bar12).mult(1/A_bar11))

    let lupData = lupDecomp(S22)

    let L11 = 1
    let U11 = A_bar11
    
    let L12 = RowVector.zero(n - 1)
    let U12 = A_bar12.clone().row(0)

    let P22 = lupData.perm.clone()
    let L21 = P22.multMat(A_bar21).mult(1/A_bar11).col(0)
    let U21 = ColVector.zero(n - 1)

    let L = createLowerFromData(L11, L12, L21, lupData.lower)
    let U = createUpperFromData(U11, U12, U21, lupData.upper)

    // Create P
    let P = Matrix2.zero(L.rows(), L.cols())
    P.setValue(0, i, 1)
    for (let k = 1; k < P.rows(); k++)
        for (let j = 0; j < i; j++)
            P.setValue(k, j, lupData.perm.value(k - 1, j))
    for (let k = 1; k < P.rows(); k++)
        P.setValue(k, i, 0)
    for (let k = 1; k < P.rows(); k++)
        for (let j = i + 1; j < P.cols(); j++)
            P.setValue(k, j, lupData.perm.value(k - 1, j - 1))

    return new LUPDecompData(L, U, P)
}

/**
 * Implements plain LU decomposition, without any pivoting.
 * 
 * @param A 
 * @returns 
 */
export function luDecomp(A: Matrix2): LUDecompData {
    let n = A.cols()
    if (n == 1) {
        let L = new Matrix2([[1]])
        let U = A.clone()
        return new LUDecompData(L, U)
    }

    let A11 = A.value(0, 0)
    let A12 = A.row(0).clone().row(0).right(1)
    let A21 = A.col(0).clone().col(0).bottom(1)
    let A22 = A.mid(new Range(1, A.rows() - 1), new Range(1, A.cols() - 1))

    let L11 = 1
    let U11 = A11
    
    // TODO: the 0 row and vector could be removed if I start from a zero matrix.
    let L12 = RowVector.zero(n - 1)
    let U12 = A12.clone().row(0)

    let L21 = A21.clone().mult(1/U11).col(0)
    let U21 = ColVector.zero(n - 1)
    
    let S22 = A22.sub(L21.multMat(U12))
    let LU22 = luDecomp(S22)

    let L = createLowerFromData(L11, L12, L21, LU22.lower)
    let U = createUpperFromData(U11, U12, U21, LU22.upper)
    
    return new LUDecompData(L, U)
}

function createLowerFromData(L11: number, L12: RowVector, L21: ColVector, L22: Matrix2): Matrix2 {
    let L = Matrix2.zero(L12.cols() + 1, L21.rows() + 1)
    L.setValue(0, 0, L11)
    for (let i = 1; i < L.cols(); i++)
        L.setValue(0, i, L12.value(i - 1))
    for (let i = 1; i < L.rows(); i++)
        L.setValue(i, 0, L21.value(i - 1))
    for (let i = 1; i < L22.rows() + 1; i++)
        for (let j = 1; j < L22.cols() + 1; j++)
            L.setValue(i, j, L22.value(i - 1, j - 1))
    return L
}

function createUpperFromData(U11: number, U12: RowVector, U21: ColVector, U22: Matrix2): Matrix2 {
    let U = Matrix2.zero(U12.cols() + 1, U21.rows() + 1)
    U.setValue(0, 0, U11)
    for (let i = 1; i < U.cols(); i++)
        U.setValue(0, i, U12.value(i - 1))
    for (let i = 1; i < U.rows(); i++)
        U.setValue(i, 0, U21.value(i - 1))
    for (let i = 1; i < U22.rows() + 1; i++)
        for (let j = 1; j < U22.cols() + 1; j++)
            U.setValue(i, j, U22.value(i - 1, j - 1))
    return U
}
