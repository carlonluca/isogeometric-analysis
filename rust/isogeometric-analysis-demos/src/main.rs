use isogeometric_analysis::bezier::Bezier;
use log;

fn main() {
    env_logger::init();
    log::info!("Hello: {}", Bezier::bernstein(2i32, 1i32, 0.3f64));
}
