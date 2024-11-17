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
    executor::Executor, http_header::HttpHeader, upload_data_provider::UploadDataProvider,
};

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
    fn request_headers_at(&Self, index: u32) -> &HttpHeader >> HttpHeader::from_ptr; // safety: out of bounds
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

    fn upload_data_provider_set<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>(
        &mut Self,
        upload_data_provider: &UploadDataProvider<UploadDateProviderCtx>
            >> UploadDataProvider::as_ptr // safey: pass ref?
    ); Cronet_UrlRequestParams_upload_data_provider_set,
    fn upload_data_provider_get<UploadDateProviderCtx, UploadDataSinkCtx, BufferCtx>(&Self)
    -> &UploadDataProvider<UploadDateProviderCtx> >> UploadDataProvider::from_ptr;
        Cronet_UrlRequestParams_upload_data_provider_get,

    fn upload_data_provider_executor_set<ExecutorCtx, RunnableCtx>(
        &mut Self,
        upload_data_provider_executor: &Executor<ExecutorCtx, RunnableCtx> >> Executor::as_ptr   // safety: pass ref?
    );Cronet_UrlRequestParams_upload_data_provider_executor_set,
    fn upload_data_provider_executor_get<ExecutorCtx, RunnableCtx>(&Self) -> &Executor<ExecutorCtx, RunnableCtx> >> Executor::from_ptr;
        Cronet_UrlRequestParams_upload_data_provider_executor_get,

    fn request_finished_listener_set<Ctx>(
        &Self,
        request_finished_listener: &RequestFinishedInfoListener<Ctx> >> RequestFinishedInfoListener::as_ptr // safety::pass_ref?
    );Cronet_UrlRequestParams_request_finished_listener_set,
    fn request_finished_listener_get<Ctx>(&Self) -> &RequestFinishedInfoListener<Ctx>
        >> RequestFinishedInfoListener::from_ptr;
        Cronet_UrlRequestParams_request_finished_listener_get,

    fn request_finished_executor_set<ExecutorCtx, RunnableCtx>(
        &mut Self,
        request_finished_executor: &Executor<ExecutorCtx, RunnableCtx> >> Executor::as_ptr // safety::pass_ref?
    ) ; Cronet_UrlRequestParams_request_finished_executor_set,
    fn request_finished_executor_get<EngineContext, RunnableCtx>( &Self ) -> &Executor<EngineContext, RunnableCtx>
        >> Executor::from_ptr;
        Cronet_UrlRequestParams_request_finished_executor_get,

}

impl UrlRequestParams {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestParams_Create();
            Self { ptr }
        }
    }
}
