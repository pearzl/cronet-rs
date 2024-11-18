use std::{ffi::CStr, marker::PhantomData};

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_RESULT, Cronet_UrlRequestPtr, Cronet_UrlRequest_Cancel,
        Cronet_UrlRequest_CancelFunc, Cronet_UrlRequest_Create, Cronet_UrlRequest_CreateWith,
        Cronet_UrlRequest_Destroy, Cronet_UrlRequest_FollowRedirect,
        Cronet_UrlRequest_FollowRedirectFunc, Cronet_UrlRequest_GetClientContext,
        Cronet_UrlRequest_GetStatus, Cronet_UrlRequest_GetStatusFunc,
        Cronet_UrlRequest_InitWithParams, Cronet_UrlRequest_InitWithParamsFunc,
        Cronet_UrlRequest_IsDone, Cronet_UrlRequest_IsDoneFunc, Cronet_UrlRequest_Read,
        Cronet_UrlRequest_ReadFunc, Cronet_UrlRequest_SetClientContext, Cronet_UrlRequest_Start,
        Cronet_UrlRequest_StartFunc,
    },
    util::define_impl,
};

use super::{
    buffer::Buffer, engine::Engine, executor::Executor, url_request_callback::UrlRequestCallback,
    url_request_params::UrlRequestParams, url_request_status_listener::UrlRequestStatusListener,
};

impl<Ctx> UrlRequest<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequest_Create();
            Self {
                ptr,
                _ctx: PhantomData,
                
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn create_with(
        init_with_params_func: Cronet_UrlRequest_InitWithParamsFunc,
        start_func: Cronet_UrlRequest_StartFunc,
        follow_redirect_func: Cronet_UrlRequest_FollowRedirectFunc,
        read_func: Cronet_UrlRequest_ReadFunc,
        cancel_func: Cronet_UrlRequest_CancelFunc,
        is_done_func: Cronet_UrlRequest_IsDoneFunc,
        get_status_func: Cronet_UrlRequest_GetStatusFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequest_CreateWith(
                init_with_params_func,
                start_func,
                follow_redirect_func,
                read_func,
                cancel_func,
                is_done_func,
                get_status_func,
            );
            Self {
                ptr,
                _ctx: PhantomData,
                
            }
        }
    }
}

define_impl! {
    UrlRequest, Cronet_UrlRequestPtr, Cronet_UrlRequest_Destroy,

    /// params: cronet copy the content, it's fine to free the mem after calling.
    fn init_with_params<EngineCtx, UrlRequestCallbackCtx, ExecutorCtx>(
        &Self,
        engine: &Engine<EngineCtx> >> Engine::as_ptr,
        url: &CStr >> CStr::as_ptr,
        params: &UrlRequestParams >> UrlRequestParams::as_ptr, 
        callback: &UrlRequestCallback<UrlRequestCallbackCtx> >> UrlRequestCallback::as_ptr,   // safety: pass ref?
        executor: &Executor<ExecutorCtx> >> Executor::as_ptr      // safety: pass ref?
    ) -> Cronet_RESULT; Cronet_UrlRequest_InitWithParams,

    fn get_status<UrlRequestStatusListenerCtx>(
        &Self,
        listener: &UrlRequestStatusListener<UrlRequestStatusListenerCtx> >> UrlRequestStatusListener::as_ptr // safety: pass ref?
    ); Cronet_UrlRequest_GetStatus,

    fn start(&Self) -> Cronet_RESULT;
    Cronet_UrlRequest_Start,

    fn follow_redirect(&Self) -> Cronet_RESULT;
    Cronet_UrlRequest_FollowRedirect,

    fn read<BufferCtx>(&Self, buffer: &mut Buffer<BufferCtx> >> Buffer::as_ptr) -> Cronet_RESULT;
    Cronet_UrlRequest_Read,

    fn cancel(&Self);
    Cronet_UrlRequest_Cancel,

    fn is_done(&Self) -> bool;
    Cronet_UrlRequest_IsDone,


    with_ctx: <Ctx>,
    get: Cronet_UrlRequest_GetClientContext,
    set: Cronet_UrlRequest_SetClientContext,
}
