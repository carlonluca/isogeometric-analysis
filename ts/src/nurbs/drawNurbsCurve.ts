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

import { RowVector } from "../core/matrix"
import { Point } from "../core/point"
import { NurbsCurve } from "./nurbs"

/**
* Draws the nurbs curve into a plot.
*
* @param controlPoints
* @param plot
*/
export let drawNurbsCurve = (
    controlPoints: Point[],
    knotVector: number[],
    weights: number[],
    p: number,
    threed: boolean,
    drawControlPoints: boolean,
    plot: string,
    bernsteinPlot: string = null) => {
    const nurbs = new NurbsCurve(controlPoints, knotVector, weights, p)
    // @ts-expect-error
    const xiValues = math.range(0, 1, 0.001).toArray()
    let xValues = []
    let yValues = []
    let zValues = []
    xiValues.map((xi: number) => {
        let p = nurbs.evaluate(xi)
        xValues.push(p.x)
        yValues.push(p.y)
        zValues.push(p.z)
    })
    const plotType = threed ? "scatter3d" : "scatter"
    const trace1 = {
        x: xValues,
        y: yValues,
        z: zValues,
        type: plotType,
        name: "NURBS curve",
        mode: "lines",
        line: {
            color: "orange",
            width: 4
        }
    }
    const data = [trace1]

    if (drawControlPoints) {
        const cpXValues = []
        const cpYValues = []
        const cpZValues = []
        for (let cp of controlPoints) {
            cpXValues.push(cp.x)
            cpYValues.push(cp.y)
            cpZValues.push(cp.z)
        }
        const trace2 = {
            x: cpXValues,
            y: cpYValues,
            z: cpZValues,
            name: "Control points",
            mode: 'lines+markers',
            type: plotType,
            marker: {
                color: "transparent",
                size: 8,
                line: {
                    color: "black",
                    width: 1
                }
            },
            line: {
                color: "red",
                width: 1,
                dash: "dot"
            }
        }
        data.push(trace2)
    }

    var layout = {
        title: {
            text: "NURBS curve",
            font: {
                family: "Ubuntu",
                size: 24,
            },
            xref: "paper",
        },
        xaxis: {
            title: {
                text: "x",
                font: {
                    family: "Ubuntu",
                    size: 18,
                    color: "#7f7f7f",
                },
            },
        },
        yaxis: {
            title: {
                text: "y",
                font: {
                    family: "Ubuntu",
                    size: 18,
                    color: "#7f7f7f",
                },
            },
        },
    };
    // @ts-expect-error
    Plotly.newPlot(plot, data, layout);

    if (bernsteinPlot)
        drawNurbsBasisFuncs(nurbs, bernsteinPlot)
}

export function drawNurbsBasisFuncs(nurbs: NurbsCurve, plot: string) {
    // @ts-expect-error
    const xiValues = math.range(0, 1, 0.001).toArray()
    const data = []
    const n = nurbs.knotVector.length - nurbs.p - 2

    for (let i = 0; i <= n; i++) {
        const upsiValues = []
        xiValues.map((xi: number) => {
            upsiValues.push(NurbsCurve.computeBasis(
                new RowVector(nurbs.knotVector),
                new RowVector(nurbs.weights),
                i, nurbs.p, xi))
        })
        const trace = {
            x: xiValues,
            y: upsiValues,
            name: "N<sub>" + i + "</sub><sup>" + nurbs.p + "</sup>"
        }
        data.push(trace)
    }

    var layout = {
        title: {
            text: "NURBS basis functions",
            font: {
                family: "Ubuntu",
                size: 24,
            },
            xref: "paper",
        },
        xaxis: {
            title: {
                text: "\u03BE",
                font: {
                    family: "Ubuntu",
                    size: 18,
                    color: "#7f7f7f",
                },
            },
        },
        yaxis: {
            title: {
                text: "\uD835\uDF10",
                font: {
                    family: "Ubuntu",
                    size: 18,
                    color: "#7f7f7f",
                },
            },
        },
    };

    // @ts-expect-error
    Plotly.newPlot(plot, data, layout);
}
