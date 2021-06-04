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
define("core/factorial", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.factorial = void 0;
    let _factorial = [];
    /**
     * Computes the factorial of n.
     * @param n
     * @returns
     */
    let factorial = (n) => {
        if (n == 0 || n == 1)
            return 1;
        if (_factorial[n] > 0)
            return _factorial[n];
        return (_factorial[n] = exports.factorial(n - 1) * n);
    };
    exports.factorial = factorial;
});
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
define("bezier/bernstein", ["require", "exports", "core/factorial"], function (require, exports, factorial_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.bernstein = void 0;
    let bernstein = (i, n, xi) => {
        return ((factorial_1.factorial(n) * Math.pow(xi, i) * Math.pow(1 - xi, n - i)) /
            (factorial_1.factorial(i) * factorial_1.factorial(n - i)));
    };
    exports.bernstein = bernstein;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.27
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
define("core/iequatable", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.27
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
define("core/size", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.Size = void 0;
    /**
     * Represents a 2D size.
     */
    class Size {
        /**
         * Ctor.
         *
         * @param width
         * @param height
         */
        constructor(width, height) {
            this.width = width;
            this.height = height;
        }
        /**
         * Equatable interface.
         *
         * @param o
         * @returns
         */
        equals(o) {
            return this.width != o.width || this.height != o.height;
        }
    }
    exports.Size = Size;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.27
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
define("core/matrix", ["require", "exports", "core/size"], function (require, exports, size_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.Matrix2 = void 0;
    /**
     * Class representing a matrix.
     */
    class Matrix2 {
        /**
         * Ctor.
         *
         * @param value
         */
        constructor(value) {
            if (typeof value == "number")
                this.m_data = Matrix2.identity(value).m_data;
            else
                this.m_data = value;
        }
        /**
         * Returns internal data.
         *
         * @returns
         */
        data() { return this.m_data; }
        /**
         * Returns the i-th row.
         *
         * @param i
         */
        row(i) { return new Matrix2([this.m_data[i]]); }
        /**
         * Returns a subrect of this matrix.
         *
         * @param topLeft
         * @param bottomRight
         * @returns
         */
        rect(topLeft, bottomRight) {
            let cols = bottomRight.x - topLeft.x + 1;
            let rows = bottomRight.y - topLeft.y + 1;
            let retData = Matrix2.createEmptyMatrixOfSize(rows, cols);
            for (let j = topLeft.x; j <= bottomRight.x; j++)
                for (let i = topLeft.y; i <= bottomRight.y; i++)
                    retData[i - topLeft.y][j - topLeft.x] = this.m_data[i][j];
            return new Matrix2(retData);
        }
        /**
         * Returns the j-th column.
         *
         * @param j
         * @returns
         */
        col(j) {
            let ret = [];
            for (let i = 0; i < this.rows(); i++)
                ret.push(this.m_data[i][j]);
            return new Matrix2([ret]);
        }
        /**
         * Number of rows.
         *
         * @returns
         */
        rows() { return this.m_data.length; }
        /**
         * Number of columns.
         *
         * @returns
         */
        cols() { return this.m_data[0].length; }
        /**
         * Returns the size.
         *
         * @returns
         */
        size() { return new size_1.Size(this.rows(), this.cols()); }
        /**
         * Accesses the element at specified position.
         *
         * @param row
         * @param col
         * @returns
         */
        value(row, col) { return this.m_data[row][col]; }
        /**
         * Sets the value of an element of the matrix.
         *
         * @param row
         * @param col
         * @param val
         */
        setValue(row, col, val) { this.m_data[row][col] = val; }
        /**
         * Logs the matrix.
         */
        print() { console.table(this.m_data); }
        /**
         * Adds the matrix to this matrix.
         *
         * @param m
         */
        add(m) {
            if (m.size().equals(this.size()))
                throw new Error("Cannot add matrices of different sizes");
            for (let i = 0; i < this.rows(); i++)
                for (let j = 0; j < this.cols(); j++)
                    this.m_data[i][j] += m.value(i, j);
            return this;
        }
        /**
         * Multiplies by a scalar.
         *
         * @param scalar
         * @returns
         */
        mult(scalar) {
            for (let i = 0; i < this.rows(); i++)
                for (let j = 0; j < this.cols(); j++)
                    this.m_data[i][j] *= scalar;
            return this;
        }
        /**
         * Multiplication by a matrix. Returns a new matrix with the result.
         *
         * @param m
         */
        multMat(m) {
            if (this.cols() != m.rows())
                throw new Error("Invalid mat sizes: " + this.size() + "·" + m.size());
            let res = Matrix2.zero(this.rows(), m.cols());
            for (let i = 0; i < this.rows(); i++) {
                for (let j = 0; j < m.cols(); j++) {
                    let e = 0;
                    for (let p = 0; p < this.cols(); p++)
                        e += this.value(i, p) * m.value(p, j);
                    res.m_data[i][j] = e;
                }
            }
            return res;
        }
        /**
         * Transposes this matrix.
         */
        transpose() {
            let newMatrix = this.transposed();
            this.m_data = newMatrix.m_data;
            return this;
        }
        /**
         * Returns a new matrix that is the transposed of this matrix.
         *
         * @returns
         */
        transposed() {
            let oldData = this.m_data;
            let newData = Matrix2.createEmptyMatrixOfSize(this.cols(), this.rows());
            for (let i = 0; i < this.rows(); i++)
                for (let j = 0; j < this.cols(); j++)
                    newData[j][i] = oldData[i][j];
            return new Matrix2(newData);
        }
        /**
         * IEquatable interface.
         *
         * @param m
         * @returns
         */
        equals(m) {
            if (this.m_data.length != m.m_data.length)
                return false;
            if (this.m_data.length <= 0)
                return true;
            if (this.m_data[0].length != m.m_data[0].length)
                return false;
            for (let i = 0; i < this.m_data.length; i++)
                for (let j = 0; j < this.m_data[i].length; j++)
                    if (this.m_data[i][j] != m.m_data[i][j])
                        return false;
            return true;
        }
        /**
         * Clones this matrix.
         *
         * @returns
         */
        clone() {
            return new Matrix2(this.m_data.map(function (arr) {
                return arr.slice();
            }));
        }
        /**
         * Create identity matrix of size size.
         *
         * @param size
         * @returns
         */
        static identity(size) {
            let values = Matrix2.createEmptyMatrixOfSize(size, size);
            for (let i = 0; i < size; i++)
                for (let j = 0; j < size; j++)
                    values[i][j] = (i === j ? 1 : 0);
            return new Matrix2(values);
        }
        /**
         * Returns a null matrix of a given size.
         *
         * @param size
         * @returns
         */
        static zeroSquare(size) {
            return this.zero(size, size);
        }
        /**
         * Returns a null matrix of a given size.
         *
         * @param size
         * @returns
         */
        static zero(rows, cols) {
            let values = Matrix2.createEmptyMatrixOfSize(rows, cols);
            for (let i = 0; i < rows; i++)
                for (let j = 0; j < cols; j++)
                    values[i][j] = 0;
            return new Matrix2(values);
        }
        /**
         * Adds two matrices.
         *
         * @param m1
         * @param m2
         * @returns
         */
        static add(m1, m2) {
            if (m1.size().equals(m2.size()))
                throw new Error("Cannot add matrices of different sizes");
            return m1.clone().add(m2);
        }
        /**
         * Multiplies by a scalar.
         *
         * @param m
         * @param scalar
         * @returns
         */
        static mult(m, scalar) {
            return m.clone().mult(scalar);
        }
        // Private portion
        /**
         * Creates empty structure.
         *
         * @param rows
         * @param cols
         * @returns
         */
        static createEmptyMatrixOfSize(rows, cols) {
            let values = new Array(rows);
            for (let i = 0; i < rows; i++)
                values[i] = new Array(cols);
            return values;
        }
    }
    exports.Matrix2 = Matrix2;
});
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
define("core/point", ["require", "exports", "core/matrix"], function (require, exports, matrix_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.Point = void 0;
    /**
     * Class representing a point on a surface.
     */
    class Point extends matrix_1.Matrix2 {
        /**
         * Ctor.
         */
        constructor(x, y, z = 0) {
            super([[x, y, z]]);
            this.x = x;
            this.y = y;
            this.z = z;
        }
    }
    exports.Point = Point;
});
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
define("bezier/bezier", ["require", "exports", "bezier/bernstein", "core/point"], function (require, exports, bernstein_1, point_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.BezierSurf = exports.BezierCurve = void 0;
    /**
     * Class representing a Bezier curve in the 2D or 3D space.
     */
    class BezierCurve {
        /**
         * Ctor.
         *
         * @param controlPoints
         */
        constructor(controlPoints) {
            this.controlPoints = controlPoints;
        }
        /**
         * Evaluates the value of the Bezier curve in the parametric space.
         *
         * @param xi
         * @returns
         */
        evaluate(xi) {
            let x = 0;
            let y = 0;
            let z = 0;
            let n = this.controlPoints.length;
            for (let i = 0; i < n; i++) {
                x = x + bernstein_1.bernstein(i, n - 1, xi) * this.controlPoints[i].x;
                y = y + bernstein_1.bernstein(i, n - 1, xi) * this.controlPoints[i].y;
                z = z + bernstein_1.bernstein(i, n - 1, xi) * this.controlPoints[i].z;
            }
            return new point_1.Point(x, y, z);
        }
    }
    exports.BezierCurve = BezierCurve;
    /**
     * Class representing a surface.
     */
    class BezierSurf {
        /**
         * Ctor.
         *
         * @param controlPoints
         */
        constructor(controlPoints) {
            this.controlPoints = controlPoints;
        }
        /**
         * Evaluates the surface in (xi, eta).
         *
         * @param xi
         * @param eta
         * @returns
         */
        evaluate(xi, eta) {
            let x = 0;
            let y = 0;
            let z = 0;
            let n = this.controlPoints.length;
            let m = this.controlPoints[0].length;
            for (let i = 0; i < n; i++) {
                for (let j = 0; j < m; j++) {
                    x += bernstein_1.bernstein(i, n - 1, xi) * bernstein_1.bernstein(j, m - 1, eta) * this.controlPoints[i][j].x;
                    y += bernstein_1.bernstein(i, n - 1, xi) * bernstein_1.bernstein(j, m - 1, eta) * this.controlPoints[i][j].y;
                    z += bernstein_1.bernstein(i, n - 1, xi) * bernstein_1.bernstein(j, m - 1, eta) * this.controlPoints[i][j].z;
                }
            }
            return new point_1.Point(x, y, z);
        }
    }
    exports.BezierSurf = BezierSurf;
});
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
define("bezier/drawBezierCurve", ["require", "exports", "bezier/bezier", "bezier/bernstein"], function (require, exports, bezier_1, bernstein_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBernsteinPolynomials = exports.drawBezierCurve = void 0;
    /**
     * Draws the bezier curve into a plot.
     *
     * @param controlPoints
     * @param plot
     */
    let drawBezierCurve = (controlPoints, threed, drawControlPoints, plot, bernsteinPlot = null) => {
        const bezier = new bezier_1.BezierCurve(controlPoints);
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.001).toArray();
        let xValues = [];
        let yValues = [];
        let zValues = [];
        xiValues.map((xi) => {
            let p = bezier.evaluate(xi);
            xValues.push(p.x);
            yValues.push(p.y);
            zValues.push(p.z);
        });
        const plotType = threed ? "scatter3d" : "scatter";
        const trace1 = {
            x: xValues,
            y: yValues,
            z: zValues,
            type: plotType,
            name: "Bezier curve",
            mode: "lines",
            line: {
                color: "orange",
                width: 4
            }
        };
        const data = [trace1];
        if (drawControlPoints) {
            const cpXValues = [];
            const cpYValues = [];
            const cpZValues = [];
            for (let cp of controlPoints) {
                cpXValues.push(cp.x);
                cpYValues.push(cp.y);
                cpZValues.push(cp.z);
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
            };
            data.push(trace2);
        }
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
        Plotly.newPlot(plot, data, layout);
        if (bernsteinPlot)
            exports.drawBernsteinPolynomials(controlPoints.length, bernsteinPlot);
    };
    exports.drawBezierCurve = drawBezierCurve;
    let drawBernsteinPolynomials = (n, plot) => {
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.001).toArray();
        const data = [];
        for (let i = 0; i <= n; i++) {
            const upsiValues = [];
            xiValues.map((xi) => {
                upsiValues.push(bernstein_2.bernstein(i, n, xi));
            });
            const trace = {
                x: xiValues,
                y: upsiValues,
                name: "B<sub>" + i + "</sub>"
            };
            data.push(trace);
        }
        var layout = {
            title: {
                text: "Bernstein Polynomials",
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
    };
    exports.drawBernsteinPolynomials = drawBernsteinPolynomials;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.25
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
define("examples/exampleCurves", ["require", "exports", "core/point"], function (require, exports, point_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.exampleCurve3D1 = exports.exampleCurve2D1 = void 0;
    exports.exampleCurve2D1 = [
        new point_2.Point(0, 0),
        new point_2.Point(1, 1),
        new point_2.Point(2, 0.5),
        new point_2.Point(3, 0.5),
        new point_2.Point(0.5, 1.5),
        new point_2.Point(1.5, 0)
    ];
    exports.exampleCurve3D1 = [
        new point_2.Point(0, 0, 0),
        new point_2.Point(1, 1, 1),
        new point_2.Point(2, 0.5, 0),
        new point_2.Point(3, 0.5, 0),
        new point_2.Point(0.5, 1.5, 0),
        new point_2.Point(1.5, 0, 1)
    ];
});
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
define("bezier/drawBezierCurveExample", ["require", "exports", "bezier/drawBezierCurve", "examples/exampleCurves"], function (require, exports, drawBezierCurve_1, exampleCurves_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBezierCurve2 = exports.drawBezierCurve1 = void 0;
    let drawBezierCurve1 = (plot, drawControlPoints, bernsteinPlot) => {
        drawBezierCurve_1.drawBezierCurve(exampleCurves_1.exampleCurve2D1, false, drawControlPoints, plot, bernsteinPlot);
    };
    exports.drawBezierCurve1 = drawBezierCurve1;
    let drawBezierCurve2 = (plot, drawControlPoints, bernsteinPlot) => {
        drawBezierCurve_1.drawBezierCurve(exampleCurves_1.exampleCurve3D1, true, drawControlPoints, plot, bernsteinPlot);
    };
    exports.drawBezierCurve2 = drawBezierCurve2;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.11
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
define("bezier/drawBezierSurf", ["require", "exports", "bezier/bezier"], function (require, exports, bezier_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBezierSurf = void 0;
    /**
     * Draws the bezier surface into a plot.
     *
     * @param controlPoints
     * @param plot
     */
    let drawBezierSurf = (controlPoints, drawControlPoints, plot) => {
        const scaleZ = 3;
        const bezier = new bezier_2.BezierSurf(controlPoints);
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.005).toArray();
        // @ts-expect-error
        const etaValues = math.range(0, 1, 0.005).toArray();
        let xValues = [];
        let yValues = [];
        let zValues = [];
        xiValues.map((xi) => {
            etaValues.map((eta) => {
                let p = bezier.evaluate(xi, eta);
                xValues.push(p.x);
                yValues.push(p.y);
                zValues.push(p.z / scaleZ);
            });
        });
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
        };
        const data = [trace1];
        if (drawControlPoints) {
            for (let cpxs of controlPoints) {
                const cpXValues = [];
                const cpYValues = [];
                const cpZValues = [];
                for (let controlPoint of cpxs) {
                    cpXValues.push(controlPoint.x);
                    cpYValues.push(controlPoint.y);
                    cpZValues.push(controlPoint.z / scaleZ);
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
                };
                data.push(trace);
            }
            for (let j = 0; j < controlPoints[0].length; j++) {
                const cpXValues = [];
                const cpYValues = [];
                const cpZValues = [];
                for (let i = 0; i < controlPoints.length; i++) {
                    cpXValues.push(controlPoints[i][j].x);
                    cpYValues.push(controlPoints[i][j].y);
                    cpZValues.push(controlPoints[i][j].z / scaleZ);
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
                    };
                    data.push(trace);
                }
            }
        }
        var layout = {
            title: {
                text: "Bezier Surface",
                font: {
                    family: "Ubuntu",
                    size: 24,
                }
            },
            height: 700,
            showlegend: false,
            xaxis: {
                title: {
                    text: "x",
                    font: {
                        family: "Ubuntu",
                        size: 18,
                        color: "#7f7f7f",
                    }
                }
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
    };
    exports.drawBezierSurf = drawBezierSurf;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.25
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
define("examples/exampleSurfs", ["require", "exports", "core/point"], function (require, exports, point_3) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.exampleSurf1ControlPoints = void 0;
    exports.exampleSurf1ControlPoints = [[
            new point_3.Point(-3, 0, 2),
            new point_3.Point(-2, 0, 6),
            new point_3.Point(-1, 0, 7),
            new point_3.Point(0, 0, 2),
        ], [
            new point_3.Point(-3, 1, 2),
            new point_3.Point(-2, 1, 4),
            new point_3.Point(-1, 1, 5),
            new point_3.Point(0, 1, 2.5),
        ], [
            new point_3.Point(-3, 3, 0),
            new point_3.Point(-2, 3, 2.5),
            new point_3.Point(-1, 3, 4.5),
            new point_3.Point(0, 3, 6.5),
        ],
    ];
});
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
define("bezier/drawBezierSurfExample", ["require", "exports", "bezier/drawBezierSurf", "examples/exampleSurfs"], function (require, exports, drawBezierSurf_1, exampleSurfs_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBezierSurfExample = void 0;
    let drawBezierSurfExample = (plot, drawControlPoints) => {
        drawBezierSurf_1.drawBezierSurf(exampleSurfs_1.exampleSurf1ControlPoints, drawControlPoints, plot);
    };
    exports.drawBezierSurfExample = drawBezierSurfExample;
});
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
define("bspline/bspline", ["require", "exports", "core/matrix", "core/point"], function (require, exports, matrix_2, point_4) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.BsplineSurf = exports.BsplineCurve = void 0;
    /**
     * Class representing a B-spline curve in the 2D or 3D space.
     */
    class BsplineCurve {
        /**
         * Ctor.
         *
         * @param controlPoints
         * @param knotVector
         * @param p
         */
        constructor(controlPoints, knotVector, p) {
            this.controlPoints = controlPoints;
            this.knotVector = knotVector;
            this.p = p;
        }
        /**
         * Evaluates the value of the B-spline curve in the parametric space.
         *
         * @param xi
         * @returns
         */
        evaluate(xi) {
            let x = 0;
            let y = 0;
            let z = 0;
            let n = this.controlPoints.length - 1;
            let span = BsplineCurve.findSpan(this.knotVector, xi, this.p, n);
            let N = BsplineCurve.computeAllNonvanishingBasis(this.knotVector, span, this.p, xi);
            for (let i = 0; i <= this.p; i++) {
                x = x + N[i] * this.controlPoints[span - this.p + i].x;
                y = y + N[i] * this.controlPoints[span - this.p + i].y;
                z = z + N[i] * this.controlPoints[span - this.p + i].z;
            }
            return new point_4.Point(x, y, z);
        }
        /**
         * Computes all non-vanishing bspline basis functions in xi.
         *
         * @param i
         * @param p
         * @param xi
         * @param Xi
         * @returns
         */
        static computeAllNonvanishingBasis(Xi, i, p, xi) {
            let N = Array(p + 1).fill(0);
            let right = Array(p + 1).fill(0);
            let left = Array(p + 1).fill(0);
            N[0] = 1;
            for (let j = 1; j <= p; j++) {
                left[j] = xi - Xi[i + 1 - j];
                right[j] = Xi[i + j] - xi;
                let saved = 0;
                let temp = 0;
                for (let r = 0; r < j; r++) {
                    temp = N[r] / (right[r + 1] + left[j - r]);
                    N[r] = saved + right[r + 1] * temp;
                    saved = left[j - r] * temp;
                }
                N[j] = saved;
            }
            return new matrix_2.Matrix2([N]);
        }
        static computeBasis(Xi, i, p, xi) {
            let n = Xi.length - 1;
            // Check to see if we're evaluating the first or the last basis function at
            // the beginning or at the end of the knot vector.
            if ((i == 0 && xi == Xi[0]) || (i == n - p - 1 && xi == Xi[n]))
                return 1;
            // When xi is out of the domain it is set to zero.
            if (xi < Xi[i] || xi >= Xi[i + p + 1])
                return 0;
            // Preallocation and computation of the temparary values of the functions to
            // be used according to the triangular table.        
            let N = Array(p + 1).fill(0);
            for (let j = 0; j <= p; j++) {
                if (xi >= Xi[i + j] && xi < Xi[i + j + 1])
                    N[j] = 1;
                else
                    N[j] = 0;
            }
            // Computation of the rest of the triangular table.
            let saved;
            for (let k = 1; k <= p; k++) {
                if (N[0] == 0)
                    saved = 0;
                else
                    saved = ((xi - Xi[i]) * N[0]) / (Xi[i + k] - Xi[i]);
                for (let j = 0; j <= p - k - 1 + 1; j++) {
                    let Xileft = Xi[i + j + 1];
                    let Xiright = Xi[i + j + k + 1];
                    if (N[j + 1] == 0) {
                        N[j] = saved;
                        saved = 0;
                    }
                    else {
                        let temp = N[j + 1] / (Xiright - Xileft);
                        N[j] = saved + (Xiright - xi) * temp;
                        saved = (xi - Xileft) * temp;
                    }
                }
            }
            return N[0];
        }
        /**
         * Finds the span in which xi lies.
         *
         * @param Xi
         * @param xi
         * @param p
         * @param n
         * @returns
         */
        static findSpan(Xi, xi, p, n) {
            if (xi == Xi[n + 1])
                return n;
            let low = p;
            let high = n + 1;
            let i = Math.floor((low + high) / 2);
            while (xi < Xi[i] || xi >= Xi[i + 1]) {
                if (xi < Xi[i])
                    high = i;
                else
                    low = i;
                i = Math.floor((low + high) / 2);
            }
            return i;
        }
    }
    exports.BsplineCurve = BsplineCurve;
    /**
     * Represents a b-spline surface.
     */
    class BsplineSurf {
        /**
         * Ctor.
         *
         * @param controlPoints
         * @param Xi
         * @param Eta
         * @param p
         * @param q
         */
        constructor(controlPoints, Xi, Eta, p, q) {
            this.controlPoints = controlPoints;
            this.Xi = Xi;
            this.Eta = Eta;
            this.p = p;
            this.q = q;
        }
        /**
         * Evaluates the surf in (xi, eta).
         *
         * @param xi
         * @param eta
         * @returns
         */
        evaluate(xi, eta) {
            return this.evaluate2(xi, eta);
        }
        /**
         * Evaluation in the summation form.
         *
         * @param xi
         * @param eta
         * @returns
         */
        evaluate1(xi, eta) {
            let n = this.controlPoints.length - 1;
            let m = this.controlPoints[0].length - 1;
            let x = 0;
            let y = 0;
            let z = 0;
            for (let i = 0; i <= n; i++) {
                for (let j = 0; j <= m; j++) {
                    let Nxi = BsplineCurve.computeBasis(this.Xi, i, this.p, xi);
                    let Neta = BsplineCurve.computeBasis(this.Eta, j, this.q, eta);
                    let prod = Nxi * Neta;
                    x = x + prod * this.controlPoints[i][j].x;
                    y = y + prod * this.controlPoints[i][j].y;
                    z = z + prod * this.controlPoints[i][j].z;
                }
            }
            return new point_4.Point(x, y, z);
        }
        /**
         * Evaluation in matrix form.
         *
         * @param xi
         * @param eta
         * @returns
         */
        evaluate2(xi, eta) {
            let n = this.controlPoints.length - 1;
            let m = this.controlPoints[0].length - 1;
            let xiSpan = BsplineCurve.findSpan(this.Xi, xi, this.p, n);
            let etaSpan = BsplineCurve.findSpan(this.Eta, eta, this.q, m);
            let Nxi = BsplineCurve.computeAllNonvanishingBasis(this.Xi, xiSpan, this.p, xi);
            let Neta = BsplineCurve.computeAllNonvanishingBasis(this.Eta, etaSpan, this.q, eta);
            let Px = this.matFromPoints("x");
            let Py = this.matFromPoints("y");
            let Pz = this.matFromPoints("z");
            let sx = Nxi.multMat(Px.rect(new point_4.Point(etaSpan - this.q, xiSpan - this.p), new point_4.Point(etaSpan, xiSpan)))
                .multMat(Neta.transposed());
            let sy = Nxi.multMat(Py.rect(new point_4.Point(etaSpan - this.q, xiSpan - this.p), new point_4.Point(etaSpan, xiSpan)))
                .multMat(Neta.transposed());
            let sz = Nxi.multMat(Pz.rect(new point_4.Point(etaSpan - this.q, xiSpan - this.p), new point_4.Point(etaSpan, xiSpan)))
                .multMat(Neta.transposed());
            return new point_4.Point(sx.value(0, 0), sy.value(0, 0), sz.value(0, 0));
        }
        /**
         * Builds a matrix from a matrix of points.
         *
         * @param d
         * @returns
         */
        matFromPoints(d) {
            let d1 = this.controlPoints.length;
            let d2 = this.controlPoints[0].length;
            let d3 = d;
            let m = matrix_2.Matrix2.zero(d1, d2);
            for (let i = 0; i < d1; i++) {
                for (let j = 0; j < d2; j++) {
                    m.setValue(i, j, this.controlPoints[i][j][d3]);
                }
            }
            return m;
        }
    }
    exports.BsplineSurf = BsplineSurf;
});
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
define("bspline/drawBsplineCurve", ["require", "exports", "bspline/bspline"], function (require, exports, bspline_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBsplineBasisFuncs = exports.drawBsplineCurve = void 0;
    /**
    * Draws the b-spline curve into a plot.
    *
    * @param controlPoints
    * @param plot
    */
    let drawBsplineCurve = (controlPoints, knotVector, p, threed, drawControlPoints, plot, bernsteinPlot = null) => {
        const bspline = new bspline_1.BsplineCurve(controlPoints, knotVector, p);
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.001).toArray();
        let xValues = [];
        let yValues = [];
        let zValues = [];
        xiValues.map((xi) => {
            let p = bspline.evaluate(xi);
            xValues.push(p.x);
            yValues.push(p.y);
            zValues.push(p.z);
        });
        const plotType = threed ? "scatter3d" : "scatter";
        const trace1 = {
            x: xValues,
            y: yValues,
            z: zValues,
            type: plotType,
            name: "B-spline curve",
            mode: "lines",
            line: {
                color: "orange",
                width: 4
            }
        };
        const data = [trace1];
        if (drawControlPoints) {
            const cpXValues = [];
            const cpYValues = [];
            const cpZValues = [];
            for (let cp of controlPoints) {
                cpXValues.push(cp.x);
                cpYValues.push(cp.y);
                cpZValues.push(cp.z);
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
            };
            data.push(trace2);
        }
        var layout = {
            title: {
                text: "B-spline curve",
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
            drawBsplineBasisFuncs(bspline, bernsteinPlot);
    };
    exports.drawBsplineCurve = drawBsplineCurve;
    function drawBsplineBasisFuncs(bspline, plot) {
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.001).toArray();
        const data = [];
        const n = bspline.knotVector.length - bspline.p - 2;
        for (let i = 0; i <= n; i++) {
            const upsiValues = [];
            xiValues.map((xi) => {
                upsiValues.push(bspline_1.BsplineCurve.computeBasis(bspline.knotVector, i, bspline.p, xi));
            });
            const trace = {
                x: xiValues,
                y: upsiValues,
                name: "N<sub>" + i + "</sub><sup>" + bspline.p + "</sup>"
            };
            data.push(trace);
        }
        var layout = {
            title: {
                text: "B-spline basis functions",
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
    exports.drawBsplineBasisFuncs = drawBsplineBasisFuncs;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.15
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
define("bspline/drawBsplineCurveExample", ["require", "exports", "core/point", "bspline/drawBsplineCurve"], function (require, exports, point_5, drawBsplineCurve_1) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBsplineCurve2 = exports.drawBsplineCurve1 = void 0;
    let drawBsplineCurve1 = (plot, drawControlPoints, bernsteinPlot) => {
        let controlPoints = [];
        controlPoints.push(new point_5.Point(0, 0));
        controlPoints.push(new point_5.Point(1, 1));
        controlPoints.push(new point_5.Point(2, 0.5));
        controlPoints.push(new point_5.Point(3, 0.5));
        controlPoints.push(new point_5.Point(0.5, 1.5));
        controlPoints.push(new point_5.Point(1.5, 0));
        let knotVector = [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1];
        drawBsplineCurve_1.drawBsplineCurve(controlPoints, knotVector, 2, false, drawControlPoints, plot, bernsteinPlot);
    };
    exports.drawBsplineCurve1 = drawBsplineCurve1;
    let drawBsplineCurve2 = (plot, drawControlPoints, bernsteinPlot) => {
        let controlPoints = [];
        controlPoints.push(new point_5.Point(0, 0, 0));
        controlPoints.push(new point_5.Point(1, 1, 1));
        controlPoints.push(new point_5.Point(2, 0.5, 0));
        controlPoints.push(new point_5.Point(3, 0.5, 0));
        controlPoints.push(new point_5.Point(0.5, 1.5, 0));
        controlPoints.push(new point_5.Point(1.5, 0, 1));
        let knotVector = [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1];
        drawBsplineCurve_1.drawBsplineCurve(controlPoints, knotVector, 2, true, drawControlPoints, plot, bernsteinPlot);
    };
    exports.drawBsplineCurve2 = drawBsplineCurve2;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.24
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
define("bspline/drawBsplineSurf", ["require", "exports", "bspline/bspline"], function (require, exports, bspline_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBsplineSurf = void 0;
    /**
     * Draws the b-spline surface into a plot.
     *
     * @param controlPoints
     * @param plot
     */
    let drawBsplineSurf = (controlPoints, Xi, Eta, p, q, drawControlPoints, plot) => {
        const scaleZ = 3;
        const bspline = new bspline_2.BsplineSurf(controlPoints, Xi, Eta, p, q);
        // @ts-expect-error
        const xiValues = math.range(0, 1, 0.005).toArray();
        // @ts-expect-error
        const etaValues = math.range(0, 1, 0.005).toArray();
        let xValues = [];
        let yValues = [];
        let zValues = [];
        xiValues.map((xi) => {
            etaValues.map((eta) => {
                let p = bspline.evaluate(xi, eta);
                xValues.push(p.x);
                yValues.push(p.y);
                zValues.push(p.z / scaleZ);
            });
        });
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
        };
        const data = [trace1];
        if (drawControlPoints) {
            for (let cpxs of controlPoints) {
                const cpXValues = [];
                const cpYValues = [];
                const cpZValues = [];
                for (let controlPoint of cpxs) {
                    cpXValues.push(controlPoint.x);
                    cpYValues.push(controlPoint.y);
                    cpZValues.push(controlPoint.z / scaleZ);
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
                };
                data.push(trace);
            }
            for (let j = 0; j < controlPoints[0].length; j++) {
                const cpXValues = [];
                const cpYValues = [];
                const cpZValues = [];
                for (let i = 0; i < controlPoints.length; i++) {
                    cpXValues.push(controlPoints[i][j].x);
                    cpYValues.push(controlPoints[i][j].y);
                    cpZValues.push(controlPoints[i][j].z / scaleZ);
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
                    };
                    data.push(trace);
                }
            }
        }
        var layout = {
            title: {
                text: "B-spline Surface",
                font: {
                    family: "Ubuntu",
                    size: 24,
                }
            },
            height: 700,
            showlegend: false,
            xaxis: {
                title: {
                    text: "x",
                    font: {
                        family: "Ubuntu",
                        size: 18,
                        color: "#7f7f7f",
                    }
                }
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
    };
    exports.drawBsplineSurf = drawBsplineSurf;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.24
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
define("bspline/drawBsplineSurfExample", ["require", "exports", "bspline/drawBsplineSurf", "examples/exampleSurfs"], function (require, exports, drawBsplineSurf_1, exampleSurfs_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.drawBsplineSurfExample2 = exports.drawBsplineSurfExample1 = void 0;
    function drawBsplineSurfExample1(plot, drawControlPoints) {
        let p = 1;
        let q = 1;
        let Xi = [0, 0, 0.5, 1, 1];
        let Eta = [0, 0, 0.3, 0.6, 1, 1];
        drawBsplineSurf_1.drawBsplineSurf(exampleSurfs_2.exampleSurf1ControlPoints, Xi, Eta, p, q, drawControlPoints, plot);
    }
    exports.drawBsplineSurfExample1 = drawBsplineSurfExample1;
    function drawBsplineSurfExample2(plot, drawControlPoints) {
        let p = 1;
        let q = 2;
        let Xi = [0, 0, 0.5, 1, 1];
        let Eta = [0, 0, 0, 0.5, 1, 1, 1];
        drawBsplineSurf_1.drawBsplineSurf(exampleSurfs_2.exampleSurf1ControlPoints, Xi, Eta, p, q, drawControlPoints, plot);
    }
    exports.drawBsplineSurfExample2 = drawBsplineSurfExample2;
});
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
define("core/math", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.in_range = void 0;
    let in_range = (val, a, b, openLeft = false, openRight = false) => {
        return val >= a && val <= b;
    };
    exports.in_range = in_range;
});
/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.05.19
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
define("core/range", ["require", "exports"], function (require, exports) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    exports.Range = void 0;
    class Range {
        constructor(a, b) {
            this.a = a;
            this.b = b;
        }
    }
    exports.Range = Range;
});
define("test/matrix_test", ["require", "exports", "core/matrix", "core/point", "core/size"], function (require, exports, matrix_3, point_6, size_2) {
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    // @ts-expect-error
    var assert = require("assert");
    // Test sum 1
    {
        let m1 = new matrix_3.Matrix2([[1, 2, 3]]);
        m1.print();
        let m2 = new matrix_3.Matrix2([[1, 1, 1]]);
        m2.print();
        let m3 = matrix_3.Matrix2.add(m1, m2);
        m3.print();
        assert(!m3.equals(m2));
        assert(m3.equals(new matrix_3.Matrix2([[2, 3, 4]])));
    }
    // Test sum 2
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7]
        ]);
        let m2 = new matrix_3.Matrix2([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        let m3 = matrix_3.Matrix2.identity(3);
        let m4 = matrix_3.Matrix2.zeroSquare(3);
        let sum = matrix_3.Matrix2.zeroSquare(3).add(m1).add(m2).add(m3).add(m4);
        assert(sum.equals(new matrix_3.Matrix2([
            [7, 8, 10],
            [5, 8, 9],
            [16, 16, 17]
        ])));
        assert(m1.equals(new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7]
        ])));
        assert(m2.equals(new matrix_3.Matrix2([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ])));
        assert(m3.equals(matrix_3.Matrix2.identity(3)));
        sum.print();
    }
    // Test mult by scalar 1
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7]
        ]);
        assert(m1.mult(9).equals(new matrix_3.Matrix2([
            [5 * 9, 6 * 9, 7 * 9],
            [1 * 9, 2 * 9, 3 * 9],
            [9 * 9, 8 * 9, 7 * 9]
        ])));
    }
    // Test mult by a scalar 2
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7]
        ]);
        assert(m1.mult(0).equals(matrix_3.Matrix2.zeroSquare(3)));
    }
    // Test mult by matrix 1
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7]
        ]);
        let m2 = new matrix_3.Matrix2([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        let m3 = matrix_3.Matrix2.identity(3);
        let m4 = matrix_3.Matrix2.zeroSquare(3);
        assert(m1.multMat(m1).equals(new matrix_3.Matrix2([
            [94, 98, 102],
            [34, 34, 34],
            [116, 126, 136]
        ])));
        assert(m1.multMat(m2).equals(new matrix_3.Matrix2([
            [78, 96, 114],
            [30, 36, 42],
            [90, 114, 138]
        ])));
        assert(m1.multMat(m3).equals(m1));
        assert(m2.multMat(m3).equals(m2));
        assert(m1.multMat(m4).equals(m4));
    }
    // Test mult by matrix 2
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7],
            [1, 1, 1]
        ]);
        let m2 = new matrix_3.Matrix2([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        let m4 = matrix_3.Matrix2.zeroSquare(3);
        try {
            m1.multMat(m1);
            assert(false);
        }
        catch (e) {
            assert(true);
        }
        assert(m1.multMat(m2).equals(new matrix_3.Matrix2([
            [78, 96, 114],
            [30, 36, 42],
            [90, 114, 138],
            [12, 15, 18]
        ])));
        assert(m1.multMat(m4).equals(matrix_3.Matrix2.zero(m1.rows(), m2.cols())));
    }
    // Test mult by matrix 2
    {
        let m = new matrix_3.Matrix2([
            [1],
            [2],
            [3]
        ]);
        assert(m.transposed().size().equals(new size_2.Size(3, 1)));
        m.transpose();
        assert(m.equals(new matrix_3.Matrix2([[1, 2, 3]])));
    }
    // Test rect extraction
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7],
            [1, 1, 1]
        ]);
        let m2 = new matrix_3.Matrix2([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        assert(m1.rect(new point_6.Point(1, 1), new point_6.Point(2, 2)).equals(new matrix_3.Matrix2([
            [2, 3],
            [8, 7]
        ])));
    }
    // Test setValue
    {
        let m1 = new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 3],
            [9, 8, 7],
            [1, 1, 1]
        ]);
        m1.setValue(1, 2, 199);
        assert(m1.equals(new matrix_3.Matrix2([
            [5, 6, 7],
            [1, 2, 199],
            [9, 8, 7],
            [1, 1, 1]
        ])));
    }
});
//# sourceMappingURL=bundle.js.map