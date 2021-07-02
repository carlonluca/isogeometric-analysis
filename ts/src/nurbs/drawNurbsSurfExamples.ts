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

import { BsplineCurve } from "../bspline/bspline";
import { Matrix2, RowVector } from "../core/matrix";
import { Point } from "../core/point"
import { NurbsPlateHole } from "../examples/nurbsPlate";
import { NurbsToroid } from "../examples/nurbsToroid";
import { drawNurbsSurf } from "./drawNurbsSurf"
import { NurbsSurf } from "./nurbs";

export let drawNurbsSurfPlateHole = (plot: string, drawControlPoints: boolean, basisPlot: string) => {
    let nurbs = new NurbsPlateHole()
    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, 2, 2, drawControlPoints, plot)
}

export function drawNurbsSurfToroid(plot: string, drawControlPoints: boolean, basisPlot: string) {
    let nurbs = new NurbsToroid()
    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, nurbs.p, nurbs.q, drawControlPoints, plot, true, 4, 4)
}

function drawNurbsKnotInsertionExample(nurbs: NurbsSurf, plot1: string, plot2: string, plot3: string, plot4: string, maxXi: number = 1, maxEta: number = 1) {

    let refine = (step: number, nurbs: NurbsSurf) => {
        for (let i = step; i <= 1 - step; i += step) {
            let k = BsplineCurve.findSpan(nurbs.Xi, i, nurbs.p, nurbs.controlPoints.length - 1)
            if (nurbs.Xi[k] != i)
                nurbs.insertKnotsXi(i, k, 0, 1)
            k = BsplineCurve.findSpan(nurbs.Eta, i, nurbs.q, nurbs.controlPoints[0].length - 1)
            if (nurbs.Eta[k] != i)
                nurbs.insertKnotsEta(i, k, 0, 1)
        }
    }

    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, nurbs.p, nurbs.q, true, plot1, true, maxXi, maxEta, "NURBS knot insertion 1")
    
    refine(0.25, nurbs)
    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, nurbs.p, nurbs.q, true, plot2, true, maxXi, maxEta, "NURBS knot insertion 2")
    
    refine(0.125, nurbs)
    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, nurbs.p, nurbs.q, true, plot3, true, maxXi, maxEta, "NURBS knot insertion 3")

    refine(0.0625, nurbs)
    drawNurbsSurf(
        nurbs.controlPoints,
        new RowVector(nurbs.Xi),
        new RowVector(nurbs.Eta),
        nurbs.weights, nurbs.p, nurbs.q, true, plot4, true, maxXi, maxEta, "NURBS knot insertion 4")
}

export function drawNURBSKnotInsertionPlateHole(plot1: string, plot2: string, plot3: string, plot4: string) {
    drawNurbsKnotInsertionExample(new NurbsPlateHole(), plot1, plot2, plot3, plot4)
}

export function drawNURBSKnotInsertionToroid(plot1: string, plot2: string, plot3: string, plot4: string) {
    drawNurbsKnotInsertionExample(new NurbsToroid(), plot1, plot2, plot3, plot4, 4, 4)
}
