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

import { Point } from "./point"
import { Bezier } from "./bezier"

/**
 * Draws the bezier curve into a plot.
 * 
 * @param controlPoints 
 * @param plot 
 */
export let drawBezierCurve = (controlPoints: Point[], plot: string) => {
    const bezier = new Bezier(controlPoints);
    // @ts-expect-error
    const xiValues = math.range(0, 1, 0.001).toArray();
    let xValues = [];
    let yValues = [];
    xiValues.map((xi) => {
        let p = bezier.evaluate(xi);
        xValues.push(p.x);
        yValues.push(p.y);
    });
    const trace1 = {
        x: xValues,
        y: yValues
    };
    const data = [trace1];

    // @ts-expect-error
    Plotly.newPlot(plot, data);
};
