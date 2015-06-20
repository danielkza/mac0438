use std::borrow::Cow;
use std::boxed::Box;
use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result::{self, Ok, Err};
use std::str::FromStr;
use std::string::ToString;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub enum EatingBias {
    Uniform,
    Proportional
}

#[derive(Clone, Debug)]
pub struct Input {
    pub max_eats: u32,
    pub num_philosophers: u32,
    pub weights: Vec<u32>,
    pub bias: EatingBias
}

impl Input {
    fn open_file(filename: &str) -> Result<io::BufReader<Box<io::Read>>, String> {
        let reader = match filename {
            "-" => Box::new(io::stdin()) as Box<io::Read>,
            _ => match File::open(filename) {
                Ok(f) => Box::new(f) as Box<io::Read>,
                Err(e) => return Err(e.to_string())
            }
        };
        
        Ok(io::BufReader::new(reader))
    }

    pub fn from_args<'s, S, I>(args: I) -> Result<Input, String>
        where S: Into<Cow<'s, str>>, I: IntoIterator<Item=S>, I::IntoIter: 's + ExactSizeIterator
    {
        let mut args = args.into_iter().map(|s| s.into());
        
        if args.len() != 3 {
            return Err("Invalid number of arguments, expected 3".to_string());
        }
        
        let input_file = try! { Self::open_file(&*args.next().unwrap()) };
        let max_eats = try! { u32::from_str(&*args.next().unwrap()).map_err(|e| e.to_string()) };
        let mode = try! {
            match &*args.next().unwrap() {
                "U" => Ok(EatingBias::Uniform),
                "P" => Ok(EatingBias::Proportional),
                m => Err(format!("Invalid mode '{}'", m))
            }
        };
        
        let mut lines = input_file.lines();
        let first_line = match lines.next() {
            Some(Ok(s)) => s,
            Some(Err(e)) => return Err(e.to_string()),
            None => return Err("Missing number of philosophers".to_string())
        };
        
        let num_philosophers = match u32::from_str(&first_line) {
            Ok(n) => n,
            _ => return Err("Invalid number of philosophers".to_string())
        };
        
        let second_line = match lines.next() {
            Some(Ok(s)) => s,
            Some(Err(e)) => return Err(e.to_string()),
            None => return Err("Missing weights".to_string())
        };

        let mut weights = Vec::<u32>::new();
        for s in second_line.split(' ') {
            if weights.len() == (num_philosophers as usize) {
                return Err(format!("Extra weights found, expected only {}", num_philosophers))
            }
        
            match u32::from_str(s) {
                Ok(n) => weights.push(n),
                Err(_) => return Err(format!("Invalid weight '{}'", s)) 
            };
        }
    
        if weights.len() != (num_philosophers as usize) {
            return Err(format!("{} weights found, expected at least {}", weights.len(),
                       num_philosophers));
        }

        Ok(Input { max_eats: max_eats, num_philosophers: num_philosophers, weights: weights,
                   bias: mode })
    }
}
