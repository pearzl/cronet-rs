use std::marker::PhantomData;

use crate::bindings::{
    Cronet_ClientContext, Cronet_UrlRequestStatusListenerPtr,
    Cronet_UrlRequestStatusListener_CreateWith, Cronet_UrlRequestStatusListener_Destroy,
    Cronet_UrlRequestStatusListener_GetClientContext, Cronet_UrlRequestStatusListener_OnStatusFunc,
    Cronet_UrlRequestStatusListener_SetClientContext,
};

use super::Borrowed;

pub(crate) struct UrlRequestStatusListener<Ctx> {
    ptr: Cronet_UrlRequestStatusListenerPtr,
    _phan: PhantomData<Ctx>,
}

impl<Ctx> UrlRequestStatusListener<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UrlRequestStatusListenerPtr {
        self.ptr
    }
}

impl<Ctx> UrlRequestStatusListener<Ctx> {
    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe {
            Cronet_UrlRequestStatusListener_SetClientContext(self.ptr, ptr as Cronet_ClientContext);
        }
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_UrlRequestStatusListener_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

    pub(crate) fn create_with(
        &self,
        on_status_func: Cronet_UrlRequestStatusListener_OnStatusFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestStatusListener_CreateWith(on_status_func);
            Self {
                ptr,
                _phan: PhantomData,
            }
        }
    }
}

impl<Ctx> Drop for UrlRequestStatusListener<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_UrlRequestStatusListener_Destroy(self.ptr) }
    }
}
