use std::ffi::CStr;

use crate::bindings::{
    Cronet_QuicHintPtr, Cronet_QuicHint_Create, Cronet_QuicHint_Destroy,
    Cronet_QuicHint_alternate_port_get, Cronet_QuicHint_alternate_port_set,
    Cronet_QuicHint_host_get, Cronet_QuicHint_host_set, Cronet_QuicHint_port_get,
    Cronet_QuicHint_port_set,
};

use super::Borrowed;

pub(crate) struct QuicHint {
    ptr: Cronet_QuicHintPtr,
}

impl<'a> QuicHint {
    pub(crate) fn as_ptr(&self) -> Cronet_QuicHintPtr {
        self.ptr
    }

    pub fn borrow_from<X>(ptr: Cronet_QuicHintPtr, lifetime: &'a X) -> Borrowed<'a, QuicHint> {
        let borrowed = QuicHint { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl Drop for QuicHint {
    fn drop(&mut self) {
        unsafe { Cronet_QuicHint_Destroy(self.ptr) }
    }
}

impl QuicHint {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_QuicHint_Create();
            Self { ptr }
        }
    }

    pub(crate) fn host_set(&mut self, host: &CStr) {
        unsafe {
            Cronet_QuicHint_host_set(self.ptr, host.as_ptr());
        }
    }

    pub(crate) fn port_set(&mut self, port: i32) {
        unsafe {
            Cronet_QuicHint_port_set(self.ptr, port);
        }
    }

    pub(crate) fn alternate_port_set(&mut self, alternate_port: i32) {
        unsafe {
            Cronet_QuicHint_alternate_port_set(self.ptr, alternate_port);
        }
    }

    pub(crate) fn host_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_QuicHint_host_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn port_get(&self) -> i32 {
        unsafe { Cronet_QuicHint_port_get(self.ptr) }
    }

    pub(crate) fn alternate_port_get(&self) -> i32 {
        unsafe { Cronet_QuicHint_alternate_port_get(self.ptr) }
    }
}
