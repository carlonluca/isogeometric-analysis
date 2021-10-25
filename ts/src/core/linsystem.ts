/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.10.25
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

import { backwardSub, forwardSub, luDecomp } from "./lu";
import { ColVector, Matrix2 } from "./matrix";

/**
 * Solves a linear system in the form:
 * 
 *     Ax = b
 * 
 * @param A 
 * @param b 
 */
export function linsolve(A: Matrix2, b: ColVector)
{
    let ludata = luDecomp(A)
    return lusolve(ludata.lower, ludata.upper, b)
}

/**
 * Solves a system in the form:
 * 
 *     LUx = b
 * 
 * where L is lower-triangular and U is upper-triangular.
 * 
 * @param L 
 * @param U 
 * @param b 
 * @returns 
 */
export function lusolve(L: Matrix2, U: Matrix2, b: ColVector)
{
    let y = forwardSub(L, b)
    let x = backwardSub(U, y)
    return x
}
