use crate::bindings::{
    Cronet_RawDataPtr, Cronet_RequestFinishedInfoPtr, Cronet_RequestFinishedInfo_Create,
    Cronet_RequestFinishedInfo_Destroy, Cronet_RequestFinishedInfo_FINISHED_REASON,
    Cronet_RequestFinishedInfo_annotations_add, Cronet_RequestFinishedInfo_annotations_at,
    Cronet_RequestFinishedInfo_annotations_clear, Cronet_RequestFinishedInfo_annotations_size,
    Cronet_RequestFinishedInfo_finished_reason_get, Cronet_RequestFinishedInfo_finished_reason_set,
    Cronet_RequestFinishedInfo_metrics_get, Cronet_RequestFinishedInfo_metrics_move,
    Cronet_RequestFinishedInfo_metrics_set,
};

use super::metrics::{BorrowedMetrics, Metrics};

pub struct RequestFinishedInfo {
    ptr: Cronet_RequestFinishedInfoPtr,
}

impl Drop for RequestFinishedInfo {
    fn drop(&mut self) {
        unsafe { Cronet_RequestFinishedInfo_Destroy(self.ptr) }
    }
}

impl RequestFinishedInfo {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfo_Create();
            RequestFinishedInfo { ptr }
        }
    }

    pub fn metrics_set(&mut self, metrics: Metrics) {
        unsafe {
            Cronet_RequestFinishedInfo_metrics_set(self.ptr, metrics.as_ptr());
        }
    }

    pub fn metrics_move(&self, metrics: Metrics) {
        unsafe {
            Cronet_RequestFinishedInfo_metrics_move(self.ptr, metrics.as_ptr());
        }
    }

    pub fn annotations_add(&self, element: Cronet_RawDataPtr) {
        unsafe {
            Cronet_RequestFinishedInfo_annotations_add(self.ptr, element);
        }
    }

    pub fn finished_reason_set(
        &mut self,
        finished_reason: Cronet_RequestFinishedInfo_FINISHED_REASON,
    ) {
        unsafe {
            Cronet_RequestFinishedInfo_finished_reason_set(self.ptr, finished_reason);
        }
    }

    pub fn metrics_get(&self) -> BorrowedMetrics {
        unsafe {
            let ptr = Cronet_RequestFinishedInfo_metrics_get(self.ptr);
            assert!(!ptr.is_null());
            BorrowedMetrics::from_ptr(ptr)
        }
    }

    pub fn annotations_size(&self) -> u32 {
        unsafe { Cronet_RequestFinishedInfo_annotations_size(self.ptr) }
    }

    pub fn annotations_at(&self, index: u32) -> Cronet_RawDataPtr {
        unsafe { Cronet_RequestFinishedInfo_annotations_at(self.ptr, index) }
    }

    pub fn annotations_clear(&self) {
        unsafe {
            Cronet_RequestFinishedInfo_annotations_clear(self.ptr);
        }
    }

    pub fn finished_reason_get(&self) -> Cronet_RequestFinishedInfo_FINISHED_REASON {
        unsafe { Cronet_RequestFinishedInfo_finished_reason_get(self.ptr) }
    }
}
