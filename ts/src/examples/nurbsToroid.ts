/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.07.2
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

import { Matrix2, RowVector } from "../core/matrix";
import { Point } from "../core/point";
import { NurbsSurf } from "../nurbs/nurbs";

/**
 * A toroid.
 */
export class NurbsToroid extends NurbsSurf {
    /**
     * Ctor.
     */
    constructor() {
        let P = [[
            new Point(5, 0, -1),
            new Point(5, 5, -1),
            new Point(0, 5, -1),
            new Point(-5, 5, -1),
            new Point(-5, 0, -1),
            new Point(-5, -5, -1),
            new Point(0, -5, -1),
            new Point(5, -5, -1),
            new Point(5, 0, -1)
        ], [
            new Point(6, 0, -1),
            new Point(6, 6, -1),
            new Point(0, 6, -1),
            new Point(-6, 6, -1),
            new Point(-6, 0, -1),
            new Point(-6, -6, -1),
            new Point(0, -6, -1),
            new Point(6, -6, -1),
            new Point(6, 0, -1)
        ], [
            new Point(6, 0, 0),
            new Point(6, 6, 0),
            new Point(0, 6, 0),
            new Point(-6, 6, 0),
            new Point(-6, 0, 0),
            new Point(-6, -6, 0),
            new Point(0, -6, 0),
            new Point(6, -6, 0),
            new Point(6, 0, 0)
        ], [
            new Point(6, 0, 1),
            new Point(6, 6, 1),
            new Point(0, 6, 1),
            new Point(-6, 6, 1),
            new Point(-6, 0, 1),
            new Point(-6, -6, 1),
            new Point(0, -6, 1),
            new Point(6, -6, 1),
            new Point(6, 0, 1)
        ], [
            new Point(5, 0, 1),
            new Point(5, 5, 1),
            new Point(0, 5, 1),
            new Point(-5, 5, 1),
            new Point(-5, 0, 1),
            new Point(-5, -5, 1),
            new Point(0, -5, 1),
            new Point(5, -5, 1),
            new Point(5, 0, 1),
        ], [
            new Point(4, 0, 1),
            new Point(4, 4, 1),
            new Point(0, 4, 1),
            new Point(-4, 4, 1),
            new Point(-4, 0, 1),
            new Point(-4, -4, 1),
            new Point(0, -4, 1),
            new Point(4, -4, 1),
            new Point(4, 0, 1),
        ], [
            new Point(4, 0, 0),
            new Point(4, 4, 0),
            new Point(0, 4, 0),
            new Point(-4, 4, 0),
            new Point(-4, 0, 0),
            new Point(-4, -4, 0),
            new Point(0, -4, 0),
            new Point(4, -4, 0),
            new Point(4, 0, 0),
        ], [
            new Point(4, 0, -1),
            new Point(4, 4, -1),
            new Point(0, 4, -1),
            new Point(-4, 4, -1),
            new Point(-4, 0, -1),
            new Point(-4, -4, -1),
            new Point(0, -4, -1),
            new Point(4, -4, -1),
            new Point(4, 0, -1),
        ], [
            new Point(5, 0, -1),
            new Point(5, 5, -1),
            new Point(0, 5, -1),
            new Point(-5, 5, -1),
            new Point(-5, 0, -1),
            new Point(-5, -5, -1),
            new Point(0, -5, -1),
            new Point(5, -5, -1),
            new Point(5, 0, -1),
        ]]
    
        let Pt: Point[][] = new Array(P[0].length)
        for (let i = 0; i < P[0].length; i++)
            Pt[i] = new Array(P.length)
        for (let i = 0; i < P.length; i++)
            for (let j = 0; j < P[i].length; j++)
                Pt[j][i] = P[i][j]
    
        P = Pt
    
        let Xi = new RowVector([0, 0, 0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1, 1, 1]).mult(4).row(0)
        let Eta = new RowVector([0, 0, 0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1, 1, 1]).mult(4).row(0)
        let p = 2
        let q = 2
    
        let w1 = new RowVector([1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1])
        let w2 = new RowVector([1/Math.sqrt(2), 1/2, 1/Math.sqrt(2), 1/2, 1/Math.sqrt(2), 1/2, 1/Math.sqrt(2), 1/2, 1/Math.sqrt(2)])
        let w = Matrix2.zero(9, 9)
            .assignCol(0, w1)
            .assignCol(1, w2)
            .assignCol(2, w1)
            .assignCol(3, w2)
            .assignCol(4, w1)
            .assignCol(5, w2)
            .assignCol(6, w1)
            .assignCol(7, w2)
            .assignCol(8, w1)
        super(P, Xi.toArray(), Eta.toArray(), w, p, q)
    }
}