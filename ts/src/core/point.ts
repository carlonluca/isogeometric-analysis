/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.02
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

import { Matrix2, RowVector } from "./matrix";

/**
 * Class representing a point on a surface.
 */
export class Point extends RowVector {
    /**
     * Ctor.
     */
    constructor(x: number, y: number, z: number = 0) {
        super([x, y, z]);
    }

    /**
     * Converts this point to homogenous coords.
     * 
     * @param l 
     * @returns 
     */
    public toHomogeneous(w: number): HomPoint {
        return new HomPoint(
            this.x()*w,
            this.y()*w,
            this.z()*w,
            w
        )
    }

    /**
     * Clones this object.
     * 
     * @returns 
     */
    public clone(): Point {
        return new Point(this.x(), this.y(), this.z())
    }

    public x(): number { return this.value(0) }
    public y(): number { return this.value(1) }
    public z(): number { return this.value(2) }

    /**
     * Builds a matrix from a matrix of points.
     * 
     * @param d 
     * @returns 
     */
     public static matFromPoints(a: Point[][], d: string): Matrix2 {
        let d1 = a.length
        let d2 = a[0].length
        let d3 = d
        let m = Matrix2.zero(d1, d2)
        for (let i = 0; i < d1; i++) {
            for (let j = 0; j < d2; j++) {
                m.setValue(i, j, a[i][j][d3]())
            }
        }

        return m
    }

    /**
     * Builds a point from a vector.
     * 
     * @param v 
     * @returns 
     */
    public static fromVector(v: RowVector) {
        return new Point(v.value(0), v.value(1), v.value(2))
    }
}

/**
 * Class representing a point in homogenous coords.
 */
export class HomPoint extends RowVector {
    /**
     * Ctor.
     */
    constructor(x: number, y: number, z: number, w: number) {
        super([x, y, z, w]);
    }

    /**
     * Clones this object.
     * 
     * @returns 
     */
     public clone(): HomPoint {
        return new HomPoint(this.x(), this.y(), this.z(), this.w())
    }

    public x(): number { return this.value(0) }
    public y(): number { return this.value(1) }
    public z(): number { return this.value(2) }
    public w(): number { return this.value(3) }

    /**
     * Builds a matrix from a matrix of points.
     * 
     * @param d 
     * @returns 
     */
     public static matFromPoints(a: HomPoint[][], d: string): Matrix2 {
        let d1 = a.length
        let d2 = a[0].length
        let d3 = d
        let m = Matrix2.zero(d1, d2)
        for (let i = 0; i < d1; i++) {
            for (let j = 0; j < d2; j++) {
                m.setValue(i, j, a[i][j][d3]())
            }
        }

        return m
    }

    /**
     * Builds a point from a vector.
     * 
     * @param v 
     * @returns 
     */
    public static fromVector(v: RowVector) {
        return new HomPoint(v.value(0), v.value(1), v.value(2), v.value(3))
    }
}
