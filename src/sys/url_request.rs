use std::ffi::CStr;

use crate::bindings::{
    Cronet_ClientContext, Cronet_RESULT, Cronet_UrlRequestPtr, Cronet_UrlRequest_Cancel,
    Cronet_UrlRequest_CancelFunc, Cronet_UrlRequest_Create, Cronet_UrlRequest_CreateWith,
    Cronet_UrlRequest_Destroy, Cronet_UrlRequest_FollowRedirect,
    Cronet_UrlRequest_FollowRedirectFunc, Cronet_UrlRequest_GetClientContext,
    Cronet_UrlRequest_GetStatus, Cronet_UrlRequest_GetStatusFunc, Cronet_UrlRequest_InitWithParams,
    Cronet_UrlRequest_InitWithParamsFunc, Cronet_UrlRequest_IsDone, Cronet_UrlRequest_IsDoneFunc,
    Cronet_UrlRequest_Read, Cronet_UrlRequest_ReadFunc, Cronet_UrlRequest_SetClientContext,
    Cronet_UrlRequest_Start, Cronet_UrlRequest_StartFunc,
};

use super::{
    buffer::Buffer, engine::Engine, executor::Executor, url_request_callback::UrlRequestCallback,
    url_request_params::UrlRequestParams, url_request_status_listener::UrlRequestStatusListener,
};

pub struct UrlRequest {
    ptr: Cronet_UrlRequestPtr,
}

impl Drop for UrlRequest {
    fn drop(&mut self) {
        unsafe { Cronet_UrlRequest_Destroy(self.ptr) }
    }
}

impl UrlRequest {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequest_Create();
            Self { ptr }
        }
    }

    pub fn set_client_conetxt(&self, client_conetxt: Cronet_ClientContext) {
        unsafe { Cronet_UrlRequest_SetClientContext(self.ptr, client_conetxt) }
    }

    pub fn get_client_conetxt(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UrlRequest_GetClientContext(self.ptr) }
    }

    pub fn init_with_params(
        &self,
        engine: &Engine,
        url: &CStr,
        params: UrlRequestParams,
        callback: UrlRequestCallback,
        executor: Executor,
    ) -> Cronet_RESULT {
        unsafe {
            Cronet_UrlRequest_InitWithParams(
                self.ptr,
                engine.as_ptr(),
                url.as_ptr(),
                params.as_ptr(),
                callback.as_ptr(),
                executor.as_ptr(),
            )
        }
    }

    pub fn start(&self) -> Cronet_RESULT {
        unsafe { Cronet_UrlRequest_Start(self.ptr) }
    }

    pub fn follow_redirect(&self) -> Cronet_RESULT {
        unsafe { Cronet_UrlRequest_FollowRedirect(self.ptr) }
    }

    pub fn read(&self, buffer: &mut Buffer) -> Cronet_RESULT {
        unsafe { Cronet_UrlRequest_Read(self.ptr, buffer.as_ptr()) }
    }

    pub fn cancel(&self) {
        unsafe { Cronet_UrlRequest_Cancel(self.ptr) }
    }

    pub fn is_done(&self) -> bool {
        unsafe { Cronet_UrlRequest_IsDone(self.ptr) }
    }

    pub fn get_status(&self, listener: &UrlRequestStatusListener) {
        unsafe { Cronet_UrlRequest_GetStatus(self.ptr, listener.as_ptr()) }
    }

    pub fn create_with(
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
            Self { ptr }
        }
    }
}
