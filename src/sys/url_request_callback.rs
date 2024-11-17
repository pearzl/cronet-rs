use std::{ffi::CStr, marker::PhantomData};

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_UrlRequestCallbackPtr, Cronet_UrlRequestCallback_CreateWith,
        Cronet_UrlRequestCallback_Destroy, Cronet_UrlRequestCallback_GetClientContext,
        Cronet_UrlRequestCallback_OnCanceled, Cronet_UrlRequestCallback_OnCanceledFunc,
        Cronet_UrlRequestCallback_OnFailed, Cronet_UrlRequestCallback_OnFailedFunc,
        Cronet_UrlRequestCallback_OnReadCompleted, Cronet_UrlRequestCallback_OnReadCompletedFunc,
        Cronet_UrlRequestCallback_OnRedirectReceived,
        Cronet_UrlRequestCallback_OnRedirectReceivedFunc,
        Cronet_UrlRequestCallback_OnResponseStarted,
        Cronet_UrlRequestCallback_OnResponseStartedFunc, Cronet_UrlRequestCallback_OnSucceeded,
        Cronet_UrlRequestCallback_OnSucceededFunc, Cronet_UrlRequestCallback_SetClientContext,
    },
    util::define_impl,
};

use super::{Buffer, Error, UrlRequest, UrlResponseInfo};

impl<Ctx> UrlRequestCallback<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UrlRequestCallbackPtr {
        self.ptr
    }
}

impl<Ctx> UrlRequestCallback<Ctx> {
    pub(crate) fn create_with(
        on_redirect_received_func: Cronet_UrlRequestCallback_OnRedirectReceivedFunc,
        on_response_started_func: Cronet_UrlRequestCallback_OnResponseStartedFunc,
        on_read_completed_func: Cronet_UrlRequestCallback_OnReadCompletedFunc,
        on_succeeded_func: Cronet_UrlRequestCallback_OnSucceededFunc,
        on_failed_func: Cronet_UrlRequestCallback_OnFailedFunc,
        on_canceled_func: Cronet_UrlRequestCallback_OnCanceledFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestCallback_CreateWith(
                on_redirect_received_func,
                on_response_started_func,
                on_read_completed_func,
                on_succeeded_func,
                on_failed_func,
                on_canceled_func,
            );
            Self {
                ptr,
                ctx: None,
                _phan: PhantomData,
            }
        }
    }
}

define_impl! {
    UrlRequestCallback, Cronet_UrlRequestCallbackPtr, Cronet_UrlRequestCallback_Destroy,


    #[cfg(test)]
    fn on_redirect_received<T1>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr,
        new_location_url: &CStr >> CStr::as_ptr
    ); Cronet_UrlRequestCallback_OnRedirectReceived,

    #[cfg(test)]
    fn on_response_started<T1>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr
    ); Cronet_UrlRequestCallback_OnResponseStarted,

    #[cfg(test)]
    fn on_read_completed<T1, T2>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr,
        buffer: &Buffer<T2> >> Buffer::as_ptr,
        bytes_read: u64
    ); Cronet_UrlRequestCallback_OnReadCompleted,

    #[cfg(test)]
    fn on_succeeded<T1>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr
    ); Cronet_UrlRequestCallback_OnSucceeded,

    #[cfg(test)]
    fn on_failed<T1>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr,
        error: &Error >> Error::as_ptr
    ); Cronet_UrlRequestCallback_OnFailed,

    #[cfg(test)]
    fn on_canceled<T1>(&Self,
        request: &UrlRequest<T1> >> UrlRequest::as_ptr,
        info: &UrlResponseInfo >> UrlResponseInfo::as_ptr
    ); Cronet_UrlRequestCallback_OnCanceled,


    with_ctx: <Ctx>,
    get: Cronet_UrlRequestCallback_GetClientContext,
    set: Cronet_UrlRequestCallback_SetClientContext,
}
