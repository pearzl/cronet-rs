use std::{ffi::CString, sync::Arc};

use futures::{
    channel::{mpsc, oneshot},
    SinkExt,
};
use http::{
    header::CONTENT_LENGTH, request::Parts, HeaderName, HeaderValue, Request, Response, StatusCode,
};

use crate::{
    bindings::Cronet_RESULT,
    body::{Body, BufferContext, Data},
    client::Client,
    error::{Error, Result},
    sys::{
        Buffer, HttpHeader, UrlRequest, UrlRequestCallback, UrlRequestCallbackExt,
        UrlRequestParams, UrlResponseInfo,
    },
    util::RunAsyncFunc,
};

pub async fn send(client: &Client, req: Request<Body>) -> Result<Response<Body>, Error> {
    let url = to_cstr(req.uri().to_string());
    let (parts, body) = req.into_parts();

    let mut request_prams = to_url_request_params(parts);
    let upload_data_provider = Body::into_upload_data_provider(body, Arc::clone(&client.run_async));
    request_prams.upload_data_provider_set(upload_data_provider);
    request_prams.upload_data_provider_executor_set(&client.executor);

    let (resp_tx, resp_rx) = oneshot::channel();
    let callback = new_callback(Arc::clone(&client.run_async), resp_tx);

    let mut url_request = UrlRequest::create();
    let ctx = UrlRequestContext { callback };
    url_request.set_client_context(ctx);

    let ctx_ref = &url_request.get_client_context().callback;
    url_request.init_with_params(
        &client.engine,
        &url,
        &request_prams,
        ctx_ref,
        &client.executor,
    );

    let ret = url_request.start();
    if ret != Cronet_RESULT::SUCCESS {
        return Err(Error::CronetResult(ret));
    }

    log::trace!("request start");
    resp_rx.await.unwrap_or(Err(Error::Canceled))
}

fn to_url_request_params(parts: Parts) -> UrlRequestParams {
    let Parts {
        method, headers, ..
    } = parts;
    let mut params = UrlRequestParams::create();

    params.http_method_set(&to_cstr(method.as_str()));

    for (k, v) in headers.iter() {
        let Ok(value) = CString::new(v.as_bytes()) else {
            continue;
        };
        let name = to_cstr(k.as_str());
        let mut header = HttpHeader::create();
        header.name_set(&name);
        header.value_set(&value);
        params.request_headers_add(&header);
    }

    params
}

/// uri, header_name and method do not contain the '\0'
fn to_cstr(s: impl Into<Vec<u8>>) -> CString {
    let buf = s.into();
    unsafe { CString::from_vec_unchecked(buf) }
}

fn new_callback(
    run_async_func: RunAsyncFunc,
    resp_tx: oneshot::Sender<Result<Response<Body>>>,
) -> UrlRequestCallback<UrlRequestCallbackContext> {
    let (body_tx, body_rx) = mpsc::channel(1);
    let ctx = UrlRequestCallbackContext {
        run_async_func,
        buffer_size: 16 * 1024,
        resp_tx: Some(resp_tx),
        body_rx: Some(body_rx),
        body_tx,
    };
    UrlRequestCallback::new(ctx)
}

pub(crate) struct UrlRequestCallbackContext {
    run_async_func: RunAsyncFunc,
    buffer_size: usize,
    // on_response_started
    resp_tx: Option<oneshot::Sender<Result<Response<Body>>>>,
    body_rx: Option<mpsc::Receiver<Result<Data, Error>>>,
    // on_read_completed
    body_tx: mpsc::Sender<Result<Data, Error>>,
}

impl UrlRequestCallbackExt<UrlRequestCallbackContext> for UrlRequestCallbackContext {
    type UrlRequestCtx = UrlRequestContext;

    type BufferCtx = BufferContext;

