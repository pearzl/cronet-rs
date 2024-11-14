use std::ffi::CStr;

use crate::bindings::{
    Cronet_ErrorPtr, Cronet_Error_Create, Cronet_Error_Destroy, Cronet_Error_ERROR_CODE,
    Cronet_Error_error_code_get, Cronet_Error_error_code_set,
    Cronet_Error_immediately_retryable_get, Cronet_Error_immediately_retryable_set,
    Cronet_Error_internal_error_code_get, Cronet_Error_internal_error_code_set,
    Cronet_Error_message_get, Cronet_Error_message_set, Cronet_Error_quic_detailed_error_code_get,
    Cronet_Error_quic_detailed_error_code_set,
};

pub(crate) struct Error {
    ptr: Cronet_ErrorPtr,
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe { Cronet_Error_Destroy(self.ptr) }
    }
}

impl Error {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Error_Create();
            Self { ptr }
        }
    }

    pub(crate) fn error_code_set(&mut self, error_code: Cronet_Error_ERROR_CODE) {
        unsafe { Cronet_Error_error_code_set(self.ptr, error_code) }
    }

    pub(crate) fn message_set(&mut self, message: &CStr) {
        unsafe { Cronet_Error_message_set(self.ptr, message.as_ptr()) }
    }

    pub(crate) fn internal_error_code_set(&mut self, internal_error_code: i32) {
        unsafe { Cronet_Error_internal_error_code_set(self.ptr, internal_error_code) }
    }

    pub(crate) fn immediately_retryable_set(&mut self, immediately_retryable: bool) {
        unsafe { Cronet_Error_immediately_retryable_set(self.ptr, immediately_retryable) }
    }

    pub(crate) fn quic_detailed_error_code_set(&mut self, quic_detailed_error_code: i32) {
        unsafe { Cronet_Error_quic_detailed_error_code_set(self.ptr, quic_detailed_error_code) }
    }

    pub(crate) fn error_code_get(&self) -> Cronet_Error_ERROR_CODE {
        unsafe { Cronet_Error_error_code_get(self.ptr) }
    }

    pub(crate) fn message_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_Error_message_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn internal_error_code_get(&self) -> i32 {
        unsafe { Cronet_Error_internal_error_code_get(self.ptr) }
    }

    pub(crate) fn immediately_retryable_get(&self) -> bool {
        unsafe { Cronet_Error_immediately_retryable_get(self.ptr) }
    }

    pub(crate) fn quic_detailed_error_code_get(&self) -> i32 {
        unsafe { Cronet_Error_quic_detailed_error_code_get(self.ptr) }
    }
}
