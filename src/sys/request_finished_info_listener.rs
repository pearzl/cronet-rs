use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_ErrorPtr, Cronet_RequestFinishedInfoListenerPtr, Cronet_RequestFinishedInfoListener_CreateWith, Cronet_RequestFinishedInfoListener_Destroy, Cronet_RequestFinishedInfoListener_GetClientContext, Cronet_RequestFinishedInfoListener_OnRequestFinishedFunc, Cronet_RequestFinishedInfoListener_SetClientContext, Cronet_RequestFinishedInfoPtr, Cronet_UrlResponseInfoPtr
    },
    util::define_impl,
};

use super::{Borrowed, Error, RequestFinishedInfo, UrlResponseInfo};

impl<'a, Ctx> RequestFinishedInfoListener<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_RequestFinishedInfoListenerPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_RequestFinishedInfoListenerPtr) -> &'a mut RequestFinishedInfoListener<Ctx> {
        let self_ = RequestFinishedInfoListener {ptr, ctx: None::<Ctx> /* fake field */};
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
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

impl<Ctx> RequestFinishedInfoListener<Ctx> 
    where Ctx: OnRequestFinishedFuncContainer<Ctx>
{
    pub(crate) fn create_with(
        on_request_finished_func: OnRequestFinishedFunc<Ctx>,
    ) -> Self {
        let _ = on_request_finished_func;
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(Some(Self::raw_on_request_finished));
            Self { ptr, ctx: None }
        }
    }

    unsafe extern "C" fn raw_on_request_finished(
        self_: Cronet_RequestFinishedInfoListenerPtr,
        request_info: Cronet_RequestFinishedInfoPtr,
        response_info: Cronet_UrlResponseInfoPtr,
        error: Cronet_ErrorPtr,
    ) {
        let self_ = RequestFinishedInfoListener::<Ctx>::borrow_from_ptr(self_);
        let request_info = RequestFinishedInfo::borrow_from_ptr(request_info);
        let response_info =  UrlResponseInfo::borrow_from_ptr(response_info);
        let error= Error::borrow_from_ptr(error);

        let ctx = self_.get_client_context();
        let on_request_finished = ctx.get();
        on_request_finished(self_, request_info, response_info, error);
    }
}

pub(crate) type OnRequestFinishedFunc<Ctx> = fn(&RequestFinishedInfoListener<Ctx>, &RequestFinishedInfo, &UrlResponseInfo, &Error);

pub(crate) trait OnRequestFinishedFuncContainer<Ctx> {
    fn get(&self) -> OnRequestFinishedFunc<Ctx>;
    fn set(&self, on_request_finished: OnRequestFinishedFunc<Ctx>);
}



define_impl! {
    RequestFinishedInfoListener, Cronet_RequestFinishedInfoListenerPtr, Cronet_RequestFinishedInfoListener_Destroy,
    with_ctx: Ctx,
    get: Cronet_RequestFinishedInfoListener_GetClientContext,
    set: Cronet_RequestFinishedInfoListener_SetClientContext,
}
