use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_UrlRequestStatusListenerPtr,
        Cronet_UrlRequestStatusListener_CreateWith, Cronet_UrlRequestStatusListener_Destroy,
        Cronet_UrlRequestStatusListener_GetClientContext, Cronet_UrlRequestStatusListener_OnStatus,
        Cronet_UrlRequestStatusListener_OnStatusFunc,
        Cronet_UrlRequestStatusListener_SetClientContext, Cronet_UrlRequestStatusListener_Status,
    },
    util::define_impl,
};

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
                ctx: None,
                _phan: PhantomData,
            }
        }
    }
}

define_impl! {
    UrlRequestStatusListener, Cronet_UrlRequestStatusListenerPtr, Cronet_UrlRequestStatusListener_Destroy,

    #[cfg(test)]
    fn on_status(&Self, status: Cronet_UrlRequestStatusListener_Status); Cronet_UrlRequestStatusListener_OnStatus,

    with_ctx: <Ctx>,
    get: Cronet_UrlRequestStatusListener_GetClientContext,
    set: Cronet_UrlRequestStatusListener_SetClientContext,
}
