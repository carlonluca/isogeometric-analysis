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
    let m4 = Matrix2.zero(3)
    let sum = Matrix2.zero(3).add(m1).add(m2).add(m3).add(m4)
    assert(sum.equals(new Matrix2([
        [7, 8, 10],
        [5, 8, 9],
        [16, 16, 17]
    ])))
    sum.print()
}