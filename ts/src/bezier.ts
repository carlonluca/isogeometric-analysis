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
import { Point } from "./point"

export class Bezier {
    controlPoints: Point[] = []

    /**
     * Ctor.
     * 
     * @param controlPoints 
     */
    constructor(controlPoints: Point[]) {
        this.controlPoints = controlPoints
    }

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
        }

        return new Point(x, y)
    }
}
