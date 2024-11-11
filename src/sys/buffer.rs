use crate::bindings::{
    Cronet_BufferCallbackPtr, Cronet_BufferPtr, Cronet_Buffer_Create, Cronet_Buffer_CreateWith, Cronet_Buffer_Destroy, Cronet_Buffer_GetClientContext, Cronet_Buffer_GetData, Cronet_Buffer_GetDataFunc, Cronet_Buffer_GetSize, Cronet_Buffer_GetSizeFunc, Cronet_Buffer_InitWithAlloc, Cronet_Buffer_InitWithAllocFunc, Cronet_Buffer_InitWithDataAndCallback, Cronet_Buffer_InitWithDataAndCallbackFunc, Cronet_Buffer_SetClientContext, Cronet_ClientContext, Cronet_RawDataPtr
};

pub struct Buffer {
    pub ptr: Cronet_BufferPtr,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            Cronet_Buffer_Destroy(self.ptr);
        }
    }
}

impl Buffer {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_Buffer_Create();
            Buffer { ptr }
        }
    }

    pub fn set_client_context(&self, client_context: Cronet_ClientContext) {
        unsafe {
            Cronet_Buffer_SetClientContext(self.ptr, client_context);
        }
    }

    pub fn get_client_context(&self) {
        unsafe {
            Cronet_Buffer_GetClientContext(self.ptr);
        }
    }

    pub fn init_with_data_and_callback(&self, data: Box<[u8]>, callback: Cronet_BufferCallbackPtr) {
        let len = data.len();
        let ptr = Box::into_raw(data);
        unsafe {
            Cronet_Buffer_InitWithDataAndCallback(
                self.ptr,
                ptr as Cronet_RawDataPtr,
                len as _,
                callback,
            );
        }
    }

    pub fn init_with_alloc(&self, size: u64) {
        unsafe {
            Cronet_Buffer_InitWithAlloc(self.ptr, size);
        }
    }

    pub fn get_size(&self) -> u64 {
        unsafe { Cronet_Buffer_GetSize(self.ptr) }
    }

    pub fn get_data(&self) -> Cronet_RawDataPtr {
        unsafe { Cronet_Buffer_GetData(self.ptr) }
    }

    pub fn crate_with(
        init_with_data_and_callback_func: Cronet_Buffer_InitWithDataAndCallbackFunc,
        init_with_alloc_func: Cronet_Buffer_InitWithAllocFunc,
        get_size_func: Cronet_Buffer_GetSizeFunc,
        get_data_func: Cronet_Buffer_GetDataFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_Buffer_CreateWith(
                init_with_data_and_callback_func,
                init_with_alloc_func,
                get_size_func,
                get_data_func,
            );
            Self { ptr }
        }
    }
}
