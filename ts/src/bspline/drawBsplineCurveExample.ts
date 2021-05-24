/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.15
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
import { drawBsplineCurve } from "./drawBsplineCurve"

export let drawBsplineCurve1 = (plot: string, drawControlPoints: boolean, bernsteinPlot: string) => {
    let controlPoints = []
    controlPoints.push(new Point(0, 0))
    controlPoints.push(new Point(1, 1))
    controlPoints.push(new Point(2, 0.5))
    controlPoints.push(new Point(3, 0.5))
    controlPoints.push(new Point(0.5, 1.5))
    controlPoints.push(new Point(1.5, 0))
    let knotVector = [ 0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1 ]
    drawBsplineCurve(controlPoints, knotVector, 2, false, drawControlPoints,  plot, bernsteinPlot)
};

export let drawBsplineCurve2 = (plot: string, drawControlPoints: boolean, bernsteinPlot: string) => {
    let controlPoints = []
    controlPoints.push(new Point(0, 0, 0))
    controlPoints.push(new Point(1, 1, 1))
    controlPoints.push(new Point(2, 0.5, 0))
    controlPoints.push(new Point(3, 0.5, 0))
    controlPoints.push(new Point(0.5, 1.5, 0))
    controlPoints.push(new Point(1.5, 0, 1))
    let knotVector = [ 0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1 ]
    drawBsplineCurve(controlPoints, knotVector, 2, true, drawControlPoints, plot, bernsteinPlot)
};