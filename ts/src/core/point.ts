/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.02
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

import { RowVector } from "./matrix";

/**
 * Class representing a point on a surface.
 */
export class Point extends RowVector {
    /**
     * Ctor.
     */
    constructor(public x: number, public y: number, public z: number = 0) {
        super([x, y, z]);
    }

    /**
     * Converts this point to homogenous coords.
     * 
     * @param l 
     * @returns 
     */
    public toHomogeneous(w: number): HomPoint {
        return new HomPoint(
            this.x*w,
            this.y*w,
            this.z*w,
            w
        )
    }
}

/**
 * Class representing a point in homogenous coords.
 */
export class HomPoint extends RowVector {
    /**
     * Ctor.
     */
    constructor(public x: number, public y: number, public z: number, public w: number) {
        super([x, y, z, w]);
    }
}
