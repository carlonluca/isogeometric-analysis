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

import { IEquatable } from "./iequatable"
import { Point } from "./point"
import { Size } from "./size"

/**
 * Class representing a matrix.
 */
export class Matrix2 implements IEquatable<Matrix2> {
    private m_data: number[][]

    /**
     * Ctor.
     * 
     * @param value 
     */
    constructor(value: number[][] | number) {
        if (typeof value == "number")
            this.m_data = Matrix2.identity(value).m_data
        else
            this.m_data = value
    }

    /**
     * Returns internal data.
     * 
     * @returns 
     */
    public data(): number[][] { return this.m_data }

    /**
     * Returns the i-th row.
     * 
     * @param i 
     */
    public row(i: number): Matrix2 { return new Matrix2([this.m_data[i]]) }

    /**
     * Returns a subrect of this matrix.
     * 
     * @param topLeft 
     * @param bottomRight 
     * @returns 
     */
    public rect(topLeft: Point, bottomRight: Point): Matrix2 {
        let cols = bottomRight.x - topLeft.x + 1
        let rows = bottomRight.y - topLeft.y + 1
        let retData = Matrix2.createEmptyMatrixOfSize(rows, cols)
        for (let j = topLeft.x; j <= bottomRight.x; j++)
            for (let i = topLeft.y; i <= bottomRight.y; i++)
                retData[i - topLeft.y][j - topLeft.x] = this.m_data[i][j]
        return new Matrix2(retData)
    }

    /**
     * Returns the j-th column.
     * 
     * @param j 
     * @returns 
     */
    public col(j: number): Matrix2 {
        let ret = []
        for (let i = 0; i < this.rows(); i++)
            ret.push(this.m_data[i][j])
        return new Matrix2([ret])
    }

    /**
     * Number of rows.
     * 
     * @returns 
     */
    public rows(): number { return this.m_data.length }

    /**
     * Number of columns.
     * 
     * @returns 
     */
    public cols(): number { return this.m_data[0].length }

    /**
     * Returns the size.
     * 
     * @returns 
     */
    public size(): Size { return new Size(this.rows(), this.cols()) }

    /**
     * Accesses the element at specified position.
     * 
     * @param row 
     * @param col 
     * @returns 
     */
    public value(row: number, col: number) { return this.m_data[row][col] }

    /**
     * Sets the value of an element of the matrix.
     * 
     * @param row 
     * @param col 
     * @param val 
     */
    public setValue(row: number, col: number, val: number) { this.m_data[row][col] = val }

    /**
     * Logs the matrix.
     */
    public print() { console.table(this.m_data) }

    /**
     * Adds the matrix to this matrix.
     * 
     * @param m 
     */
    public add(m: Matrix2): Matrix2 {
        if (m.size().equals(this.size()))
            throw new Error("Cannot add matrices of different sizes")
        for (let i = 0; i < this.rows(); i++)
            for (let j = 0; j < this.cols(); j++)
                this.m_data[i][j] += m.value(i, j)
        return this
    }

    /**
     * Multiplies by a scalar.
     * 
     * @param scalar 
     * @returns 
     */
    public mult(scalar: number): Matrix2 {
        for (let i = 0; i < this.rows(); i++)
            for (let j = 0; j < this.cols(); j++)
                this.m_data[i][j] *= scalar
        return this
    }

    /**
     * Multiplication by a matrix. Returns a new matrix with the result.
     * 
     * @param m 
     */
    public multMat(m: Matrix2): Matrix2 {
        if (this.cols() != m.rows())
            throw new Error("Invalid mat sizes: " + this.size() + "·" + m.size())
        let res = Matrix2.zero(this.rows(), m.cols())
        for (let i = 0; i < this.rows(); i++) {
            for (let j = 0; j < m.cols(); j++) {
                let e = 0
                for (let p = 0; p < this.cols(); p++)
                    e += this.value(i, p)*m.value(p, j)
                res.m_data[i][j] = e
            }
        }

        return res
    }

    /**
     * Transposes this matrix.
     */
    public transpose(): Matrix2 {
        let newMatrix = this.transposed()
        this.m_data = newMatrix.m_data
        return this
    }

    /**
     * Returns a new matrix that is the transposed of this matrix.
     * 
     * @returns 
     */
    public transposed(): Matrix2 {
        let oldData = this.m_data
        let newData = Matrix2.createEmptyMatrixOfSize(this.cols(), this.rows())
        for (let i = 0; i < this.rows(); i++)
            for (let j = 0; j < this.cols(); j++)
                newData[j][i] = oldData[i][j]
        return new Matrix2(newData)
    }

    /**
     * IEquatable interface.
     * 
     * @param m 
     * @returns 
     */
    public equals(m: Matrix2): boolean {
        if (this.m_data.length != m.m_data.length)
            return false
        if (this.m_data.length <= 0)
            return true
        if (this.m_data[0].length != m.m_data[0].length)
            return false
        for (let i = 0; i < this.m_data.length; i++)
            for (let j = 0; j < this.m_data[i].length; j++)
                if (this.m_data[i][j] != m.m_data[i][j])
                    return false
        return true
    }

    /**
     * Clones this matrix.
     * 
     * @returns 
     */
    public clone(): Matrix2 {
        return new Matrix2(this.m_data.map(function(arr) {
            return arr.slice();
        }))
    }

    /**
     * Create identity matrix of size size.
     * 
     * @param size 
     * @returns 
     */
    public static identity(size: number): Matrix2 {
        let values = Matrix2.createEmptyMatrixOfSize(size, size)
        for (let i = 0; i < size; i++)
            for (let j = 0; j < size; j++)
                values[i][j] = (i === j ? 1 : 0)
        return new Matrix2(values)
    }

    /**
     * Returns a null matrix of a given size.
     * 
     * @param size 
     * @returns 
     */
    public static zeroSquare(size: number): Matrix2 {
        return this.zero(size, size)
    }

    /**
     * Returns a null matrix of a given size.
     * 
     * @param size 
     * @returns 
     */
    public static zero(rows: number, cols: number): Matrix2 {
        let values = Matrix2.createEmptyMatrixOfSize(rows, cols)
        for (let i = 0; i < rows; i++)
            for (let j = 0; j < cols; j++)
                values[i][j] = 0
        return new Matrix2(values)
    }

    /**
     * Adds two matrices.
     * 
     * @param m1 
     * @param m2 
     * @returns 
     */
    public static add(m1: Matrix2, m2: Matrix2) {
        if (m1.size().equals(m2.size()))
            throw new Error("Cannot add matrices of different sizes")
        return m1.clone().add(m2)
    }

    /**
     * Multiplies by a scalar.
     * 
     * @param m 
     * @param scalar 
     * @returns 
     */
    public static mult(m: Matrix2, scalar: number) {
        return m.clone().mult(scalar)
    }

    // Private portion

    /**
     * Creates empty structure.
     * 
     * @param rows 
     * @param cols 
     * @returns 
     */
    private static createEmptyMatrixOfSize(rows: number, cols: number): number[][] {
        let values: number[][] = new Array(rows)
        for (let i = 0; i < rows; i++)
            values[i] = new Array(cols)
        return values
    }
}