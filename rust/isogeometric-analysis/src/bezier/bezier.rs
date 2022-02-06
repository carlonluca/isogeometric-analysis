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
use crate::core::{RealPoint, RealPoint1d, RealPoint2d, RealPoint3d, p2};
use crate::core::Mapping;
use std::f64::consts::PI;
use num::traits::Pow;
use array2d::Array2D;

///
/// Represents a Bernstein polynomial. Bernstein polynomials are functions
/// f:ℝ→ℝ.
/// 
/// # Example
/// 
/// ```rust
/// use isogeometric_analysis::bezier::Bernstein;
/// use isogeometric_analysis::core::Evaluator;
/// let b = Bernstein::create(5, 2).unwrap();
/// let (xpoints, ypoints) = Evaluator::<1, 1, 1000>::evaluate_parametric_range1d(&b, &0f64, &1f64);
/// ```
/// 
pub struct Bernstein {
    n: u32,
    i: u32
}

impl Mapping<f64, f64, 1, 1> for Bernstein {
    ///
    /// Evaluate without creating a new object.
    ///
    #[inline(always)]
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
/// Implements Bezier curves and surfaces. Bezier surfaces can be computed with
/// a direct method based on the definition of with the De Casteljau's algorithm.
/// BezierCurve is therefore a function f:ℝ→ℝ^SIZE, where SIZE is the dimension
/// of the space in which the curve is to be defined (typically 2).
/// 
/// # Example
/// 
/// ```rust
/// use isogeometric_analysis::core::p2;
/// use isogeometric_analysis::bezier::{BezierCurve, BezierCurveDemo1};
/// use isogeometric_analysis::core::Evaluator;
/// let cpoints = BezierCurveDemo1::create().p;
/// let bez = BezierCurve::create(cpoints);
/// let (xpoints, ypoints) = Evaluator::<1, 2, 10>::evaluate_parametric_range1d(&bez, &0f64, &1f64);
/// ```
/// 
pub struct BezierCurve<const SIZE: usize> {
    pub p: Vec<RealPoint<SIZE>>,
    bernstein: Vec<Bernstein>
}

impl<const SIZE: usize> Mapping<f64, f64, 1, SIZE> for BezierCurve<SIZE> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    ///
    #[inline(always)]
    fn evaluate_fill<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        self.evaluate_direct(xi, output)
    }
}

impl<const SIZE: usize> BezierCurve<SIZE> {
    #[inline(always)]
    pub fn create(cpoints: Vec<RealPoint<SIZE>>) -> BezierCurve<SIZE> {
        let n = (cpoints.len() - 1) as u32;
        let mut b = Vec::<Bernstein>::new();
        for i in 0u32..=n {
            b.push(Bernstein::create(n, i).unwrap());
        }

        BezierCurve {
            p: cpoints,
            bernstein: b
        }
    }

    ///
    /// Computes the value of the Bezier curve in xi using the direct algorithm. This
    /// technique is not numerically stable.
    ///
    #[inline(always)]
    pub fn evaluate_direct<'a>(&self, xi: &RealPoint<1>, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        let n = self.p.len() - 1;
        if n == 1 {
            return self.evaluate_direct_linear(&xi, output);
        }
        else if n == 2 {
            return self.evaluate_direct_quadratic(&xi, output);
        }
        else if n == 3 {
            return self.evaluate_direct_cubic(&xi, output);
        }
        let mut tmp = RealPoint1d::origin();
        output.reset();
        for i in 0..=n {
            let bout = self.bernstein[i].evaluate_fill(&xi, &mut tmp).x();
            *output += self.p[i]*bout;
        }

