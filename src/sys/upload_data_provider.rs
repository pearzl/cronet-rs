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
    util::{define_impl, Borrowed},
};

use super::{Buffer, UploadDataSink};

impl<Ctx> UploadDataProvider<Ctx>
where
    Ctx: UploadDataProviderExt<Ctx>,
{
    pub(crate) fn create_with(
        _get_length_func: GetLengthFunc<Ctx>,
        _read_func: ReadFunc<
            Ctx,
            <Ctx as UploadDataProviderExt<Ctx>>::UploadDataSinkCtx,
            <Ctx as UploadDataProviderExt<Ctx>>::BufferCtx,
        >,
        _rewind_func: RewindFunc<Ctx, <Ctx as UploadDataProviderExt<Ctx>>::UploadDataSinkCtx>,
        _close_func: CloseFunc<Ctx>,
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
                _ctx: PhantomData,
                
            }
        }
    }

    unsafe extern "C" fn raw_get_length(self_: Cronet_UploadDataProviderPtr) -> i64 {
        let self_ = UploadDataProvider::<Ctx>::from_ptr(self_);

        let get_length = <Ctx as UploadDataProviderExt<Ctx>>::get_length_func();
        get_length(&self_)
    }

    unsafe extern "C" fn raw_read(
        self_: Cronet_UploadDataProviderPtr,
        upload_data_sink: Cronet_UploadDataSinkPtr,
        buffer: Cronet_BufferPtr,
    ) {
        let self_ = UploadDataProvider::<Ctx>::borrow_from(self_);
        let upload_data_sink = UploadDataSink::borrow_from(upload_data_sink);
        let buffer = Buffer::borrow_from(buffer);

        let read = <Ctx as UploadDataProviderExt<Ctx>>::read_func();
        read(self_, upload_data_sink, buffer)
    }

    unsafe extern "C" fn raw_rewind(
        self_: Cronet_UploadDataProviderPtr,
        upload_data_sink: Cronet_UploadDataSinkPtr,
    ) {
        let self_ = UploadDataProvider::<Ctx>::borrow_from(self_);
        let upload_data_sink = UploadDataSink::borrow_from(upload_data_sink);

        let rewind = <Ctx as UploadDataProviderExt<Ctx>>::rewind_func();
        rewind(self_, upload_data_sink)
    }

    unsafe extern "C" fn raw_close(self_: Cronet_UploadDataProviderPtr) {
        let self_ = UploadDataProvider::from_raw(self_);

        let close = <Ctx as UploadDataProviderExt<Ctx>>::close_func();
        close(self_)
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(
            <Ctx as UploadDataProviderExt<Ctx>>::get_length_func(),
            <Ctx as UploadDataProviderExt<Ctx>>::read_func(),
            <Ctx as UploadDataProviderExt<Ctx>>::rewind_func(),
            <Ctx as UploadDataProviderExt<Ctx>>::close_func(),
        );
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) trait UploadDataProviderExt<Ctx> {
    type BufferCtx;
    type UploadDataSinkCtx;
    fn get_length_func() -> GetLengthFunc<Ctx>;
    fn read_func() -> ReadFunc<Ctx, Self::UploadDataSinkCtx, Self::BufferCtx>;
    fn rewind_func() -> RewindFunc<Ctx, Self::UploadDataSinkCtx>;
    fn close_func() -> CloseFunc<Ctx>;
}

pub(crate) type GetLengthFunc<Ctx> = fn(&UploadDataProvider<Ctx>) -> i64;
pub(crate) type ReadFunc<Ctx, UploadDataSinkCtx, BufferCtx> =
    fn(Borrowed<UploadDataProvider<Ctx>>, Borrowed<UploadDataSink<UploadDataSinkCtx>>, Borrowed<Buffer<BufferCtx>>);
pub(crate) type RewindFunc<Ctx, UploadDataSinkCtx> =
    fn(Borrowed<UploadDataProvider<Ctx>>, Borrowed<UploadDataSink<UploadDataSinkCtx>>);
pub(crate) type CloseFunc<Ctx> = fn(UploadDataProvider<Ctx>);

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

    with_ctx: <Ctx>,
    get: Cronet_UploadDataProvider_GetClientContext,
    set: Cronet_UploadDataProvider_SetClientContext,
}

unsafe impl<Ctx> Send for UploadDataProvider<Ctx> where Ctx: Send {}
unsafe impl<Ctx> Sync for UploadDataProvider<Ctx> where Ctx: Sync {}
