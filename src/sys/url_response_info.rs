use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_UrlResponseInfoPtr, Cronet_UrlResponseInfo_Create, Cronet_UrlResponseInfo_Destroy,
        Cronet_UrlResponseInfo_all_headers_list_add, Cronet_UrlResponseInfo_all_headers_list_at,
        Cronet_UrlResponseInfo_all_headers_list_clear,
        Cronet_UrlResponseInfo_all_headers_list_size, Cronet_UrlResponseInfo_http_status_code_get,
        Cronet_UrlResponseInfo_http_status_code_set, Cronet_UrlResponseInfo_http_status_text_get,
        Cronet_UrlResponseInfo_http_status_text_set,
        Cronet_UrlResponseInfo_negotiated_protocol_get,
        Cronet_UrlResponseInfo_negotiated_protocol_set, Cronet_UrlResponseInfo_proxy_server_get,
        Cronet_UrlResponseInfo_proxy_server_set, Cronet_UrlResponseInfo_received_byte_count_get,
        Cronet_UrlResponseInfo_received_byte_count_set, Cronet_UrlResponseInfo_url_chain_add,
        Cronet_UrlResponseInfo_url_chain_at, Cronet_UrlResponseInfo_url_chain_clear,
        Cronet_UrlResponseInfo_url_chain_size, Cronet_UrlResponseInfo_url_get,
        Cronet_UrlResponseInfo_url_set, Cronet_UrlResponseInfo_was_cached_get,
        Cronet_UrlResponseInfo_was_cached_set,
    },
    util::define_impl,
};

use super::http_header::HttpHeader;

define_impl! {
    UrlResponseInfo, Cronet_UrlResponseInfoPtr, Cronet_UrlResponseInfo_Destroy,

    fn url_set(&mut Self, url: &CStr >> CStr::as_ptr);
        Cronet_UrlResponseInfo_url_set,
    fn url_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_UrlResponseInfo_url_get,

    fn url_chain_add(&Self, element: &CStr >> CStr::as_ptr);
        Cronet_UrlResponseInfo_url_chain_add,
    fn url_chain_size(&Self) -> u32;
        Cronet_UrlResponseInfo_url_chain_size,
    fn url_chain_at(&Self, index: u32) -> &CStr >> CStr::from_ptr; // safety: out of bounds
        Cronet_UrlResponseInfo_url_chain_at,
    fn url_chain_clear(&mut Self);
        Cronet_UrlResponseInfo_url_chain_clear,

    fn http_status_code_set(&mut Self, http_status_code: i32);
        Cronet_UrlResponseInfo_http_status_code_set,
    fn http_status_code_get(&Self) -> i32;
        Cronet_UrlResponseInfo_http_status_code_get,

    fn http_status_text_set(&mut Self, http_status_text: &CStr >> CStr::as_ptr);
        Cronet_UrlResponseInfo_http_status_text_set,
    fn http_status_text_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_UrlResponseInfo_http_status_text_get,

    fn all_headers_list_add(&mut Self, element: &HttpHeader >> HttpHeader::as_ptr); // safety: cloned
        Cronet_UrlResponseInfo_all_headers_list_add,
    fn all_headers_list_size(&Self) -> u32;
        Cronet_UrlResponseInfo_all_headers_list_size,
    fn all_headers_list_at(&Self, index: u32) -> &HttpHeader >> HttpHeader::from_ptr; // safety: out of bounds
        Cronet_UrlResponseInfo_all_headers_list_at,
    fn all_headers_list_clear(&mut Self);
        Cronet_UrlResponseInfo_all_headers_list_clear,

    fn was_cached_set(&mut Self, was_cached: bool);
        Cronet_UrlResponseInfo_was_cached_set,
    fn was_cached_get(&Self) -> bool;
        Cronet_UrlResponseInfo_was_cached_get,

    fn negotiated_protocol_set(&mut Self, negotiated_protocol: &CStr >> CStr::as_ptr);
        Cronet_UrlResponseInfo_negotiated_protocol_set,
    fn negotiated_protocol_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_UrlResponseInfo_negotiated_protocol_get,

    fn proxy_server_set(&mut Self, proxy_server: &CStr >> CStr::as_ptr);
        Cronet_UrlResponseInfo_proxy_server_set,
    fn proxy_server_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_UrlResponseInfo_proxy_server_get,

    fn received_byte_count(&Self, received_byte_count: i64);
        Cronet_UrlResponseInfo_received_byte_count_set,
    fn received_byte_count_get(&Self) -> i64;
        Cronet_UrlResponseInfo_received_byte_count_get,

}

impl UrlResponseInfo {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_Create();
            Self { ptr }
        }
    }
}
