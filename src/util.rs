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
