use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_UrlRequestStatusListenerPtr,
        Cronet_UrlRequestStatusListener_CreateWith, Cronet_UrlRequestStatusListener_Destroy,
        Cronet_UrlRequestStatusListener_GetClientContext,
        Cronet_UrlRequestStatusListener_OnStatusFunc,
        Cronet_UrlRequestStatusListener_SetClientContext,
    },
    util::impl_client_context,
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

impl_client_context! {
    UrlRequestStatusListener,
    Cronet_UrlRequestStatusListener_GetClientContext,
    Cronet_UrlRequestStatusListener_SetClientContext,
}
