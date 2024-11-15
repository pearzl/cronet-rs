use std::marker::PhantomData;

use crate::bindings::{
    Cronet_ClientContext, Cronet_UploadDataProviderPtr, Cronet_UploadDataProvider_CloseFunc,
    Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy,
    Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLengthFunc,
    Cronet_UploadDataProvider_ReadFunc, Cronet_UploadDataProvider_RewindFunc,
    Cronet_UploadDataProvider_SetClientContext,
};

use super::Borrowed;

pub(crate) struct UploadDataProvider<Ctx> {
    ptr: Cronet_UploadDataProviderPtr,
    _phan: PhantomData<Ctx>,
}

impl<'a, Ctx> UploadDataProvider<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UploadDataProviderPtr {
        self.ptr
    }

    pub fn borrow_from<X>(
        ptr: Cronet_UploadDataProviderPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, UploadDataProvider<Ctx>> {
        let borrowed = UploadDataProvider {
            ptr,
            _phan: PhantomData,
        };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> Drop for UploadDataProvider<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_UploadDataProvider_Destroy(self.ptr) }
    }
}

impl<Ctx> UploadDataProvider<Ctx> {
    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe { Cronet_UploadDataProvider_SetClientContext(self.ptr, ptr as Cronet_ClientContext) }
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_UploadDataProvider_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

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
            Self {
                ptr,
                _phan: PhantomData,
            }
        }
    }
}
