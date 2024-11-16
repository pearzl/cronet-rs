use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_RequestFinishedInfoListenerPtr,
        Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy,
        Cronet_RequestFinishedInfoListener_GetClientContext,
        Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
        Cronet_RequestFinishedInfoListener_SetClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<'a, Ctx> RequestFinishedInfoListener<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
    }

    pub fn borrow_from<X>(
        ptr: Cronet_RequestFinishedInfoListenerPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, RequestFinishedInfoListener<Ctx>> {
        let borrowed = RequestFinishedInfoListener { ptr, ctx: None };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> RequestFinishedInfoListener<Ctx> {
    pub(crate) fn create_with(
        on_request_finished_func: Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(on_request_finished_func);
            Self { ptr, ctx: None }
        }
    }
}

define_impl! { 
    RequestFinishedInfoListener, Cronet_RequestFinishedInfoListenerPtr, Cronet_RequestFinishedInfoListener_Destroy,
    with_ctx: Ctx,
    get: Cronet_RequestFinishedInfoListener_GetClientContext,
    set: Cronet_RequestFinishedInfoListener_SetClientContext,
}
