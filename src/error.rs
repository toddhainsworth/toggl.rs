use core::fmt::{self, Display};
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct TogglError {
    inner: Context<String>,
}

impl Fail for TogglError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for TogglError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

// Allows writing `TogglError::from("oops"))?`
impl From<&'static str> for TogglError {
    fn from(msg: &'static str) -> TogglError {
        TogglError {
            inner: Context::new(msg.into()),
        }
    }
}

// Allows adding more context via a String
impl From<Context<String>> for TogglError {
    fn from(inner: Context<String>) -> TogglError {
        TogglError { inner }
    }
}

// Allows adding more context via a &str
impl From<Context<&'static str>> for TogglError {
    fn from(inner: Context<&'static str>) -> TogglError {
        TogglError {
            inner: inner.map(|s| s.to_string()),
        }
    }
}
