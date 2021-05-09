import { Point } from "../core/point"
import { BezierSurf } from "./bezier"

/**
 * Draws the bezier surface into a plot.
 *
 * @param controlPoints
 * @param plot
 */
export let drawBezierSurf = (controlPoints: Point[][], drawControlPoints: boolean, plot: string) => {
    const bezier = new BezierSurf(controlPoints)
    // @ts-expect-error
    const xiValues = math.range(0, 1, 0.001).toArray()
    // @ts-expect-error
    const etaValues = math.range(0, 1, 0.001).toArray()
    let xValues = []
    let yValues = []
    let zValues = []
    xiValues.map((xi: number) => {
        etaValues.map((eta: number) => {
            let p = bezier.evaluate(xi, eta)
            xValues.push(p.x)
            yValues.push(p.y)
            zValues.push(p.z)
        })
    })
    let min = zValues.reduce(function(a, b) {
        return Math.min(a, b);
    });
    let max = zValues.reduce(function(a, b) {
        return Math.max(a, b);
    });
    const trace1 = {
        x: xValues,
        y: yValues,
        z: zValues,
        mode: "lines",
        type: "scatter3d",
        name: "Bezier surface",
        line: {
            color: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            colorscale: "viridis"
        }
    }
    const data = [ trace1 ]

    /*if (drawControlPoints) {
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
    }*/
    
    var layout = {
        title: {
            text: "Bezier Curve",
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
    Plotly.newPlot(plot, data, layout)
}