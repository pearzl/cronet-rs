use crate::bindings::{
    Cronet_ClientContext, Cronet_RunnablePtr, Cronet_Runnable_CreateWith, Cronet_Runnable_Destroy,
    Cronet_Runnable_GetClientContext, Cronet_Runnable_RunFunc, Cronet_Runnable_SetClientContext,
};

pub struct Runnable {
    ptr: Cronet_RunnablePtr,
}

impl Drop for Runnable {
    fn drop(&mut self) {
        unsafe { Cronet_Runnable_Destroy(self.ptr) }
    }
}

impl Runnable {
    pub fn set_client_context(&self, client_context: Cronet_ClientContext) {
        unsafe { Cronet_Runnable_SetClientContext(self.ptr, client_context) }
    }

    pub fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_Runnable_GetClientContext(self.ptr) }
    }

    pub fn create_with(run_func: Cronet_Runnable_RunFunc) -> Self {
        unsafe {
            let ptr = Cronet_Runnable_CreateWith(run_func);
            Self { ptr }
        }
    }
}
