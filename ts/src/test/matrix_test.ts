/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.27
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

import { Matrix2, RowVector } from "../core/matrix"
import { Point } from "../core/point"
import { Range } from "../core/range"
import { Size } from "../core/size"
// @ts-expect-error
var assert = require("assert")

// Test sum 1
{
    let m1 = new Matrix2([[1, 2, 3]])
    m1.print()
    let m2 = new Matrix2([[1, 1, 1]])
    m2.print()
    let m3 = Matrix2.add(m1, m2)
    m3.print()
    assert(!m3.equals(m2))
    assert(m3.equals(new Matrix2([[2, 3, 4]])))
}

// Test sum 2
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7]
    ])
    let m2 = new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])
    let m3 = Matrix2.identity(3)
    let m4 = Matrix2.zeroSquare(3)
    let sum = Matrix2.zeroSquare(3).add(m1).add(m2).add(m3).add(m4)
    assert(sum.equals(new Matrix2([
        [7, 8, 10],
        [5, 8, 9],
        [16, 16, 17]
    ])))
    assert(m1.equals(new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7]
    ])))
    assert(m2.equals(new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])))
    assert(m3.equals(Matrix2.identity(3)))
    sum.print()
}

// Test mult by scalar 1
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7]
    ])
    assert(m1.mult(9).equals(new Matrix2([
        [5*9, 6*9, 7*9],
        [1*9, 2*9, 3*9],
        [9*9, 8*9, 7*9]
    ])))
}

// Test mult by a scalar 2
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7]
    ])
    assert(m1.mult(0).equals(Matrix2.zeroSquare(3)))
}

// Test mult by matrix 1
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7]
    ])
    let m2 = new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])
    let m3 = Matrix2.identity(3)
    let m4 = Matrix2.zeroSquare(3)
    assert(m1.multMat(m1).equals(new Matrix2([
        [94, 98, 102],
        [34, 34, 34],
        [116, 126, 136]
    ])))
    assert(m1.multMat(m2).equals(new Matrix2([
        [78, 96, 114],
        [30, 36, 42],
        [90, 114, 138]
    ])))
    assert(m1.multMat(m3).equals(m1))
    assert(m2.multMat(m3).equals(m2))
    assert(m1.multMat(m4).equals(m4))
}

// Test mult by matrix 2
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7],
        [1, 1, 1]
    ])
    let m2 = new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])
    let m4 = Matrix2.zeroSquare(3)
    try { m1.multMat(m1); assert(false) } catch (e) { assert(true) }
    assert(m1.multMat(m2).equals(new Matrix2([
        [78, 96, 114],
        [30, 36, 42],
        [90, 114, 138],
        [12, 15, 18]
    ])))
    assert(m1.multMat(m4).equals(Matrix2.zero(m1.rows(), m2.cols())))
}

// Test mult by matrix 2
{
    let m = new Matrix2([
        [1],
        [2],
        [3]
    ])
    assert(m.transposed().size().equals(new Size(3, 1)))

    m.transpose()
    assert(m.equals(new Matrix2([[1, 2, 3]])))
}

// Test rect extraction
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7],
        [1, 1, 1]
    ])
    let m2 = new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])
    assert(m1.rect(new Point(1, 1), new Point(2, 2)).equals(new Matrix2([
        [2, 3],
        [8, 7]
    ])))
}

// Test range extraction
{
    let m1 = new RowVector([5, 6, 7, 1, 2, 3, 9, 8, 7, 1, 1, 1])
    assert(m1.range(new Range(2, 4)).equals(new RowVector([7, 1, 2])))
}

// Test setValue
{
    let m1 = new Matrix2([
        [5, 6, 7],
        [1, 2, 3],
        [9, 8, 7],
        [1, 1, 1]
    ])
    m1.setValue(1, 2, 199)
    assert(m1.equals(new Matrix2([
        [5, 6, 7],
        [1, 2, 199],
        [9, 8, 7],
        [1, 1, 1]
    ])))
}

{
    let m1 = new Matrix2([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ])
    let m2 = m1.mid(new Range(1, 2), new Range(0, 1))
    assert(m2.equals(new Matrix2([
        [4, 5],
        [7, 8]
    ])))
}

{
    let v1 = new RowVector([1, 2, 3, 4, 5, 6, 7, 8, 9])
    let v2 = v1.left(4)
    let v3 = v1.right(4)
    assert(v2.equals(new RowVector([1, 2, 3, 4, 5])))
    assert(v3.equals(new RowVector([5, 6, 7, 8, 9])))
}