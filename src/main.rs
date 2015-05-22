use std::env;

use taylor::params::Params;

pub mod taylor;

fn main() {
    let args = std::env::args().collect();
    let params = Params::from_args(args);
}
