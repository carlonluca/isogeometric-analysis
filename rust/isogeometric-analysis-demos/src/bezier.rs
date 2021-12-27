/**
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

use isogeometric_analysis::bezier::{Bernstein, BezierCurve};
use isogeometric_analysis::core::Evaluator;
use isogeometric_analysis::core::HslProvider;
use isogeometric_analysis::core::RealPoint;
use gnuplot::{Figure, Caption, Color, AxesCommon};
use std::time::Instant;

///
/// Shows the first demo of a Bezier curve.
/// 
pub fn show_bezier_demo_1()
{
    let cpoints = vec![
        RealPoint::point2d(0f64, 0f64),
        RealPoint::point2d(1f64, 1f64),
        RealPoint::point2d(2f64, 0.5f64),
        RealPoint::point2d(3f64, 0.5f64),
        RealPoint::point2d(0.6f64, 1.5f64),
        RealPoint::point2d(1.5f64, 0f64)
    ];
    show_bezier_demo(cpoints);
}

///
/// Shows the second demo of a Bezier curve.
/// 
pub fn show_bezier_demo_2()
{
    let cpoints = vec![
        RealPoint::point3d(0f64, 0f64, 0f64),
        RealPoint::point3d(1f64, 1f64, 1f64),
        RealPoint::point3d(2f64, 0.5f64, 0f64),
        RealPoint::point3d(3f64, 0.5f64, 0f64),
        RealPoint::point3d(0.5f64, 1.5f64, 0f64),
        RealPoint::point3d(1.5f64, 0f64, 1f64)
    ];
    show_bezier_demo(cpoints);
}

///
/// Shows a Bezier curve with its bernstein basis polynomials.
/// 
pub fn show_bezier_demo(cpoints: Vec<RealPoint>)
{
    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let axes2d1 = fg.axes2d()
        .set_pos_grid(2, 1, 0)
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("x", &[])
        .set_y_label("y", &[]);
    let n = cpoints.len() as u32;
    let bez = BezierCurve { p: cpoints };
    let before = Instant::now();
    let (_xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&bez, &0f64, &1f64, &1000);
    log::info!("Bezier curve computed in: {} μs", before.elapsed().as_micros());
    let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
    axes2d1.lines(&xvalues, &yvalues, &[Caption(""), Color("orange")]);

    // Draw the bernstein polynomials.
    let axes2d2 = fg.axes2d()
        .set_pos_grid(2, 1, 1)
        .set_y_grid(true)
        .set_x_grid(true)
        .set_x_label("ξ", &[])
        .set_y_label("υ", &[]);
    let deg = n - 1;
    let hsl = HslProvider { count: deg + 1 };
    for i in 0..(deg + 1) {
        let b = Bernstein::create(deg, i).unwrap();
        let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &1000);
        let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &xpoints, 0, &ypoints, 0, &xpoints);
        let caption = format!("B_{{{}}}^{{{}}}", i, deg);
        let color_hex = hsl.hex_color_for_index(i);
        let color = Color(color_hex.as_str());
        axes2d2.lines(&xvalues, &yvalues, &[Caption(&caption), color]);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}
