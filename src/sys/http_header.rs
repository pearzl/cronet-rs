use std::{ffi::CStr, mem::ManuallyDrop, ops::Deref};

use crate::bindings::{
    Cronet_HttpHeaderPtr, Cronet_HttpHeader_Create, Cronet_HttpHeader_Destroy,
    Cronet_HttpHeader_name_get, Cronet_HttpHeader_name_set, Cronet_HttpHeader_value_get,
    Cronet_HttpHeader_value_set,
};

pub(crate) struct HttpHeader {
    ptr: Cronet_HttpHeaderPtr,
}

impl HttpHeader {
    pub(crate) fn as_ptr(&self) -> Cronet_HttpHeaderPtr {
        self.ptr
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

pub(crate) struct BorrowedHttpHeader {
    inner: ManuallyDrop<HttpHeader>,
}

impl BorrowedHttpHeader {
    pub(crate) fn from_ptr(ptr: Cronet_HttpHeaderPtr) -> Self {
        let value = HttpHeader { ptr };
        BorrowedHttpHeader {
            inner: ManuallyDrop::new(value),
        }
    }
}

impl Deref for BorrowedHttpHeader {
    type Target = HttpHeader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
