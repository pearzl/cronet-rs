use std::marker::PhantomData;

use crate::{
    bindings::{
        Cronet_BufferPtr, Cronet_ClientContext, Cronet_UploadDataProviderPtr, Cronet_UploadDataProvider_CloseFunc, Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy, Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLengthFunc, Cronet_UploadDataProvider_ReadFunc, Cronet_UploadDataProvider_RewindFunc, Cronet_UploadDataProvider_SetClientContext, Cronet_UploadDataSinkPtr
    },
    util::define_impl,
};

use super::{Borrowed, Buffer, UploadDataSink};

impl<'a, Ctx> UploadDataProvider<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_UploadDataProviderPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_UploadDataProviderPtr) -> &'a mut UploadDataProvider<Ctx> {
        let self_ = UploadDataProvider {ptr, ctx: None::<Ctx> /* fake field */};
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }

    pub fn borrow_from<X>(
        ptr: Cronet_UploadDataProviderPtr,
        lifetime: &'a X,
    ) -> Borrowed<'a, UploadDataProvider<Ctx>> {
        let borrowed = UploadDataProvider { ptr, ctx: None };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

impl<Ctx> UploadDataProvider<Ctx> 
    where Ctx: UploadDataProviderCallback<Ctx>
{
    pub(crate) fn create_with(
        _get_length_func: GetLengthFunc<Ctx>,
        _read_func: ReadFunc<Ctx>,
        _rewind_func: RewindFunc<Ctx>,
        _close_func: CloseFunc<Ctx>,
    ) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataProvider_CreateWith(
                Some(Self::raw_get_length),
                Some(Self::raw_read),
                Some(Self::raw_rewind),
                Some(Self::raw_close),
            );
            Self { ptr, ctx: None }
        }
    }

    unsafe extern "C" fn raw_get_length(self_: Cronet_UploadDataProviderPtr) -> i64 {
        let self_ = UploadDataProvider::<Ctx>::borrow_from_ptr(self_);

        let ctx = self_.get_client_context();
        let get_length = ctx.get_length_func();
        get_length(&self_)
    }

    unsafe extern "C" fn raw_read(
        self_: Cronet_UploadDataProviderPtr,
        upload_data_sink: Cronet_UploadDataSinkPtr,
        buffer: Cronet_BufferPtr,
    ) {
        let self_ = UploadDataProvider::<Ctx>::borrow_from_ptr(self_);
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
        let self_ = UploadDataProvider::<Ctx>::borrow_from_ptr(self_);
        let upload_data_sink = UploadDataSink::borrow_from_ptr(upload_data_sink);

        let ctx = self_.get_client_context();
        let rewind = ctx.rewind_func();
        rewind(&self_, upload_data_sink)
    }

    unsafe extern "C" fn raw_close(self_: Cronet_UploadDataProviderPtr) {
        let self_ = UploadDataProvider::<Ctx>::borrow_from_ptr(self_);

        let ctx = self_.get_client_context();
        let close = ctx.close_func();
        close(&self_)
    }

    pub(crate) fn new(ctx: Ctx) -> Self {
        let mut self_ = Self::create_with(ctx.get_length_func(), ctx.read_func(), ctx.rewind_func(), ctx.close_func());
        self_.set_client_context(ctx);
        self_
    }
}

pub(crate) trait UploadDataProviderCallback<Ctx> {
    fn get_length_func(&self) -> GetLengthFunc<Ctx>;
    fn read_func(&self) -> ReadFunc<Ctx>;
    fn rewind_func(&self) -> RewindFunc<Ctx>;
    fn close_func(&self) -> CloseFunc<Ctx>;
}

pub(crate) type GetLengthFunc<Ctx> = fn(&UploadDataProvider<Ctx>) -> i64;
pub(crate) type ReadFunc<Ctx> = fn(&UploadDataProvider<Ctx>, &UploadDataSink<()>, &Buffer<()>); // todo: generics param
pub(crate) type RewindFunc<Ctx> = fn(&UploadDataProvider<Ctx>, &UploadDataSink<()>); // todo: generics param
pub(crate) type CloseFunc<Ctx> = fn(&UploadDataProvider<Ctx>);

define_impl! {
    UploadDataProvider, Cronet_UploadDataProviderPtr,Cronet_UploadDataProvider_Destroy,
    with_ctx: Ctx,
    get: Cronet_UploadDataProvider_GetClientContext,
    set: Cronet_UploadDataProvider_SetClientContext,
}

unsafe impl<T> Send for UploadDataProvider<T> where T: Send {}
unsafe impl<T> Sync for UploadDataProvider<T> where T: Sync {}
