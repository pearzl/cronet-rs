use crate::bindings::{
    Cronet_ClientContext, Cronet_UrlRequestCallbackPtr, Cronet_UrlRequestCallback_CreateWith,
    Cronet_UrlRequestCallback_Destroy, Cronet_UrlRequestCallback_GetClientContext,
    Cronet_UrlRequestCallback_OnCanceledFunc, Cronet_UrlRequestCallback_OnFailedFunc,
    Cronet_UrlRequestCallback_OnReadCompletedFunc,
    Cronet_UrlRequestCallback_OnRedirectReceivedFunc,
    Cronet_UrlRequestCallback_OnResponseStartedFunc, Cronet_UrlRequestCallback_OnSucceededFunc,
    Cronet_UrlRequestCallback_SetClientContext,
};

pub struct UrlRequestCallback {
    ptr: Cronet_UrlRequestCallbackPtr,
}

impl UrlRequestCallback {
    pub fn as_ptr(&self) -> Cronet_UrlRequestCallbackPtr {
        self.ptr
    }
}

impl Drop for UrlRequestCallback {
    fn drop(&mut self) {
        unsafe { Cronet_UrlRequestCallback_Destroy(self.ptr) }
    }
}

impl UrlRequestCallback {
    pub fn set_client_conetxt(&mut self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_UrlRequestCallback_SetClientContext(self.ptr, client_context) }
    }

    pub fn get_client_conetxt(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UrlRequestCallback_GetClientContext(self.ptr) }
    }

    pub fn create_with(
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
            Self { ptr }
        }
    }
}
