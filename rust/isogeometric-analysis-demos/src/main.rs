use isogeometric_analysis::bezier::Bernstein;
use isogeometric_analysis::core::Evaluator;
use gnuplot::{Figure, Caption, Color};

fn main() {
    env_logger::init();

    let mut fg = Figure::new();
    let mut axes2d = fg.axes2d();
    let deg = 2;
    for i in 0..(deg + 1) {
        let b = Bernstein { n: deg, i: i };
        let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &100);
        let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &xpoints, 0, &ypoints, 0, &xpoints);
        let caption = format!("B_{}^{}", i, 2);
        let mut color;
        if i == 0 { color = Color("black") }
        else if i == 1 { color = Color("yellow") }
        else { color = Color("red") }
        axes2d.lines(&xvalues, &yvalues, &[Caption(&caption), color]);
    }

    fg.show();
}
