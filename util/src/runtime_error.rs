use core::fmt;

use std::error;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct RuntimeError(String);
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl RuntimeError {
    pub fn new<S>(message: S) -> RuntimeError
    where
        S: Into<String>,
    {
        RuntimeError(message.into())
    }
}
impl error::Error for RuntimeError {}

pub fn make_runtime_error(message: String) -> Box<dyn error::Error> {
    Box::new(RuntimeError::new(message))
}
