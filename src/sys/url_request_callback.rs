use std::{ffi::CStr, marker::PhantomData};

use crate::{
    bindings::{
        Cronet_BufferPtr, Cronet_ClientContext, Cronet_ErrorPtr, Cronet_String,
        Cronet_UrlRequestCallbackPtr, Cronet_UrlRequestCallback_CreateWith,
        Cronet_UrlRequestCallback_Destroy, Cronet_UrlRequestCallback_GetClientContext,
        Cronet_UrlRequestCallback_OnCanceled, Cronet_UrlRequestCallback_OnCanceledFunc,
        Cronet_UrlRequestCallback_OnFailed, Cronet_UrlRequestCallback_OnFailedFunc,
        Cronet_UrlRequestCallback_OnReadCompleted, Cronet_UrlRequestCallback_OnReadCompletedFunc,
        Cronet_UrlRequestCallback_OnRedirectReceived,
        Cronet_UrlRequestCallback_OnRedirectReceivedFunc,
        Cronet_UrlRequestCallback_OnResponseStarted,
        Cronet_UrlRequestCallback_OnResponseStartedFunc, Cronet_UrlRequestCallback_OnSucceeded,
        Cronet_UrlRequestCallback_OnSucceededFunc, Cronet_UrlRequestCallback_SetClientContext,
        Cronet_UrlRequestPtr, Cronet_UrlResponseInfoPtr,
    },
    util::define_impl,
};

use super::{Buffer, Error, UrlRequest, UrlResponseInfo};

impl<Ctx> UrlRequestCallback<Ctx>
where
    Ctx: UrlRequestCallbackExt<Ctx>,
{
    pub(crate) fn create_with(
        _on_redirect_received_func: OnRedirectReceivedFunc<
            Ctx,
            <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx,
        >,
        _on_response_started_func: OnResponseStartedFunc<
            Ctx,
            <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx,
        >,
        _on_read_completed_func: OnReadCompletedFunc<
            Ctx,
            <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx,
            <Ctx as UrlRequestCallbackExt<Ctx>>::BufferCtx,
        >,
        _on_succeeded_func: OnSucceededFunc<
            Ctx,
            <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx,
        >,
        _on_failed_func: OnFailedFunc<Ctx, <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>,
        _on_canceled_func: OnCanceledFunc<Ctx, <Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestCallback_CreateWith(
                Some(Self::raw_on_redirect_received_func),
                Some(Self::raw_on_response_started_func),
                Some(Self::raw_on_read_completed_func),
                Some(Self::raw_on_succeeded_func),
                Some(Self::raw_on_failed_func),
                Some(Self::raw_on_canceled_func),
            );
            Self {
                ptr,
                _ctx: PhantomData,
                
            }
        }
    }

    unsafe extern "C" fn raw_on_redirect_received_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
        new_location_url: Cronet_String,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = UrlResponseInfo::from_ptr(info);
        let new_localtion_url = CStr::from_ptr(new_location_url);

        let on_redirect_received = <Ctx as UrlRequestCallbackExt<Ctx>>::on_redirect_received_func();
        on_redirect_received(&self_, request, info, new_localtion_url)
    }

    unsafe extern "C" fn raw_on_response_started_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = UrlResponseInfo::from_ptr(info);

        let on_response_started = <Ctx as UrlRequestCallbackExt<Ctx>>::on_response_started_func();
        on_response_started(self_, request, info)
    }

    unsafe extern "C" fn raw_on_read_completed_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
        buffer: Cronet_BufferPtr,
        bytes_read: u64,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = UrlResponseInfo::from_ptr(info);
        let buffer = Buffer::from_raw(buffer);

        let on_read_completed = <Ctx as UrlRequestCallbackExt<Ctx>>::on_read_completed_func();
        on_read_completed(&self_, request, info, buffer, bytes_read)
    }

    unsafe extern "C" fn raw_on_succeeded_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = UrlResponseInfo::from_ptr(info);

        let on_succeeded = <Ctx as UrlRequestCallbackExt<Ctx>>::on_succeeded_func();
        on_succeeded(&self_, request, info)
    }

    unsafe extern "C" fn raw_on_failed_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
        error: Cronet_ErrorPtr,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = (!info.is_null()).then(||UrlResponseInfo::from_ptr(info) as &_);
        let error = Error::from_ptr(error);

        let on_failed = <Ctx as UrlRequestCallbackExt<Ctx>>::on_failed_func();
        on_failed(&self_, request, info, error)
    }

    unsafe extern "C" fn raw_on_canceled_func(
        self_: Cronet_UrlRequestCallbackPtr,
        request: Cronet_UrlRequestPtr,
        info: Cronet_UrlResponseInfoPtr,
    ) {
        let self_ = UrlRequestCallback::<Ctx>::from_ptr(self_);
        let request =
            UrlRequest::<<Ctx as UrlRequestCallbackExt<Ctx>>::UrlRequestCtx>::from_ptr(request);
        let info = (!info.is_null()).then(||UrlResponseInfo::from_ptr(info) as &_);

        let on_canceled = <Ctx as UrlRequestCallbackExt<Ctx>>::on_canceled_func();
        on_canceled(&self_, request, info)
    }

    pub fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_redirect_received_func(),
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_response_started_func(),
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_read_completed_func(),
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_succeeded_func(),
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_failed_func(),
            <Ctx as UrlRequestCallbackExt<Ctx>>::on_canceled_func(),
        );
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) trait UrlRequestCallbackExt<Ctx> {
    type UrlRequestCtx;
    type BufferCtx;
    fn on_redirect_received_func() -> OnRedirectReceivedFunc<Ctx, Self::UrlRequestCtx>;
    fn on_response_started_func() -> OnResponseStartedFunc<Ctx, Self::UrlRequestCtx>;
    fn on_read_completed_func() -> OnReadCompletedFunc<Ctx, Self::UrlRequestCtx, Self::BufferCtx>;
    fn on_succeeded_func() -> OnSucceededFunc<Ctx, Self::UrlRequestCtx>;
    fn on_failed_func() -> OnFailedFunc<Ctx, Self::UrlRequestCtx>;
    fn on_canceled_func() -> OnCanceledFunc<Ctx, Self::UrlRequestCtx>;
}

pub(crate) type OnRedirectReceivedFunc<Ctx, UrlRequestCtx> = fn(
    self_: &UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: &UrlResponseInfo,
    new_location_url: &CStr,
);
pub(crate) type OnResponseStartedFunc<Ctx, UrlRequestCtx> = fn(
    self_: &mut UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: &UrlResponseInfo,
);
pub(crate) type OnReadCompletedFunc<Ctx, UrlRequestCtx, BufferCtx> = fn(
    self_: &mut UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: &UrlResponseInfo,
    buffer: Buffer<BufferCtx>,
    bytes_read: u64,
);
pub(crate) type OnSucceededFunc<Ctx, UrlRequestCtx> = fn(
    self_: &UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: &UrlResponseInfo,
);
pub(crate) type OnFailedFunc<Ctx, UrlRequestCtx> = fn(
    self_: &UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: Option<&UrlResponseInfo>,
    error: &Error,
);
pub(crate) type OnCanceledFunc<Ctx, UrlRequestCtx> = fn(
    self_: &UrlRequestCallback<Ctx>,
    request: &UrlRequest<UrlRequestCtx>,
    info: Option<&UrlResponseInfo>,
);

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
