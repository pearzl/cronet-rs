use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_BufferPtr, Cronet_ClientContext, Cronet_UploadDataProviderPtr,
        Cronet_UploadDataProvider_Close, Cronet_UploadDataProvider_CloseFunc,
        Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy,
        Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLength,
        Cronet_UploadDataProvider_GetLengthFunc, Cronet_UploadDataProvider_Read,
        Cronet_UploadDataProvider_ReadFunc, Cronet_UploadDataProvider_Rewind,
        Cronet_UploadDataProvider_RewindFunc, Cronet_UploadDataProvider_SetClientContext,
        Cronet_UploadDataSinkPtr,
    },
    util::define_impl,
};

use super::{Buffer, UploadDataSink};

impl<'a, Ctx, UploadDataSinkCtx, BufferCtx> UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UploadDataProviderPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(
        ptr: Cronet_UploadDataProviderPtr,
    ) -> &'a mut UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx> {
        let self_ = UploadDataProvider {
            ptr,
            ctx: None::<Ctx>, /* fake field */
            _phan: PhantomData,
        };
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }
}

impl<Ctx, UploadDataSinkCtx, BufferCtx> UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>
where
    Ctx: UploadDataProviderCallback<Ctx, UploadDataSinkCtx, BufferCtx>,
{
    pub(crate) fn create_with(
        _get_length_func: GetLengthFunc<Ctx, UploadDataSinkCtx, BufferCtx>,
        _read_func: ReadFunc<Ctx, UploadDataSinkCtx, BufferCtx>,
        _rewind_func: RewindFunc<Ctx, UploadDataSinkCtx, BufferCtx>,
        _close_func: CloseFunc<Ctx, UploadDataSinkCtx, BufferCtx>,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataProvider_CreateWith(
                Some(Self::raw_get_length),
                Some(Self::raw_read),
                Some(Self::raw_rewind),
                Some(Self::raw_close),
            );
            Self {
                ptr,
                ctx: None,
                _phan: PhantomData,
            }
        }
    }

    unsafe extern "C" fn raw_get_length(self_: Cronet_UploadDataProviderPtr) -> i64 {
        let self_ = UploadDataProvider::<Ctx, UploadDataSinkCtx, BufferCtx>::borrow_from_ptr(self_);

        let ctx = self_.get_client_context();
        let get_length = ctx.get_length_func();
        get_length(&self_)
    }

    unsafe extern "C" fn raw_read(
        self_: Cronet_UploadDataProviderPtr,
        upload_data_sink: Cronet_UploadDataSinkPtr,
        buffer: Cronet_BufferPtr,
    ) {
        let self_ = UploadDataProvider::<Ctx, UploadDataSinkCtx, BufferCtx>::borrow_from_ptr(self_);
        let upload_data_sink = UploadDataSink::borrow_from_ptr(upload_data_sink);
        let buffer = Buffer::borrow_from_ptr(buffer);

        let ctx = self_.get_client_context();
        let read = ctx.read_func();
        read(&self_, upload_data_sink, buffer)
    }

    unsafe extern "C" fn raw_rewind(
        self_: Cronet_UploadDataProviderPtr,
        upload_data_sink: Cronet_UploadDataSinkPtr,
    ) {
        let self_ = UploadDataProvider::<Ctx, UploadDataSinkCtx, BufferCtx>::borrow_from_ptr(self_);
        let upload_data_sink = UploadDataSink::borrow_from_ptr(upload_data_sink);

        let ctx = self_.get_client_context();
        let rewind = ctx.rewind_func();
        rewind(&self_, upload_data_sink)
    }

    unsafe extern "C" fn raw_close(self_: Cronet_UploadDataProviderPtr) {
        let self_ = UploadDataProvider::<Ctx, UploadDataSinkCtx, BufferCtx>::borrow_from_ptr(self_);

        let ctx = self_.get_client_context();
        let close = ctx.close_func();
        close(&self_)
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(
            ctx.get_length_func(),
            ctx.read_func(),
            ctx.rewind_func(),
            ctx.close_func(),
        );
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) trait UploadDataProviderCallback<Ctx, UploadDataSinkCtx, BufferCtx> {
    fn get_length_func(&self) -> GetLengthFunc<Ctx, UploadDataSinkCtx, BufferCtx>;
    fn read_func(&self) -> ReadFunc<Ctx, UploadDataSinkCtx, BufferCtx>;
    fn rewind_func(&self) -> RewindFunc<Ctx, UploadDataSinkCtx, BufferCtx>;
    fn close_func(&self) -> CloseFunc<Ctx, UploadDataSinkCtx, BufferCtx>;
}

pub(crate) type GetLengthFunc<Ctx, UploadDataSinkCtx, BufferCtx> =
    fn(&UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>) -> i64;
pub(crate) type ReadFunc<Ctx, UploadDataSinkCtx, BufferCtx> = fn(
    &UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>,
    &UploadDataSink<UploadDataSinkCtx>,
    &Buffer<BufferCtx>,
);
pub(crate) type RewindFunc<Ctx, UploadDataSinkCtx, BufferCtx> =
    fn(&UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>, &UploadDataSink<UploadDataSinkCtx>);
pub(crate) type CloseFunc<Ctx, UploadDataSinkCtx, BufferCtx> =
    fn(&UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>);

define_impl! {
    UploadDataProvider, Cronet_UploadDataProviderPtr,Cronet_UploadDataProvider_Destroy,

    #[cfg(test)]
    fn get_length(&Self) -> i64; Cronet_UploadDataProvider_GetLength,

    #[cfg(test)]
    fn read<T1, T2>(&Self,
        upload_data_sink: &UploadDataSink<T1> >> UploadDataSink::as_ptr,
        buffer: &Buffer<T2> >> Buffer::as_ptr
    ); Cronet_UploadDataProvider_Read, // safety: pass ref?

    #[cfg(test)]
    fn rewind<T>(&Self, upload_data_sink: &UploadDataSink<T> >> UploadDataSink::as_ptr);
        Cronet_UploadDataProvider_Rewind, // safety: pass ref?

    #[cfg(test)]
    fn close(&Self); Cronet_UploadDataProvider_Close,

    with_ctx: <Ctx, UploadDataSinkCtx, BufferCtx>,
    get: Cronet_UploadDataProvider_GetClientContext,
    set: Cronet_UploadDataProvider_SetClientContext,
}

unsafe impl<Ctx, UploadDataSinkCtx, BufferCtx> Send
    for UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>
where
    Ctx: Send,
{
}
unsafe impl<Ctx, UploadDataSinkCtx, BufferCtx> Sync
    for UploadDataProvider<Ctx, UploadDataSinkCtx, BufferCtx>
where
    Ctx: Sync,
{
}
