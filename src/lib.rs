//! Package implement Persistent Ordered Map.

use std::{error, fmt, result};

// error.rs
// lib.rs
// llrb_common.rs
// llrb_node.rs
// llrb.rs
// mvcc.rs

// Short form to compose Error values.
//
// Here are few possible ways:
//
// ```ignore
// use crate::Error;
// err_at!(ParseError, msg: format!("bad argument"));
// ```
//
// ```ignore
// use crate::Error;
// err_at!(ParseError, std::io::read(buf));
// ```
//
// ```ignore
// use crate::Error;
// err_at!(ParseError, std::fs::read(file_path), format!("read failed"));
// ```
//
macro_rules! err_at {
    ($v:ident, msg: $($arg:expr),+) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!($($arg),+)))
    }};
    ($v:ident, $e:expr) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    }};
    ($v:ident, $e:expr, $($arg:expr),+) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                let msg = format!($($arg),+);
                Err(Error::$v(prefix, format!("{} {}", err, msg)))
            }
        }
    }};
}

mod arc;
mod mem_db;
mod node;
mod op;

pub use mem_db::Mdb;

/// Error variants that can be returned by this package's API.
///
/// Each variant carries a prefix, typically identifying the
/// error location.
pub enum Error {
    Fatal(String, String),
    KeyNotFound(String, String),
    InvalidCAS(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::*;

        match self {
            Fatal(p, msg) => write!(f, "{} Fatal: {}", p, msg),
            KeyNotFound(p, msg) => write!(f, "{} KeyNotFound: {}", p, msg),
            InvalidCAS(p, msg) => write!(f, "{} InvalidCAS: {}", p, msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

/// Type alias for Result return type, used by this package.
pub type Result<T> = result::Result<T, Error>;
