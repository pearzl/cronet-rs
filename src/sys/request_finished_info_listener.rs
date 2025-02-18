use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_ErrorPtr, Cronet_RequestFinishedInfoListenerPtr,
        Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
        Cronet_RequestFinishedInfoListener_GetClientContext,
        Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
        Cronet_RequestFinishedInfoListener_SetClientContext, Cronet_RequestFinishedInfoPtr,
        Cronet_UrlResponseInfoPtr,
    },
    util::define_impl,
};

use super::{Error, RequestFinishedInfo, UrlResponseInfo};

impl<Ctx> RequestFinishedInfoListener<Ctx>
where
    Ctx: RequestFinishedInfoListenerExt<Ctx>,
{
    pub(crate) fn create_with(_on_request_finished_func: OnRequestFinishedFunc<Ctx>) -> Self {
        unsafe {
            let ptr =
                Cronet_RequestFinishedInfoListener_CreateWith(Some(Self::raw_on_request_finished));
            Self {
                ptr,
                _ctx: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_on_request_finished(
        self_: Cronet_RequestFinishedInfoListenerPtr,
        request_info: Cronet_RequestFinishedInfoPtr,
        response_info: Cronet_UrlResponseInfoPtr,
        error: Cronet_ErrorPtr,
    ) {
        let self_ = RequestFinishedInfoListener::<Ctx>::from_ptr(self_);
        let request_info = RequestFinishedInfo::from_ptr(request_info);
        let response_info = UrlResponseInfo::from_ptr(response_info);
        let error = (!error.is_null()).then(|| Error::from_ptr(error) as _);

        let on_request_finished =
            <Ctx as RequestFinishedInfoListenerExt<Ctx>>::on_request_finished_func();
        on_request_finished(self_, request_info, response_info, error);
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(
            <Ctx as RequestFinishedInfoListenerExt<Ctx>>::on_request_finished_func(),
        );
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) type OnRequestFinishedFunc<Ctx> =
    fn(&RequestFinishedInfoListener<Ctx>, &RequestFinishedInfo, &UrlResponseInfo, Option<&Error>);

pub(crate) trait RequestFinishedInfoListenerExt<Ctx> {
    fn on_request_finished_func() -> OnRequestFinishedFunc<Ctx>;
}

define_impl! {
    RequestFinishedInfoListener, Cronet_RequestFinishedInfoListenerPtr, Cronet_RequestFinishedInfoListener_Destroy,
    with_ctx: <Ctx>,
    get: Cronet_RequestFinishedInfoListener_GetClientContext,
    set: Cronet_RequestFinishedInfoListener_SetClientContext,
}
