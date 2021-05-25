/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.24
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
import { drawBsplineSurf } from "./drawBsplineSurf"
import { exampleSurf1ControlPoints } from "../examples/exampleSurf1"

export function drawBsplineSurfExample(plot: string, drawControlPoints: boolean) {
    let controlPoints: Point[][] = exampleSurf1ControlPoints

    let p = 1
    let q = 2

    let Xi = [0, 0, 0.5, 1, 1]
    let Eta = [0, 0, 0, 0.5, 1, 1, 1]

    drawBsplineSurf(controlPoints, Xi, Eta, p, q, drawControlPoints, plot)
}