/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.06.05
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
import { HomPoint, Point } from "../core/point"
import { BsplineCurve } from "../bspline/bspline"
import { Range } from "../core/range"

/**
 * Representation of a NURBS curve.
 */
export class NurbsCurve {
    /**
     * Ctor.
     *
     * @param controlPoints
     * @param knotVector
     * @param p
     */
    constructor(
        public controlPoints: Point[],
        public knotVector: number[],
        public weights: number[],
        public p: number
    ) { }

    /**
     * Computes the curve in xi.
     *
     * @param xi
     */
    public evaluate(xi: number): Point {
        let x = 0
        let y = 0
        let z = 0
        let n = this.controlPoints.length - 1
        let Xi = new RowVector(this.knotVector)
        let w = new RowVector(this.weights)
        for (let i = 0; i <= n; i++) {
            let N = NurbsCurve.computeBasis(Xi, w, i, this.p, xi)
            x = x + N * this.controlPoints[i].x()
            y = y + N * this.controlPoints[i].y()
            z = z + N * this.controlPoints[i].z()
        }

        return new Point(x, y, z)
    }

    /**
     * Implements knot insertion on this NURBS.
     * 
     * @param barxi value of the new knot to insert.
     * @param index where the new knot is to be inserted.
     * @param s initial multiplicity.
     * @param r multiplicity of the knot to add.
     */
    public insertKnot(barxi: number, index: number, s: number, r: number): NurbsCurve {
        let Xi = new RowVector(this.knotVector)
        let n = Xi.length() - this.p - 2
        let barXi = RowVector.zero(Xi.length() + r)
        let barN = n + r - s

        // Prepare the new knot vector.
        for (let i = 0; i <= index; i++)
            barXi.setValue(i, Xi.value(i))
        for (let i = index + 1; i <= index + r; i++)
            barXi.setValue(i, barxi)
        for (let i = index + r + 1; i < barXi.length(); i++)
            barXi.setValue(i, Xi.value(index + 1 + i - index - r - 1))
        
        let Pw = NurbsCurve.toWeightedControlPoints(this.controlPoints, new RowVector(this.weights))
        let barPw: HomPoint[] = new Array(barN)
        for (let i = 0; i <= barN; i++) {
            if (i <= index - this.p)
                barPw[i] = Pw[i].clone()
            else if (i <= index && i >= index - this.p + 1) {
                let alpha = (barxi - Xi.value(i))/(Xi.value(i + this.p) - Xi.value(i))
                let _Pw1 = Pw[i].clone().mult(alpha)
                let _Pw2 = Pw[i - 1].clone().mult(1 - alpha)
                barPw[i] = HomPoint.fromVector(_Pw1.add(_Pw2).row(0))
            }
            else
                barPw[i] = Pw[i - 1]
        }

        let [barP, barW] = NurbsCurve.fromWeightedControlPoints(barPw)
        this.controlPoints = barP
        this.knotVector = barXi.toArray()
        this.weights = barW.toArray()

        return this
    }

    /**
     * Computes the i-th NURBS basis function.
     *
     * @param Xi
     * @param w
     * @param i
     * @param p
     * @param xi
     * @returns
     */
    public static computeBasis(Xi: RowVector, w: RowVector, i: number, p: number, xi: number): number {
        let n = Xi.length() - 1
        let xiSpan = BsplineCurve.findSpan(Xi.toArray(), xi, p, n)
        if (i < xiSpan - p || i > xiSpan)
            return 0
        let N = BsplineCurve.computeAllNonvanishingBasis(
            Xi.toArray(),
            xiSpan,
            p,
            xi
        )
        let R =
            (N.value(0, p - (xiSpan - i)) * w.value(i)) /
            N.multMat(w.range(new Range(xiSpan - p, xiSpan)).transposed()).value(
                0,
                0
            )
        return R
    }

