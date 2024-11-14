use std::{mem::ManuallyDrop, ops::Deref};

use crate::bindings::{
    Cronet_ClientContext, Cronet_UploadDataProviderPtr, Cronet_UploadDataProvider_CloseFunc,
    Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy,
    Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLengthFunc,
    Cronet_UploadDataProvider_ReadFunc, Cronet_UploadDataProvider_RewindFunc,
    Cronet_UploadDataProvider_SetClientContext,
};

pub struct UploadDataProvider {
    ptr: Cronet_UploadDataProviderPtr,
}

impl UploadDataProvider {
    pub fn as_ptr(&self) -> Cronet_UploadDataProviderPtr {
        self.ptr
    }
}

impl Drop for UploadDataProvider {
    fn drop(&mut self) {
        unsafe { Cronet_UploadDataProvider_Destroy(self.ptr) }
    }
}

impl UploadDataProvider {
    pub fn set_client_context(&mut self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_UploadDataProvider_SetClientContext(self.ptr, client_context) }
    }

    pub fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UploadDataProvider_GetClientContext(self.ptr) }
    }

    pub fn create_with(
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
            Self { ptr }
        }
    }
}

pub struct BorrowedUploadDataProvider {
    inner: ManuallyDrop<UploadDataProvider>,
}

impl BorrowedUploadDataProvider {
    pub fn from_ptr(ptr: Cronet_UploadDataProviderPtr) -> Self {
        let value = UploadDataProvider { ptr };
        BorrowedUploadDataProvider {
            inner: ManuallyDrop::new(value),
        }
    }
}

impl Deref for BorrowedUploadDataProvider {
    type Target = UploadDataProvider;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
