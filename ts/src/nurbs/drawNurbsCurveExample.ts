/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.06
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

import { RowVector } from "../core/matrix";
import { Point } from "../core/point"
import { NurbsCirle } from "../examples/nurbsCircle";
import { drawNurbsCurve } from "./drawNurbsCurve"

export let drawNurbsCurveExample1 = (plot: string, drawControlPoints: boolean, basisPlot: string) => {
    let controlPoints = []
    controlPoints.push(new Point(0, 0))
    controlPoints.push(new Point(1, 1))
    controlPoints.push(new Point(2, 0.5))
    controlPoints.push(new Point(3, 0.5))
    controlPoints.push(new Point(0.5, 1.5))
    controlPoints.push(new Point(1.5, 0))
    let knotVector = [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1]
    let w = RowVector.one(controlPoints.length)
    drawNurbsCurve(controlPoints, knotVector, w.toArray(), 2, false, drawControlPoints, plot, basisPlot)
}

export let drawNurbsCurveExample2 = (plot: string, drawControlPoints: boolean, basisPlot: string) => {
    let controlPoints = []
    controlPoints.push(new Point(0, 0, 0))
    controlPoints.push(new Point(1, 1, 1))
    controlPoints.push(new Point(2, 0.5, 0))
    controlPoints.push(new Point(3, 0.5, 0))
    controlPoints.push(new Point(0.5, 1.5, 0))
    controlPoints.push(new Point(1.5, 0, 1))
    let knotVector = [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1]
    let w = RowVector.one(controlPoints.length)
    drawNurbsCurve(controlPoints, knotVector, w.toArray(), 2, true, drawControlPoints, plot, basisPlot)
}

export let drawNurbsCurveExampleCircle = (plot: string, drawControlPoints: boolean, basisPlot: string) => {
    let circle = new NurbsCirle()
    drawNurbsCurve(circle.controlPoints, circle.knotVector, circle.weights, 2, false, drawControlPoints, plot, basisPlot, true)
}

export function drawNurbsKnotInsertionExample(plot1: string, plot2: string, plot3: string, plot4: string) {
    let circle = new NurbsCirle()
    drawNurbsCurve(circle.controlPoints, circle.knotVector, circle.weights, 2, false, true, plot1, null, true, "Knot Insertion 1")
    circle.insertKnot(0.6, 6, 0, 1)
    drawNurbsCurve(circle.controlPoints, circle.knotVector, circle.weights, 2, false, true, plot2, null, true, "Knot Insertion 2")
    circle.insertKnot(0.3, 4, 0, 1)
    drawNurbsCurve(circle.controlPoints, circle.knotVector, circle.weights, 2, false, true, plot3, null, true, "Knot Insertion 3")
    circle.insertKnot(0.2, 2, 0, 1)
    drawNurbsCurve(circle.controlPoints, circle.knotVector, circle.weights, 2, false, true, plot4, null, true, "Knot Insertion 4")
}
