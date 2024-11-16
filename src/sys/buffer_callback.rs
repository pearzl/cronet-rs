use std::{marker::PhantomData, ops::Deref};

use crate::{
    bindings::{
        Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith, Cronet_BufferCallback_Destroy,
        Cronet_BufferCallback_GetClientContext, Cronet_BufferCallback_OnDestroyFunc,
        Cronet_BufferCallback_SetClientContext, Cronet_ClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<Ctx> Drop for BufferCallback<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_BufferCallback_Destroy(self.ptr) }
    }
}

impl<Ctx> BufferCallback<Ctx> {
    pub(crate) fn create_with(on_destroy_func: Cronet_BufferCallback_OnDestroyFunc) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(on_destroy_func);
            Self { ptr, ctx: None }
        }
    }
}

define_impl! {
    BufferCallback, Cronet_BufferCallbackPtr,
    with_ctx: Ctx,
    get:  Cronet_BufferCallback_GetClientContext,
    set:  Cronet_BufferCallback_SetClientContext,
}
