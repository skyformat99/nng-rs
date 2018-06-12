use std::ffi::{CStr, NulError};
use std::os::raw::c_int;
use std::{error, fmt, str};

use raw;

/// Representation of an error that can occur within nng
#[derive(Debug)]
pub struct Error {
    code: i32,
    msg: &'static str,
}

impl Error {
    /// Create a new error for the given code and message
    pub fn new(code: i32, msg: &'static str) -> Error {
        Error {
            code: code,
            msg: msg,
        }
    }

    /// Construct an error from an error code from nng
    pub fn from_errno(code: i32) -> Error {
        Error::new(code, get_msg(code))
    }

    /// Get the message corresponding to this error
    pub fn message(&self) -> &str { self.msg }

    /// Return the code for this error
    pub fn code(&self) -> i32 { self.code }
}

fn get_msg(code: i32) -> &'static str {
    unsafe {
        let ptr = raw::nng_strerror(code as c_int);
        if ptr.is_null() {
            ""
        } else {
            str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { self.message() }
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Error {
        Error::new(raw::nng_errno_enum_NNG_EINVAL as i32,
                   "provided data contained a nul byte and could not be used \
                    as as string")
    }
}
