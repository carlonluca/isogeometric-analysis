use isogeometric_analysis::bezier::{Bernstein, BezierCurve};
use isogeometric_analysis::core::Evaluator;
use isogeometric_analysis::core::HslProvider;
use isogeometric_analysis::core::RealPoint;
use gnuplot::{Figure, Caption, Color, AxesCommon};

fn main() {
    env_logger::init();

    let mut fg = Figure::new();

    // Draw the Bezier curve.
    let axes2d1 = fg.axes2d().set_pos_grid(2, 1, 0);
    let cpoints = vec![
        RealPoint::point2d(0f64, 0f64),
        RealPoint::point2d(1f64, 1f64),
        RealPoint::point2d(2f64, 0.5f64),
        RealPoint::point2d(3f64, 0.5f64),
        RealPoint::point2d(0.6f64, 1.5f64),
        RealPoint::point2d(1.5f64, 0f64)
    ];
    let n = cpoints.len() as u32;
    let bez = BezierCurve { p: cpoints };
    let (_xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&bez, &0f64, &1f64, &100);
    let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &ypoints, 1, &ypoints, 2, &ypoints);
    axes2d1.lines(&xvalues, &yvalues, &[Caption(""), Color("orange")]);

    // Draw the bernstein polynomials.
    let axes2d2 = fg.axes2d().set_pos_grid(2, 1, 1);
    let deg = n - 1;
    let hsl = HslProvider { count: deg + 1 };
    for i in 0..(deg + 1) {
        let b = Bernstein::create(deg, i).unwrap();
        let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &100);
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
