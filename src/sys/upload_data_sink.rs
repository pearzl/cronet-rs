use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_UploadDataSinkPtr, Cronet_UploadDataSink_Create,
        Cronet_UploadDataSink_CreateWith, Cronet_UploadDataSink_Destroy,
        Cronet_UploadDataSink_GetClientContext, Cronet_UploadDataSink_OnReadErrorFunc,
        Cronet_UploadDataSink_OnReadSucceededFunc, Cronet_UploadDataSink_OnRewindErrorFunc,
        Cronet_UploadDataSink_OnRewindSucceededFunc, Cronet_UploadDataSink_SetClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<'a, Ctx> UploadDataSink<Ctx> {
    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_UploadDataSinkPtr) -> &'a mut UploadDataSink<Ctx> {
        let self_ = UploadDataSink {ptr, ctx: None::<Ctx> /* fake field */, _phan: PhantomData};
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }

    pub fn borrow_from<X>(
        ptr: Cronet_UploadDataSinkPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, UploadDataSink<Ctx>> {
        let borrowed = UploadDataSink { ptr, ctx: None, _phan: PhantomData };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> UploadDataSink<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UploadDataSink_Create();
            Self { ptr, ctx: None, _phan: PhantomData }
        }
    }

    pub(crate) fn create_with(
        on_read_succeeded_func: Cronet_UploadDataSink_OnReadSucceededFunc,
        on_read_error_func: Cronet_UploadDataSink_OnReadErrorFunc,
        on_rewind_succeeded_func: Cronet_UploadDataSink_OnRewindSucceededFunc,
        on_rewind_error_func: Cronet_UploadDataSink_OnRewindErrorFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataSink_CreateWith(
                on_read_succeeded_func,
                on_read_error_func,
                on_rewind_succeeded_func,
                on_rewind_error_func,
            );
            Self { ptr, ctx: None, _phan: PhantomData }
        }
    }
}

unsafe impl<Ctx> Send for UploadDataSink<Ctx> {}
unsafe impl<Ctx> Sync for UploadDataSink<Ctx> {}

define_impl! {
    UploadDataSink, Cronet_UploadDataSinkPtr, Cronet_UploadDataSink_Destroy,
    with_ctx: <Ctx>,
    get: Cronet_UploadDataSink_GetClientContext,
    set: Cronet_UploadDataSink_SetClientContext,
}
