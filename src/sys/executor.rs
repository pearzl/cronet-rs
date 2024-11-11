use crate::bindings::{
    Cronet_ClientContext, Cronet_ExecutorPtr, Cronet_Executor_CreateWith, Cronet_Executor_Destroy,
    Cronet_Executor_ExecuteFunc, Cronet_Executor_GetClientContext,
    Cronet_Executor_SetClientContext,
};

pub struct Executor {
    pub ptr: Cronet_ExecutorPtr,
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe { Cronet_Executor_Destroy(self.ptr) }
    }
}

impl Executor {
    pub fn set_client_conetxt(&self, client_conetxt: Cronet_ClientContext) {
        unsafe { Cronet_Executor_SetClientContext(self.ptr, client_conetxt) }
    }

    pub fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_Executor_GetClientContext(self.ptr) }
    }

    pub fn create_with(execute_func: Cronet_Executor_ExecuteFunc) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(execute_func);
            Self { ptr }
        }
    }
}
