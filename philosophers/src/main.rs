use std::env;

mod params;
mod philosopher;

use params::Input;

fn main() {
    let params = match Input::from_args(std::env::args().skip(1)) {
        Ok(params) => params,
        Err(err) => panic!("Error: {}", err)  
    };

    println!("params: {:?}", &params);
}
