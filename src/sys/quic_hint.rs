use std::ffi::CStr;

use crate::bindings::{
    Cronet_QuicHintPtr, Cronet_QuicHint_Create, Cronet_QuicHint_Destroy,
    Cronet_QuicHint_alternate_port_get, Cronet_QuicHint_alternate_port_set,
    Cronet_QuicHint_host_get, Cronet_QuicHint_host_set, Cronet_QuicHint_port_get,
    Cronet_QuicHint_port_set,
};

pub struct QuicHint {
    pub ptr: Cronet_QuicHintPtr,
}

impl Drop for QuicHint {
    fn drop(&mut self) {
        unsafe { Cronet_QuicHint_Destroy(self.ptr) }
    }
}

impl QuicHint {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_QuicHint_Create();
            Self { ptr }
        }
    }

    pub fn host_set(&self, host: &CStr) {
        unsafe {
            Cronet_QuicHint_host_set(self.ptr, host.as_ptr());
        }
    }

    pub fn port_set(&self, port: i32) {
        unsafe {
            Cronet_QuicHint_port_set(self.ptr, port);
        }
    }

    pub fn alternate_port_set(&self, alternate_port: i32) {
        unsafe {
            Cronet_QuicHint_alternate_port_set(self.ptr, alternate_port);
        }
    }

    pub fn host_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_QuicHint_host_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub fn port_get(&self) -> i32 {
        unsafe { Cronet_QuicHint_port_get(self.ptr) }
    }

    pub fn alternate_port_get(&self) -> i32 {
        unsafe { Cronet_QuicHint_alternate_port_get(self.ptr) }
    }
}
