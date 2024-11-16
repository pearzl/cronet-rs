use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_ErrorPtr, Cronet_Error_Create, Cronet_Error_Destroy, Cronet_Error_ERROR_CODE,
        Cronet_Error_error_code_get, Cronet_Error_error_code_set,
        Cronet_Error_immediately_retryable_get, Cronet_Error_immediately_retryable_set,
        Cronet_Error_internal_error_code_get, Cronet_Error_internal_error_code_set,
        Cronet_Error_message_get, Cronet_Error_message_set,
        Cronet_Error_quic_detailed_error_code_get, Cronet_Error_quic_detailed_error_code_set,
    },
    util::define_impl,
};

impl Error {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Error_Create();
            Self { ptr }
        }
    }
}

define_impl! {
    Error, Cronet_ErrorPtr, Cronet_Error_Destroy,


    fn error_code_set(&mut Self, error_code: Cronet_Error_ERROR_CODE);
        Cronet_Error_error_code_set,
    fn error_code_get(&Self) -> Cronet_Error_ERROR_CODE;
        Cronet_Error_error_code_get,

    fn message_set(&mut Self, message: &CStr >> CStr::as_ptr);
        Cronet_Error_message_set,
    fn message_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_Error_message_get,

    fn internal_error_code_set(&mut Self, internal_error_code: i32);
        Cronet_Error_internal_error_code_set,
    fn internal_error_code_get(&Self) -> i32;
        Cronet_Error_internal_error_code_get,

    fn immediately_retryable_set(&mut Self, immediately_retryable: bool);
        Cronet_Error_immediately_retryable_set,
    fn immediately_retryable_get(&Self) -> bool;
        Cronet_Error_immediately_retryable_get,

    fn quic_detailed_error_code_set(&mut Self, quic_detailed_error_code: i32);
        Cronet_Error_quic_detailed_error_code_set,
    fn quic_detailed_error_code_get(&Self) -> i32;
        Cronet_Error_quic_detailed_error_code_get,

}
