use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_UploadDataProviderPtr, Cronet_UploadDataProvider_CloseFunc,
        Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy,
        Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLengthFunc,
        Cronet_UploadDataProvider_ReadFunc, Cronet_UploadDataProvider_RewindFunc,
        Cronet_UploadDataProvider_SetClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<'a, Ctx> UploadDataProvider<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UploadDataProviderPtr {
        self.ptr
    }

    pub fn borrow_from<X>(
        ptr: Cronet_UploadDataProviderPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, UploadDataProvider<Ctx>> {
        let borrowed = UploadDataProvider { ptr, ctx: None };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> UploadDataProvider<Ctx> {
    pub(crate) fn create_with(
        get_length_func: Cronet_UploadDataProvider_GetLengthFunc,
        read_func: Cronet_UploadDataProvider_ReadFunc,
        rewind_func: Cronet_UploadDataProvider_RewindFunc,
        close_func: Cronet_UploadDataProvider_CloseFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataProvider_CreateWith(
                get_length_func,
                read_func,
                rewind_func,
                close_func,
            );
            Self { ptr, ctx: None }
        }
    }
}

define_impl! {
    UploadDataProvider, Cronet_UploadDataProviderPtr,Cronet_UploadDataProvider_Destroy,
    with_ctx: Ctx,
    get: Cronet_UploadDataProvider_GetClientContext,
    set: Cronet_UploadDataProvider_SetClientContext,
}

unsafe impl<T> Send for UploadDataProvider<T> where T: Send {}
unsafe impl<T> Sync for UploadDataProvider<T> where T: Sync {}
