/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2022.01.30
 *
 * Copyright (c) 2022 Luca Carlon. All rights reserved.
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

import { BezierCurve } from "../bezier/bezier"
import { measure } from "../core/time";
import { exampleCurve2D1, exampleCurve3D1 } from "../examples/exampleCurves"
// @ts-expect-error
var assert = require("assert")

// Test the implementation of Bezier curves.
{
    measure("2D Bezier curve", () => {
        let bezier = new BezierCurve(exampleCurve2D1);
        for (let i = 0; i < 10000; i++) {
            let xi = i/10000;
            bezier.evaluate(xi);
        }
    })

    measure("3D Bezier curve", () => {
        let bezier = new BezierCurve(exampleCurve3D1);
        for (let i = 0; i < 10000; i++) {
            let xi = i/10000;
            bezier.evaluate(xi);
        }
    })
}
