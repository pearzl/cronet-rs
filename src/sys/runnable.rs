use std::marker::PhantomData;

use crate::bindings::{
    Cronet_ClientContext, Cronet_RunnablePtr, Cronet_Runnable_CreateWith, Cronet_Runnable_Destroy,
    Cronet_Runnable_GetClientContext, Cronet_Runnable_RunFunc, Cronet_Runnable_SetClientContext,
};

use super::Borrowed;

pub(crate) struct Runnable<Ctx> {
    ptr: Cronet_RunnablePtr,
    _phan: PhantomData<Ctx>,
}

impl<Ctx> Drop for Runnable<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_Runnable_Destroy(self.ptr) }
    }
}

impl<Ctx> Runnable<Ctx> {
    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe { Cronet_Runnable_SetClientContext(self.ptr, ptr as Cronet_ClientContext) }
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_Runnable_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

    pub(crate) fn create_with(run_func: Cronet_Runnable_RunFunc) -> Self {
        unsafe {
            let ptr = Cronet_Runnable_CreateWith(run_func);
            Self {
                ptr,
                _phan: PhantomData,
            }
        }
    }
}
