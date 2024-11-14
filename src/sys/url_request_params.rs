use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_RawDataPtr, Cronet_UrlRequestParamsPtr, Cronet_UrlRequestParams_Create,
        Cronet_UrlRequestParams_Destroy, Cronet_UrlRequestParams_IDEMPOTENCY,
        Cronet_UrlRequestParams_REQUEST_PRIORITY,
        Cronet_UrlRequestParams_allow_direct_executor_get,
        Cronet_UrlRequestParams_allow_direct_executor_set, Cronet_UrlRequestParams_annotations_add,
        Cronet_UrlRequestParams_annotations_at, Cronet_UrlRequestParams_annotations_clear,
        Cronet_UrlRequestParams_annotations_size, Cronet_UrlRequestParams_disable_cache_get,
        Cronet_UrlRequestParams_disable_cache_set, Cronet_UrlRequestParams_http_method_get,
        Cronet_UrlRequestParams_http_method_set, Cronet_UrlRequestParams_idempotency_get,
        Cronet_UrlRequestParams_idempotency_set, Cronet_UrlRequestParams_priority_get,
        Cronet_UrlRequestParams_priority_set,
        Cronet_UrlRequestParams_request_finished_executor_get,
        Cronet_UrlRequestParams_request_finished_executor_set,
        Cronet_UrlRequestParams_request_finished_listener_get,
        Cronet_UrlRequestParams_request_finished_listener_set,
        Cronet_UrlRequestParams_request_headers_add, Cronet_UrlRequestParams_request_headers_at,
        Cronet_UrlRequestParams_request_headers_clear,
        Cronet_UrlRequestParams_request_headers_size,
        Cronet_UrlRequestParams_upload_data_provider_executor_get,
        Cronet_UrlRequestParams_upload_data_provider_executor_set,
        Cronet_UrlRequestParams_upload_data_provider_get,
        Cronet_UrlRequestParams_upload_data_provider_set,
    },
    sys::{
        executor::BorrowedExecutor,
        request_finished_info_listener::BorrowedRequestFinishedInfoListener,
        upload_data_provider::BorrowedUploadDataProvider,
    },
};

use super::{
    executor::Executor,
    http_header::{BorrowedHttpHeader, HttpHeader},
    request_finished_info_listener::RequestFinishedInfoListener,
    upload_data_provider::UploadDataProvider,
};

pub(crate) struct UrlRequestParams {
    ptr: Cronet_UrlRequestParamsPtr,
}

impl UrlRequestParams {
    pub(crate) fn as_ptr(&self) -> Cronet_UrlRequestParamsPtr {
        self.ptr
    }
}

impl Drop for UrlRequestParams {
    fn drop(&mut self) {
        unsafe { Cronet_UrlRequestParams_Destroy(self.ptr) }
    }
}

impl UrlRequestParams {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestParams_Create();
            Self { ptr }
        }
    }

    pub(crate) fn http_method_set(&mut self, http_method: &CStr) {
        unsafe { Cronet_UrlRequestParams_http_method_set(self.ptr, http_method.as_ptr()) }
    }

    pub(crate) fn request_headers_add(&self, element: &HttpHeader) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_add(self.ptr, element.as_ptr());
        }
    }

    pub(crate) fn disable_cache_set(&mut self, disable_cache: bool) {
        unsafe {
            Cronet_UrlRequestParams_disable_cache_set(self.ptr, disable_cache);
        }
    }

    pub(crate) fn priority_set(&mut self, priority: Cronet_UrlRequestParams_REQUEST_PRIORITY) {
        unsafe {
            Cronet_UrlRequestParams_priority_set(self.ptr, priority);
        }
    }

    pub(crate) fn upload_data_provider_set(&mut self, upload_data_provider: UploadDataProvider) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_set(
                self.ptr,
                upload_data_provider.as_ptr(),
            );
        }
    }

    pub(crate) fn upload_data_provider_executor_set(&mut self, upload_data_provider_executor: Executor) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_executor_set(
                self.ptr,
                upload_data_provider_executor.as_ptr(),
            );
        }
    }

    pub(crate) fn allow_direct_executor_set(&mut self, allow_direct_executor: bool) {
        unsafe {
            Cronet_UrlRequestParams_allow_direct_executor_set(self.ptr, allow_direct_executor);
        }
    }

    pub(crate) fn annotations_add(&self, element: Cronet_RawDataPtr) {
        unsafe {
            Cronet_UrlRequestParams_annotations_add(self.ptr, element);
        }
    }

    pub(crate) fn request_finished_listener_set(
        &self,
        request_finished_listener: RequestFinishedInfoListener,
    ) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_listener_set(
                self.ptr,
                request_finished_listener.as_ptr(),
            );
        }
    }

    pub(crate) fn request_finished_executor_set(&mut self, request_finished_executor: Executor) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_executor_set(
                self.ptr,
                request_finished_executor.as_ptr(),
            );
        }
    }

    pub(crate) fn idempotency_set(&mut self, idempotency: Cronet_UrlRequestParams_IDEMPOTENCY) {
        unsafe {
            Cronet_UrlRequestParams_idempotency_set(self.ptr, idempotency);
        }
    }

    pub(crate) fn http_method_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_UrlRequestParams_http_method_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn request_headers_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_request_headers_size(self.ptr) }
    }

    pub(crate) fn request_headers_at(&self, index: u32) -> BorrowedHttpHeader {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_headers_at(self.ptr, index);
            assert!(!ptr.is_null());
            BorrowedHttpHeader::from_ptr(ptr)
        }
    }

    pub(crate) fn request_headers_clear(&self) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_clear(self.ptr);
        }
    }

    pub(crate) fn disable_cache_get(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_disable_cache_get(self.ptr) }
    }

    pub(crate) fn priority_get(&self) -> Cronet_UrlRequestParams_REQUEST_PRIORITY {
        unsafe { Cronet_UrlRequestParams_priority_get(self.ptr) }
    }

    pub(crate) fn upload_data_provider_get(&self) -> BorrowedUploadDataProvider {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_get(self.ptr);
            assert!(!ptr.is_null());
            BorrowedUploadDataProvider::from_ptr(ptr)
        }
    }

    pub(crate) fn upload_data_provider_executor_get(&self) -> BorrowedExecutor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_executor_get(self.ptr);
            assert!(!ptr.is_null());
            BorrowedExecutor::from_ptr(ptr)
        }
    }

    pub(crate) fn allow_direct_executor_get(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_allow_direct_executor_get(self.ptr) }
    }

    pub(crate) fn annotations_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_annotations_size(self.ptr) }
    }

    pub(crate) fn annotations_at(&self, index: u32) -> Cronet_RawDataPtr {
        unsafe { Cronet_UrlRequestParams_annotations_at(self.ptr, index) }
    }

    pub(crate) fn annotaions_clear(&self) {
        unsafe {
            Cronet_UrlRequestParams_annotations_clear(self.ptr);
        }
    }

    pub(crate) fn request_finished_listener_get(&self) -> BorrowedRequestFinishedInfoListener {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_listener_get(self.ptr);
            assert!(!ptr.is_null());
            BorrowedRequestFinishedInfoListener::from_ptr(ptr)
        }
    }

    pub(crate) fn request_finished_executor_get(&self) -> BorrowedExecutor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_executor_get(self.ptr);
            assert!(!ptr.is_null());
            BorrowedExecutor::from_ptr(ptr)
        }
    }

    pub(crate) fn idempotency_get(&self) -> Cronet_UrlRequestParams_IDEMPOTENCY {
        unsafe { Cronet_UrlRequestParams_idempotency_get(self.ptr) }
    }
}
