use std::ffi::CStr;

use futures::{channel::mpsc, executor::LocalPool, StreamExt};
use http::{Request, Response};

use crate::{
    bindings::{Cronet_EngineParams_HTTP_CACHE_MODE, Cronet_RESULT},
    body::Body,
    error::Error,
    sys::{Engine, EngineParams, ExecuteExt, Executor, Runnable},
    util::RunAsyncFunc,
};

pub struct Client {
    pub(crate) engine: Engine<EngineContext>,
    pub(crate) run_async: crate::util::RunAsyncFunc,
    pub(crate) executor: Executor<ExecutorContext>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            engine_params: EngineParams::create(),
        }
    }

    pub async fn fetch(&self, req: Request<Body>) -> Result<Response<Body>, Error> {
        crate::fetch::send(self, req).await
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
    pub fn construct(self, run_async: RunAsyncFunc) -> Result<Client, Cronet_RESULT> {
        let engine = Engine::create();
        let ret = engine.start_with_params(&self.engine_params);
        if ret != Cronet_RESULT::SUCCESS {
            return Err(ret);
        }

        let executor = Executor::new(ExecutorContext::new());

        Ok(Client {
            engine,
            run_async,
            executor,
        })
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

pub(crate) struct ExecutorContext {
    command_tx: mpsc::UnboundedSender<Runnable<RunnableContext>>,
}

impl ExecutorContext {
    fn new() -> Self {
        let (tx, rx) = mpsc::unbounded::<Runnable<RunnableContext>>();
        std::thread::Builder::new()
            .name("cronet-rs: execute runnable".into())
            .spawn(move || {
                let mut pool = LocalPool::new();
                pool.run_until(Self::run_runnable_loop(rx));
            })
            .unwrap();
        Self { command_tx: tx }
    }

    async fn run_runnable_loop(mut rx: mpsc::UnboundedReceiver<Runnable<RunnableContext>>) {
        while let Some(runnable) = rx.next().await {
            runnable.run();
        }
    }
}

impl ExecuteExt<ExecutorContext> for ExecutorContext {
    type RunnableCtx = RunnableContext;

    fn execute_func() -> crate::sys::ExecuteFunc<ExecutorContext, Self::RunnableCtx> {
        |executor, command| {
            let ctx = executor.get_client_context();
            let ret = ctx.command_tx.unbounded_send(command);
            debug_assert!(ret.is_ok())
        }
    }
}

pub(crate) struct RunnableContext {}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use futures::executor::ThreadPool;
    use futures_scopes::{relay::new_relay_scope, ScopedSpawnExt, SpawnScope};
    use http::Method;

    use super::*;

    #[test]
    fn new_client() {
        env_logger::init();

        let pool = ThreadPool::new().unwrap();
        let client = Client::builder()
            .construct(Arc::new(move |fut|{
                let scope = new_relay_scope!();
                scope.relay_to(&pool).unwrap();
                scope.spawner().spawn_scoped(fut).unwrap();
                let fut = scope.until_empty();
                std::thread::sleep(std::time::Duration::from_millis(1)); // have no idea, but works!
                pool.spawn_ok(fut);
            }))
            .unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .uri("http://www.rust-lang.org/")
            .body(Body::empty())
            .unwrap();

        let mut pool = LocalPool::new();
        let mut resp = pool.run_until(async {client.fetch(req).await}).unwrap();
        log::debug!("{:?}\n{:#?}", resp.status(), resp.headers());
        let body = pool.run_until(async{
            let mut body_buf = vec![];
            while let Some(data) = resp.body_mut().next().await {
                let data = data.unwrap();
                log::debug!("get body: {:?}", data.len());
                body_buf.extend_from_slice(&data);
            }
            body_buf
        });
        log::trace!("{:?}", String::from_utf8(body));
    }
}
