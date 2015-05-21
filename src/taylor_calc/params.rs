mod taylor_calc;

use std::cmp;
use std::num;
use std::vec;
use std::string::FromStr;
use std::error::Error;
use std::result::Result::{self, Ok, Err};

extern crate num_cpus;

pub enum NumThreads {
    MaxAvailable,
    Fixed(u16)
}

pub enum TerminationCriteria<T> {
    Difference(T),
    AbsoluteValue(T)
}

pub enum RunMode {
    Default,
    Debug,
    Sequential
}

pub struct Params<T>
        where T: PartialOrd<T> {
    numThreads: NumThreads,
    terminationCriteria: TerminationCriteria<T>,
    inputValue: V,
    runMode: RunMode
}

enum ParamsErrorKind {
    InvalidTerminationMode,
    InvalidTerminationValue(Error)
    InvalidNumThreads(Error),
    InvalidRunMode
}

struct ParamsError(ParamsErrorKind, Option<&Error> = None)

type ParamsResult<T> = Result<Params<T>, ParamsError>

// *** //

impl FromStr for NumThreads {
    type Err = ParamsError
    pub fn from_str(s: &str) -> Result<NumThreads, ParamsError> {
        match s {
            "0" => Ok(NumThreads::MaxAvailable),
            ss => match ss.parse<u16>() {
                Ok(n) => Ok(NumThreads::Fixed(n)),
                Err(e) => Err(ParamsError(ParamsErrorKind::InvalidNumThreads, e))
            }
        }
    }
}

impl<T> TerminationCriteria<T: FromStr> {
    pub fn from_args(mode: &str, value: &str) -> Result<TerminationCriteria<T>, ParamsError> {
        let result = match from_str::<T>(value) {
            Ok(v) => v,
            Err(e) => return Err(ParamsError(ParamsErrorKind::InvalidTerminationValue, e))
        }

        match mode {
            "f" => Ok(Difference(parsedValue)),
            "m" => Ok(AbsoluteValue(parsedValue)),
            _ => Err(ParamsError(ParamsErrorKind::InvalidTerminationMode))
        }
    }
}

impl FromStr for RunMode {
    type Err = ParamsError
    pub fn from_str(s: &str) -> Result<RunMode, ParamsError> {
        match s { 
            "d" => Ok(RunMode::Debug),
            "s" => Ok(RunMode::Sequential),
            "" => Ok(RunMode::Default),
            _ => Err(ParamsError(ParamsErrorKind::InvalidRunMode)) 
        }
    }
}

impl<T> Params<T> {
    pub fn from_args(args: &[&str]) -> ParamsResult {
        let args_i = args.iter()
        let num_threads: NumThreads = try!(args.next().parse())
        let term_mode: TerminationMode = try! {
            TerminationCriteria::from_args(args.next(), args.next())
        }
        let input_value: T = try!(args.next().parse())
    }
}

impl Error for ParamsError  {
    fn description(&self) -> &str {
        match *self.kind {
            InvalidTerminationMode => "Invalid termination mode, must be 'f' or 'm'",
            InvalidValue => format!("Invalid termination value: {0}", self.cause.unwrap().description())
            InvalidNumThreads => format!("Invalid number of threads, must be 0 or a positive int: {0}",
                                         self.cause.unwrap().description())
            InvalidRunMode => formmat!("Invalid run mode, must be 'd', 's' or not present")
        }
    }
}




impl FromStr for RunMode {
    fn from_str(s: &str) -> Result<NumThreads, InvalidRunMode> {
}






enum ParamsResult<T> {
    Params<T>,
    ParamsError(message: &str)
}

impl Params<T> where T: Float {
    pub fn fromArgs(args: [&str]) = {


    }

}
