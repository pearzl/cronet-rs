use std::ffi::CStr;

use crate::bindings::{
    Cronet_PublicKeyPinsPtr, Cronet_PublicKeyPins_Create, Cronet_PublicKeyPins_Destroy,
    Cronet_PublicKeyPins_expiration_date_get, Cronet_PublicKeyPins_expiration_date_set,
    Cronet_PublicKeyPins_host_get, Cronet_PublicKeyPins_host_set,
    Cronet_PublicKeyPins_include_subdomains_get, Cronet_PublicKeyPins_include_subdomains_set,
    Cronet_PublicKeyPins_pins_sha256_add, Cronet_PublicKeyPins_pins_sha256_at,
    Cronet_PublicKeyPins_pins_sha256_clear, Cronet_PublicKeyPins_pins_sha256_size,
};

pub struct PublicKeyPins {
    ptr: Cronet_PublicKeyPinsPtr,
    is_owned_ptr: bool,
}

impl PublicKeyPins {
    pub fn as_ptr(&self) -> Cronet_PublicKeyPinsPtr {
        self.ptr
    }

    pub fn from_borrowed_ptr(ptr: Cronet_PublicKeyPinsPtr) -> Self {
        PublicKeyPins{ptr, is_owned_ptr: false}
    }
}


impl Drop for PublicKeyPins {
    fn drop(&mut self) {
        if self.is_owned_ptr {
            unsafe {
                Cronet_PublicKeyPins_Destroy(self.ptr);
            }
        }
    }
}

impl PublicKeyPins {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_PublicKeyPins_Create();
            Self { ptr, is_owned_ptr:true }
        }
    }

    pub fn host_set(&self, host: &CStr) {
        unsafe {
            Cronet_PublicKeyPins_host_set(self.ptr, host.as_ptr());
        }
    }

    pub fn pins_sha256_add(&self, element: &CStr) {
        unsafe { Cronet_PublicKeyPins_pins_sha256_add(self.ptr, element.as_ptr()) }
    }

    pub fn include_subdomains_set(&self, include_subdomains: bool) {
        unsafe {
            Cronet_PublicKeyPins_include_subdomains_set(self.ptr, include_subdomains);
        }
    }

    pub fn expiration_date_set(&self, expiration_date: i64) {
        unsafe {
            Cronet_PublicKeyPins_expiration_date_set(self.ptr, expiration_date);
        }
    }

    pub fn host_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_PublicKeyPins_host_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub fn pins_sha256_size(&self) -> u32 {
        unsafe { Cronet_PublicKeyPins_pins_sha256_size(self.ptr) }
    }

    pub fn pins_sha256_at(&self, index: u32) -> &CStr {
        unsafe {
            let ptr = Cronet_PublicKeyPins_pins_sha256_at(self.ptr, index);
            CStr::from_ptr(ptr)
        }
    }

    pub fn pins_sha256_clear(&self) {
        unsafe { Cronet_PublicKeyPins_pins_sha256_clear(self.ptr) }
    }

    pub fn include_subdomains_get(&self) -> bool {
        unsafe { Cronet_PublicKeyPins_include_subdomains_get(self.ptr) }
    }

    pub fn expiration_date_get(&self) -> i64 {
        unsafe { Cronet_PublicKeyPins_expiration_date_get(self.ptr) }
    }
}
