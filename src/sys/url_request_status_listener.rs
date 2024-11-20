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

impl<Ctx> UrlRequestStatusListener<Ctx>
where
    Ctx: UrlRequestStatusListenerExt<Ctx>,
{
    pub(crate) fn create_with(_on_status_func: OnStatusFunc<Ctx>) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestStatusListener_CreateWith(Some(Self::raw_on_status_func));
            Self {
                ptr,
                _ctx: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_on_status_func(
        self_: Cronet_UrlRequestStatusListenerPtr,
        status: Cronet_UrlRequestStatusListener_Status,
    ) {
        let self_ = UrlRequestStatusListener::<Ctx>::from_ptr(self_);
        let on_status = <Ctx as UrlRequestStatusListenerExt<Ctx>>::on_status_func();
        on_status(&self_, status)
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ =
            Self::create_with(<Ctx as UrlRequestStatusListenerExt<Ctx>>::on_status_func());
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) type OnStatusFunc<Ctx> =
    fn(self_: &UrlRequestStatusListener<Ctx>, status: Cronet_UrlRequestStatusListener_Status);

pub(crate) trait UrlRequestStatusListenerExt<Ctx> {
    fn on_status_func() -> OnStatusFunc<Ctx>;
}

define_impl! {
    UrlRequestStatusListener, Cronet_UrlRequestStatusListenerPtr, Cronet_UrlRequestStatusListener_Destroy,

    #[cfg(test)]
    fn on_status(&Self, status: Cronet_UrlRequestStatusListener_Status); Cronet_UrlRequestStatusListener_OnStatus,

    with_ctx: <Ctx>,
    get: Cronet_UrlRequestStatusListener_GetClientContext,
    set: Cronet_UrlRequestStatusListener_SetClientContext,
}
