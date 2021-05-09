/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.04
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
import { drawBezierSurf } from "./drawBezierSurf"

export let drawBezierSurfExample = (plot: string, drawControlPoints: boolean) => {
    let controlPoints: Point[][] = [[
            new Point(-3, 0, 2),
            new Point(-2, 0, 6),
            new Point(-1, 0, 7),
            new Point(0, 0, 2)
        ], [
            new Point(-3, 1, 2),
            new Point(-2, 1, 4),
            new Point(-1, 1, 5),
            new Point(0, 1, 2.5)
        ], [
            new Point(-3, 3, 0),
            new Point(-2, 3, 2.5),
            new Point(-1, 3, 4.5),
            new Point(0, 3, 6.5)
        ]
    ]
    
    drawBezierSurf(controlPoints, drawControlPoints, plot)
}