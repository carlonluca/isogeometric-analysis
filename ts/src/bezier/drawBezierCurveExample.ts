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

import { drawBezierCurve } from "./drawBezierCurve"
import { exampleCurve2D1, exampleCurve3D1 } from "../examples/exampleCurves"

export let drawBezierCurve1 = (plot: string, drawControlPoints: boolean, bernsteinPlot: string) => {
    drawBezierCurve(exampleCurve2D1, false, drawControlPoints,  plot, bernsteinPlot)
};

export let drawBezierCurve2 = (plot: string, drawControlPoints: boolean, bernsteinPlot: string) => {
    drawBezierCurve(exampleCurve3D1, true, drawControlPoints, plot, bernsteinPlot);
};