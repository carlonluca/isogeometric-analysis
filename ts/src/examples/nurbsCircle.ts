/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.15
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

import { Point } from "../core/point";
import { NurbsCurve } from "../nurbs/nurbs";

/**
 * NURBS circle.
 */
export class NurbsCirle extends NurbsCurve {
    /**
     * Ctor.
     */
    constructor() {
        let P = [
            new Point(1, 0),
            new Point(1, 1),
            new Point(0, 1),
            new Point(-1 ,1),
            new Point(-1, 0),
            new Point(-1, -1),
            new Point(0, -1),
            new Point(1, -1),
            new Point(1, 0)
        ]
        let kv = [0, 0, 0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1, 1, 1]
        let w = [1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1, 1/Math.sqrt(2), 1]
        super(P, kv, w, 2)
    }
}
