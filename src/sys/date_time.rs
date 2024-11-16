use crate::{
    bindings::{
        Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy,
        Cronet_DateTime_value_get, Cronet_DateTime_value_set,
    },
    util::define_impl,
};

use super::Borrowed;

impl<'a> DateTime {
    pub(crate) fn as_ptr(&self) -> Cronet_DateTimePtr {
        self.ptr
    }

    pub(crate) fn into_raw(self) -> Cronet_DateTimePtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_DateTimePtr) -> Option<&'a mut DateTime> {
        if ptr.is_null() {
            return None;
        }
        let borrowed = DateTime { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Some(&mut *ptr)
    }

    pub fn borrow_from<X>(ptr: Cronet_DateTimePtr, lifetime: &'a X) -> Borrowed<'a, DateTime> {
        let borrowed = DateTime { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl DateTime {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_DateTime_Create();
            DateTime { ptr }
        }
    }
}

define_impl! {
    DateTime, Cronet_DateTimePtr, Cronet_DateTime_Destroy,

    fn value_set(&mut Self, value: i64); Cronet_DateTime_value_set,
    fn value_get(&Self) -> i64; Cronet_DateTime_value_get,
}
