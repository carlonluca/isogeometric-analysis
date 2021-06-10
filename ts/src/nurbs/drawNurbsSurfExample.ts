/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.10
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
import { Point } from "../core/point"
import { drawNurbsSurf } from "./drawNurbsSurf"

export let drawNurbsSurfPlateHole = (plot: string, drawControlPoints: boolean, basisPlot: string) => {
    let P = [[
        new Point(-1, 0, 0),
        new Point(-2.5, 0, 0),
        new Point(-4, 0, 0)
    ], [
        new Point(-1, Math.sqrt(2) - 1, 0),
        new Point(-2.5, 0.75, 0),
        new Point(-4, 4, 0)
    ], [
        new Point(1 - Math.sqrt(2), 1, 0),
        new Point(-0.75, 2.5, 0),
        new Point(-4, 4, 0)
    ], [
        new Point(0, 1, 0),
        new Point(0, 2.5, 0),
        new Point(0, 4, 0)
    ]]
    let Xi = new RowVector([0, 0, 0, 0.5, 1, 1, 1])
    let Eta = new RowVector([0, 0, 0, 1, 1, 1])
    let w = Matrix2.one(4, 3)
    drawNurbsSurf(P, Xi, Eta, w, 2, 2, drawControlPoints, plot)
}
