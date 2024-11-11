use crate::bindings::{
    Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy, Cronet_DateTime_value_get,
    Cronet_DateTime_value_set,
};

pub struct DateTime {
    pub ptr: Cronet_DateTimePtr,
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

    pub fn value_set(&self, value: i64) {
        unsafe {
            Cronet_DateTime_value_set(self.ptr, value);
        }
    }

    pub fn value_get(&self) -> i64 {
        unsafe { Cronet_DateTime_value_get(self.ptr) }
    }
}
