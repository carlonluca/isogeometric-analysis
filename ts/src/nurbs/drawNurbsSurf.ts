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

import { Matrix2, RowVector } from "../core/matrix"
import { Point } from "../core/point"
import { NurbsSurf } from "./nurbs"

/**
 * Draws a NURBS surface.
 * 
 * @param controlPoints 
 * @param Xi 
 * @param Eta 
 * @param w 
 * @param p 
 * @param q 
 * @param drawControlPoints 
 * @param plot 
 */
export let drawNurbsSurf = (
    controlPoints: Point[][],
    Xi: RowVector,
    Eta: RowVector,
    w: Matrix2,
    p: number,
    q: number,
    drawControlPoints: boolean,
    plot: string,
    sameScale: boolean = false,
    maxXi = 1,
    maxEta = 1,
    title: string = "NURBS surface") => {
    const nurbs = new NurbsSurf(controlPoints, Xi.toArray(), Eta.toArray(), w, p, q)
    // @ts-expect-error
    const xiValues = math.range(0, maxXi, 0.005*maxXi).toArray()
    // @ts-expect-error
    const etaValues = math.range(0, maxEta, 0.005*maxEta).toArray()
    let xValues = []
    let yValues = []
    let zValues = []
    xiValues.map((xi: number) => {
        etaValues.map((eta: number) => {
            let p = nurbs.evaluate(xi, eta)
            xValues.push(p.x())
            yValues.push(p.y())
            zValues.push(p.z())
        })
    })
    let min = zValues.reduce(function (a, b) {
        return Math.min(a, b);
    });
    let max = zValues.reduce(function (a, b) {
        return Math.max(a, b);
    });
    const trace1 = {
        x: xValues,
        y: yValues,
        z: zValues,
        mode: "markers",
        type: "scatter3d",
        marker: {
            size: 1,
            color: zValues.slice(0).sort(),
            colorscale: "Jet",
            cmin: min,
            cmax: max
        },
        line: {}
    }
    const data: any[] = [trace1]

    if (drawControlPoints) {
        for (let cpxs of controlPoints) {
            const cpXValues = []
            const cpYValues = []
            const cpZValues = []
            for (let controlPoint of cpxs) {
                cpXValues.push(controlPoint.x())
                cpYValues.push(controlPoint.y())
                cpZValues.push(controlPoint.z())
            }

            const trace = {
                x: cpXValues,
                y: cpYValues,
                z: cpZValues,
                name: "Control points",
                mode: 'lines',
                type: "scatter3d",
                line: {
                    color: "red",
                    width: 3,
                    dash: "dashdot"
                }
            }
            data.push(trace)
        }

        for (let j = 0; j < controlPoints[0].length; j++) {
            const cpXValues = []
            const cpYValues = []
            const cpZValues = []
            for (let i = 0; i < controlPoints.length; i++) {
                cpXValues.push(controlPoints[i][j].x())
                cpYValues.push(controlPoints[i][j].y())
                cpZValues.push(controlPoints[i][j].z())

                const trace = {
                    x: cpXValues,
                    y: cpYValues,
                    z: cpZValues,
                    name: "Control points",
                    mode: 'lines+markers',
                    type: "scatter3d",
                    line: {
                        color: "red",
                        width: 3,
                        dash: "dashdot"
                    },
                    marker: {
                        color: "transparent",
                        size: 8,
                        line: {
                            color: "black",
                            width: 2
                        }
                    }
                }
                data.push(trace)
            }
        }
    }

    var layout = {
        title: {
            text: title,
            font: {
                family: "Ubuntu",
                size: 24,
            }
        },
        height: 700,
        showlegend: false,
        scene: {
            aspectmode: "data",
            aspectratio: {
                x: 1,
                y: 1,
                z: 1
            },
            yaxis: {
                title: {
                    text: "y",
                    font: {
                        family: "Ubuntu",
                        size: 18,
                        color: "#7f7f7f"
                    },
                }
            },
            zaxis: {
                title: {
                    text: "z",
                    font: {
                        family: "Ubuntu",
                        size: 18,
                        color: "#7f7f7f"
                    }
                }
            }
        },
        margin: {
            l: 0,
            r: 0,
            b: 0
        }
    };
    // @ts-expect-error
    Plotly.newPlot(plot, data, layout)
}