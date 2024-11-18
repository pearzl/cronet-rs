use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_RunnablePtr, Cronet_Runnable_CreateWith,
        Cronet_Runnable_Destroy, Cronet_Runnable_GetClientContext, Cronet_Runnable_Run,
        Cronet_Runnable_RunFunc, Cronet_Runnable_SetClientContext,
    },
    util::define_impl,
};

impl<Ctx> Runnable<Ctx>
where
    Ctx: RunnableExt<Ctx>,
{
    pub(crate) fn create_with(_run_func: RunFunc<Ctx>) -> Self {
        unsafe {
            let ptr = Cronet_Runnable_CreateWith(Some(Self::raw_run_func));
            Self {
                ptr,
                _ctx: PhantomData,
                _phan: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_run_func(self_: Cronet_RunnablePtr) {
        let self_ = Self::from_ptr(self_);

        let run = <Ctx as RunnableExt<Ctx>>::run_func();
        run(self_)
    }

    pub(crate) fn new() -> Self {
        Self::create_with(<Ctx as RunnableExt<Ctx>>::run_func())
    }
}

pub(crate) type RunFunc<Ctx> = fn(self_: &Runnable<Ctx>);

pub(crate) trait RunnableExt<Ctx> {
    fn run_func() -> RunFunc<Ctx>;
}

define_impl! {
    Runnable, Cronet_RunnablePtr, Cronet_Runnable_Destroy,

    #[cfg(test)]
    fn run(&Self); Cronet_Runnable_Run,

    with_ctx: <Ctx>,
    get: Cronet_Runnable_GetClientContext,
    set: Cronet_Runnable_SetClientContext,
}
