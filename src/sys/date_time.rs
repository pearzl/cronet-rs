use crate::bindings::{
    Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy, Cronet_DateTime_value_get,
    Cronet_DateTime_value_set,
};

use super::Borrowed;

pub struct DateTime {
    ptr: Cronet_DateTimePtr,
}

impl DateTime {
    pub(crate) fn as_ptr(&self) -> Cronet_DateTimePtr {
        self.ptr
    }

    pub fn borrow_from(ptr: Cronet_DateTimePtr) -> Borrowed<DateTime> {
        let borrowed = DateTime { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed { inner: ptr }
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        unsafe { Cronet_DateTime_Destroy(self.ptr) }
    }
}

impl DateTime {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_DateTime_Create();
            DateTime { ptr }
        }
    }

    pub(crate) fn value_set(&mut self, value: i64) {
        unsafe {
            Cronet_DateTime_value_set(self.ptr, value);
        }
    }

    pub(crate) fn value_get(&self) -> i64 {
        unsafe { Cronet_DateTime_value_get(self.ptr) }
    }
}
