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

impl<Ctx> Executor<Ctx>
where
    Ctx: ExecuteExt<Ctx>,
{
    pub(crate) fn create_with(_execute_func: ExecuteFunc<Ctx, <Ctx as ExecuteExt<Ctx>>::RunnableCtx>) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(Some(Self::raw_execute_func));
            Self {
                ptr,
                _ctx: PhantomData,
                
            }
        }
    }

    unsafe extern "C" fn raw_execute_func(self_: Cronet_ExecutorPtr, command: Cronet_RunnablePtr) {
        let self_ = Executor::<Ctx>::from_ptr(self_);
        let command = Runnable::<<Ctx as ExecuteExt<Ctx>>::RunnableCtx>::from_ptr(command);

        let execute = <Ctx as ExecuteExt<Ctx>>::execute_func();
        execute(self_, command)
    }
}

pub(crate) type ExecuteFunc<Ctx, RunnableCtx> =
    fn(self_: &Executor<Ctx>, command: &Runnable<RunnableCtx>);

pub(crate) trait ExecuteExt<Ctx> {
    type RunnableCtx;
    fn execute_func() -> ExecuteFunc<Ctx, Self::RunnableCtx>;
}

define_impl! {
    Executor, Cronet_ExecutorPtr, Cronet_Executor_Destroy,

    #[test]
    fn execute<RunnableCtx>(&Self, command: &Runnable<RunnableCtx> >> Runnable::as_ptr); Cronet_Executor_Execute,

    with_ctx: <Ctx>,
    get: Cronet_Executor_GetClientContext,
    set: Cronet_Executor_SetClientContext,
}
