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

use isogeometric_analysis::core::{RealPoint, Evaluator};
use isogeometric_analysis::bezier::{BezierCurve, Bernstein};
use std::time::Instant;

fn main() {
    env_logger::init();

    let iterations = 20;

    { // Bezier
        let cpoints = vec![
            RealPoint::point3d(0f64, 0f64, 0f64),
            RealPoint::point3d(1f64, 1f64, 0f64),
            RealPoint::point3d(2f64, 0.5f64, 0f64),
            RealPoint::point3d(3f64, 0.5f64, 0f64),
            RealPoint::point3d(0.6f64, 1.5f64, 0f64),
            RealPoint::point3d(1.5f64, 0f64, 0f64)
        ];
        let bez = BezierCurve { p: cpoints };
        let before = Instant::now();
        for _i in 0..iterations {
            let (_xpoints, _ypoints) = Evaluator::evaluate_r_to_r3(&bez, &0f64, &1f64, &10000);
        }
        log::info!("Bezier curve computed on 10000 points in: {} μs", before.elapsed().as_micros()/iterations);
    }
    
    { // Bernstein
        let b = Bernstein::create(5, 2).unwrap();
        let before = Instant::now();
        for _i in 0..iterations {
            let (_xpoints, _ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &10000);
        }
        log::info!("Bernstein polynomial B_2^5 curve computed on 10000 points in: {} μs", before.elapsed().as_micros()/iterations);
    }
}
