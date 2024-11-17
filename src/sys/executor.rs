use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_ExecutorPtr, Cronet_Executor_CreateWith,
        Cronet_Executor_Destroy, Cronet_Executor_Execute, Cronet_Executor_ExecuteFunc,
        Cronet_Executor_GetClientContext, Cronet_Executor_SetClientContext, Cronet_RunnablePtr,
    },
    util::define_impl,
};

use super::Runnable;


impl<Ctx, RunnableCtx> Executor<Ctx, RunnableCtx> 
where
    Ctx: ExecuteExt<Ctx, RunnableCtx>
{
    pub(crate) fn create_with(_execute_func: ExecuteFunc<Ctx, RunnableCtx>) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(Some(Self::raw_execute_func));
            Self {
                ptr,
                ctx: None,
                _phan: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_execute_func(self_: Cronet_ExecutorPtr, command: Cronet_RunnablePtr) {
        let self_ = Executor::<Ctx, RunnableCtx>::from_ptr(self_);
        let command = Runnable::<RunnableCtx>::from_ptr(command);

        let ctx = self_.get_client_context();
        let execute = ctx.execute_func();
        execute(self_, command)
    }
}

pub(crate) type ExecuteFunc<Ctx, RunnableCtx> = fn(self_: &Executor<Ctx, RunnableCtx>, command: &Runnable<RunnableCtx>);

pub(crate) trait ExecuteExt<Ctx, RunnableCtx> {
    fn execute_func(&self) -> ExecuteFunc<Ctx, RunnableCtx>;
}

define_impl! {
    Executor, Cronet_ExecutorPtr, Cronet_Executor_Destroy,

    #[test]
    fn execute(&Self, command: &Runnable<RunnableCtx> >> Runnable::as_ptr); Cronet_Executor_Execute,

    with_ctx: <Ctx, RunnableCtx>,
    get: Cronet_Executor_GetClientContext,
    set: Cronet_Executor_SetClientContext,
}
