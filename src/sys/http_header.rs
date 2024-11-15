use std::ffi::CStr;

use crate::bindings::{
    Cronet_HttpHeaderPtr, Cronet_HttpHeader_Create, Cronet_HttpHeader_Destroy,
    Cronet_HttpHeader_name_get, Cronet_HttpHeader_name_set, Cronet_HttpHeader_value_get,
    Cronet_HttpHeader_value_set,
};

use super::Borrowed;

pub(crate) struct HttpHeader {
    ptr: Cronet_HttpHeaderPtr,
}

impl HttpHeader {
    pub(crate) fn as_ptr(&self) -> Cronet_HttpHeaderPtr {
        self.ptr
    }

    pub fn borrow_from(ptr: Cronet_HttpHeaderPtr) -> Borrowed<HttpHeader> {
        let borrowed = HttpHeader { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed { inner: ptr }
    }
}

impl Drop for HttpHeader {
    fn drop(&mut self) {
        unsafe {
            Cronet_HttpHeader_Destroy(self.ptr);
        }
    }
}

impl HttpHeader {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_HttpHeader_Create();
            Self { ptr }
        }
    }

    pub(crate) fn name_set(&mut self, name: &CStr) {
        unsafe {
            Cronet_HttpHeader_name_set(self.ptr, name.as_ptr());
        }
    }

    pub(crate) fn value_set(&mut self, value: &CStr) {
        unsafe {
            Cronet_HttpHeader_value_set(self.ptr, value.as_ptr());
        }
    }

    pub(crate) fn name_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_HttpHeader_name_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn value_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_HttpHeader_value_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }
}
