/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.09
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

import { Matrix2 } from "./matrix"
import { Point } from "./point"

/** 
 * Builds a matrix from a matrix of points.
 * 
 * @param d 
 * @returns 
 */
export function matFromPoints(points: Point[][], d: string): Matrix2 {
   let d1 = points.length
   let d2 = points[0].length
   let d3 = d
   let m = Matrix2.zero(d1, d2)
   for (let i = 0; i < d1; i++) {
       for (let j = 0; j < d2; j++) {
           m.setValue(i, j, this.controlPoints[i][j][d3])
       }
   }

   return m
}
