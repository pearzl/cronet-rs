use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use futures::{Stream, StreamExt};

use crate::bindings::{Cronet_BufferPtr, Cronet_UploadDataProviderPtr, Cronet_UploadDataSinkPtr};
use crate::error::Result;
use crate::sys::{Borrowed, UploadDataProvider, UploadDataSink};
use crate::util::BoxedFuture;

pub struct Body {
    data: BoxedStream,
    len: Option<u64>,
}

pub(crate) type BoxedStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send + Sync + 'static>>;

impl Body {
    pub fn empty() -> Self {
        Body {
            data: Box::pin(futures::stream::empty()),
            len: Some(0),
        }
    }

    pub fn stream(stream: BoxedStream, len: Option<u64>) -> Self {
        Body { data: stream, len }
    }

    pub fn length(&self) -> Option<u64> {
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
    pub(crate) fn to_upload_data_provider(self) -> UploadDataProvider<ReqBodyContext> {
        unsafe extern "C" fn get_length_func(
            ptr: crate::bindings::Cronet_UploadDataProviderPtr,
        ) -> i64 {
            let data_provider = UploadDataProvider::<ReqBodyContext>::borrow_from(ptr, &ptr);
            let ctx = data_provider.get_client_context();
            ctx.body
                .len
                .and_then(|i| i64::try_from(i).ok())
                .unwrap_or(-1)
        }

        unsafe extern "C" fn read_func(
            data_provider: Cronet_UploadDataProviderPtr,
            upload_data_sink: Cronet_UploadDataSinkPtr,
            buffer: Cronet_BufferPtr,
        ) {
            let data_provider1 =
                UploadDataProvider::<ReqBodyContext>::borrow_from(data_provider, &"");
            let data_provider2 =
                UploadDataProvider::<ReqBodyContext>::borrow_from(data_provider, &"");
            let upload_data_sink = UploadDataSink::<()>::borrow_from(upload_data_sink, &"");
            // let buffer = Borrowed::new(buffer, &buffer);
            let mut ctx = data_provider1.get_client_context();
            (ctx.run_async)(Box::pin(async move {
                let mut ctx = data_provider2.get_client_context();
                match ctx.body.next().await {
                    Some(Ok(data)) => {
                        todo!()
                    }
                    Some(Err(err)) => {
                        todo!()
                    }
                    None => {
                        todo!()
                    }
                }
            }))
        }

        unsafe extern "C" fn rewind_func(
            self_: Cronet_UploadDataProviderPtr,
            upload_data_sink: Cronet_UploadDataSinkPtr,
        ) {
            // todo
        }

        unsafe extern "C" fn close_func(self_: Cronet_UploadDataProviderPtr) {
            // todo
        }

        let mut data_provider = UploadDataProvider::create_with(
            Some(get_length_func),
            Some(read_func),
            Some(rewind_func),
            Some(close_func),
        );

        let ctx = ReqBodyContext {
            body: self,
            run_async: todo!(),
        };
        data_provider.set_client_context(ctx);
        data_provider
    }
}

pub(crate) struct ReqBodyContext {
    body: Body,
    run_async: Box<dyn Fn(BoxedFuture<()>) + Send + Sync + 'static>,
}

unsafe impl Send for ReqBodyContext {}
unsafe impl Sync for ReqBodyContext {}

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
