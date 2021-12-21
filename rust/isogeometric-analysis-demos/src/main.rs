use isogeometric_analysis::bezier::Bernstein;
use isogeometric_analysis::core::Evaluator;
use isogeometric_analysis::core::HslProvider;
use gnuplot::{Figure, Caption, Color};

fn main() {
    env_logger::init();

    let mut fg = Figure::new();
    let axes2d = fg.axes2d();
    let deg = 17u32;
    let hsl = HslProvider { count: deg + 1 };
    for i in 0..(deg + 1) {
        let b = Bernstein::create(deg, i).unwrap();
        let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &100);
        let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &xpoints, 0, &ypoints, 0, &xpoints);
        let caption = format!("B_{{{}}}^{{{}}}", i, 2);
        let color_hex = hsl.hex_color_for_index(i);
        let color = Color(color_hex.as_str());
        axes2d.lines(&xvalues, &yvalues, &[Caption(&caption), color]);
    }

    match fg.show() {
        Err(_e) => { log::warn!("Could not show plot") },
        Ok(_v) => {}
    }
}
