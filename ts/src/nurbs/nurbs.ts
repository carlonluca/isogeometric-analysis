/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.05
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
import { BsplineCurve } from "../bspline/bspline"
import { Range } from "../core/range"

/**
 * Representation of a NURBS curve.
 */
export class NurbsCurve {
    /**
    * Ctor.
    * 
    * @param controlPoints 
    * @param knotVector
    * @param p
    */
    constructor(
        public controlPoints: Point[],
        public knotVector: number[],
        public weights: number[],
        public p: number) { }
    
    /**
     * Computes the curve in xi.
     * 
     * @param xi 
     */
    public evaluate(xi: number): Point {
        let x = 0
        let y = 0
        let z = 0
        

        return new Point(x, y, z)
    }

    /**
     * Computes the i-th NURBS basis function.
     * 
     * @param Xi 
     * @param w 
     * @param i 
     * @param p 
     * @param xi 
     * @returns 
     */
    public static computeBasis(Xi: RowVector, w: RowVector, i: number, p: number, xi: number): number {
        let n = Xi.length() - 1
        let xiSpan = BsplineCurve.findSpan(Xi.toArray(), xi, p, n)
        if (i < xiSpan - p || i > xiSpan)
            return 0
        let N = BsplineCurve.computeAllNonvanishingBasis(Xi.toArray(), xiSpan, p, xi)
        let R = N.value(0, p - (xiSpan - i))*w.value(i)/N.multMat(w.range(new Range(xiSpan - p, xiSpan)).transposed()).value(0, 0)
        return R
    }
}
