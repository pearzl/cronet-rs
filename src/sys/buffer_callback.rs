use std::{marker::PhantomData, ops::Deref};

use crate::bindings::{
    Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith, Cronet_BufferCallback_Destroy,
    Cronet_BufferCallback_GetClientContext, Cronet_BufferCallback_OnDestroyFunc,
    Cronet_BufferCallback_SetClientContext, Cronet_ClientContext,
};

use super::Borrowed;

pub(crate) struct BufferCallback<Ctx> {
    ptr: Cronet_BufferCallbackPtr,
    _phan: PhantomData<Ctx>
}

impl<Ctx> Drop for BufferCallback<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr= self.get_client_context().inner;
        let ctx = unsafe{Box::from_raw(ctx_ptr)};
        drop(ctx);
        unsafe{Cronet_BufferCallback_Destroy(self.ptr) }
    }
}

impl<Ctx> BufferCallback<Ctx> {
    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_BufferCallback_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe { Cronet_BufferCallback_SetClientContext(self.ptr, ptr as Cronet_ClientContext) }
    }

    pub(crate) fn create_with(on_destroy_func: Cronet_BufferCallback_OnDestroyFunc) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(on_destroy_func);
            Self { ptr, _phan: PhantomData }
        }
    }
}
