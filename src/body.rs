use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use futures::channel::mpsc;
use futures::{Stream, StreamExt};

use crate::error::{Error, Result};
use crate::sys::{
    CloseFunc, GetLengthFunc, ReadFunc, RewindFunc, UploadDataProvider, UploadDataProviderExt,
};
use crate::util::RunAsyncFunc;

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
        run_async: RunAsyncFunc,
    ) -> UploadDataProvider<ReqBodyContext> {
        let ctx = ReqBodyContext {
            body: self,
            run_async,
        };

        let upload_data_provider = UploadDataProvider::new(ctx);

        upload_data_provider
    }
}

pub(crate) struct ReqBodyContext {
    body: Body,
    run_async: RunAsyncFunc,
}

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
        |mut upload_data_provider, upload_data_sink, mut buffer| {
            let ctx = upload_data_provider.get_client_context_mut();
            let run_async = Arc::clone(&ctx.run_async);
            run_async(Box::pin(async move {
                match ctx.body.next().await {
                    Some(Ok(data)) => {
                        // todo: buffer < data  -> save data; buffer > data -> continue write;
                        let (bytes_read, _) = buffer.write(&data);
                        upload_data_sink.on_read_succeeded(bytes_read as u64, false);
                    }
                    Some(Err(_err)) => {
                        let msg =
                            unsafe { CStr::from_bytes_with_nul_unchecked(b"read body failed\0") };
                        upload_data_sink.on_read_error(msg);
                    }
                    None => {
                        upload_data_sink.on_read_succeeded(0, ctx.body.len.is_none());
                    }
                }
            }));
        }
    }

    // todo: rewind support
    fn rewind_func() -> RewindFunc<ReqBodyContext, UploadDataSinkContext> {
        |upload_data_provider, upload_data_sink| {
            let ctx = upload_data_provider.get_client_context();
            let run_async = Arc::clone(&ctx.run_async);
            run_async(Box::pin(async move {
                let msg = unsafe { CStr::from_bytes_with_nul_unchecked(b"rewind failed\0") };
                upload_data_sink.on_rewind_error(msg);
            }))
        }
    }

    fn close_func() -> CloseFunc<ReqBodyContext> {
        |upload_data_provider| {
            let _ = upload_data_provider;
        }
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
