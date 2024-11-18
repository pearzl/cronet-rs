use std::{ffi::CStr, marker::PhantomData};

use crate::{
    bindings::{
        Cronet_ClientContext, Cronet_EnginePtr, Cronet_Engine_AddRequestFinishedListener,
        Cronet_Engine_AddRequestFinishedListenerFunc, Cronet_Engine_Create,
        Cronet_Engine_CreateWith, Cronet_Engine_Destroy, Cronet_Engine_GetClientContext,
        Cronet_Engine_GetDefaultUserAgent, Cronet_Engine_GetDefaultUserAgentFunc,
        Cronet_Engine_GetVersionString, Cronet_Engine_GetVersionStringFunc,
        Cronet_Engine_RemoveRequestFinishedListener,
        Cronet_Engine_RemoveRequestFinishedListenerFunc, Cronet_Engine_SetClientContext,
        Cronet_Engine_Shutdown, Cronet_Engine_ShutdownFunc, Cronet_Engine_StartNetLogToFile,
        Cronet_Engine_StartNetLogToFileFunc, Cronet_Engine_StartWithParams,
        Cronet_Engine_StartWithParamsFunc, Cronet_Engine_StopNetLog, Cronet_Engine_StopNetLogFunc,
        Cronet_RESULT,
    },
    util::define_impl,
};

use super::{
    engine_params::EngineParams, executor::Executor,
    request_finished_info_listener::RequestFinishedInfoListener,
};

impl<Ctx> Engine<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            Engine {
                ptr: Cronet_Engine_Create(),
                _ctx: PhantomData,
                
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_with(
        start_with_params_func: Cronet_Engine_StartWithParamsFunc,
        start_net_log_to_file_func: Cronet_Engine_StartNetLogToFileFunc,
        stop_net_log_func: Cronet_Engine_StopNetLogFunc,
        shutdown_func: Cronet_Engine_ShutdownFunc,
        get_version_string_func: Cronet_Engine_GetVersionStringFunc,
        get_default_user_agent_func: Cronet_Engine_GetDefaultUserAgentFunc,
        add_request_finished_listener_func: Cronet_Engine_AddRequestFinishedListenerFunc,
        remove_request_finished_listener_func: Cronet_Engine_RemoveRequestFinishedListenerFunc,
    ) -> Self {
        unsafe {
            let ptr = Cronet_Engine_CreateWith(
                start_with_params_func,
                start_net_log_to_file_func,
                stop_net_log_func,
                shutdown_func,
                get_version_string_func,
                get_default_user_agent_func,
                add_request_finished_listener_func,
                remove_request_finished_listener_func,
            );
            Engine {
                ptr,
                _ctx: PhantomData,
                
            }
        }
    }
}

define_impl! {
    Engine, Cronet_EnginePtr, Cronet_Engine_Destroy,

    fn add_request_finished_listener<ListenerCtx, ExecutorCtx>(
        &Self,
        listener: &RequestFinishedInfoListener<ListenerCtx> >> RequestFinishedInfoListener::as_ptr, // safety: pass ref?
        executor: &Executor<ExecutorCtx> >> Executor::as_ptr   // safety:: pass ref?
    );Cronet_Engine_AddRequestFinishedListener,

    fn remove_request_finished_listener<ListenerCtx>(
        &Self,
        listener: &RequestFinishedInfoListener<ListenerCtx> >> RequestFinishedInfoListener::as_ptr // safety:: pass ref?
    ); Cronet_Engine_RemoveRequestFinishedListener,

    fn start_with_params(&Self, params: &EngineParams >> EngineParams::as_ptr) -> Cronet_RESULT; // safety:: pass ref?
        Cronet_Engine_StartWithParams,

    fn start_net_log_to_file(&Self, file_name: &CStr >> CStr::as_ptr, log_all: bool) -> bool;
        Cronet_Engine_StartNetLogToFile,

    fn stop_net_log(&Self);
        Cronet_Engine_StopNetLog,

    fn shutdown(&Self) -> Cronet_RESULT;
        Cronet_Engine_Shutdown,

    fn get_version_string(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_Engine_GetVersionString,

    fn get_default_user_agent(&Self) -> &CStr >> CStr::from_ptr;
        Cronet_Engine_GetDefaultUserAgent,


    with_ctx: <Ctx>,
    get: Cronet_Engine_GetClientContext,
    set: Cronet_Engine_SetClientContext,
}
