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
mod bezier;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "isogeometric-analysis-demos", about = "A demo for the isogeometric_analysis crate")]
struct Opt {
    #[structopt(long, help = "Shows the Bernstein polynomials of degree 2")]
    bernstein2: bool,
    #[structopt(long, help = "Shows the Bezier demo 1")]
    bezier1: bool,
    #[structopt(long, help = "Shows the Bezier demo 2")]
    bezier2: bool,
    #[structopt(long, help = "Shows the Bezier surface demo 1")]
    bezier_surf1: bool,
    #[structopt(long, help = "Shows the teapot")]
    bezier_teapot: bool,
    #[structopt(long, help = "Shows a circle drawn with a rational bezier curve")]
    ratbezier_circle1: bool,
    #[structopt(long, help = "Shows an arc")]
    ratbezier_arc1: bool,
    #[structopt(long, help = "Multiplot")]
    multiplot: bool
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    if opt.bernstein2 {
        bezier::show_bernstein_2();
        return;
    }

    if opt.bezier1 {
        bezier::show_bezier_curve_demo_1(opt.multiplot);
        return;
    }

    if opt.bezier2 {
        bezier::show_bezier_curve_demo_2(opt.multiplot);
        return;
    }

    if opt.bezier_surf1 {
        bezier::show_bezier_surf_demo_1(opt.multiplot);
        return;
    }

    if opt.ratbezier_circle1 {
        bezier::show_ratbezier_circle_demo();
        return;
    }

    if opt.ratbezier_arc1 {
        bezier::show_ratbezier_arc_demo();
        return;
    }

    if opt.bezier_teapot {
        bezier::show_teapot();
        return;
    }
}
