pub mod taylor;

use std::env;
use taylor::params::Params;


fn main() {
    let args = std::env::args().collect();
    let params = Params::from_args(args);
    println!("{:?}", params);
    0
}
