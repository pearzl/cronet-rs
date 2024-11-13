use crate::bindings::{
    Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
    Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
    Cronet_RequestFinishedInfoListener_GetClientContext,
    Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    Cronet_RequestFinishedInfoListener_SetClientContext,
};

pub struct RequestFinishedInfoListener {
    ptr: Cronet_RequestFinishedInfoListenerPtr,
    is_owned_ptr: bool,
}

impl RequestFinishedInfoListener {
    pub fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
    }

    pub fn from_borrowed_ptr(ptr: Cronet_RequestFinishedInfoListenerPtr) -> Self {
        RequestFinishedInfoListener {ptr, is_owned_ptr: false}
    }
}



impl Drop for RequestFinishedInfoListener {
    fn drop(&mut self) {
        if self.is_owned_ptr{

            unsafe { Cronet_RequestFinishedInfoListener_Destroy(self.ptr) }
        }
    }
}

impl RequestFinishedInfoListener {
    pub fn set_client_context(&self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_RequestFinishedInfoListener_SetClientContext(self.ptr, client_context) }
    }

    pub fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_RequestFinishedInfoListener_GetClientContext(self.ptr) }
    }

    pub fn create_with(
        on_request_finished_func: Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(on_request_finished_func);
            Self { ptr, is_owned_ptr: true }
        }
    }
}
