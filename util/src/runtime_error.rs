use core::fmt;

use std::error;

#[derive(Clone, Debug)]
pub struct RuntimeError(String);
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
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