        return output;
    }

    ///
    /// Computes the value of the Bezier curve in xi using the De Casteljau's algorithm.
    ///
    #[inline(always)]
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
    /// Evaluates quickly a linear Bezier.
    ///
    #[inline(always)]
    pub fn evaluate_direct_linear<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        *output = self.p[0]*(1. - xi.x());
        *output += self.p[1]*xi.x();
        output
    }

    ///
    /// Evaluates quickly a quadratic Bezier.
    ///
    #[inline(always)]
    pub fn evaluate_direct_quadratic<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        *output = self.p[0]*(Pow::<f64>::pow(1. - xi.x(), 2.));
        *output += self.p[1]*(2.*xi.x()*(1. - xi.x()));
        *output += self.p[2]*Pow::<f64>::pow(xi.x(), 2.);
        output
    }

    ///
    /// Evaluates quickly a cubic Bezier.
    ///
    #[inline(always)]
    pub fn evaluate_direct_cubic<'a>(&self, xi: &RealPoint1d, output: &'a mut RealPoint<SIZE>) -> &'a mut RealPoint<SIZE> {
        *output = self.p[0]*Pow::<f64>::pow(1. - xi.x(), 3.);
        *output += self.p[1]*3.*xi.x()*Pow::<f64>::pow(1. - xi.x(), 2.);
        *output += self.p[2]*2.*Pow::<f64>::pow(xi.x(), 2.)*(1. - xi.x());
        *output += self.p[3]*Pow::<f64>::pow(xi.x(), 3.);
        output
    }

    ///
    /// Returns the degree of the Bezier curve.
    ///
    #[inline(always)]
    pub fn degree(&self) -> u32 {
        (self.p.len() - 1) as u32
    }

    ///
    /// Returns a copy of the control points.
    ///
    #[inline(always)]
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
    #[inline(always)]
    pub fn degree_xi(&self) -> u32 { (self.data.column_len() - 1) as u32 }

    ///
    /// Returns the degree on the Eta axis.
    ///
    #[inline(always)]
    pub fn degree_eta(&self) -> u32 { (self.data.row_len() - 1) as u32 }
}

impl<const S: usize> Mapping<f64, f64, 2, S> for BezierSurf<S> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    ///
    #[inline(always)]
    fn evaluate_fill<'a>(&self, input: &RealPoint2d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        self.evaluate_de_casteljau(input, output)
    }
}

impl<const S: usize> BezierSurf<S> {
    ///
    /// Evaluates a Bezier surface by using the definition.
    ///
    #[inline(always)]
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
    #[inline(always)]
    pub fn evaluate_de_casteljau<'a>(&self, input: &RealPoint2d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        output.reset();
        let n = self.degree_xi();
        let eta = RealPoint1d::point1d(input.y());
        let xi = RealPoint1d::point1d(input.x());
        let mut q = Vec::<RealPoint<S>>::new();
        for i in 0..=n {
            let bezcurve1 = BezierCurve::<S>::create(self.data.as_rows()[i as usize].clone());
            q.push(bezcurve1.evaluate_de_casteljau(&eta));
        }

        let bezcurve2 = BezierCurve::<S>::create(q);
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
    pub bez: BezierCurve<H>
}

impl<const S: usize, const H: usize> RatBezierCurve<S, H> {
    ///
    /// Creates a new rational Bezier curve by passing control points and weights.
    ///
    #[inline(always)]
    pub fn create(p: Vec<RealPoint<S>>, weights: Vec<f64>) -> RatBezierCurve<S, H> {
        if p.len() != weights.len() {
            panic!()
        }
        let mut pw = Vec::<RealPoint<H>>::new();
        for i in 0..p.len() {
            pw.push(p[i].to_homogeneous::<H>(weights[i]));
        }

        // FIXME: do I really need two clones here?
        RatBezierCurve::<S, H> {
            p: p,
            weights: weights,
            pw: pw.clone(),
            bez: BezierCurve::<H>::create(pw.clone())
        }
    }
}

