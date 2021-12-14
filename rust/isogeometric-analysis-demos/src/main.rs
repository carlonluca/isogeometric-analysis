use isogeometric_analysis::bezier::Bernstein;
use isogeometric_analysis::core::RowVector;
use isogeometric_analysis::core::Evaluator;
use gnuplot::{Figure, Caption, Color};
use log;

fn main() {
    env_logger::init();

    let b = Bernstein { n: 2, i: 0 };
    let (xpoints, ypoints) = Evaluator::evaluate_r_to_r3(&b, &0f64, &1f64, &100);
    let mut xvalues = Vec::new();
    let mut yvalues = Vec::new();
    for p in &xpoints {
        xvalues.push(p.x);
    }
    for p in &ypoints {
        yvalues.push(p.x);
    }

    let mut fg = Figure::new();
    fg.axes2d().lines(&xvalues, &yvalues, &[Caption("A line"), Color("black")]);
    fg.show();
}
