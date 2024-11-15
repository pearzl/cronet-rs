use crate::bindings::{
    Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
    Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
    Cronet_RequestFinishedInfoListener_GetClientContext,
    Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    Cronet_RequestFinishedInfoListener_SetClientContext,
};

use super::Borrowed;

pub(crate) struct RequestFinishedInfoListener {
    ptr: Cronet_RequestFinishedInfoListenerPtr,
}

impl<'a> RequestFinishedInfoListener {
    pub(crate) fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
    }

    pub fn borrow_from<X>(
        ptr: Cronet_RequestFinishedInfoListenerPtr,
        lifetime: &'a X
    ) -> Borrowed<'a, RequestFinishedInfoListener> {
        let borrowed = RequestFinishedInfoListener { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl Drop for RequestFinishedInfoListener {
    fn drop(&mut self) {
        unsafe { Cronet_RequestFinishedInfoListener_Destroy(self.ptr) }
    }
}

impl RequestFinishedInfoListener {
    pub(crate) fn set_client_context(&mut self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_RequestFinishedInfoListener_SetClientContext(self.ptr, client_context) }
    }

    pub(crate) fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_RequestFinishedInfoListener_GetClientContext(self.ptr) }
    }

    pub(crate) fn create_with(
        on_request_finished_func: Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(on_request_finished_func);
            Self { ptr }
        }
    }
}
