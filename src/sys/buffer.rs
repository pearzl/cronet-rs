use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_BufferCallbackPtr, Cronet_BufferPtr, Cronet_Buffer_Create, Cronet_Buffer_CreateWith,
        Cronet_Buffer_Destroy, Cronet_Buffer_GetClientContext, Cronet_Buffer_GetData,
        Cronet_Buffer_GetDataFunc, Cronet_Buffer_GetSize, Cronet_Buffer_GetSizeFunc,
        Cronet_Buffer_InitWithAlloc, Cronet_Buffer_InitWithAllocFunc,
        Cronet_Buffer_InitWithDataAndCallback, Cronet_Buffer_InitWithDataAndCallbackFunc,
        Cronet_Buffer_SetClientContext, Cronet_ClientContext, Cronet_RawDataPtr,
    },
    util::define_impl,
};

impl<Ctx> Buffer<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Buffer_Create();
            Buffer {
                ptr,
                _ctx: PhantomData,
                _phan: PhantomData,
            }
        }
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

    #[cfg(test)]
    pub(crate) fn create_with(
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
                _ctx: PhantomData,
                _phan: PhantomData,
            }
        }
    }
}

impl<Ctx> Buffer<Ctx> {
    /// return (bytes_filled, bytes_free)
    pub(crate) fn write(&mut self, bytes: &[u8]) -> (usize, u64){
        let buf_len = self.get_size();
        let buf = self.get_data() as *mut u8;
        let size_to_write = std::cmp::min(buf_len, bytes.len() as u64) as usize;
        unsafe {
            buf.copy_from_nonoverlapping(bytes.as_ptr(), size_to_write);
        }
        (size_to_write, buf_len - size_to_write as u64)
    }
}

define_impl! {
    Buffer, Cronet_BufferPtr, Cronet_Buffer_Destroy,

    fn get_size(&Self) -> u64; Cronet_Buffer_GetSize,
    fn init_with_alloc(&mut Self, size: u64); Cronet_Buffer_InitWithAlloc,
    fn get_data(&Self) -> Cronet_RawDataPtr; Cronet_Buffer_GetData,

    with_ctx: <Ctx>,
    get: Cronet_Buffer_GetClientContext,
    set: Cronet_Buffer_SetClientContext,
}


unsafe impl<Ctx> Send for Buffer<Ctx> {}
unsafe impl<Ctx> Sync for Buffer<Ctx> {}