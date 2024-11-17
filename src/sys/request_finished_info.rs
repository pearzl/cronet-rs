use crate::{
    bindings::{
        Cronet_RawDataPtr, Cronet_RequestFinishedInfoPtr, Cronet_RequestFinishedInfo_Create,
        Cronet_RequestFinishedInfo_Destroy, Cronet_RequestFinishedInfo_FINISHED_REASON,
        Cronet_RequestFinishedInfo_annotations_add, Cronet_RequestFinishedInfo_annotations_at,
        Cronet_RequestFinishedInfo_annotations_clear, Cronet_RequestFinishedInfo_annotations_size,
        Cronet_RequestFinishedInfo_finished_reason_get,
        Cronet_RequestFinishedInfo_finished_reason_set, Cronet_RequestFinishedInfo_metrics_get,
        Cronet_RequestFinishedInfo_metrics_move, Cronet_RequestFinishedInfo_metrics_set,
    },
    util::define_impl,
};

use super::{metrics::Metrics, Borrowed};

impl<'a> RequestFinishedInfo {
    pub(crate) unsafe fn borrow_from_ptr(
        ptr: Cronet_RequestFinishedInfoPtr,
    ) -> &'a mut RequestFinishedInfo {
        let self_ = RequestFinishedInfo { ptr };
        let self_ = Box::into_raw(Box::new(self_));
        &mut *self_
    }
}

define_impl! {
    RequestFinishedInfo, Cronet_RequestFinishedInfoPtr, Cronet_RequestFinishedInfo_Destroy,


    fn metrics_set(&mut Self, metrics: &Metrics >> Metrics::as_ptr); // safety: cloned
        Cronet_RequestFinishedInfo_metrics_set,
    fn metrics_move(&Self, metrics: Metrics >> Metrics::into_raw);  // safety: moved
        Cronet_RequestFinishedInfo_metrics_move,
    fn metrics_get(&Self) -> Option<&mut Metrics> >> Metrics::borrow_from_ptr; // safety: null -> None
        Cronet_RequestFinishedInfo_metrics_get,

    fn annotations_add(&mut Self, element: Cronet_RawDataPtr);
        Cronet_RequestFinishedInfo_annotations_add,
    fn annotations_size(&Self) -> u32;
        Cronet_RequestFinishedInfo_annotations_size,
    fn annotations_at(&Self, index: u32) -> Cronet_RawDataPtr;  // todo: not return ptr
        Cronet_RequestFinishedInfo_annotations_at,
    fn annotations_clear(&mut Self);
        Cronet_RequestFinishedInfo_annotations_clear,

    fn finished_reason_set(&mut Self, finished_reason: Cronet_RequestFinishedInfo_FINISHED_REASON);
        Cronet_RequestFinishedInfo_finished_reason_set,
    fn finished_reason_get(&Self) -> Cronet_RequestFinishedInfo_FINISHED_REASON;
        Cronet_RequestFinishedInfo_finished_reason_get,
}

impl RequestFinishedInfo {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfo_Create();
            RequestFinishedInfo { ptr }
        }
    }
}
