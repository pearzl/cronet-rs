use std::{future::Future, pin::Pin};

pub(crate) type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

macro_rules! impl_client_context {
    (
        $struct_name: tt,
        $get_func: ident,
        $set_func: ident,
    ) => {
        impl<Ctx> $struct_name<Ctx> {
            pub(crate) fn get_client_context(&self) -> crate::sys::Borrowed<Ctx> {
                let void_ptr = unsafe { $get_func(self.ptr) };
                let ctx_ptr = void_ptr as *mut Ctx;
                crate::sys::Borrowed::new(ctx_ptr, self)
            }

            pub(crate) fn set_client_context(&mut self, client_context: Ctx) {
                let ptr = Box::into_raw(Box::new(client_context));
                unsafe { $set_func(self.ptr, ptr as Cronet_ClientContext) }
            }
        }
    };
}

pub(crate) use impl_client_context;

macro_rules! define_impl {
    (
        $struct_name: tt, $ptr: ty,
        $(
            with_ctx: $ctx: tt,
            get: $get_func: ident , $cronet_get: ident,
            set: $set_func: ident , $cronet_set: ident,
        )?
    ) => {
        pub struct $struct_name $(<$ctx>)? {
            ptr: $ptr,
            $(ctx: Option<$ctx>)?
        }

        // impl ctx
        $(
            impl <$ctx> $struct_name <$ctx> {
                pub(crate) fn $get_func(&self) -> crate::sys::Borrowed<$ctx> {
                    let void_ptr = unsafe { $cronet_get(self.ptr) };
                    let ctx_ptr = void_ptr as *mut Ctx;
                    crate::sys::Borrowed::new(ctx_ptr, self)
                }
                pub(crate) fn $set_func(&mut self, mut client_context: $ctx) {
                    let ptr = &mut client_context as *mut $ctx;
                    let _ = self.ctx.replace(client_context);
                    unsafe { $cronet_set(self.ptr, ptr as crate::bindings::Cronet_ClientContext) }
                }
            }
        )?
    };
}
pub(crate) use define_impl;
