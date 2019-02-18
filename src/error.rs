use std::{io, fmt, string, time, sync::Arc, error::Error as StdError};
use serde_json;
use serde_yaml;
use rlua_serde;
use rlua;
use git2;
use base64;
use scl;
use rlua::Error as LuaError;
use splitdiff_rs;
use patch_rs;

//Due to number of number of crates that have different errors, we will handle them this way for the time being which would just create a base for
//handling errors properly. The errors will be bound to change in the near future

#[derive(Debug)]
pub enum Error {
    SodiumInitFailure,
    InvalidNonce,
    InvalidNonceObject,
    FailedToDecrypt,
    VerifyError,
    InvalidKeys,
    InvalidSignature,
    InternalError,
    LuaError(rlua::Error),
    IoError(io::Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    LuaSerdeError(rlua_serde::error::Error),   
    GitError(git2::Error),
    Base64Error(base64::DecodeError),
    StringUtf8Error(string::FromUtf8Error),
    SclError(scl::Error),
    SysTimeError(time::SystemTimeError),
    SplitDiffError(splitdiff_rs::Error),
    PatchError(patch_rs::PatchError),
    //TODO: This is only temp as a place holder for anything making use of a string for error messages
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SodiumInitFailure => writeln!(fmt, "Failed to initialize libsodium."),
            Error::InvalidNonce => writeln!(fmt, "Failed to load nonce, data is invalid."),
            Error::InvalidNonceObject => writeln!(fmt, "Object passed is in not Nonce."),
            Error::FailedToDecrypt => writeln!(fmt, "Failed to decrypt."),
            Error::VerifyError => writeln!(fmt, "Failed to verify signed message."),
            Error::InvalidSignature => writeln!(fmt, "Failed to load key, data is invalid."),
            Error::InvalidKeys => writeln!(fmt, "Failed verify, signature is invalid."),
            Error::InternalError => writeln!(fmt, "Internal error as occured."),
            Error::LuaError(ref err) => writeln!(fmt, "{}", err),
            Error::IoError(ref err) => writeln!(fmt, "{}", err),
            Error::JsonError(ref err) => writeln!(fmt, "{}", err),
            Error::YamlError(ref err) => writeln!(fmt, "{}", err),
            Error::LuaSerdeError(ref err) => writeln!(fmt, "{}", err),
            Error::GitError(ref err) => writeln!(fmt, "{}", err),
            Error::Base64Error(ref err) => writeln!(fmt, "{}", err),
            Error::StringUtf8Error(ref err) => writeln!(fmt, "{}", err),
            Error::SclError(ref err) => writeln!(fmt, "{}", err),
            Error::SysTimeError(ref err) => writeln!(fmt, "{}", err),
            Error::SplitDiffError(ref err) => writeln!(fmt, "{}", err),
            Error::PatchError(ref err) => writeln!(fmt, "{}", err),
            Error::Other(ref err) => writeln!(fmt, "{}", err),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::LuaError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::JsonError(ref err) => Some(err),
            Error::YamlError(ref err) => Some(err),
            Error::LuaSerdeError(ref err) => Some(err),
            Error::GitError(ref err) => Some(err),
            Error::Base64Error(ref err) => Some(err),
            Error::StringUtf8Error(ref err) => Some(err),
            Error::SysTimeError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<rlua::Error> for Error {
    fn from(err: rlua::prelude::LuaError) -> Error {
        Error::LuaError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::YamlError(err)
    }
}

impl From<rlua_serde::error::Error> for Error {
    fn from(err: rlua_serde::error::Error) -> Error {
        Error::LuaSerdeError(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::GitError(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::Base64Error(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringUtf8Error(err)
    }
}

impl From<scl::Error> for Error {
    fn from(err: scl::Error) -> Error {
        Error::SclError(err)
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(err: time::SystemTimeError) -> Error {
        Error::SysTimeError(err)
    }
}

impl From<splitdiff_rs::Error> for Error {
    fn from(err: splitdiff_rs::Error) -> Error {
        Error::SplitDiffError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}

pub fn create_lua_error <T> (err: T) -> LuaError
   where T: StdError + Sync + Send + 'static {
    LuaError::ExternalError(Arc::new(Box::new(err)))
}

#[macro_export]
macro_rules! format_err {
    ($($arg:tt)*) => { $crate::error::Error::Other(format!($($arg)*)) }
}