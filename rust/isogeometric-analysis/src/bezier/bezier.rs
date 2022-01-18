/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.03
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

use crate::core::fact;
use crate::core::{RealPoint, RealPoint1d, RealPoint2d, p2};
use crate::core::Evaluatable;
use std::f64::consts::PI;
use num::traits::Pow;
use array2d::Array2D;

///
/// Represents a Bernstein basis polynomial.
/// 
/// # Example
/// 
/// ```rust
/// use isogeometric_analysis::bezier::Bernstein;
/// use isogeometric_analysis::core::Evaluator;
/// let b = Bernstein::create(5, 2).unwrap();
/// let (xpoints, ypoints) = Evaluator::<1, 1>::evaluate_parametric_range1d(&b, &0f64, &1f64, &10000);
/// ```
/// 
pub struct Bernstein {
    n: u32,
    i: u32
}

impl Evaluatable<f64, f64, 1, 1> for Bernstein {
    ///
    /// Evaluate without creating a new object.
    /// 
    fn evaluate_fill<'a>(&self, input: &RealPoint1d, output: &'a mut RealPoint1d) -> &'a mut RealPoint1d {
        let num = (fact(self.n) as f64)*input.x().pow(self.i as f64)*(1f64 - input.x()).pow((self.n - self.i) as f64);
        let den = (fact(self.i)*fact(self.n - self.i)) as f64;
        output.set_x(num/den);
        output
    }
}

impl Bernstein {
    ///
    /// Creates the `i`-th bernstein basis polynomial of degree `n`.
    /// 
    pub fn create(n: u32, i: u32) -> Option<Bernstein> {
        if i > n {
            log::warn!("Index cannot be grater than degree");
            return None;
        }
        Some(Bernstein { n: n, i: i })
    }
}

///
/// Implements Bezier curves and surfaces.
/// 
pub struct BezierCurve<const SIZE: usize> {
    pub p: Vec<RealPoint<SIZE>>
}

impl<const SIZE: usize> Evaluatable<f64, f64, 1, SIZE> for BezierCurve<SIZE> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    /// 
    fn evaluate_fill<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        self.evaluate_direct(xi, output)
    }
}

impl<const SIZE: usize> BezierCurve<SIZE> {
    ///
    /// Computes the value of the Bezier curve in xi using the direct algorithm. This
    /// technique is not numerically stable.
    ///
    pub fn evaluate_direct<'a>(&self, input: &RealPoint<1>, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        let n = self.p.len();
        let mut tmp = RealPoint1d::origin();
        output.reset();
        for i in 0..n {
            let bernstein = Bernstein::create((n - 1) as u32, i as u32).unwrap();
            let bout = bernstein.evaluate_fill(&input, &mut tmp).x();
            *output += self.p[i]*bout;
        }

        return output;
    }

    ///
    /// Computes the value of the Bezier curve in xi using the De Casteljau's algorithm.
    ///
    pub fn evaluate_de_casteljau(&self, xi: &RealPoint1d) -> RealPoint<SIZE> {
        let n = self.p.len() - 1;
        let mut q = Vec::<RealPoint<SIZE>>::new();
        for i in 0..(n + 1) {
            q.push(self.p[i]);
        }
        for k in 1..(n + 1) {
            for i in 0..(n - k + 1) {
                q[i] = q[i]*(1f64 - xi.x()) + q[i + 1]*xi.x();
            }
        }

        return q[0];
    }

    ///
    /// Returns the degree of the Bezier curve.
    /// 
    pub fn degree(&self) -> u32 {
        (self.p.len() - 1) as u32
    }

    ///
    /// Returns a copy of the control points.
    /// 
    pub fn control_points(&self) -> &Vec<RealPoint<SIZE>> {
        &self.p
    }
}

///
/// Represents a Bezier surface.
/// 
pub struct BezierSurf<const S: usize> {
    pub data: Array2D<RealPoint<S>>
}

impl<const S: usize> BezierSurf<S> {
    ///
    /// Returns the degree on the Xi axis.
    /// 
    pub fn degree_xi(&self) -> u32 { (self.data.column_len() - 1) as u32 }

    ///
    /// Returns the degree on the Eta axis.
    /// 
    pub fn degree_eta(&self) -> u32 { (self.data.row_len() - 1) as u32 }
}

impl<const S: usize> Evaluatable<f64, f64, 2, S> for BezierSurf<S> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    /// 
    fn evaluate_fill<'a>(&self, input: &RealPoint2d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        self.evaluate_de_casteljau(input, output)
    }
}

impl<const S: usize> BezierSurf<S> {
    ///
    /// Evaluates a Bezier surface by using the definition.
    /// 
    pub fn evaluate_direct<'a>(&self, input: &RealPoint2d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        output.reset();
        let n = self.degree_xi();
        let m = self.degree_eta();
        let mut tmpinputi = RealPoint1d::origin();
        let mut tmpinputj = RealPoint1d::origin();
        let mut tmpi = RealPoint1d::origin();
        let mut tmpj = RealPoint1d::origin();
        for i in 0..=n {
            for j in 0..=m {
                let bin = Bernstein { n: n, i: i };
                let bjm = Bernstein { n: m, i: j };
                tmpinputi.set_x(input.x());
                tmpinputj.set_x(input.y());
                bin.evaluate_fill(&tmpinputi, &mut tmpi);
                bjm.evaluate_fill(&tmpinputj, &mut tmpj);
                *output += self.data[(i as usize, j as usize)]*tmpi.x()*tmpj.x();
            }
        }

