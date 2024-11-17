use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_PublicKeyPinsPtr, Cronet_PublicKeyPins_Create, Cronet_PublicKeyPins_Destroy,
        Cronet_PublicKeyPins_expiration_date_get, Cronet_PublicKeyPins_expiration_date_set,
        Cronet_PublicKeyPins_host_get, Cronet_PublicKeyPins_host_set,
        Cronet_PublicKeyPins_include_subdomains_get, Cronet_PublicKeyPins_include_subdomains_set,
        Cronet_PublicKeyPins_pins_sha256_add, Cronet_PublicKeyPins_pins_sha256_at,
        Cronet_PublicKeyPins_pins_sha256_clear, Cronet_PublicKeyPins_pins_sha256_size,
    },
    util::define_impl,
};

impl<'a> PublicKeyPins {
    pub(crate) fn as_ptr(&self) -> Cronet_PublicKeyPinsPtr {
        self.ptr
    }
}

define_impl! {
    PublicKeyPins, Cronet_PublicKeyPinsPtr, Cronet_PublicKeyPins_Destroy,

    fn host_set(&mut Self, host: &CStr >> CStr::as_ptr);
        Cronet_PublicKeyPins_host_set,
    fn host_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_PublicKeyPins_host_get,

    fn pins_sha256_add(&mut Self, element: &CStr >> CStr::as_ptr);// safety: cloned
        Cronet_PublicKeyPins_pins_sha256_add,
    fn pins_sha256_size(&Self) -> u32;
        Cronet_PublicKeyPins_pins_sha256_size,
    fn pins_sha256_at(&Self, index: u32) -> &CStr >> CStr::from_ptr; // safety: out of bounds
        Cronet_PublicKeyPins_pins_sha256_at,
    fn pins_sha256_clear(&mut Self);
        Cronet_PublicKeyPins_pins_sha256_clear,

    fn include_subdomains_set(&mut Self, include_subdomains: bool);
        Cronet_PublicKeyPins_include_subdomains_set,
    fn include_subdomains_get(&Self) -> bool;
        Cronet_PublicKeyPins_include_subdomains_get,

    fn expiration_date_set(&mut Self, expiration_date: i64);
        Cronet_PublicKeyPins_expiration_date_set,
    fn expiration_date_get(&Self) -> i64;
        Cronet_PublicKeyPins_expiration_date_get,
}

impl PublicKeyPins {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_PublicKeyPins_Create();
            Self { ptr }
        }
    }
}
