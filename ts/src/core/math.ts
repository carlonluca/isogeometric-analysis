/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.14
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

import { Point } from "./point"

 export let in_range = (val: number, a: number, b: number, openLeft: boolean = false, openRight: boolean = false) => {
     return val >= a && val <= b
}

export function approxEqual(val1: number, val2: number, epsilon: number = 1E-6): boolean {
    return Math.abs(val1 - val2) <= epsilon
}

export function approxEqualPoints(p1: Point, p2: Point, epsilon: number = 1E-6): boolean {
    return approxEqual(p1.x(), p2.x(), epsilon) && approxEqual(p1.y(), p2.y(), epsilon) && approxEqual(p1.z(), p2.z())
}