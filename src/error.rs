use std::fmt::{Display, Formatter};

// TODO: better exit codes?

macro_rules! enum_InnerError {
    ($($error_t:ident($ty:ty)),*) => {
        #[derive(Debug)]
        enum InnerError {
            $($error_t($ty),)*
        }

        impl Display for InnerError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(InnerError::$error_t(x) => x.fmt(f),)*
                }
            }
        }
    }
}

enum_InnerError! {
    Other(Generic),
    IO(std::io::Error)
}

#[derive(Debug)]
pub struct Error {
    inner: InnerError,
    pub exit: i32
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error::from(Generic::new(message))
    }

    pub fn result<T, E>(error: E) -> Result<T, Error> where Error: std::convert::From<E> {
        Err(Error::from(error))
    }

    pub fn new_result<T>(message: &str) -> Result<T, Error> {
        Err(Error::new(message))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            inner: InnerError::IO(error),
            exit: 2
        }
    }
}

impl From<Generic> for Error {
    fn from(error: Generic) -> Self {
        Error {
            inner: InnerError::Other(error),
            exit: 3
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug)]
struct Generic {
    msg: String
}

impl Generic {
    pub fn new(message: &str) -> Generic {
        Generic {
            msg: message.to_string()
        }
    }
}

impl Display for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
