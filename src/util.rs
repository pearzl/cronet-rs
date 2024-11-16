use std::{future::Future, pin::Pin};

pub(crate) type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

macro_rules! define_impl {
    (
        $struct_name: tt, $ptr: ty, $drop_fn: ident,
        $(
            with_ctx: $ctx: tt,
            get:  $cronet_get: ident,
            set:  $cronet_set: ident,
        )?
    ) => {
        pub struct $struct_name $(<$ctx>)? {
            ptr: $ptr,
            $(ctx: Option<$ctx>)?
        }

        impl $(<$ctx>)? Drop for $struct_name $(<$ctx>)? {
            fn drop(&mut self) {
                unsafe { $drop_fn(self.ptr) }
            }
        }
        

        // impl ctx
        $(
            impl <$ctx> $struct_name <$ctx> {
                pub(crate) fn get_client_context(&self) -> crate::sys::Borrowed<$ctx> {
                    let void_ptr = unsafe { $cronet_get(self.ptr) };
                    let ctx_ptr = void_ptr as *mut Ctx;
                    crate::sys::Borrowed::new(ctx_ptr, self)
                }
                pub(crate) fn set_client_context(&mut self, mut client_context: $ctx) {
                    let ptr = &mut client_context as *mut $ctx;
                    let _ = self.ctx.replace(client_context);
                    unsafe { $cronet_set(self.ptr, ptr as crate::bindings::Cronet_ClientContext) }
                }
            }
        )?
    };
}
pub(crate) use define_impl;
