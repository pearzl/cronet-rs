use std::marker::PhantomData;

pub(crate) use buffer::Buffer;
pub(crate) use buffer_callback::BufferCallback;
pub(crate) use date_time::DateTime;
pub(crate) use engine::Engine;
pub(crate) use engine_params::EngineParams;
pub(crate) use error::Error;
pub(crate) use executor::Executor;
pub(crate) use http_header::HttpHeader;
pub(crate) use metrics::Metrics;
pub(crate) use public_key_pins::PublicKeyPins;
pub(crate) use quic_hint::QuicHint;
pub(crate) use request_finished_info::RequestFinishedInfo;
pub(crate) use request_finished_info_listener::RequestFinishedInfoListener;
pub(crate) use runnable::Runnable;
pub(crate) use upload_data_provider::UploadDataProvider;
pub(crate) use upload_data_sink::UploadDataSink;
pub(crate) use url_request::UrlRequest;
pub(crate) use url_request_callback::UrlRequestCallback;
pub(crate) use url_request_params::UrlRequestParams;
pub(crate) use url_request_status_listener::UrlRequestStatusListener;
pub(crate) use url_response_info::UrlResponseInfo;

mod buffer;
mod buffer_callback;
mod date_time;
mod engine;
mod engine_params;
mod error;
mod executor;
mod http_header;
mod metrics;
mod public_key_pins;
mod quic_hint;
mod request_finished_info;
mod request_finished_info_listener;
mod runnable;
mod upload_data_provider;
mod upload_data_sink;
mod url_request;
mod url_request_callback;
mod url_request_params;
mod url_request_status_listener;
mod url_response_info;


pub struct Borrowed<'a, T> {
    inner: *mut T,
    _phan: PhantomData<&'a ()>,
}

impl<'a, T> Borrowed<'a, T> {
    pub fn new<X>(inner: *mut T, _life: &'a X) -> Self {
        Self { inner , _phan: PhantomData}
    }
}

impl<'a, T> std::ops::Deref for Borrowed<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl<'a, T> std::ops::DerefMut for Borrowed<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}