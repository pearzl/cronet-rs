use std::{ffi::CString, sync::Arc};

use http::{request::Parts, Request, Response, Uri};

use crate::{
    body::Body,
    client::Client,
    error::Error,
    sys::{HttpHeader, UrlRequest, UrlRequestCallback, UrlRequestParams},
};

pub async fn send(client: &Client, req: Request<Body>) -> Result<Response<Body>, Error> {
    let url = to_cstr(req.uri().to_string());
    let (parts, body) = req.into_parts();

    let mut request_prams = to_url_request_params(parts);
    let upload_data_provider = Body::to_upload_data_provider(body, Arc::clone(&client.run_async));
    request_prams.upload_data_provider_set(upload_data_provider);
    request_prams.upload_data_provider_executor_set(&client.executor);
    
    let url_request = UrlRequest::<()>::create();
    let callback = new_callback();
    url_request.init_with_params(&client.engine, &url, &request_prams, &callback, &client.executor);

    todo!()
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

fn new_callback() -> UrlRequestCallback<()> {

    todo!()
}

