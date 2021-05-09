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

import { bernstein } from "./bernstein"
import { Point } from "../core/point"

/**
 * Class representing a Bezier curve in the 2D or 3D space.
 */
export class BezierCurve {
    /**
     * Ctor.
     * 
     * @param controlPoints 
     */
    constructor(public controlPoints: Point[]) {}

    /**
     * Evaluates the value of the Bezier curve in the parametric space.
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
            x = x + bernstein(i, n - 1, xi)*this.controlPoints[i].x
            y = y + bernstein(i, n - 1, xi)*this.controlPoints[i].y
            z = z + bernstein(i, n - 1, xi)*this.controlPoints[i].z
        }

        return new Point(x, y, z)
    }
}

/**
 * Class representing a surface.
 */
export class BezierSurf {
    /**
     * Ctor.
     * 
     * @param controlPoints 
     */
    constructor(public controlPoints: Point[][]) {}

    /**
     * Evaluates the surface in (xi, eta).
     * 
     * @param xi 
     * @param eta 
     * @returns 
     */
    public evaluate(xi: number, eta: number): Point {
        let x = 0
        let y = 0
        let z = 0
        let n = this.controlPoints.length
        let m = this.controlPoints[0].length

        for (let i = 0; i < n; i++) {
            for (let j = 0; j < m; j++) {
                x += bernstein(i, n - 1, xi)*bernstein(j, m - 1, eta)*this.controlPoints[i][j].x
                y += bernstein(i, n - 1, xi)*bernstein(j, m - 1, eta)*this.controlPoints[i][j].y
                z += bernstein(i, n - 1, xi)*bernstein(j, m - 1, eta)*this.controlPoints[i][j].z
            }
        }

        return new Point(x, y, z)
    }
}
