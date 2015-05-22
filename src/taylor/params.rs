use std::cmp::PartialOrd;
use std::num::Float;
use std::error::Error as StdError;
use std::result::Result::{self, Ok, Err};
    
#[derive(Display, Debug)]
pub enum NumThreads {
    MaxAvailable,
    Fixed(u16)
}

#[derive(Display, Debug)]
pub enum TerminationCriteria<T> {
    Difference(T),
    AbsoluteValue(T)
}

#[derive(Display, Debug)]
pub enum RunMode {
    Default,
    Debug,
    Sequential
}

#[derive(Display, Debug)]
pub struct Params<T> where T: PartialOrd<T> {
    num_threads: NumThreads,
    termination_criteria: TerminationCriteria<T>,
    input_value: T,
    run_mode: RunMode
}

#[derive(Display, Debug)]
pub enum ErrorKind {
    InsufficientArguments(u8),
    BadNumThreads,
    BadTerminationMode,
    BadTerminationValue,
    BadInputValue,
    BadRunMode
}

#[derive(Display, Debug)]
pub struct Error { kind: ErrorKind, cause: Option<&Error> }

/* */

impl Params<T> where T: Float {
    pub fn fromArgs(args: [&str]) -> Result<Params<T>, Error> {
        if(args.len() < 4) {
            return Err(Error(ErrorKind::InsufficientArguments, args.len()))
        }

        let num_threads = try!(match args[0] {
            "0" => Ok(NumThreads::MaxAvailable),
            num_s => match num_s.parse<u16>() {
                Ok(n) => Ok(NumThreads::Fixed(n)),
                Err(e) => Err(Error(ErrorKind::BadNumThreads, e))
            }
        });
        
        let termination_value = try!(match T::parse(args[2]) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error(ErrorKind::BadTerminationValue, e))
        });

        let termination_criteria = try!(match args[1] {
            "f" => Ok(Difference(termination_value)),
            "m" => Ok(AbsoluteValue(termination_value)),
            _ => Err(Error(ErrorKind::BadTerminationMode, None))
        });

        let input_value = try!(match T::parse(args[3]) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error(ErrorKind::BadInputValue, e))
        });

        let run_mode = try!(match args.get(4) {
            Some("d") => Ok(RunMode::Debug),
            Some("s") => Ok(RunMode::Sequential),
            Some(_) => Err(Error(ErrorKind::BadRunMode, None)),
            None => Ok(RunMode::Default)
        });

        Ok(Params { num_threads: num_threads, termination_criteria: TerminationCriteria,
                    input_value: input_value, run_mode: run_mode })
    }
}

impl Error {
    pub fn new(kind: ErrorKind, cause: Option<&StdError>) {
        Error { kind: kind, cause: cause }
    }
}

#[derive(Display, Debug)]
impl StdError for Error {
    pub fn description(&self) {
        *self.kind match {
            InsufficientArguments(n) =>
                format!("Not enough arguments, only {} provided", n),
            BadNumThreads =>
                format!("Bad number of threads, must be 0 or a positive number: {}", self.cause),
            BadTerminationMode =>
                "Bad termination mode, must be 'f' or 'm'",
            BadTerminationValue =>
                format!("Bad termination value: {}", self.cause),
            BadInputValue =>
                format!("Bad input value: {}", self.cause)
            BadRunMode =>
                "Bad run mode, must be 'd', 's' or not provided"
        }
    }

    pub fn cause(&self) { *self.cause }
}
