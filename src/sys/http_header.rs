use std::ffi::CStr;

use crate::bindings::{
    Cronet_HttpHeaderPtr, Cronet_HttpHeader_Create, Cronet_HttpHeader_Destroy,
    Cronet_HttpHeader_name_get, Cronet_HttpHeader_name_set, Cronet_HttpHeader_value_get,
    Cronet_HttpHeader_value_set,
};

pub struct HttpHeader {
    ptr: Cronet_HttpHeaderPtr,
    is_owned_ptr: bool,
}

impl HttpHeader {
    pub fn as_ptr(&self) -> Cronet_HttpHeaderPtr {
        self.ptr
    }

    pub fn from_borrowed_ptr(ptr: Cronet_HttpHeaderPtr) -> HttpHeader {
        HttpHeader {
            ptr,
            is_owned_ptr: false,
        }
    }
}

impl Drop for HttpHeader {
    fn drop(&mut self) {
        if self.is_owned_ptr {
            unsafe {
                Cronet_HttpHeader_Destroy(self.ptr);
            }
        }
    }
}

impl HttpHeader {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_HttpHeader_Create();
            Self {
                ptr,
                is_owned_ptr: true,
            }
        }
    }

    pub fn name_set(&mut self, name: &CStr) {
        unsafe {
            Cronet_HttpHeader_name_set(self.ptr, name.as_ptr());
        }
    }

    pub fn value_set(&mut self, value: &CStr) {
        unsafe {
            Cronet_HttpHeader_value_set(self.ptr, value.as_ptr());
        }
    }

    pub fn name_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_HttpHeader_name_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub fn value_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_HttpHeader_value_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }
}
