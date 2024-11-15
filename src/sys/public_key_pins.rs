use std::ffi::CStr;

use crate::bindings::{
    Cronet_PublicKeyPinsPtr, Cronet_PublicKeyPins_Create, Cronet_PublicKeyPins_Destroy,
    Cronet_PublicKeyPins_expiration_date_get, Cronet_PublicKeyPins_expiration_date_set,
    Cronet_PublicKeyPins_host_get, Cronet_PublicKeyPins_host_set,
    Cronet_PublicKeyPins_include_subdomains_get, Cronet_PublicKeyPins_include_subdomains_set,
    Cronet_PublicKeyPins_pins_sha256_add, Cronet_PublicKeyPins_pins_sha256_at,
    Cronet_PublicKeyPins_pins_sha256_clear, Cronet_PublicKeyPins_pins_sha256_size,
};

use super::Borrowed;

pub(crate) struct PublicKeyPins {
    ptr: Cronet_PublicKeyPinsPtr,
}

impl PublicKeyPins {
    pub(crate) fn as_ptr(&self) -> Cronet_PublicKeyPinsPtr {
        self.ptr
    }

    pub fn borrow_from(ptr: Cronet_PublicKeyPinsPtr) -> Borrowed<PublicKeyPins> {
        let borrowed = PublicKeyPins { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed { inner: ptr }
    }
}

impl Drop for PublicKeyPins {
    fn drop(&mut self) {
        unsafe {
            Cronet_PublicKeyPins_Destroy(self.ptr);
        }
    }
}

impl PublicKeyPins {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_PublicKeyPins_Create();
            Self { ptr }
        }
    }

    pub(crate) fn host_set(&mut self, host: &CStr) {
        unsafe {
            Cronet_PublicKeyPins_host_set(self.ptr, host.as_ptr());
        }
    }

    pub(crate) fn pins_sha256_add(&self, element: &CStr) {
        unsafe { Cronet_PublicKeyPins_pins_sha256_add(self.ptr, element.as_ptr()) }
    }

    pub(crate) fn include_subdomains_set(&mut self, include_subdomains: bool) {
        unsafe {
            Cronet_PublicKeyPins_include_subdomains_set(self.ptr, include_subdomains);
        }
    }

    pub(crate) fn expiration_date_set(&mut self, expiration_date: i64) {
        unsafe {
            Cronet_PublicKeyPins_expiration_date_set(self.ptr, expiration_date);
        }
    }

    pub(crate) fn host_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_PublicKeyPins_host_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn pins_sha256_size(&self) -> u32 {
        unsafe { Cronet_PublicKeyPins_pins_sha256_size(self.ptr) }
    }

    pub(crate) fn pins_sha256_at(&self, index: u32) -> &CStr {
        unsafe {
            let ptr = Cronet_PublicKeyPins_pins_sha256_at(self.ptr, index);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn pins_sha256_clear(&mut self) {
        unsafe { Cronet_PublicKeyPins_pins_sha256_clear(self.ptr) }
    }

    pub(crate) fn include_subdomains_get(&self) -> bool {
        unsafe { Cronet_PublicKeyPins_include_subdomains_get(self.ptr) }
    }

    pub(crate) fn expiration_date_get(&self) -> i64 {
        unsafe { Cronet_PublicKeyPins_expiration_date_get(self.ptr) }
    }
}
