use crate::bindings::{
    Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith, Cronet_BufferCallback_Destroy,
    Cronet_BufferCallback_GetClientContext, Cronet_BufferCallback_OnDestroyFunc,
    Cronet_BufferCallback_SetClientContext, Cronet_ClientContext,
};

pub struct BufferCallback {
    ptr: Cronet_BufferCallbackPtr,
}

impl Drop for BufferCallback {
    fn drop(&mut self) {
        unsafe { Cronet_BufferCallback_Destroy(self.ptr) }
    }
}

impl BufferCallback {
    pub fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_BufferCallback_GetClientContext(self.ptr) }
    }

    pub fn set_client_context(&self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_BufferCallback_SetClientContext(self.ptr, client_context) }
    }

    pub fn create_with(on_destroy_func: Cronet_BufferCallback_OnDestroyFunc) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(on_destroy_func);
            Self { ptr }
        }
    }
}
