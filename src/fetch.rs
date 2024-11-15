use std::ffi::CString;

use http::{request::Parts, Request, Response};

use crate::{body::Body, client::Client, error::Error, sys::{HttpHeader, UrlRequestParams}};


pub async fn send(client: &Client, req: Request<Body>) -> Result<Response<Body>, Error> {
    let (parts, body) = req.into_parts();

    let request_prams = to_url_request_params(parts);
    // let data_provider = Body::to

    todo!()

}

fn to_url_request_params(parts: Parts) -> UrlRequestParams {
    let Parts { method, headers, .. } = parts;
    let mut params = UrlRequestParams::create();
    
    params.http_method_set(&to_cstr(method.as_str()));

    // > The first yielded item will have HeaderName set.
    let mut name = unsafe {CString::from_vec_unchecked(vec![])};
    for (k, v) in headers {
        let Ok(value) = CString::new(v.as_bytes()) else { continue};
        if let Some(key) = k {
            name = to_cstr(key.as_str());
        }
        let mut header = HttpHeader::create();
        header.name_set(&name);
        header.value_set(&value);
        params.request_headers_add(&header);
    }

    params
}

/// header_name and method do not contain the '\0'
fn to_cstr(s: &str) -> CString {
    let mut buf = s.as_bytes().to_vec();
    buf.push(b'\0');
    unsafe {
        CString::from_vec_with_nul_unchecked(buf)
    }
}