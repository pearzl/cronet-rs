use std::{ffi::CStr, marker::PhantomData};

use crate::bindings::{
    Cronet_ClientContext, Cronet_EnginePtr, Cronet_Engine_AddRequestFinishedListener,
    Cronet_Engine_AddRequestFinishedListenerFunc, Cronet_Engine_Create, Cronet_Engine_CreateWith,
    Cronet_Engine_Destroy, Cronet_Engine_GetClientContext, Cronet_Engine_GetDefaultUserAgent,
    Cronet_Engine_GetDefaultUserAgentFunc, Cronet_Engine_GetVersionString,
    Cronet_Engine_GetVersionStringFunc, Cronet_Engine_RemoveRequestFinishedListener,
    Cronet_Engine_RemoveRequestFinishedListenerFunc, Cronet_Engine_SetClientContext,
    Cronet_Engine_Shutdown, Cronet_Engine_ShutdownFunc, Cronet_Engine_StartNetLogToFile,
    Cronet_Engine_StartNetLogToFileFunc, Cronet_Engine_StartWithParams,
    Cronet_Engine_StartWithParamsFunc, Cronet_Engine_StopNetLog, Cronet_Engine_StopNetLogFunc,
    Cronet_RESULT,
};

use super::{
    engine_params::EngineParams, executor::Executor,
    request_finished_info_listener::RequestFinishedInfoListener, Borrowed,
};

pub(crate) struct Engine<Ctx> {
    ptr: Cronet_EnginePtr,
    _phan: PhantomData<Ctx>,
}

impl<Ctx> Engine<Ctx> {
    pub(crate) fn as_ptr(&self) -> Cronet_EnginePtr {
        self.ptr
    }
}

impl<Ctx> Engine<Ctx> {
    pub(crate) fn create() -> Self {
        unsafe {
            Engine {
                ptr: Cronet_Engine_Create(),
                _phan: PhantomData,
            }
        }
    }

    pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
        let ptr = Box::into_raw(Box::new(client_context));
        unsafe { Cronet_Engine_SetClientContext(self.ptr, ptr as Cronet_ClientContext) }
    }

    pub(crate) fn get_client_context(&self) -> Borrowed<Ctx> {
        let void_ptr = unsafe { Cronet_Engine_GetClientContext(self.ptr) };
        let ctx_ptr = void_ptr as *mut Ctx;
        Borrowed::new(ctx_ptr, self)
    }

    #[must_use]
    pub(crate) fn start_with_params(&self, params: &EngineParams) -> Cronet_RESULT {
        unsafe { Cronet_Engine_StartWithParams(self.ptr, params.as_ptr()) }
    }

    pub(crate) fn start_net_log_to_file(&self, file_name: &CStr, log_all: bool) -> bool {
        unsafe { Cronet_Engine_StartNetLogToFile(self.ptr, file_name.as_ptr(), log_all) }
    }

    pub(crate) fn stop_net_log(&self) {
        unsafe {
            Cronet_Engine_StopNetLog(self.ptr);
        }
    }

    pub(crate) fn shutdown(&self) -> Cronet_RESULT {
        unsafe { Cronet_Engine_Shutdown(self.ptr) }
    }

    pub(crate) fn get_version_string(&self) -> &CStr {
        unsafe {
            let v = Cronet_Engine_GetVersionString(self.ptr);
            CStr::from_ptr(v)
        }
    }

    pub(crate) fn get_default_user_agent(&self) -> &CStr {
        unsafe {
            let v = Cronet_Engine_GetDefaultUserAgent(self.ptr);
            CStr::from_ptr(v)
        }
    }

    pub(crate) fn add_request_finished_listener<ListenerCtx>(
        &self,
        listener: RequestFinishedInfoListener<ListenerCtx>,
        executor: Executor,
    ) {
        unsafe {
            Cronet_Engine_AddRequestFinishedListener(
                self.ptr,
                listener.as_ptr(),
                executor.as_ptr(),
            );
        }
    }

    pub(crate) fn remove_request_finished_listener<ListenerCtx>(
        &self,
        listener: &RequestFinishedInfoListener<ListenerCtx>,
    ) {
        unsafe { Cronet_Engine_RemoveRequestFinishedListener(self.ptr, listener.as_ptr()) }
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
                _phan: PhantomData,
            }
        }
    }
}

impl<Ctx> Drop for Engine<Ctx> {
    fn drop(&mut self) {
        let ctx_ptr = self.get_client_context().inner;
        if !ctx_ptr.is_null() {
            let _ = unsafe { Box::from_raw(ctx_ptr) };
        }
        unsafe { Cronet_Engine_Destroy(self.ptr) }
    }
}
