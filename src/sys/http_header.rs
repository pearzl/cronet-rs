use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_HttpHeaderPtr, Cronet_HttpHeader_Create, Cronet_HttpHeader_Destroy,
        Cronet_HttpHeader_name_get, Cronet_HttpHeader_name_set, Cronet_HttpHeader_value_get,
        Cronet_HttpHeader_value_set,
    },
    util::define_impl,
};

define_impl! {
    HttpHeader, Cronet_HttpHeaderPtr, Cronet_HttpHeader_Destroy,

    fn name_set(&mut Self, name: &CStr >> CStr::as_ptr);
        Cronet_HttpHeader_name_set,
    fn name_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_HttpHeader_name_get,

    fn value_set(&mut Self, value: &CStr >> CStr::as_ptr);
        Cronet_HttpHeader_value_set,
    fn value_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_HttpHeader_value_get,

}

impl HttpHeader {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_HttpHeader_Create();
            Self { ptr }
        }
    }
}
