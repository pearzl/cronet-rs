use crate::bindings::{
    Cronet_ClientContext, Cronet_ExecutorPtr, Cronet_Executor_CreateWith, Cronet_Executor_Destroy,
    Cronet_Executor_ExecuteFunc, Cronet_Executor_GetClientContext,
    Cronet_Executor_SetClientContext,
};

use super::Borrowed;

pub(crate) struct Executor {
    ptr: Cronet_ExecutorPtr,
}

impl Executor {
    pub(crate) fn as_ptr(&self) -> Cronet_ExecutorPtr {
        self.ptr
    }

    pub fn borrow_from(ptr: Cronet_ExecutorPtr) -> Borrowed<Executor> {
        let borrowed = Executor { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed { inner: ptr }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe { Cronet_Executor_Destroy(self.ptr) }
    }
}

impl Executor {
    pub(crate) fn set_client_conetxt(&mut self, client_conetxt: Cronet_ClientContext) {
        unsafe { Cronet_Executor_SetClientContext(self.ptr, client_conetxt) }
    }

    pub(crate) fn get_client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_Executor_GetClientContext(self.ptr) }
    }

    pub(crate) fn create_with(execute_func: Cronet_Executor_ExecuteFunc) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(execute_func);
            Self { ptr }
        }
    }
}
