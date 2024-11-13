use std::ffi::CStr;

use crate::bindings::{
    Cronet_ErrorPtr, Cronet_Error_Create, Cronet_Error_Destroy, Cronet_Error_ERROR_CODE,
    Cronet_Error_error_code_get, Cronet_Error_error_code_set,
    Cronet_Error_immediately_retryable_get, Cronet_Error_immediately_retryable_set,
    Cronet_Error_internal_error_code_get, Cronet_Error_internal_error_code_set,
    Cronet_Error_message_get, Cronet_Error_message_set, Cronet_Error_quic_detailed_error_code_get,
    Cronet_Error_quic_detailed_error_code_set,
};

pub struct Error {
    ptr: Cronet_ErrorPtr,
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe { Cronet_Error_Destroy(self.ptr) }
    }
}

impl Error {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_Error_Create();
            Self { ptr }
        }
    }

    pub fn error_code_set(&self, error_code: Cronet_Error_ERROR_CODE) {
        unsafe { Cronet_Error_error_code_set(self.ptr, error_code) }
    }

    pub fn message_set(&self, message: &CStr) {
        unsafe { Cronet_Error_message_set(self.ptr, message.as_ptr()) }
    }

    pub fn internal_error_code_set(&self, internal_error_code: i32) {
        unsafe { Cronet_Error_internal_error_code_set(self.ptr, internal_error_code) }
    }

    pub fn immediately_retryable_set(&self, immediately_retryable: bool) {
        unsafe { Cronet_Error_immediately_retryable_set(self.ptr, immediately_retryable) }
    }

    pub fn quic_detailed_error_code_set(&self, quic_detailed_error_code: i32) {
        unsafe { Cronet_Error_quic_detailed_error_code_set(self.ptr, quic_detailed_error_code) }
    }

    pub fn error_code_get(&self) -> Cronet_Error_ERROR_CODE {
        unsafe { Cronet_Error_error_code_get(self.ptr) }
    }

    pub fn message_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_Error_message_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub fn internal_error_code_get(&self) -> i32 {
        unsafe { Cronet_Error_internal_error_code_get(self.ptr) }
    }

    pub fn immediately_retryable_get(&self) -> bool {
        unsafe { Cronet_Error_immediately_retryable_get(self.ptr) }
    }

    pub fn quic_detailed_error_code_get(&self) -> i32 {
        unsafe { Cronet_Error_quic_detailed_error_code_get(self.ptr) }
    }
}
