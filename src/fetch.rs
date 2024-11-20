use std::{ffi::CString, sync::Arc};

use bytes::Bytes;
use futures::{channel::{mpsc, oneshot}, SinkExt};
use http::{header::CONTENT_LENGTH, request::Parts, status, HeaderName, HeaderValue, Request, Response, StatusCode, Uri};

use crate::{
    body::{Body, BufferContext},
    client::Client,
    error::Error,
    sys::{Buffer, HttpHeader, UrlRequest, UrlRequestCallback, UrlRequestCallbackExt, UrlRequestParams, UrlResponseInfo},
};

pub async fn send(client: &Client, req: Request<Body>) -> Result<Response<Body>, Error> {
    let url = to_cstr(req.uri().to_string());
    let (parts, body) = req.into_parts();

    let mut request_prams = to_url_request_params(parts);
    let upload_data_provider = Body::to_upload_data_provider(body, Arc::clone(&client.run_async));
    request_prams.upload_data_provider_set(upload_data_provider);
    request_prams.upload_data_provider_executor_set(&client.executor);
    
    let (resp_tx, resp_rx) = oneshot::channel();
    let callback = new_callback(resp_tx);


    let url_request = UrlRequest::<()>::create();
    url_request.init_with_params(&client.engine, &url, &request_prams, &callback, &client.executor);


    resp_rx.await.map_err(|_e| Error) // todo: error trans
}

fn to_url_request_params(parts: Parts) -> UrlRequestParams {
    let Parts {
        method, headers, ..
    } = parts;
    let mut params = UrlRequestParams::create();

    params.http_method_set(&to_cstr(method.as_str()));


    for (k,v) in headers.iter() {
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

fn new_callback(resp_tx: oneshot::Sender<Response<Body>>) -> UrlRequestCallback<UrlRequestCallbackContext> {
    
    let (body_tx, body_rx) = mpsc::channel(1);
    let ctx = UrlRequestCallbackContext{
        resp_tx: Some(resp_tx),
        body_rx: Some(body_rx),
        body_tx,
    };
    let url_request_callback = UrlRequestCallback::new(ctx);
    url_request_callback
}

pub(crate) struct UrlRequestCallbackContext {
    buffer_size: usize,
    // on_response_started
    resp_tx: Option<oneshot::Sender<Response<Body>>>,
    body_rx: Option<mpsc::Receiver<Result<Bytes, Error>>>,
    // on_read_completed
    body_tx: mpsc::Sender<Result<Bytes, Error>>,
}

impl UrlRequestCallbackExt<UrlRequestCallbackContext> for UrlRequestCallbackContext {
    type UrlRequestCtx = ();

    type BufferCtx = BufferContext;

    fn on_redirect_received_func() -> crate::sys::OnRedirectReceivedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_,request,info,new_location_url|{
            let _ret = request.follow_redirect(); // todo: how to deal with _ret?
            let _ = (self_, info, new_location_url);
        }
    }

    fn on_response_started_func() -> crate::sys::OnResponseStartedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        |self_, request, info|{
            let ctx = self_.get_client_context_mut();
            let resp = to_response(info);
            
            let body_len = resp.headers().get(CONTENT_LENGTH)
                .and_then(|v|v.to_str().ok())
                .and_then(|s|s.parse().ok());
            let body_stream = ctx.body_rx.take().unwrap();
            let body = Body::stream(Box::pin(body_stream), body_len);

            let resp_tx = ctx.resp_tx.take().unwrap();
            let _ = resp_tx.send(resp.map(|_|body));

            let mut buffer: Buffer<()> = Buffer::create();
            buffer.init_with_alloc(ctx.buffer_size as u64);
            request.read(buffer); // todo: Error
        }
    }

    fn on_read_completed_func() -> crate::sys::OnReadCompletedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx, Self::BufferCtx> {
        |self_, requset, _info, buffer, bytes_read|{
            let ctx = self_.get_client_context_mut();

            let buf = buffer.get_n(bytes_read as usize); 
            let data = Bytes::copy_from_slice(buf);

            ctx.body_tx.send(Ok(data));

            let buffer: Buffer<()> = Buffer::create();
            requset.read(buffer); // todo: Error
        }
    }

    fn on_succeeded_func() -> crate::sys::OnSucceededFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        todo!()
    }

    fn on_failed_func() -> crate::sys::OnFailedFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        todo!()
    }

    fn on_canceled_func() -> crate::sys::OnCanceledFunc<UrlRequestCallbackContext, Self::UrlRequestCtx> {
        todo!()
    }
}

fn to_response(info: &UrlResponseInfo) -> Response<()> {
    let mut resp = Response::new(());

    let status_code = info.http_status_code_get();
    if let Ok(Ok(s)) = u16::try_from(status_code).map(http::StatusCode::from_u16){
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
            continue
        };
        let Ok(header_value) = HeaderValue::from_bytes(value.to_bytes()) else {
            invalid_headers.push((name.to_owned(), value.to_owned()));
            continue
        };
        resp.headers_mut().append(header_name, header_value);
    }
    resp.extensions_mut().insert(InvalidHeaders(invalid_headers));

    resp
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidStatusCode(pub i32);

#[derive(Clone, Debug)]
pub struct InvalidHeaders(pub Vec<(CString, CString)>);