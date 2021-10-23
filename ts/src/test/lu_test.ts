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

import { backwardSub, forwardSub, luDecomp } from "../core/lu"
import { ColVector, Matrix2 } from "../core/matrix"
// @ts-expect-error
var assert = require("assert")

// Test 1
{
    let L = new Matrix2([
        [1, 0, 0, 0],
        [-1, 1, 0, 0],
        [0, 1/2, 1, 0],
        [6, 1, 14, 1]
    ])
    let b = new ColVector([1, -1, 2, 1])
    let x = new ColVector([1, 0, 2, -33])
    
    assert(forwardSub(L, b).equals(x))
}

// Test 2
{
    let L = new Matrix2([
        [1, 0, 0],
        [-0.25, 1, 0],
        [0, 3/3.5, 1]
    ])
    let b = new ColVector([-5, 3, 12])
    let x = new ColVector([-5, 1.75, 10.5])
    
    assert(forwardSub(L, b).equals(x))
}

// Test 3
{
    let L = new Matrix2([
        [1, 0, 0, 0],
        [2, 1, 0, 0],
        [3, 1, 1, 0],
        [1, -1, -1, 1]
    ])
    let b = new ColVector([7, 11, 31, 15])
    let x = new ColVector([7, -3, 13, 18])
    
    assert(forwardSub(L, b).equals(x))
}

// Test 1
{
    let U = new Matrix2([
        [1, -1/2, 3/2],
        [0, 1, -1/5],
        [0, 0, 1]
    ])
    let b = new ColVector([-9/2, 11/5, -1])
    let x = new ColVector([-2, 2, -1])

    assert(backwardSub(U, b).equals(x))
}

// Test 2
{
    let U = new Matrix2([
        [20, -12, -5],
        [0, 9, -7.25],
        [0, 0, 2.65]
    ])
    let b = new ColVector([10, 22.5, 45.5])
    let x = new ColVector([14.59, 16.33, 17.17])

    assert(backwardSub(U, b).round(2).equals(x))
}

// Test LU
{
    let A = new Matrix2([
        [7, 4],
        [3, 5]
    ])
    let LU = luDecomp(A)
    assert(LU.lower.round(2).equals(new Matrix2([
        [1, 0],
        [3/7, 1]
    ]).round(2)))
    assert(LU.upper.round(2).equals(new Matrix2([
        [7, 4],
        [0, 23/7]
    ]).round(2)))
}

{
    let A = new Matrix2([
        [1, 2, 2],
        [4, 4, 2],
        [4, 6, 4]
    ])
    let LU = luDecomp(A)
    assert(LU.lower.round(2).equals(new Matrix2([
        [1, 0, 0],
        [4, 1, 0],
        [4, 0.5, 1]
    ]).round(2)))
    assert(LU.upper.round(2).equals(new Matrix2([
        [1, 2, 2],
        [0, -4, -6],
        [0, 0, -1]
    ]).round(2)))
}