/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.26
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

import { BsplineCurve } from "../bspline/bspline"
import { approxEqualPoints } from "../core/math"
import { nurbsCurveSample2D } from "../examples/exampleCurves"
import { NurbsCirle } from "../examples/nurbsCircle"
import { NurbsPlateHole } from "../examples/nurbsPlate"
import { NurbsCurve, NurbsSurf } from "../nurbs/nurbs"
// @ts-expect-error
var assert = require("assert")

function measure(label: string, f: () => void) {
    console.time(label)
    f()
    console.timeEnd(label)
}

function testEvaluationNurbs(nurbs: NurbsCurve) {
    for (let xi = 0; xi < 1; xi += 0.05)
        assert(approxEqualPoints(nurbs.evaluate1(xi), nurbs.evaluate2(xi)))
}

function testNurbs(n1: NurbsSurf, n2: NurbsSurf) {
    for (let xi = 0; xi <= 1; xi += 0.01)
        for (let eta = 0; eta <= 1; eta += 0.01)
            assert(approxEqualPoints(n1.evaluate(xi, eta), n2.evaluate(xi, eta)))
}

// Test the two implementations of a b-spline curve.
{
    testEvaluationNurbs(nurbsCurveSample2D())
    testEvaluationNurbs(new NurbsCirle())
}

//  Test knot insertion.
{
    measure("surf_knot_insertion_xi", () => {
        let n1 = new NurbsPlateHole()
        let n2 = new NurbsPlateHole()
        for (let i = 0.1; i <= 1; i += 0.9) {
            if (n2.Xi[i] != i) {
                let k = BsplineCurve.findSpan(n2.Xi, i, n2.p, n2.controlPoints.length - 1)
                testNurbs(n1, n2.insertKnotsXi(i, k, 0, 1))
            }
        }
    })

    measure("surf_knot_insertion_eta", () => {
        let n1 = new NurbsPlateHole()
        let n2 = new NurbsPlateHole()
        for (let i = 0.1; i <= 1; i += 0.9) {
            if (n2.Xi[i] != i) {
                let k = BsplineCurve.findSpan(n2.Xi, i, n2.p, n2.controlPoints.length - 1)
                testNurbs(n1, n2.insertKnotsEta(i, k, 0, 1))
            }
        }
    })
}
