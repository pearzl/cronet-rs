use std::{mem::ManuallyDrop, ops::Deref};

use crate::bindings::{
    Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy, Cronet_DateTime_value_get,
    Cronet_DateTime_value_set,
};

pub struct DateTime {
    ptr: Cronet_DateTimePtr,
}

impl DateTime {
    pub fn as_ptr(&self) -> Cronet_DateTimePtr {
        self.ptr
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        unsafe { Cronet_DateTime_Destroy(self.ptr) }
    }
}

impl DateTime {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_DateTime_Create();
            DateTime { ptr }
        }
    }

    pub fn value_set(&mut self, value: i64) {
        unsafe {
            Cronet_DateTime_value_set(self.ptr, value);
        }
    }

    pub fn value_get(&self) -> i64 {
        unsafe { Cronet_DateTime_value_get(self.ptr) }
    }
}

pub struct BorrowedDateTime {
    inner: ManuallyDrop<DateTime>,
}

impl BorrowedDateTime {
    pub fn from_ptr(ptr: Cronet_DateTimePtr) -> Self {
        let value = DateTime { ptr };
        BorrowedDateTime {
            inner: ManuallyDrop::new(value),
        }
    }
}

impl Deref for BorrowedDateTime {
    type Target = DateTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
