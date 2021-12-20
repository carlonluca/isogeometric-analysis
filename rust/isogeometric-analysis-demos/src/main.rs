use isogeometric_analysis::bezier::Bernstein;
use isogeometric_analysis::core::Evaluator;
use gnuplot::{Figure, Caption, Color};

fn main() {
    env_logger::init();

    let b = Bernstein { n: 2, i: 0 };
    let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &100);
    let (xvalues, yvalues, _zvalues) = Evaluator::split_coords(0, &xpoints, 0, &ypoints, 0, &xpoints);

    let mut fg = Figure::new();
    fg.axes2d().lines(&xvalues, &yvalues, &[Caption("A line"), Color("black")]);
    fg.show();
}
