/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.14
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

import { Point } from "../core/point"
import { Range } from "../core/range"

/**
 * Class representing a B-spline curve in the 2D or 3D space.
 */
export class BsplineCurve {
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
        public p: number) {}

    /**
     * Evaluates the value of the B-spline curve in the parametric space.
     * 
     * @param xi 
     * @returns 
     */
    public evaluate(xi: number): Point {
        let x = 0
        let y = 0
        let z = 0
        let n = this.controlPoints.length
        for (let i = 0; i < n; i++) {
            let N = this.computeBasis(i, xi)
            x = x + N[i]*this.controlPoints[i].x
            y = y + N[i]*this.controlPoints[i].y
            z = z + N[i]*this.controlPoints[i].z
        }

        return new Point(x, y, z)
    }

    /**
     * Computes all non-vanishing bspline basis functions in xi.
     * 
     * @param i 
     * @param p 
     * @param xi 
     * @param Xi 
     * @returns 
     */
    public computeAllNonvanishingBasis(i: number, xi: number): number[] {
        let Xi = this.knotVector
        let p = this.p
        let N = Array(p + 1).fill(0)
        let right = Array(p + 1).fill(0)
        let left = Array(p + 1).fill(0)
        N[0] = 1
    
        for (let j = 1; j <= p; j++) {
            left[j] = xi - Xi[i + 1 - j]
            right[j] = Xi[i + j] - xi
            
            let saved: number = 0
            let temp: number = 0
            for (let r = 0; r < j; r++) {
                temp = N[r]/(right[r + 1] + left[j - r])
                N[r] = saved + right[r + 1]*temp
                saved = left[j - r]*temp
            }
    
            N[j] = saved
        }
    
        return N
    }

    public computeBasis(i: number, xi: number): number {
        let p = this.p
        let n = this.knotVector.length - 1

        // Check to see if we're evaluating the first or the last basis function at
        // the beginning or at the end of the knot vector.
        let Xi = this.knotVector
        if ((i == 0 && xi == Xi[0]) || (i == n - p - 1 && xi == Xi[n]))
            return 1
        
        // When xi is out of the domain it is set to zero.
        if (xi < Xi[i] || xi >= Xi[i + p + 1])
            return 0
        
        // Preallocation and computation of the temparary values of the functions to
        // be used according to the triangular table.        
        let N = Array(p + 1).fill(0)
        for (let j = 0; j <= p; j++) {
            if (xi >= Xi[i + j] && xi < Xi[i + j + 1])
                N[j] = 1
            else
                N[j] = 0
        }

        // Computation of the rest of the triangular table.
        let saved: number
        for (let k = 1; k <= p; k++) {
            if (N[0] == 0)
                saved = 0
            else
                saved = ((xi - Xi[i])*N[0])/(Xi[i + k] - Xi[i])
            for (let j = 0; j <= p - k - 1 + 1; j++) {
                let Xileft = Xi[i + j + 1]
                let Xiright = Xi[i + j + k + 1]
                if (N[j + 1] == 0) {
                    N[j] = saved
                    saved = 0
                }
                else {
                    let temp = N[j + 1]/(Xiright - Xileft)
                    N[j] = saved + (Xiright - xi)*temp
                    saved = (xi - Xileft)*temp
                }
            }
        }

        return N[0]
    }

    /**
     * Alias for computeAllNonvanishingBasis.
     * 
     * @param i 
     * @param xi 
     * @returns 
     */
    public N(i: number, xi: number): number[] {
        return this.computeAllNonvanishingBasis(i, xi)
    }

    /**
     * Finds the span in which xi lies.
     * 
     * @param Xi 
     * @param xi 
     * @param p 
     * @param n 
     * @returns 
     */
    public static findSpan(Xi: number[], xi: number, p: number, n: number): number {
        if (xi == Xi[n + 1])
            return n
        let low = p
        let high = n + 1
        let i = Math.floor((low + high)/2)
        while (xi < Xi[i] || xi >= Xi[i + 1]) {
            if (xi < Xi[i])
                high = i
            else
                low = i
            i = Math.floor((low + high)/2)
        }

        return i
    }
}