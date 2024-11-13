use crate::bindings::{
    Cronet_ClientContext, Cronet_UploadDataSinkPtr, Cronet_UploadDataSink_Create,
    Cronet_UploadDataSink_CreateWith, Cronet_UploadDataSink_Destroy,
    Cronet_UploadDataSink_GetClientContext, Cronet_UploadDataSink_OnReadErrorFunc,
    Cronet_UploadDataSink_OnReadSucceededFunc, Cronet_UploadDataSink_OnRewindErrorFunc,
    Cronet_UploadDataSink_OnRewindSucceededFunc, Cronet_UploadDataSink_SetClientContext,
};

pub struct UploadDataSink {
    ptr: Cronet_UploadDataSinkPtr,
}

impl Drop for UploadDataSink {
    fn drop(&mut self) {
        unsafe { Cronet_UploadDataSink_Destroy(self.ptr) }
    }
}

impl UploadDataSink {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_UploadDataSink_Create();
            Self { ptr }
        }
    }

    pub fn set_client_conetxt(&mut self, client_conetxt: Cronet_ClientContext) {
        unsafe { Cronet_UploadDataSink_SetClientContext(self.ptr, client_conetxt) }
    }

    pub fn get_client_conetxt(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UploadDataSink_GetClientContext(self.ptr) }
    }

    pub fn create_with(
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
            Self { ptr }
        }
    }
}