    fn on_redirect_received_func(
    ) -> crate::sys::OnRedirectReceivedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_, request, info, new_location_url| {
            let _ret = request.follow_redirect(); // todo: how to deal with _ret?
            let _ = (self_, info, new_location_url);
        }
    }

    fn on_response_started_func(
    ) -> crate::sys::OnResponseStartedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_, request, info| {
            let ctx = self_.get_client_context_mut();
            let resp = to_response(info);

            let body_len = resp
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok());
            let body_stream = ctx.body_rx.take().unwrap();
            let body = Body::stream(Box::pin(body_stream), body_len);

            let resp_tx = ctx.resp_tx.take().unwrap();
            let is_canceled = resp_tx.send(Ok(resp.map(|_| body))).is_err();
            if is_canceled {
                request.cancel();
                return;
            }

            let buffer: Buffer<()> = Buffer::with_capacity(ctx.buffer_size);
            let ret = request.read(buffer);
            assert_eq!(ret, Cronet_RESULT::SUCCESS);
        }
    }

    fn on_read_completed_func() -> crate::sys::OnReadCompletedFunc<
        UrlRequestCallbackContext,
        Self::UrlRequestCtx,
        Self::BufferCtx,
    > {
        |self_, request, _info, buffer, bytes_read| {
            let ctx = self_.get_client_context_mut();
            let run_async = Arc::clone(&ctx.run_async_func);

            log::trace!("on read completed");
            run_async(Box::pin(async move {
                log::trace!("begin send data");

                // the buffer we created is smaller then usize::MAX
                let data = Data::from_buffer(buffer, bytes_read as _); 

                log::trace!("send data: {}", bytes_read);
                let is_canceled = ctx.body_tx.send(Ok(data)).await.is_err();
                if is_canceled {
                    request.cancel();
                    return;
                }

                let buffer: Buffer<()> = Buffer::with_capacity(ctx.buffer_size);
                let ret = request.read(buffer);
                assert_eq!(ret, Cronet_RESULT::SUCCESS);
            }));
        }
    }

    fn on_succeeded_func(
    ) -> crate::sys::OnSucceededFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_, request, info| {
            log::trace!("on success");
            let _ = (self_, request, info);
        }
    }

    fn on_failed_func() -> crate::sys::OnFailedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx>
    {
        |self_, request, info, error| {
            log::trace!("on failed");
            let _ = (request, info);
            let ctx = self_.get_client_context_mut();
            let error = error.into();
            if let Some(resp_tx) = ctx.resp_tx.take() {
                let _ret = resp_tx.send(Err(error));
            } else {
                let run_async = Arc::clone(&ctx.run_async_func);
                run_async(Box::pin(async move {
                    let _ret = ctx.body_tx.send(Err(error)).await;
                }));
            }
        }
    }

    fn on_canceled_func(
    ) -> crate::sys::OnCanceledFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_, request, info| {
            log::trace!("on canceled");
            let _ = (self_, request, info);
        }
    }
}

fn to_response(info: &UrlResponseInfo) -> Response<()> {
    let mut resp = Response::new(());

    let status_code = info.http_status_code_get();
    if let Ok(Ok(s)) = u16::try_from(status_code).map(http::StatusCode::from_u16) {
        *resp.status_mut() = s
    } else {
        *resp.status_mut() = StatusCode::from_u16(999).unwrap();
        resp.extensions_mut().insert(InvalidStatusCode(status_code));
    }

    let header_size = info.all_headers_list_size();
    let mut invalid_headers = vec![];
    for i in 0..header_size {
        let header = info.all_headers_list_at(i);
        let name = header.name_get();
        let value = header.value_get();
        let Ok(header_name) = HeaderName::from_bytes(name.to_bytes()) else {
            invalid_headers.push((name.to_owned(), value.to_owned()));
            continue;
        };
        let Ok(header_value) = HeaderValue::from_bytes(value.to_bytes()) else {
            invalid_headers.push((name.to_owned(), value.to_owned()));
            continue;
        };
        resp.headers_mut().append(header_name, header_value);
    }
    resp.extensions_mut()
        .insert(InvalidHeaders(invalid_headers));

    resp
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidStatusCode(pub i32);

#[derive(Clone, Debug)]
pub struct InvalidHeaders(pub Vec<(CString, CString)>);

pub(crate) struct UrlRequestContext {
    callback: UrlRequestCallback<UrlRequestCallbackContext>,
}
