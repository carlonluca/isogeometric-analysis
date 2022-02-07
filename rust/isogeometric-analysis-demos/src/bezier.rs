/*
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.12.22
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

use isogeometric_analysis::bezier::{Bernstein, BezierCurve, BezierSurf, BezierCircle, RatBezierCurve, BezierCurveDemo1};
use isogeometric_analysis::core::Evaluator;
use isogeometric_analysis::core::HslProvider;
use isogeometric_analysis::core::{RealPoint, RealPoint2d, RealPoint3d};
use isogeometric_analysis::core::RealRange;
use isogeometric_analysis::core::p3;
use gnuplot::{Figure, Caption, Color, LineStyle, AxesCommon, Axes2D, Axes3D, DashType, AutoOption, PlotOption, PointSymbol};
use std::time::Instant;
use array2d::Array2D;

pub struct AxisRange {
    pub min: AutoOption<f64>,
    pub max: AutoOption<f64>
}

pub fn show_3d_patches(patches: Vec<BezierSurf<3>>, xrange: AxisRange, yrange: AxisRange, zrange: AxisRange) {
    let mut fg = Figure::new();

    let axes3d = fg.axes3d()
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .set_x_range(xrange.min, xrange.max)
        .set_y_range(yrange.min, yrange.max)
        .set_z_range(zrange.min, zrange.max)
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .set_z_label("z", &[]);

    for patch in patches {
        show_control_points_3d(axes3d, &patch.data.as_row_major(), false);
        let r = RealRange { a: 0f64, b: 1f64 };
        let (_xpoints, ypoints) = Evaluator::<2, 3, 100>::evaluate_parametric_range2d(&patch, &r, &r);
        let (xvalues, yvalues, zvalues) = Evaluator::<2, 3, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
        axes3d.points(xvalues, yvalues, zvalues, &[Caption(""), Color("orange"), PointSymbol('.')]);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

pub fn show_bernstein_2() {
    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let axes2d1 = fg.axes2d()
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .set_x_range(AutoOption::Fix(0.), AutoOption::Fix(1.))
        .set_y_range(AutoOption::Fix(0.), AutoOption::Fix(1.))
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[]);
    
    show_bernstein(axes2d1, 2, 0);

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

///
/// Shows the first demo of a Bezier curve.
/// 
pub fn show_bezier_curve_demo_1(multiplot: bool) {
    let cpoints = BezierCurveDemo1::create().p;
    show_bezier_curve_demo(cpoints, multiplot);
}

///
/// Shows the second demo of a Bezier curve.
/// 
pub fn show_bezier_curve_demo_2(multiplot: bool) {
    let cpoints = vec![
        p3(0f64, 0f64, 0f64),
        p3(1f64, 1f64, 1f64),
        p3(2f64, 0.5f64, 0f64),
        p3(3f64, 0.5f64, 0f64),
        p3(0.5f64, 1.5f64, 0f64),
        p3(1.5f64, 0f64, 1f64)
    ];
    show_bezier_curve_demo(cpoints, multiplot);
}

pub fn show_bezier_surf_demo_1(multiplot: bool) {
    let cpoints = vec![
        vec![p3(-3.0, 0.0, 2.0), p3(-2.0, 0.0, 6.0), p3(-1.0, 0.0, 7.0), p3(0.0, 0.0, 2.0)],
        vec![p3(-3.0, 1.0, 2.0), p3(-2.0, 1.0, 4.0), p3(-1.0, 1.0, 5.0), p3(0.0, 1.0, 2.5)],
        vec![p3(-3.0, 3.0, 0.0), p3(-2.0, 3.0, 2.5), p3(-1.0, 3.0, 4.5), p3(0.0, 3.0, 6.5)]
    ];
    show_bezier_surf_demo(Array2D::from_rows(&cpoints), multiplot);
}

///
/// Shows a Bezier curve with its bernstein basis polynomials.
/// 
pub fn show_bezier_curve_demo<const SIZE: usize>(cpoints: Vec<RealPoint<SIZE>>, multiplot: bool)
{
    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let axes2d1 = fg.axes2d()
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[]);
    if multiplot {
        axes2d1.set_pos_grid(2, 1, 0);
    }

    let n = cpoints.len() as u32;
    let bez = BezierCurve::create(cpoints);
    let before = Instant::now();
    let (_xpoints, ypoints) = Evaluator::<SIZE, SIZE, 1000>::evaluate_parametric_range1d(&bez, &0f64, &1f64);
    log::info!("Bezier curve computed in: {} μs", before.elapsed().as_micros());
    let (xvalues, yvalues, _zvalues) = Evaluator::<1, SIZE, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
    axes2d1.lines(&xvalues, &yvalues, &[Caption("Bezier"), Color("orange")]);

    // Draw the bernstein polynomials.
    if multiplot {
        show_bernstein(fg.axes2d(), n - 1, 1);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

///
/// Draw an arc.
/// 
pub fn show_ratbezier_arc_demo() {
    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let mut axes2d1 = fg.axes2d()
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .set_x_range(AutoOption::Fix(-1.), AutoOption::Fix(2.))
        .set_y_range(AutoOption::Fix(-1.), AutoOption::Fix(2.))
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[]);
    
    let cp = vec![
        RealPoint2d::point2d(0f64, 1f64),
        RealPoint2d::point2d(1f64, 1f64),
        RealPoint2d::point2d(1f64, 0f64)
    ];

    {
        let bez = BezierCurve::create(cp.clone());
        let (_xpoints, ypoints) = Evaluator::<1, 2, 1000>::evaluate_parametric_range1d(&bez, &0f64, &1f64);
        let (xvalues, yvalues, _zvalues) = Evaluator::<1, 2, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
        axes2d1.lines(&xvalues, &yvalues, &[Caption("Bezier"), Color("orange"), LineStyle(DashType::Dot)]);
    }

    {
        let bez = RatBezierCurve::<2, 3>::create(
            cp.clone(),
            vec![1f64, 2f64.sqrt()/2.0, 1f64]
        );
        let (_xpoints, ypoints) = Evaluator::<1, 2, 1000>::evaluate_parametric_range1d(&bez, &0f64, &1f64);
        let (xvalues, yvalues, _zvalues) = Evaluator::<1, 2, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
        axes2d1.lines(&xvalues, &yvalues, &[Caption("Rational Bezier"), Color("red")]);
    }

    show_control_points(&mut axes2d1, &cp, true);

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

pub fn show_ratbezier_circle_demo() {
    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let mut axes2d1 = fg.axes2d()
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .set_x_range(AutoOption::Fix(-7.), AutoOption::Fix(7.))
        .set_y_range(AutoOption::Fix(-6.), AutoOption::Fix(8.))
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[]);

    let curves = BezierCircle {
        radius: 3,
        segments: 3
    }.compute().unwrap();
    let before = Instant::now();

    for i in 0..curves.len() {
        let (_xpoints, ypoints) = Evaluator::<1, 2, 1000>::evaluate_parametric_range1d(&curves[i], &0f64, &1f64);
        log::info!("Bezier curve computed in: {} μs", before.elapsed().as_micros());
        let (xvalues, yvalues, _zvalues) = Evaluator::<1, 2, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
        if i == 0 {
            axes2d1.lines(&xvalues, &yvalues, &[Caption("R. Bezier"), Color("orange")]);
        }
        else {
            axes2d1.lines(&xvalues, &yvalues, &[Color("orange")]);
        }
    }

    for i in 0..curves.len() {
        show_control_points(&mut axes2d1, &curves[i].p, i == 0);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

///
/// Shows a Bezier curve with its bernstein basis polynomials.
/// 
pub fn show_bezier_surf_demo(cpoints: Array2D<RealPoint3d>, multiplot: bool)
{
    let mut fg = Figure::new();

    // Draw the Bezier surface.
    let axes2d1 = fg.axes3d()
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .set_z_label("z", &[]);
    if multiplot {
        axes2d1.set_pos_grid(2, 1, 0);
    }

    let bez = BezierSurf { data: cpoints };
    let n = bez.degree_xi();
    let before = Instant::now();
    let r = RealRange { a: 0f64, b: 1f64 };
    let (_xpoints, ypoints) = Evaluator::<2, 3, 100>::evaluate_parametric_range2d(&bez, &r, &r);
    log::info!("Bezier curve computed in: {} μs", before.elapsed().as_micros());
    let (_xvalues, _yvalues, zvalues) = Evaluator::<2, 3, 0>::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
    axes2d1.surface(zvalues, 100, 100, None, &[Caption(""), Color("orange")]);

    // Draw the bernstein polynomials.
    if multiplot {
        show_bernstein(fg.axes2d(), n, 1);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}

///
/// Draws the control points.
/// 
pub fn show_control_points<const S: usize>(axes: &mut Axes2D, cp: &Vec<RealPoint<S>>, add_legend: bool) {
    let (xpoints, ypoints, _) = Evaluator::<1, S, 0>::split_coords(0, &cp, 1, &cp, 2, &cp);
    let mut p_options = vec![ Color("black"), PlotOption::PointSymbol('O') ];
    let mut l_options = vec![ Color("black"), LineStyle(DashType::Dash) ];
    if add_legend {
        p_options.push(Caption("Control points"));
        l_options.push(Caption("Control points"));
    }
    axes.points(&xpoints, &ypoints, &p_options);
    axes.lines(&xpoints, &ypoints, &l_options);
}

///
/// Draws the control points.
/// 
pub fn show_control_points_3d<const S: usize>(axes: &mut Axes3D, cp: &Vec<RealPoint<S>>, add_legend: bool) {
    let (xpoints, ypoints, zpoints) = Evaluator::<1, S, 0>::split_coords(0, &cp, 1, &cp, 2, &cp);
    let mut p_options = vec![ Color("black"), PlotOption::PointSymbol('O') ];
    let mut l_options = vec![ Color("black"), LineStyle(DashType::Dash) ];
    if add_legend {
        p_options.push(Caption("Control points"));
        l_options.push(Caption("Control points"));
    }
    axes.points(&xpoints, &ypoints, &zpoints, &p_options);
    axes.lines(&xpoints, &ypoints, &zpoints, &l_options);
}

///
/// Draws the Bernstein basis functions of degree deg.
/// 
pub fn show_bernstein(axes2d2: &mut Axes2D, deg: u32, cell_index: u32)
{
    axes2d2
        .set_pos_grid(2, 1, cell_index)
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("ξ", &[])
        .set_y_label("υ", &[]);
    let hsl = HslProvider { count: deg + 1 };
    for i in 0..(deg + 1) {
        let b = Bernstein::create(deg, i).unwrap();
        let (xpoints, ypoints) = Evaluator::<1, 1, 1000>::evaluate_parametric_range1d(&b, &0f64, &1f64);
        let (xvalues, yvalues, _zvalues) = Evaluator::<1, 1, 0>::split_coords(0, &xpoints, 0, &ypoints, 0, &xpoints);
        let caption = format!("B_{{{}}}^{{{}}}", i, deg);
        let color_hex = hsl.hex_color_for_index(i);
        let color = Color(color_hex.as_str());
        axes2d2.lines(&xvalues, &yvalues, &[Caption(&caption), color]);
    }
}
