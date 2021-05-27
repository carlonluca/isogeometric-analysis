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

import { Size } from "./size"

/**
 * Class representing a matrix.
 */
export class Matrix2 {
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
     * Create identity matrix of size size.
     * 
     * @param size 
     * @returns 
     */
    public static identity(size: number) {
        let values: number[][] = []
        for (let i = 0; i < size; i++)
            for (let j = 0; j < size; j++)
                values[i][j] = i == j ? 1 : 0
        return new Matrix2(values)
    }

    /**
     * Returns a null matrix of a given size.
     * 
     * @param size 
     * @returns 
     */
    public static zero(size: number) {
        let values: number[][] = []
        for (let i = 0; i < size; i++)
            for (let j = 0; j < size; j++)
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
        let newData: number[][] = m1.data().map(function(arr) {
            return arr.slice();
        })
        return new Matrix2(newData).add(m2)
    }
}
