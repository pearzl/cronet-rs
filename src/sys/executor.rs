use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_ExecutorPtr, Cronet_Executor_CreateWith,
        Cronet_Executor_Destroy, Cronet_Executor_Execute, Cronet_Executor_ExecuteFunc,
        Cronet_Executor_GetClientContext, Cronet_Executor_SetClientContext,
    },
    util::define_impl,
};

use super::Runnable;

impl<'a, Ctx> Executor<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_ExecutorPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_ExecutorPtr) -> &'a mut Executor<Ctx> {
        let self_ = Executor {
            ptr,
            ctx: None::<Ctx>, /* fake field */
            _phan: PhantomData,
        };
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }
}

impl<Ctx> Executor<Ctx> {
    pub(crate) fn create_with(execute_func: Cronet_Executor_ExecuteFunc) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(execute_func);
            Self {
                ptr,
                ctx: None,
                _phan: PhantomData,
            }
        }
    }
}

define_impl! {
    Executor, Cronet_ExecutorPtr, Cronet_Executor_Destroy,

    #[test]
    fn execute<T>(&Self, command: &Runnable<T> >> Runnable::as_ptr); Cronet_Executor_Execute,


    with_ctx: <Ctx>,
    get: Cronet_Executor_GetClientContext,
    set: Cronet_Executor_SetClientContext,
}
