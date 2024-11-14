use std::{mem::ManuallyDrop, ops::Deref};

use crate::bindings::{
    Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
    Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
    Cronet_RequestFinishedInfoListener_GetClientContext,
    Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    Cronet_RequestFinishedInfoListener_SetClientContext,
};

pub(crate) struct RequestFinishedInfoListener {
    ptr: Cronet_RequestFinishedInfoListenerPtr,
}

impl RequestFinishedInfoListener {
    pub(crate) fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
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

pub(crate) struct BorrowedRequestFinishedInfoListener {
    inner: ManuallyDrop<RequestFinishedInfoListener>,
}

impl BorrowedRequestFinishedInfoListener {
    pub(crate) fn from_ptr(ptr: Cronet_RequestFinishedInfoListenerPtr) -> Self {
        let value = RequestFinishedInfoListener { ptr };
        BorrowedRequestFinishedInfoListener {
            inner: ManuallyDrop::new(value),
        }
    }
}

impl Deref for BorrowedRequestFinishedInfoListener {
    type Target = RequestFinishedInfoListener;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
