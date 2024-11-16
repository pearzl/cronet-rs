use std::ffi::CStr;

use crate::{
    bindings::{
        Cronet_EngineParamsPtr, Cronet_EngineParams_Create, Cronet_EngineParams_Destroy,
        Cronet_EngineParams_HTTP_CACHE_MODE, Cronet_EngineParams_accept_language_get,
        Cronet_EngineParams_accept_language_set, Cronet_EngineParams_enable_brotli_get,
        Cronet_EngineParams_enable_brotli_set, Cronet_EngineParams_enable_check_result_get,
        Cronet_EngineParams_enable_check_result_set, Cronet_EngineParams_enable_http2_get,
        Cronet_EngineParams_enable_http2_set,
        Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_get,
        Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_set,
        Cronet_EngineParams_enable_quic_get, Cronet_EngineParams_enable_quic_set,
        Cronet_EngineParams_experimental_options_get, Cronet_EngineParams_experimental_options_set,
        Cronet_EngineParams_http_cache_max_size_get, Cronet_EngineParams_http_cache_max_size_set,
        Cronet_EngineParams_http_cache_mode_get, Cronet_EngineParams_http_cache_mode_set,
        Cronet_EngineParams_network_thread_priority_get,
        Cronet_EngineParams_network_thread_priority_set, Cronet_EngineParams_public_key_pins_add,
        Cronet_EngineParams_public_key_pins_at, Cronet_EngineParams_public_key_pins_clear,
        Cronet_EngineParams_public_key_pins_size, Cronet_EngineParams_quic_hints_add,
        Cronet_EngineParams_quic_hints_at, Cronet_EngineParams_quic_hints_clear,
        Cronet_EngineParams_quic_hints_size, Cronet_EngineParams_storage_path_get,
        Cronet_EngineParams_storage_path_set, Cronet_EngineParams_user_agent_get,
        Cronet_EngineParams_user_agent_set,
    },
    util::define_impl,
};

use super::{public_key_pins::PublicKeyPins, quic_hint::QuicHint, Borrowed};

impl EngineParams {
    pub(crate) fn as_ptr(&self) -> Cronet_EngineParamsPtr {
        self.ptr
    }
}

impl EngineParams {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_EngineParams_Create();
            Self { ptr }
        }
    }

    pub(crate) fn user_agent_set(&mut self, user_agent: &CStr) {
        unsafe {
            Cronet_EngineParams_user_agent_set(self.ptr, user_agent.as_ptr());
        }
    }

    pub(crate) fn accept_language_set(&mut self, accept_language: &CStr) {
        unsafe {
            Cronet_EngineParams_accept_language_set(self.ptr, accept_language.as_ptr());
        }
    }

    pub(crate) fn storage_path_set(&mut self, storage_path: &CStr) {
        unsafe {
            Cronet_EngineParams_storage_path_set(self.ptr, storage_path.as_ptr());
        }
    }

    pub(crate) fn quic_hint_add(&self, element: &QuicHint) {
        unsafe {
            Cronet_EngineParams_quic_hints_add(self.ptr, element.as_ptr());
        }
    }

    pub(crate) fn public_key_pins_add(&self, element: &PublicKeyPins) {
        unsafe {
            Cronet_EngineParams_public_key_pins_add(self.ptr, element.as_ptr());
        }
    }

    pub(crate) fn experimental_options_set(&mut self, experimental_options: &CStr) {
        unsafe {
            Cronet_EngineParams_experimental_options_set(self.ptr, experimental_options.as_ptr());
        }
    }

    pub(crate) fn user_agent_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_EngineParams_user_agent_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn accept_language_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_EngineParams_accept_language_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn storage_path_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_EngineParams_storage_path_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }

    pub(crate) fn quic_hints_at(&self, index: u32) -> Borrowed<QuicHint> {
        unsafe {
            let ptr = Cronet_EngineParams_quic_hints_at(self.ptr, index);
            assert!(!ptr.is_null());
            QuicHint::borrow_from(ptr, self)
        }
    }

    pub(crate) fn public_key_pins_at(&self, index: u32) -> Borrowed<PublicKeyPins> {
        unsafe {
            let ptr = Cronet_EngineParams_public_key_pins_at(self.ptr, index);
            assert!(!ptr.is_null());
            PublicKeyPins::borrow_from(ptr, self)
        }
    }

    pub(crate) fn experimental_options_get(&self) -> &CStr {
        unsafe {
            let ptr = Cronet_EngineParams_experimental_options_get(self.ptr);
            CStr::from_ptr(ptr)
        }
    }
}

define_impl! {
    EngineParams, Cronet_EngineParamsPtr, Cronet_EngineParams_Destroy,

    fn enable_check_result_set(&mut Self, enable_check_result: bool);   
        Cronet_EngineParams_enable_check_result_set,
    fn enable_check_result_get(&Self) -> bool ; 
        Cronet_EngineParams_enable_check_result_get,

    fn enable_quic_set(&mut Self, enable_quic: bool);
        Cronet_EngineParams_enable_quic_set,

    fn enable_http2_set(&mut Self, enable_http2: bool);
        Cronet_EngineParams_enable_http2_set,

    fn enable_brotli_set(&mut Self, enable_brotli: bool);
        Cronet_EngineParams_enable_brotli_set,

    fn http_cache_mode_set(&mut Self,http_cache_mode: Cronet_EngineParams_HTTP_CACHE_MODE);
        Cronet_EngineParams_http_cache_mode_set,

    fn http_cache_max_size_set(&mut Self, http_cache_max_size: i64);
        Cronet_EngineParams_http_cache_max_size_set,

    fn enable_public_key_pinning_bypass_for_local_trust_anchors_set(&mut Self, enable_public_key_pinning_bypass_for_local_trust_anchors: bool);
        Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_set,

    fn network_thread_priority_set(&mut Self, network_thread_priority: f64);
        Cronet_EngineParams_network_thread_priority_set,

    fn quic_hints_clear(&mut Self);
        Cronet_EngineParams_quic_hints_clear,

    fn public_key_pins_size(&Self) -> u32;
        Cronet_EngineParams_public_key_pins_size,

    fn public_key_pins_clear(&Self);
        Cronet_EngineParams_public_key_pins_clear,

    fn enable_public_key_pinning_bypass_for_local_trust_anchors_get(&Self) -> bool ;
        Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_get,

    fn network_thread_priority_get(&Self) -> f64;
        Cronet_EngineParams_network_thread_priority_get,
}
