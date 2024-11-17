use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use crate::error::Result;
use crate::sys::{
    CloseFunc, GetLengthFunc, ReadFunc, RewindFunc, UploadDataProvider, UploadDataProviderExt,
};
use crate::util::BoxedFuture;

pub struct Body {
    data: BoxedStream,
    len: Option<u32>,
}

pub(crate) type BoxedStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send + Sync + 'static>>;

impl Body {
    pub fn empty() -> Self {
        Body {
            data: Box::pin(futures::stream::empty()),
            len: Some(0),
        }
    }

    pub fn stream(stream: BoxedStream, len: Option<u32>) -> Self {
        Body { data: stream, len }
    }

    pub fn length(&self) -> Option<u32> {
        self.len
    }
}

impl Deref for Body {
    type Target = BoxedStream;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Body {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Body {
    pub(crate) fn to_upload_data_provider(
        self,
        run_async: Box<dyn Fn(BoxedFuture<()>) + Send + Sync + 'static>,
    ) -> UploadDataProvider<ReqBodyContext> {
        let _ctx = ReqBodyContext {
            body: self,
            run_async,
        };
        let upload_data_provider = UploadDataProvider::new();

        upload_data_provider
    }
}

pub(crate) struct ReqBodyContext {
    body: Body,
    run_async: Box<dyn Fn(BoxedFuture<()>) + Send + Sync + 'static>,
}

unsafe impl Send for ReqBodyContext {}
unsafe impl Sync for ReqBodyContext {}

impl UploadDataProviderExt<ReqBodyContext> for ReqBodyContext {
    type BufferCtx = BufferContext;
    type UploadDataSinkCtx = UploadDataSinkContext;

    fn get_length_func() -> GetLengthFunc<ReqBodyContext> {
        |upload_data_provider| {
            let ctx = upload_data_provider.get_client_context();
            ctx.body.len.map(Into::into).unwrap_or(-1)
        }
    }

    fn read_func() -> ReadFunc<ReqBodyContext, UploadDataSinkContext, BufferContext> {
        todo!()
    }

    fn rewind_func() -> RewindFunc<ReqBodyContext, UploadDataSinkContext> {
        todo!()
    }

    fn close_func() -> CloseFunc<ReqBodyContext> {
        todo!()
    }
}

pub(crate) struct UploadDataSinkContext {}

pub(crate) struct BufferContext {}

#[cfg(test)]
mod test {
    use futures::StreamExt;

    use super::*;

    #[test]
    fn body() {
        let mut body = Body::empty();

        let mut executor = futures::executor::LocalPool::new();
        executor.run_until(async move {
            while let Some(ret) = body.next().await {
                let _ = ret.unwrap();
            }
        });
    }
}
