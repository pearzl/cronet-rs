use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_QuicHintPtr, Cronet_QuicHint_Create, Cronet_QuicHint_Destroy,
        Cronet_QuicHint_alternate_port_get, Cronet_QuicHint_alternate_port_set,
        Cronet_QuicHint_host_get, Cronet_QuicHint_host_set, Cronet_QuicHint_port_get,
        Cronet_QuicHint_port_set,
    },
    util::define_impl,
};

use super::Borrowed;

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

define_impl! {
    QuicHint, Cronet_QuicHintPtr, Cronet_QuicHint_Destroy,

    fn host_set(&mut Self, host: &CStr >> CStr::as_ptr);
        Cronet_QuicHint_host_set,
    fn host_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_QuicHint_host_get,

    fn port_set(&mut Self, port: i32);
        Cronet_QuicHint_port_set,
    fn port_get(&Self) -> i32;
        Cronet_QuicHint_port_get,

    fn alternate_port_set(&mut Self, alternate_port: i32);
        Cronet_QuicHint_alternate_port_set,
    fn alternate_port_get(&Self) -> i32;
        Cronet_QuicHint_alternate_port_get,
}

impl QuicHint {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_QuicHint_Create();
            Self { ptr }
        }
    }
}
