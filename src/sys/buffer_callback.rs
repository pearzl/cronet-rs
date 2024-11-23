use std::{marker::PhantomData, ops::Deref};

use crate::{
    bindings::{
        Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith, Cronet_BufferCallback_Destroy,
        Cronet_BufferCallback_GetClientContext, Cronet_BufferCallback_OnDestroy,
        Cronet_BufferCallback_OnDestroyFunc, Cronet_BufferCallback_SetClientContext,
        Cronet_BufferPtr, Cronet_ClientContext,
    },
    util::define_impl,
};

use super::Buffer;

impl<Ctx> BufferCallback<Ctx>
where
    Ctx: BufferCallbackExt<Ctx>,
{
    pub(crate) fn create_with(
        _on_destroy_func: OnDestoryFunc<Ctx, <Ctx as BufferCallbackExt<Ctx>>::BufferCtx>,
    ) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(Some(Self::raw_on_destroy_func));
            Self {
                ptr,
                _ctx: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_on_destroy_func(
        self_: Cronet_BufferCallbackPtr,
        buffer: Cronet_BufferPtr,
    ) {
        let self_ = BufferCallback::from_ptr(self_);
        let buffer = Buffer::from_raw(buffer);

        let on_destroy = <Ctx as BufferCallbackExt<Ctx>>::on_destroy_func();
        on_destroy(self_, buffer)
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(<Ctx as BufferCallbackExt<Ctx>>::on_destroy_func());
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) type OnDestoryFunc<Ctx, BufferCtx> =
    fn(self_: &BufferCallback<Ctx>, buffer: Buffer<BufferCtx>);

pub(crate) trait BufferCallbackExt<Ctx> {
    type BufferCtx;
    fn on_destroy_func() -> OnDestoryFunc<Ctx, Self::BufferCtx>;
}

define_impl! {
    BufferCallback, Cronet_BufferCallbackPtr, Cronet_BufferCallback_Destroy,

    #[cfg(test)]
    fn on_destroy<T>(&Self, buffer: &Buffer<T> >> Buffer::as_ptr); Cronet_BufferCallback_OnDestroy,

    with_ctx: <Ctx>,
    get:  Cronet_BufferCallback_GetClientContext,
    set:  Cronet_BufferCallback_SetClientContext,
}
