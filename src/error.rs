use std::ffi::CString;

use crate::{bindings::{Cronet_Error_ERROR_CODE, Cronet_RESULT}, sys};

#[derive(Debug)]
pub enum Error {
    CronetError{
        code: Cronet_Error_ERROR_CODE,
        message: CString,
        internal_error_code: i32,
        immediately_retryable: bool,
        quic_detailed_error_code: i32,
    },
    Canceled, // not expected to been seen by user.
    CronetResult(Cronet_RESULT),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;


impl<'a> From<&'a sys::Error> for Error {
    fn from(value: &'a sys::Error) -> Self {
        Self::CronetError { code: value.error_code_get(), message: value.message_get().to_owned(), internal_error_code: value.internal_error_code_get(), immediately_retryable: value.immediately_retryable_get(), quic_detailed_error_code: value.quic_detailed_error_code_get() }
    }
}
