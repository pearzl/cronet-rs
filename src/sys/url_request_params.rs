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
    sys::request_finished_info_listener::RequestFinishedInfoListener,
    util::define_impl,
};

use super::{
    executor::Executor, http_header::HttpHeader, upload_data_provider::UploadDataProvider, Borrowed,
};

impl UrlRequestParams {
    pub(crate) fn as_ptr(&self) -> Cronet_UrlRequestParamsPtr {
        self.ptr
    }
}

define_impl! {
    UrlRequestParams, Cronet_UrlRequestParamsPtr, Cronet_UrlRequestParams_Destroy,


    fn http_method_set(&mut Self, http_method: &CStr >> CStr::as_ptr);
        Cronet_UrlRequestParams_http_method_set,
    fn http_method_get(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_UrlRequestParams_http_method_get,

    fn request_headers_add(&mut Self, element: &HttpHeader >> HttpHeader::as_ptr); // safety: cloned
        Cronet_UrlRequestParams_request_headers_add,
    fn request_headers_size(&Self) -> u32;
        Cronet_UrlRequestParams_request_headers_size,
    fn request_headers_at(&Self, index: u32) -> &HttpHeader >> HttpHeader::borrow_from_ptr; // safety: null -> None
        Cronet_UrlRequestParams_request_headers_at,
    fn request_headers_clear(&mut Self);
        Cronet_UrlRequestParams_request_headers_clear,

    fn disable_cache_set(&mut Self, disable_cache: bool);
        Cronet_UrlRequestParams_disable_cache_set,
    fn disable_cache_get(&Self) -> bool;
        Cronet_UrlRequestParams_disable_cache_get,

    fn priority_set(&mut Self, priority: Cronet_UrlRequestParams_REQUEST_PRIORITY);
        Cronet_UrlRequestParams_priority_set,
    fn priority_get(&Self) -> Cronet_UrlRequestParams_REQUEST_PRIORITY;
        Cronet_UrlRequestParams_priority_get,

    fn allow_direct_executor_set(&mut Self, allow_direct_executor: bool);
        Cronet_UrlRequestParams_allow_direct_executor_set,
    fn allow_direct_executor_get(&Self) -> bool;
        Cronet_UrlRequestParams_allow_direct_executor_get,

    fn annotations_add(&mut Self, element: Cronet_RawDataPtr);  // todo: no ptr
        Cronet_UrlRequestParams_annotations_add,
    fn annotations_size(&Self) -> u32;
        Cronet_UrlRequestParams_annotations_size,
    fn annotations_at(&Self, index: u32) -> Cronet_RawDataPtr;  // todo: no ptr
        Cronet_UrlRequestParams_annotations_at,
    fn annotaions_clear(&mut Self);
        Cronet_UrlRequestParams_annotations_clear,

    fn idempotency_set(&mut Self, idempotency: Cronet_UrlRequestParams_IDEMPOTENCY);
        Cronet_UrlRequestParams_idempotency_set,
    fn idempotency_get(&Self) -> Cronet_UrlRequestParams_IDEMPOTENCY;
        Cronet_UrlRequestParams_idempotency_get,
}

impl UrlRequestParams {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestParams_Create();
            Self { ptr }
        }
    }

    pub(crate) fn upload_data_provider_set<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>(
        &mut self,
        upload_data_provider: UploadDataProvider<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>,
    ) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_set(
                self.ptr,
                upload_data_provider.as_ptr(),
            );
        }
    }

    pub(crate) fn upload_data_provider_executor_set<ExecutorCtx>(
        &mut self,
        upload_data_provider_executor: Executor<ExecutorCtx>,
    ) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_executor_set(
                self.ptr,
                upload_data_provider_executor.as_ptr(),
            );
        }
    }

    pub(crate) fn request_finished_listener_set<Ctx>(
        &self,
        request_finished_listener: RequestFinishedInfoListener<Ctx>,
    ) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_listener_set(
                self.ptr,
                request_finished_listener.as_ptr(),
            );
        }
    }

    pub(crate) fn request_finished_executor_set<ExecutorCtx>(
        &mut self,
        request_finished_executor: Executor<ExecutorCtx>,
    ) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_executor_set(
                self.ptr,
                request_finished_executor.as_ptr(),
            );
        }
    }

    pub(crate) fn upload_data_provider_get<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>(
        &self,
    ) -> Borrowed<UploadDataProvider<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>> {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_get(self.ptr);
            assert!(!ptr.is_null());
            UploadDataProvider::borrow_from(ptr, self)
        }
    }

    pub(crate) fn upload_data_provider_executor_get<ExecutorCtx>(
        &self,
    ) -> Borrowed<Executor<ExecutorCtx>> {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_executor_get(self.ptr);
            assert!(!ptr.is_null());
            Executor::borrow_from(ptr, self)
        }
    }

    pub(crate) fn request_finished_listener_get<Ctx>(
        &self,
    ) -> Borrowed<RequestFinishedInfoListener<Ctx>> {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_listener_get(self.ptr);
            assert!(!ptr.is_null());
            RequestFinishedInfoListener::borrow_from(ptr, self)
        }
    }

    pub(crate) fn request_finished_executor_get<EngineContext>(
        &self,
    ) -> Borrowed<Executor<EngineContext>> {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_executor_get(self.ptr);
            assert!(!ptr.is_null());
            Executor::borrow_from(ptr, self)
        }
    }
}
