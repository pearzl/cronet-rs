use std::ffi::CStr;

use crate::bindings::{
    Cronet_UrlResponseInfoPtr, Cronet_UrlResponseInfo_Create, Cronet_UrlResponseInfo_Destroy,
    Cronet_UrlResponseInfo_all_headers_list_add, Cronet_UrlResponseInfo_all_headers_list_at,
    Cronet_UrlResponseInfo_all_headers_list_clear, Cronet_UrlResponseInfo_all_headers_list_size,
    Cronet_UrlResponseInfo_http_status_code_get, Cronet_UrlResponseInfo_http_status_code_set,
    Cronet_UrlResponseInfo_http_status_text_get, Cronet_UrlResponseInfo_http_status_text_set,
    Cronet_UrlResponseInfo_negotiated_protocol_get, Cronet_UrlResponseInfo_negotiated_protocol_set,
    Cronet_UrlResponseInfo_proxy_server_get, Cronet_UrlResponseInfo_proxy_server_set,
    Cronet_UrlResponseInfo_received_byte_count_get, Cronet_UrlResponseInfo_received_byte_count_set,
    Cronet_UrlResponseInfo_url_chain_add, Cronet_UrlResponseInfo_url_chain_at,
    Cronet_UrlResponseInfo_url_chain_clear, Cronet_UrlResponseInfo_url_chain_size,
    Cronet_UrlResponseInfo_url_get, Cronet_UrlResponseInfo_url_set,
    Cronet_UrlResponseInfo_was_cached_get, Cronet_UrlResponseInfo_was_cached_set,
};

use super::{http_header::HttpHeader, Borrowed};

pub(crate) struct UrlResponseInfo {
    ptr: Cronet_UrlResponseInfoPtr,
}

impl Drop for UrlResponseInfo {
    fn drop(&mut self) {
        unsafe { Cronet_UrlResponseInfo_Destroy(self.ptr) }
    }
}

impl UrlResponseInfo {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_Create();
            Self { ptr }
        }
    }

    pub(crate) fn url_set(&mut self, url: &CStr) {
        unsafe {
            Cronet_UrlResponseInfo_url_set(self.ptr, url.as_ptr());
        }
    }

    pub(crate) fn url_chain_add(&self, element: &CStr) {
        unsafe {
            Cronet_UrlResponseInfo_url_chain_add(self.ptr, element.as_ptr());
        }
    }

    pub(crate) fn http_status_code_set(&mut self, http_status_code: i32) {
        unsafe {
            Cronet_UrlResponseInfo_http_status_code_set(self.ptr, http_status_code);
        }
    }

    pub(crate) fn http_status_text_set(&mut self, http_status_text: &CStr) {
        unsafe {
            Cronet_UrlResponseInfo_http_status_text_set(self.ptr, http_status_text.as_ptr());
        }
    }

    pub(crate) fn all_headers_list_add(&self, element: &HttpHeader) {
        unsafe {
            Cronet_UrlResponseInfo_all_headers_list_add(self.ptr, element.as_ptr());
        }
    }

    pub(crate) fn was_cached_set(&mut self, was_cached: bool) {
        unsafe {
            Cronet_UrlResponseInfo_was_cached_set(self.ptr, was_cached);
        }
    }

    pub(crate) fn negotiated_protocol_set(&mut self, negotiated_protocol: &CStr) {
        unsafe {
            Cronet_UrlResponseInfo_negotiated_protocol_set(self.ptr, negotiated_protocol.as_ptr());
        }
    }

    pub(crate) fn proxy_server_set(&mut self, proxy_server: &CStr) {
        unsafe {
            Cronet_UrlResponseInfo_proxy_server_set(self.ptr, proxy_server.as_ptr());
        }
    }

    pub(crate) fn received_byte_count(&self, received_byte_count: i64) {
        unsafe {
            Cronet_UrlResponseInfo_received_byte_count_set(self.ptr, received_byte_count);
        }
    }

    pub(crate) fn url_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_url_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn url_chain_size(&self) -> u32 {
        unsafe { Cronet_UrlResponseInfo_url_chain_size(self.ptr) }
    }

    pub(crate) fn url_chain_at(&self, index: u32) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_url_chain_at(self.ptr, index);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn url_chain_clear(&mut self) {
        unsafe {
            Cronet_UrlResponseInfo_url_chain_clear(self.ptr);
        }
    }

    pub(crate) fn http_status_code_get(&self) -> i32 {
        unsafe { Cronet_UrlResponseInfo_http_status_code_get(self.ptr) }
    }

    pub(crate) fn http_status_text_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_http_status_text_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn all_headers_list_size(&self) -> u32 {
        unsafe { Cronet_UrlResponseInfo_all_headers_list_size(self.ptr) }
    }

    pub(crate) fn all_headers_list_at(&self, index: u32) -> Borrowed<HttpHeader> {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_all_headers_list_at(self.ptr, index);
            assert!(!ptr.is_null());
            HttpHeader::borrow_from(ptr, self)
        }
    }

    pub(crate) fn all_headers_list_clear(&mut self) {
        unsafe {
            Cronet_UrlResponseInfo_all_headers_list_clear(self.ptr);
        }
    }

    pub(crate) fn was_cached_get(&self) -> bool {
        unsafe { Cronet_UrlResponseInfo_was_cached_get(self.ptr) }
    }

    pub(crate) fn negotiated_protocol_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_negotiated_protocol_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn proxy_server_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_proxy_server_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn received_byte_count_get(&self) -> i64 {
        unsafe { Cronet_UrlResponseInfo_received_byte_count_get(self.ptr) }
    }
}
