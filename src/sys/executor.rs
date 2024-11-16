use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_ExecutorPtr, Cronet_Executor_CreateWith,
        Cronet_Executor_Destroy, Cronet_Executor_ExecuteFunc, Cronet_Executor_GetClientContext,
        Cronet_Executor_SetClientContext,
    },
    util::define_impl,
};

use super::Borrowed;

impl<'a, Ctx> Executor<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_ExecutorPtr {
        self.ptr
    }

    pub fn borrow_from<X>(ptr: Cronet_ExecutorPtr, lifetime: &'a X) -> Borrowed<'a, Executor<Ctx>> {
        let borrowed = Executor { ptr, ctx: None };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> Executor<Ctx> {
    pub(crate) fn create_with(execute_func: Cronet_Executor_ExecuteFunc) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(execute_func);
            Self { ptr, ctx: None }
        }
    }
}

define_impl! {
    Executor, Cronet_ExecutorPtr, Cronet_Executor_Destroy,
    with_ctx: Ctx,
    get: Cronet_Executor_GetClientContext,
    set: Cronet_Executor_SetClientContext,
}
