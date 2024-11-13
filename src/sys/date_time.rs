use crate::bindings::{
    Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy, Cronet_DateTime_value_get,
    Cronet_DateTime_value_set,
};

pub struct DateTime {
    ptr: Cronet_DateTimePtr,
    is_owned_ptr: bool,
}

impl DateTime {
    pub fn as_ptr(&self) -> Cronet_DateTimePtr {
        self.ptr
    }

    pub fn from_borrowed_ptr(ptr: Cronet_DateTimePtr) -> DateTime {
        DateTime{
            ptr, is_owned_ptr: false
        }
    }
}

impl Drop for DateTime {
    fn drop(&mut self) {
        if self.is_owned_ptr {
            unsafe { Cronet_DateTime_Destroy(self.ptr) }
        }
    }
}

impl DateTime {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_DateTime_Create();
            DateTime { ptr, is_owned_ptr: true }
        }
    }

    pub fn value_set(&self, value: i64) {
        unsafe {
            Cronet_DateTime_value_set(self.ptr, value);
        }
    }

    pub fn value_get(&self) -> i64 {
        unsafe { Cronet_DateTime_value_get(self.ptr) }
    }
}
