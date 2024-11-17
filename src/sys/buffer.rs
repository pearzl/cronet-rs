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

use super::Borrowed;

impl<Ctx> Buffer<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_BufferPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr<'a>(ptr: Cronet_BufferPtr) -> &'a mut Buffer<Ctx> {
        let self_ = Buffer {ptr, ctx: None::<Ctx> /* fake field */, _phan: PhantomData};
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }
}

impl<Ctx> Buffer<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Buffer_Create();
            Buffer { ptr, ctx: None, _phan: PhantomData }
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
            Self { ptr, ctx: None, _phan: PhantomData }
        }
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
