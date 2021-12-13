use isogeometric_analysis::bezier::Bezier;
use isogeometric_analysis::core::RowVector;
use gnuplot::{Figure, Caption, Color};
use log;

fn main() {
    env_logger::init();

    let xvalues = RowVector::evenly_spaced(0f64, 1f64, 1000).to_vec();
    let mut yvalues = Vec::new();
    for x in &xvalues {
        yvalues.push(Bezier::bernstein(2, 0, x));
    }
    let mut fg = Figure::new();
    fg.axes2d().lines(&xvalues, &yvalues, &[Caption("A line"), Color("black")]);
    fg.show();
}
