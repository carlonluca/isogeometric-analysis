import { Matrix2 } from "../core/matrix"
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