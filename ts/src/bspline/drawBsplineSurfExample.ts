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

import { drawBsplineSurf } from "./drawBsplineSurf"
import { exampleSurf1ControlPoints } from "../examples/exampleSurfs"

export function drawBsplineSurfExample1(plot: string, drawControlPoints: boolean) {
    let p = 1
    let q = 1

    let Xi = [0, 0, 0.5, 1, 1]
    let Eta = [0, 0, 0.3, 0.6, 1, 1]

    drawBsplineSurf(exampleSurf1ControlPoints, Xi, Eta, p, q, drawControlPoints, plot)
}

export function drawBsplineSurfExample2(plot: string, drawControlPoints: boolean) {
    let p = 1
    let q = 2

    let Xi = [0, 0, 0.5, 1, 1]
    let Eta = [0, 0, 0, 0.5, 1, 1, 1]

    drawBsplineSurf(exampleSurf1ControlPoints, Xi, Eta, p, q, drawControlPoints, plot)
}