use std::marker::PhantomData;

use crate::bindings::{
    Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
    Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
    Cronet_RequestFinishedInfoListener_GetClientContext,
    Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    Cronet_RequestFinishedInfoListener_SetClientContext,
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
    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe {
            Cronet_RequestFinishedInfoListener_SetClientContext(
                self.ptr,
                ptr as Cronet_ClientContext,
            )
        }
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_RequestFinishedInfoListener_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

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
