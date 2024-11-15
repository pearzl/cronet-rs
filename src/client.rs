use std::ffi::CStr;

use crate::{
    bindings::{Cronet_EngineParams_HTTP_CACHE_MODE, Cronet_RESULT},
    sys::{Engine, EngineParams},
};

pub struct Client {
    engine: Engine<EngineContext>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            engine_params: EngineParams::create(),
        }
    }

    pub fn start_net_log_to_file(&self, file_name: &CStr, log_all: bool) -> bool {
        self.engine.start_net_log_to_file(file_name, log_all)
    }

    pub fn stop_net_log(&self) {
        self.engine.stop_net_log();
    }

    pub fn shutdown(&self) -> Cronet_RESULT {
        self.engine.shutdown()
    }

    pub fn get_version_string(&self) -> &CStr {
        self.engine.get_version_string()
    }

    pub fn get_default_user_agent(&self) -> &CStr {
        self.engine.get_default_user_agent()
    }
}

pub struct ClientBuilder {
    engine_params: EngineParams,
}

impl ClientBuilder {
    pub fn construct(self) -> Result<Client, Cronet_RESULT> {
        let engine = Engine::create();
        let ret = engine.start_with_params(&self.engine_params);
        if ret != Cronet_RESULT::SUCCESS {
            return Err(ret);
        }
        Ok(Client { engine })
    }

    pub fn enable_check_result_set(mut self, enable_check_result: bool) -> Self {
        self.engine_params
            .enable_check_result_set(enable_check_result);
        self
    }

    pub fn user_agent_set(mut self, user_agent: &CStr) -> Self {
        self.engine_params.user_agent_set(user_agent);
        self
    }

    pub fn accept_language_set(mut self, accept_language: &CStr) -> Self {
        self.engine_params.accept_language_set(accept_language);
        self
    }

    pub fn storage_path_set(mut self, storage_path: &CStr) -> Self {
        self.engine_params.storage_path_set(storage_path);
        self
    }

    pub fn enable_quic_set(mut self, enable_quic: bool) -> Self {
        self.engine_params.enable_quic_set(enable_quic);
        self
    }

    pub fn enable_http2_set(mut self, enable_http2: bool) -> Self {
        self.engine_params.enable_http2_set(enable_http2);
        self
    }
    pub fn enable_brotli_set(mut self, enable_brotli: bool) -> Self {
        self.engine_params.enable_brotli_set(enable_brotli);
        self
    }
    pub fn http_cache_mode_set(
        mut self,
        http_cache_mode: Cronet_EngineParams_HTTP_CACHE_MODE,
    ) -> Self {
        self.engine_params.http_cache_mode_set(http_cache_mode);
        self
    }
    pub fn http_cache_max_size_set(mut self, http_cache_max_size: i64) -> Self {
        self.engine_params
            .http_cache_max_size_set(http_cache_max_size);
        self
    }
    // pub fn quic_hint_add(&self, element: &QuicHint) {
    // pub fn public_key_pins_add(&self, element: &PublicKeyPins) {
    pub fn enable_public_key_pinning_bypass_for_local_trust_anchors_set(
        mut self,
        enable_public_key_pinning_bypass_for_local_trust_anchors: bool,
    ) -> Self {
        self.engine_params
            .enable_public_key_pinning_bypass_for_local_trust_anchors_set(
                enable_public_key_pinning_bypass_for_local_trust_anchors,
            );
        self
    }
    pub fn network_thread_priority_set(mut self, network_thread_priority: f64) -> Self {
        self.engine_params
            .network_thread_priority_set(network_thread_priority);
        self
    }
    pub fn experimental_options_set(mut self, experimental_options: &CStr) -> Self {
        self.engine_params
            .experimental_options_set(experimental_options);
        self
    }
}

pub(crate) struct EngineContext {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_client() {
        let _client = Client::builder().construct().unwrap();
    }
}
