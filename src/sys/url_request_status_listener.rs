use crate::bindings::{
    Cronet_ClientContext, Cronet_UrlRequestStatusListenerPtr,
    Cronet_UrlRequestStatusListener_CreateWith, Cronet_UrlRequestStatusListener_Destroy,
    Cronet_UrlRequestStatusListener_GetClientContext, Cronet_UrlRequestStatusListener_OnStatusFunc,
    Cronet_UrlRequestStatusListener_SetClientContext,
};

pub(crate) struct UrlRequestStatusListener {
    ptr: Cronet_UrlRequestStatusListenerPtr,
}

impl UrlRequestStatusListener {
    pub(crate) fn as_ptr(&self) -> Cronet_UrlRequestStatusListenerPtr {
        self.ptr
    }
}

impl UrlRequestStatusListener {
    pub(crate) fn set_client_context(&mut self, client_context: Cronet_ClientContext) {
        unsafe {
            Cronet_UrlRequestStatusListener_SetClientContext(self.ptr, client_context);
        }
    }

    pub(crate) fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UrlRequestStatusListener_GetClientContext(self.ptr) }
    }

    pub(crate) fn create_with(
        &self,
        on_status_func: Cronet_UrlRequestStatusListener_OnStatusFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestStatusListener_CreateWith(on_status_func);
            Self { ptr }
        }
    }
}

impl Drop for UrlRequestStatusListener {
    fn drop(&mut self) {
        unsafe { Cronet_UrlRequestStatusListener_Destroy(self.ptr) }
    }
}
