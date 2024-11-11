use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use crate::error::Result;

pub struct Body {
    data: BoxedStream,
    len: Option<u64>,
}

pub(crate) type BoxedStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send + 'static>>;

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
