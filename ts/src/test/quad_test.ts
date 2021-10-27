/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.10.27
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

import { quadSimpson } from "../core/quad"

// @ts-expect-error
var assert = require("assert")

// Test 1
{
    let f = (x: number) => 1/Math.pow((Math.pow(x, 5) + 7), 1/3)
    assert(quadSimpson(f, 0, 1, 4) === 0.518798359105237)
    assert(quadSimpson(f, 0, 1, 200) === 0.5188095062580791)
    assert(quadSimpson(f, 0, 10, 200) === 1.538444380729461)
}

// Test 2
{
    let f = (x: number) => Math.pow(x, 2) + (Math.pow(x, 4) + 1)*Math.sin(x)/(Math.pow(x, 2) - 1)
    assert(quadSimpson(f, 5, 10, 200) == 380.2652329293029)
}
