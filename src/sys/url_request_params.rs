use std::ffi::CStr;

use crate::bindings::{
    Cronet_RawDataPtr, Cronet_UrlRequestParamsPtr, Cronet_UrlRequestParams_Create,
    Cronet_UrlRequestParams_Destroy, Cronet_UrlRequestParams_IDEMPOTENCY,
    Cronet_UrlRequestParams_REQUEST_PRIORITY, Cronet_UrlRequestParams_allow_direct_executor_get,
    Cronet_UrlRequestParams_allow_direct_executor_set, Cronet_UrlRequestParams_annotations_add,
    Cronet_UrlRequestParams_annotations_at, Cronet_UrlRequestParams_annotations_clear,
    Cronet_UrlRequestParams_annotations_size, Cronet_UrlRequestParams_disable_cache_get,
    Cronet_UrlRequestParams_disable_cache_set, Cronet_UrlRequestParams_http_method_get,
    Cronet_UrlRequestParams_http_method_set, Cronet_UrlRequestParams_idempotency_get,
    Cronet_UrlRequestParams_idempotency_set, Cronet_UrlRequestParams_priority_get,
    Cronet_UrlRequestParams_priority_set, Cronet_UrlRequestParams_request_finished_executor_get,
    Cronet_UrlRequestParams_request_finished_executor_set,
    Cronet_UrlRequestParams_request_finished_listener_get,
    Cronet_UrlRequestParams_request_finished_listener_set,
    Cronet_UrlRequestParams_request_headers_add, Cronet_UrlRequestParams_request_headers_at,
    Cronet_UrlRequestParams_request_headers_clear, Cronet_UrlRequestParams_request_headers_size,
    Cronet_UrlRequestParams_upload_data_provider_executor_get,
    Cronet_UrlRequestParams_upload_data_provider_executor_set,
    Cronet_UrlRequestParams_upload_data_provider_get,
    Cronet_UrlRequestParams_upload_data_provider_set,
};

use super::{
    executor::Executor, http_header::HttpHeader,
    request_finished_info_listener::RequestFinishedInfoListener,
    upload_data_provider::UploadDataProvider,
};

pub struct UrlRequestParams {
    pub ptr: Cronet_UrlRequestParamsPtr,
}

impl Drop for UrlRequestParams {
    fn drop(&mut self) {
        unsafe { Cronet_UrlRequestParams_Destroy(self.ptr) }
    }
}

impl UrlRequestParams {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestParams_Create();
            Self { ptr }
        }
    }

    pub fn http_method_set(&self, http_method: &CStr) {
        unsafe { Cronet_UrlRequestParams_http_method_set(self.ptr, http_method.as_ptr()) }
    }

    pub fn request_headers_add(&self, element: &HttpHeader) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_add(self.ptr, element.ptr);
        }
    }

    pub fn disable_cache_set(&self, disable_cache: bool) {
        unsafe {
            Cronet_UrlRequestParams_disable_cache_set(self.ptr, disable_cache);
        }
    }

    pub fn priority_set(&self, priority: Cronet_UrlRequestParams_REQUEST_PRIORITY) {
        unsafe {
            Cronet_UrlRequestParams_priority_set(self.ptr, priority);
        }
    }

    pub fn upload_data_provider_set(&self, upload_data_provider: UploadDataProvider) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_set(self.ptr, upload_data_provider.ptr);
        }
    }

    pub fn upload_data_provider_executor_set(&self, upload_data_provider_executor: Executor) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_executor_set(
                self.ptr,
                upload_data_provider_executor.ptr,
            );
        }
    }

    pub fn allow_direct_executor_set(&self, allow_direct_executor: bool) {
        unsafe {
            Cronet_UrlRequestParams_allow_direct_executor_set(self.ptr, allow_direct_executor);
        }
    }

    pub fn annotations_add(&self, element: Cronet_RawDataPtr) {
        unsafe {
            Cronet_UrlRequestParams_annotations_add(self.ptr, element);
        }
    }

    pub fn request_finished_listener_set(
        &self,
        request_finished_listener: RequestFinishedInfoListener,
    ) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_listener_set(
                self.ptr,
                request_finished_listener.ptr,
            );
        }
    }

    pub fn request_finished_executor_set(&self, request_finished_executor: Executor) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_executor_set(
                self.ptr,
                request_finished_executor.ptr,
            );
        }
    }

    pub fn idempotency_set(&self, idempotency: Cronet_UrlRequestParams_IDEMPOTENCY) {
        unsafe {
            Cronet_UrlRequestParams_idempotency_set(self.ptr, idempotency);
        }
    }

    pub fn http_method_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlRequestParams_http_method_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub fn request_headers_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_request_headers_size(self.ptr) }
    }

    pub fn request_headers_at(&self, index: u32) -> HttpHeader {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_headers_at(self.ptr, index);
            HttpHeader { ptr }
        }
    }

    pub fn request_headers_clear(&self) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_clear(self.ptr);
        }
    }

    pub fn disable_cache_get(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_disable_cache_get(self.ptr) }
    }

    pub fn priority_get(&self) -> Cronet_UrlRequestParams_REQUEST_PRIORITY {
        unsafe { Cronet_UrlRequestParams_priority_get(self.ptr) }
    }

    pub fn upload_data_provider_get(&self) -> UploadDataProvider {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_get(self.ptr);
            UploadDataProvider { ptr }
        }
    }

    pub fn upload_data_provider_executor_get(&self) -> Executor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_executor_get(self.ptr);
            Executor { ptr }
        }
    }

    pub fn allow_direct_executor_get(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_allow_direct_executor_get(self.ptr) }
    }

    pub fn annotations_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_annotations_size(self.ptr) }
    }

    pub fn annotations_at(&self, index: u32) -> Cronet_RawDataPtr {
        unsafe { Cronet_UrlRequestParams_annotations_at(self.ptr, index) }
    }

    pub fn annotaions_clear(&self) {
        unsafe {
            Cronet_UrlRequestParams_annotations_clear(self.ptr);
        }
    }

    pub fn request_finished_listener_get(&self) -> RequestFinishedInfoListener {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_listener_get(self.ptr);
            RequestFinishedInfoListener { ptr }
        }
    }

    pub fn request_finished_executor_get(&self) -> Executor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_executor_get(self.ptr);
            Executor { ptr }
        }
    }

    pub fn idempotency_get(&self) -> Cronet_UrlRequestParams_IDEMPOTENCY {
        unsafe { Cronet_UrlRequestParams_idempotency_get(self.ptr) }
    }
}
