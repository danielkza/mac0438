use std::cmp::PartialOrd;
use std::error;
use std::fmt::{self, Debug, Display, Formatter};
use std::result::Result::{self, Ok, Err};
use std::str::FromStr;

#[derive(Debug)]
pub enum NumThreads {
    MaxAvailable,
    Fixed(u16)
}

#[derive(Debug)]
pub enum TerminationCriteria<T> {
    Difference(T),
    AbsoluteValue(T)
}

#[derive(Debug)]
pub enum RunMode {
    Default,
    Debug,
    Sequential
}

#[derive(Debug)]
pub struct Params<T> where T: PartialOrd<T> {
    num_threads: NumThreads,
    termination_criteria: TerminationCriteria<T>,
    input_value: T,
    run_mode: RunMode
}

#[derive(Debug)]
pub enum Error {
    InsufficientArguments(u8),
    BadNumThreads(error::Error),
    BadTerminationMode,
    BadTerminationValue(error::Error),
    BadInputValue(error::Error),
    BadRunMode
}

/* */

impl<T> Params<T> where T: FromStr + PartialOrd {
    pub fn fromArgs(args: [&str]) -> Result<Params<T>, Error> {
        use self::Error::*;
        
        if(args.len() < 4) {
            return Err(InsufficientArguments(args.len()))
        }

        let num_threads = try!(match args[0] {
            "0" => Ok(NumThreads::MaxAvailable),
            num_s => match num_s.parse() {
                Ok(n) => Ok(NumThreads::Fixed(n)),
                Err(e) => Err(BadNumThreads(e))
            }
        });
        
        let termination_value = try!(match T::parse(args[2]) {
            Ok(v) => Ok(v),
            Err(e) => Err(BadTerminationValue(e))
        });

        let termination_criteria = try!(match args[1] {
            "f" => Ok(TerminationCriteria::Difference(termination_value)),
            "m" => Ok(TerminationCriteria::AbsoluteValue(termination_value)),
            _ => Err(BadTerminationMode)
        });

        let input_value = try!(match T::parse(args[3]) {
            Ok(v) => Ok(v),
            Err(e) => Err(BadInputValue(e))
        });

        let run_mode = try!(match args.get(4) {
            Some("d") => Ok(RunMode::Debug),
            Some("s") => Ok(RunMode::Sequential),
            Some(_) => Err(BadRunMode),
            None => Ok(RunMode::Default)
        });

        Ok(Params { num_threads: num_threads, termination_criteria: termination_criteria,
                    input_value: input_value, run_mode: run_mode })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(error::Error::description(self), f)
    }
}

impl error::Error for Error {
    pub fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            InsufficientArguments(n) =>
                format!("Not enough arguments, only {} provided", n),
            BadNumThreads(ref e) =>
                format!("Bad number of threads, must be 0 or a positive number: {:?}", e),
            BadTerminationMode =>
                "Bad termination mode, must be 'f' or 'm'",
            BadTerminationValue(ref e) =>
                format!("Bad termination value: {:?}", e),
            BadInputValue(ref e) =>
                format!("Bad input value: {:?}", e),
            BadRunMode =>
                "Bad run mode, must be 'd', 's' or not provided"
        }
    }

    pub fn cause(&self) -> Option<&error::Error>{
        use self::Error::*;
        match *self {
            BadNumThreads(ref e) | BadTerminationValue(ref e) | BadInputValue(ref e) => Some(e),
            _ => None
        }
    }
}
