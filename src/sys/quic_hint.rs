use std::{ffi::CStr, mem::ManuallyDrop, ops::Deref};

use crate::bindings::{
    Cronet_QuicHintPtr, Cronet_QuicHint_Create, Cronet_QuicHint_Destroy,
    Cronet_QuicHint_alternate_port_get, Cronet_QuicHint_alternate_port_set,
    Cronet_QuicHint_host_get, Cronet_QuicHint_host_set, Cronet_QuicHint_port_get,
    Cronet_QuicHint_port_set,
};

pub(crate) struct QuicHint {
    ptr: Cronet_QuicHintPtr,
}

impl QuicHint {
    pub(crate) fn as_ptr(&self) -> Cronet_QuicHintPtr {
        self.ptr
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

pub(crate) struct BorrowedQuicHint {
    inner: ManuallyDrop<QuicHint>,
}

impl BorrowedQuicHint {
    pub(crate) fn from_ptr(ptr: Cronet_QuicHintPtr) -> Self {
        let value = QuicHint { ptr };
        BorrowedQuicHint {
            inner: ManuallyDrop::new(value),
        }
    }
}

impl Deref for BorrowedQuicHint {
    type Target = QuicHint;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