        return output;
    }

    ///
    /// Evaluates a Bezier surface by using the De Casteljau's algorithm.
    /// 
    pub fn evaluate_de_casteljau<'a>(&self, input: &RealPoint2d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        output.reset();
        let n = self.degree_xi();
        let eta = RealPoint1d::point1d(input.y());
        let xi = RealPoint1d::point1d(input.x());
        let mut q = Vec::<RealPoint<S>>::new();
        for i in 0..=n {
            let bezcurve1 = BezierCurve::<S> { p: self.data.as_rows()[i as usize].clone() };
            q.push(bezcurve1.evaluate_de_casteljau(&eta));
        }

        let bezcurve2 = BezierCurve::<S> { p: q };
        let res = bezcurve2.evaluate_de_casteljau(&xi);

        for i in 0..output.dim() {
            output.set_value(i, res.value(i));
        }

        return output;
    }
}

///
/// Implementation of a rational Bezier curve.
/// 
pub struct RatBezierCurve<const S: usize, const H: usize> {
    pub p: Vec<RealPoint<S>>,
    pub weights: Vec<f64>,
    pub pw: Vec<RealPoint<H>>,
    bez: BezierCurve<H>
}

impl<const S: usize, const H: usize> RatBezierCurve<S, H> {
    ///
    /// Creates a new rational Bezier curve by passing control points and weights.
    /// 
    pub fn create(p: Vec<RealPoint<S>>, weights: Vec<f64>) -> RatBezierCurve<S, H> {
        if p.len() != weights.len() {
            panic!()
        }
        let mut pw = Vec::<RealPoint<H>>::new();
        for i in 0..p.len() {
            pw.push(p[i].to_homogeneous::<H>(weights[i]));
            log::info!("I: {}", p[i].to_homogeneous::<H>(weights[i]));
        }

        // FIXME: do I really need two clones here?
        RatBezierCurve::<S, H> {
            p: p,
            weights: weights,
            pw: pw.clone(),
            bez: BezierCurve::<H> {
                p: pw.clone()
            }
        }
    }
}

impl<const S: usize, const H: usize> Evaluatable<f64, f64, 1, S> for RatBezierCurve<S, H> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    /// 
    fn evaluate_fill<'a>(&self, input: &RealPoint1d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        let mut cw = RealPoint::<H>::origin();
        self.bez.evaluate_direct(&input, &mut cw);
        let c = cw.to_cartesian();
        c.clone_to(output);
        log::info!("Pw: {} -> {} -> {}", cw, c, output);
        output
    }
}

///
/// Struct to compute a circle with rational bezier curves.
/// 
pub struct BezierCircle {
    pub radius: u32,
    pub segments: u32
}

impl BezierCircle {
    ///
    /// Returns the number of points for this circle.
    /// 
    pub fn points(&self) -> u32 {
        self.segments*2 + 1
    }

    ///
    /// Computes the rational Bezier curve that will draw the requested circle.
    /// 
    pub fn compute(&self) -> RatBezierCurve::<2, 3> {
        let r = self.radius as f64;
        let n = self.segments as f64;
        let alpha = 2.0*PI/(2.0*n);
        let outerr = r/alpha.cos();
        let mut cpoints = Vec::<RealPoint2d>::new();
        let mut weights = Vec::<f64>::new();
        for i in 0..=self.segments {
            let mut p = RealPoint2d::point2d(
                r*(2.0*(i as f64)*alpha).sin(),
                -1.0*r*(2.0*(i as f64)*alpha).cos()
            );
            let mut w = 1.0;
            cpoints.push(p);
            weights.push(w);

            log::info!("P: {}", p);

            if i < self.segments {
                p = RealPoint2d::point2d(
                    outerr*((2.0*(i as f64) + 1.0)*alpha).sin(),
                    -1.0*outerr*((2.0*(i as f64) + 1.0)*alpha).cos()
                );
                w = 0.5;
                cpoints.push(p);
                weights.push(w);

                log::info!("P: {}", p);
            }
            
        }

        RatBezierCurve::<2, 3>::create(cpoints, weights)
    }
}

///
/// This struct can be used to build a sample Bezier curve for debugging.
/// 
pub struct BezierCurveDemo1 {}

impl BezierCurveDemo1 {
    ///
    /// Returns the BezierCurve.
    /// 
    pub fn create() -> BezierCurve<2> {
        BezierCurve {
            p: vec![
                p2(0f64, 0f64),
                p2(1f64, 1f64),
                p2(2f64, 0.5f64),
                p2(3f64, 0.5f64),
                p2(0.6f64, 1.5f64),
                p2(1.5f64, 0f64)
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bezier::RatBezierCurve;
    use crate::bezier::BezierCurveDemo1;
    use crate::core::RealPoint1d;
    use crate::core::RealPoint2d;
    use crate::core::Evaluatable;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_eq() {
        let demo1 = BezierCurveDemo1::create();
        let mut w = Vec::<f64>::new();
        for _i in 0..demo1.control_points().len() {
            w.push(1.0);
        }
        let demorat1 = RatBezierCurve::<2, 3>::create(demo1.control_points().clone(), w);
        for i in 0..=100 {
            let input = RealPoint1d::point1d((i as f64)/100f64);
            let mut outputrat = RealPoint2d::origin();
            let mut output = RealPoint2d::origin();
            demorat1.evaluate_fill(&input, &mut outputrat);
            demo1.evaluate_fill(&input, &mut output);
            assert_approx_eq!(RealPoint2d, output, outputrat);
        }
    }
}
