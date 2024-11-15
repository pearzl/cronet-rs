use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
        Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
        Cronet_RequestFinishedInfoListener_GetClientContext,
        Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
        Cronet_RequestFinishedInfoListener_SetClientContext,
    },
    util::impl_client_context,
};

use super::Borrowed;

pub(crate) struct RequestFinishedInfoListener<Ctx> {
    ptr: Cronet_RequestFinishedInfoListenerPtr,
    _phan: PhantomData<Ctx>,
}

impl<'a, Ctx> RequestFinishedInfoListener<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
    }

    pub fn borrow_from<X>(
        ptr: Cronet_RequestFinishedInfoListenerPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, RequestFinishedInfoListener<Ctx>> {
        let borrowed = RequestFinishedInfoListener {
            ptr,
            _phan: PhantomData,
        };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> Drop for RequestFinishedInfoListener<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_RequestFinishedInfoListener_Destroy(self.ptr) }
    }
}

impl<Ctx> RequestFinishedInfoListener<Ctx> {
    pub(crate) fn create_with(
        on_request_finished_func: Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(on_request_finished_func);
            Self {
                ptr,
                _phan: PhantomData,
            }
        }
    }
}

impl_client_context! {
    RequestFinishedInfoListener,
    Cronet_RequestFinishedInfoListener_GetClientContext,
    Cronet_RequestFinishedInfoListener_SetClientContext,
}
