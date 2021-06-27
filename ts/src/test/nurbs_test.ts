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

import { nurbsCurveSample2D } from "../examples/exampleCurves"
import { NurbsCirle } from "../examples/nurbsCircle"
import { NurbsCurve } from "../nurbs/nurbs"
// @ts-expect-error
var assert = require("assert")

function testEvaluationNurbs(nurbs: NurbsCurve) {
    let epsilon = 1E-6
    for (let xi = 0; xi < 1; xi += 0.05)
        assert(nurbs.evaluate1(xi).sub(nurbs.evaluate2(xi)).row(0).norm() < epsilon)
}

// Test the two implementations of a b-spline curve.
{
    testEvaluationNurbs(nurbsCurveSample2D())
    testEvaluationNurbs(new NurbsCirle())
}
