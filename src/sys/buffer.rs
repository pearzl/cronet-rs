use std::marker::PhantomData;

use crate::bindings::{
    Cronet_BufferCallbackPtr, Cronet_BufferPtr, Cronet_Buffer_Create, Cronet_Buffer_CreateWith,
    Cronet_Buffer_Destroy, Cronet_Buffer_GetClientContext, Cronet_Buffer_GetData,
    Cronet_Buffer_GetDataFunc, Cronet_Buffer_GetSize, Cronet_Buffer_GetSizeFunc,
    Cronet_Buffer_InitWithAlloc, Cronet_Buffer_InitWithAllocFunc,
    Cronet_Buffer_InitWithDataAndCallback, Cronet_Buffer_InitWithDataAndCallbackFunc,
    Cronet_Buffer_SetClientContext, Cronet_ClientContext, Cronet_RawDataPtr,
};

use super::Borrowed;

pub(crate) struct Buffer<Ctx> {
    ptr: Cronet_BufferPtr,
    _phan: PhantomData<Ctx>,
}

impl<Ctx> Buffer<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_BufferPtr {
        self.ptr
    }
}

impl<Ctx> Drop for Buffer<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_Buffer_Destroy(self.ptr) };
    }
}

impl<Ctx> Buffer<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Buffer_Create();
            Buffer {
                ptr,
                _phan: PhantomData,
            }
        }
    }

    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe { Cronet_Buffer_SetClientContext(self.ptr, ptr as Cronet_ClientContext) };
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_Buffer_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

    pub(crate) fn init_with_data_and_callback(
        &self,
        data: Box<[u8]>,
        callback: Cronet_BufferCallbackPtr,
    ) {
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

    pub(crate) fn init_with_alloc(&self, size: u64) {
        unsafe {
            Cronet_Buffer_InitWithAlloc(self.ptr, size);
        }
    }

    pub(crate) fn get_size(&self) -> u64 {
        unsafe { Cronet_Buffer_GetSize(self.ptr) }
    }

    pub(crate) fn get_data(&self) -> Cronet_RawDataPtr {
        unsafe { Cronet_Buffer_GetData(self.ptr) }
    }

    pub(crate) fn crate_with(
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
            Self {
                ptr,
                _phan: PhantomData,
            }
        }
    }
}