    /**
     * To weighted control points.
     * 
     * @param P 
     * @param w 
     * @returns 
     */
    public static toWeightedControlPoints(P: Point[], w: RowVector): HomPoint[] {
        let Pw: HomPoint[] = new Array(P.length)
        for (let i = 0; i < P.length; i++)
            Pw[i] = P[i].toHomogeneous(w.value(i))
        return Pw
    }

    /**
     * Converts weighted points to points and weights.
     * 
     * @param Pw 
     * @returns 
     */
    public static fromWeightedControlPoints(Pw: HomPoint[]): [Point[], RowVector] {
        let P: Point[] = new Array(Pw.length)
        let wData: number[] = new Array(Pw.length)
        for (let i = 0; i < Pw.length; i++) {
            wData[i] = Pw[i].w()
            P[i] = Point.fromVector(Pw[i].clone().mult(1/Pw[i].w()).row(0))
        }

        return [
            P,
            new RowVector(wData)
        ]
    }
}

/**
 * Represents a NURBS surface.
 */
export class NurbsSurf {
    /**
     * Ctor.
     * 
     * @param controlPoints 
     * @param Xi 
     * @param Eta 
     * @param p 
     * @param q 
     */
     constructor(
        public controlPoints: Point[][],
        public Xi: number[],
        public Eta: number[],
        public weights: Matrix2,
        public p: number,
        public q: number) {}
    
    /**
     * Evaluates the NURBS surface in (xi, eta).
     * 
     * @param xi 
     * @param eta 
     * @returns 
     */
    public evaluate(xi: number, eta: number): Point {
        let n = this.controlPoints.length - 1
        let m = this.controlPoints[0].length - 1
        let xiSpan = BsplineCurve.findSpan(this.Xi, xi, this.p, n)
        let etaSpan = BsplineCurve.findSpan(this.Eta, eta, this.q, m)
        let Nxi = BsplineCurve.computeAllNonvanishingBasis(this.Xi, xiSpan, this.p, xi)
        let Neta = BsplineCurve.computeAllNonvanishingBasis(this.Eta, etaSpan, this.q, eta)

        // Convert to homogeneous coords.
        let Pw = NurbsSurf.toWeightedControlPoints(this.controlPoints, this.weights)

        let P_x = HomPoint.matFromPoints(Pw, "x")
        let P_y = HomPoint.matFromPoints(Pw, "y")
        let P_z = HomPoint.matFromPoints(Pw, "z")
        let P_w = HomPoint.matFromPoints(Pw, "w")

        let sx = Nxi.multMat(P_x.rect(new Point(etaSpan - this.q, xiSpan - this.p), new Point(etaSpan, xiSpan)))
            .multMat(Neta.transposed()).value(0, 0)
        let sy = Nxi.multMat(P_y.rect(new Point(etaSpan - this.q, xiSpan - this.p), new Point(etaSpan, xiSpan)))
            .multMat(Neta.transposed()).value(0, 0)
        let sz = Nxi.multMat(P_z.rect(new Point(etaSpan - this.q, xiSpan - this.p), new Point(etaSpan, xiSpan)))
            .multMat(Neta.transposed()).value(0, 0)
        let sw = Nxi.multMat(P_w.rect(new Point(etaSpan - this.q, xiSpan - this.p), new Point(etaSpan, xiSpan)))
            .multMat(Neta.transposed()).value(0, 0)
        
        return new Point(sx/sw, sy/sw, sz/sw)
    }

    /**
     * Converts a matrix of control points to a matrix of weighted control points.
     * 
     * @param P 
     * @param w 
     */
     public static toWeightedControlPoints(P: Point[][], w: Matrix2): HomPoint[][] {
        let Pw: HomPoint[][] = new Array(P.length)
        for (let i = 0; i < P.length; i++) {
            Pw[i] = new Array(P[i].length)
            for (let j = 0; j < P[i].length; j++) {
                Pw[i][j] = P[i][j].toHomogeneous(w.value(i, j))
            }
        }

        return Pw
    }
}