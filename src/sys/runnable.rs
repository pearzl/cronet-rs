use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_RunnablePtr, Cronet_Runnable_CreateWith,
        Cronet_Runnable_Destroy, Cronet_Runnable_GetClientContext, Cronet_Runnable_RunFunc,
        Cronet_Runnable_SetClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<Ctx> Drop for Runnable<Ctx> {
    fn drop(&mut self) {
        unsafe { Cronet_Runnable_Destroy(self.ptr) }
    }
}

impl<Ctx> Runnable<Ctx> {
    pub(crate) fn create_with(run_func: Cronet_Runnable_RunFunc) -> Self {
        unsafe {
            let ptr = Cronet_Runnable_CreateWith(run_func);
            Self { ptr, ctx: None }
        }
    }
}

define_impl! {
    Runnable, Cronet_RunnablePtr,
    with_ctx: Ctx,
    get: Cronet_Runnable_GetClientContext,
    set: Cronet_Runnable_SetClientContext,
}
