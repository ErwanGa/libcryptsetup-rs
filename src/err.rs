use std::{
    error::Error,
    ffi::NulError,
    fmt::{self, Display},
    io,
    str::Utf8Error,
};

use uuid::parser::ParseError;

#[derive(Debug)]
/// Error returned from any libcryptsetup-rs function
pub enum LibcryptErr {
    /// Wrapper for `io::Error`
    IOError(io::Error),
    /// Wrapper for `uuid::parser::ParseError`
    UuidError(ParseError),
    /// Wrapper for `ffi::NulError`
    NullError(NulError),
    /// Wrapper for `str::Utf8Error`
    Utf8Error(Utf8Error),
    /// Wrapper for `serde_json::Error`
    JsonError(serde_json::Error),
    /// Indicates that a Rust/C conversion was unsuccessful
    InvalidConversion,
    /// Indicates that a pointer returned was null signifying an error
    NullPtr,
    /// Indicates that a `&'static str` was not created with `c_str!()` macro
    NoNull(&'static str),
}

impl Display for LibcryptErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibcryptErr::IOError(ref e) => write!(f, "IO error occurred: {}", e),
            LibcryptErr::UuidError(ref e) => write!(f, "Failed to parse UUID from C string: {}", e),
            LibcryptErr::NullError(ref e) => write!(
                f,
                "Null error occurred when handling &str conversion: {}",
                e
            ),
            LibcryptErr::Utf8Error(ref e) => write!(
                f,
                "UTF8 error occurred when handling &str conversion: {}",
                e
            ),
            LibcryptErr::JsonError(ref e) => {
                write!(f, "Failed to parse the provided string into JSON: {}", e)
            }
            LibcryptErr::InvalidConversion => {
                write!(f, "Failed to perform the specified conversion")
            }
            LibcryptErr::NullPtr => write!(f, "Cryptsetup returned a null pointer"),
            LibcryptErr::NoNull(s) => {
                write!(f, "Static string {} was not created with c_str!() macro", s)
            }
        }
    }
}

impl Error for LibcryptErr {}