impl<const S: usize, const H: usize> Mapping<f64, f64, 1, S> for RatBezierCurve<S, H> {
    ///
    /// Evaluates the Bezier curve in point xi. Point xi exists in the parametric space.
    ///
    #[inline(always)]
    fn evaluate_fill<'a>(&self, input: &RealPoint1d, output: &'a mut RealPoint<S>) -> &'a mut RealPoint<S> {
        let mut cw = RealPoint::<H>::origin();
        self.bez.evaluate_direct(&input, &mut cw);
        let c = cw.to_cartesian();
        c.clone_to(output);
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
    #[inline(always)]
    pub fn points(&self) -> u32 {
        self.segments*2 + 1
    }

    ///
    /// Computes the rational Bezier curve that will draw the requested circle.
    ///
    #[inline(always)]
    pub fn compute(&self) -> Option<Vec<RatBezierCurve::<2, 3>>> {
        if self.segments < 2 { return None; }
        let r = self.radius as f64;
        let n = self.segments as f64;
        let alpha = 2.*PI/(2.*n);
        let outerr = r/alpha.cos();
        let mut cpoints = Vec::<RealPoint2d>::new();
        let mut weights = Vec::<f64>::new();
        for i in 0..=self.segments {
            let mut a = 2.*(i as f64)*alpha;
            let mut p = RealPoint2d::point2d(
                r*(a).sin(),
                -r*(a).cos()
            );
            cpoints.push(p);
            weights.push(1.);

            if i < self.segments {
                a = (2.*(i as f64) + 1.)*alpha;
                p = RealPoint2d::point2d(
                    outerr*a.sin(),
                    -outerr*a.cos()
                );
                cpoints.push(p);
                weights.push(alpha.cos());
            }
        }

        let mut idx = 0;
        let mut curves = Vec::new();
        for _i in 0..self.segments {
            let cpoints_ = vec![
                cpoints[idx],
                cpoints[idx + 1],
                cpoints[idx + 2]
            ];
            let weights_ = vec![
                weights[idx],
                weights[idx + 1],
                weights[idx + 2]
            ];
            idx += 2;
            curves.push(RatBezierCurve::<2, 3>::create(cpoints_, weights_));
        }
        
        return Some(curves);
    }
}

///
/// This struct can be used to build a sample Bezier curve for debugging.
/// 
/// # Example
/// 
/// ```rust
/// use isogeometric_analysis::core::Evaluator;
/// use isogeometric_analysis::bezier::{BezierCurveDemo1, BezierCurve};
/// let bezier = BezierCurveDemo1::create();
/// let (xpoints, ypoints) = Evaluator::<1, 2, 10>::evaluate_parametric_range1d(&bezier, &0f64, &1f64);
/// ```
/// 
pub struct BezierCurveDemo1 {}

impl BezierCurveDemo1 {
    ///
    /// Returns the BezierCurve.
    /// 
    pub fn create() -> BezierCurve<2> {
        BezierCurve::<2>::create(vec![
            p2(0f64, 0f64),
            p2(1f64, 1f64),
            p2(2f64, 0.5f64),
            p2(3f64, 0.5f64),
            p2(0.6f64, 1.5f64),
            p2(1.5f64, 0f64)
        ])
    }
}

pub struct BezierFactory {}

impl BezierFactory {
    pub fn from_indexed_vertices(patch_array: Vec<[usize; 16]>, vertex_array: Vec<[f64; 3]>) -> Vec<BezierSurf<3>> {
        let mut vertices = Vec::<RealPoint3d>::new();
        for i in 0..vertex_array.len() {
            vertices.push(RealPoint3d::point3d(vertex_array[i][0], vertex_array[i][1], vertex_array[i][2]));
        }

        let mut patches = Vec::<BezierSurf<3>>::new();
		for i in 0..patch_array.len() {
			let mut patch_cps = Vec::<RealPoint3d>::new();
			for j in 0..patch_array[i].len() {
                log::info!("IDX: {}", patch_array[i][j]);
				let idx = patch_array[i][j] - 1;
				patch_cps.push(vertices[idx].clone());
			}
			let bdata = Array2D::<RealPoint3d>::from_row_major(&patch_cps, 4, 4);
			patches.push(BezierSurf::<3> { data: bdata });
		}

		patches
    }
}

#[cfg(test)]
mod tests {
    use crate::bezier::RatBezierCurve;
    use crate::bezier::BezierCurveDemo1;
    use crate::bezier::BezierCircle;
    use crate::core::RealPoint1d;
    use crate::core::RealPoint2d;
    use crate::core::Mapping;
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

    #[test]
    fn test_circle() {
        for r in 0..7 {
            for s in 3..10 {
                let circle = BezierCircle {
                    radius: r,
                    segments: s
                };
                let ratbezs = circle.compute().unwrap();
                for ratbez in ratbezs.iter() {
                    let mut computed = RealPoint2d::origin();
                    for i in 0..=1000 {
                        let input = RealPoint1d::point1d((i as f64)/1000.);
                        ratbez.evaluate_fill(&input, &mut computed);
                        let dist = RealPoint2d::origin().dist(&computed);
                        assert_approx_eq!(f64, dist, r as f64, epsilon = 1E-6);
                    }
                }
            }
        }
    }
}
