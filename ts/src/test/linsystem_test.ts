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

import { linsolve } from "../core/linsystem"
import { backwardSub, forwardSub, luDecomp } from "../core/lu"
import { approxEqual } from "../core/math"
import { ColVector, Matrix2 } from "../core/matrix"
// @ts-expect-error
var assert = require("assert")

function testLinSystem(A: Matrix2, b: ColVector, expected: ColVector)
{
    let x = linsolve(A, b)
    assert(x.round(2).equals(expected))
}

// Test 1
{
    testLinSystem(new Matrix2([
        [1, 1],
        [-3, 1]
    ]),
    new ColVector([6, 2]),
    new ColVector([1, 5])
    )
}

// Test 2
{
    testLinSystem(new Matrix2([
        [-3, 1],
        [4, 1]
    ]),
    new ColVector([-1, -8]),
    new ColVector([-1, -4])
    )
}

// Test 3
{
    testLinSystem(new Matrix2([
        [1, -2],
        [7, -3]
    ]),
    new ColVector([-2, 19]),
    new ColVector([4, 3])
    )
}
