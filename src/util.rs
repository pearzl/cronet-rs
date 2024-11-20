use std::{future::Future, ops::{Deref, DerefMut}, pin::Pin, sync::Arc};

pub(crate) type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;
pub(crate) type RunAsyncFunc = Arc<dyn Fn(BoxedFuture<()>) + Send + Sync + 'static>;

pub(crate) struct Borrowed<T> {
    inner: *mut T
}

impl<T> Borrowed<T> {
    pub(crate) fn new(ptr: *mut T) -> Self{
        Self { inner: ptr }
    }

}

impl<T> Deref for Borrowed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {&*self.inner}
    }
}

impl<T> DerefMut for Borrowed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.inner}
    }
}

unsafe impl<T> Send for Borrowed<T> where T: Send{}
unsafe impl<T> Sync for Borrowed<T> where T: Sync{}

macro_rules! define_impl {
    (
        $struct_name: tt $(<$($gens: tt),*>)?, $ptr: ty, $drop_fn: ident,

        $(
            $(#[$attr: meta])*
            fn $func_name: ident $(<$($gen_param:tt),*>)? ($self_: ty $(,$arg_name: ident : $arg_type: ty $(>> $arg_trans_func: path)?)* )
            $(-> $return_type: ty $(>> $return_trans_func: path)? )? ;
            $cronet_func: ident,
        )*

        $(
            with_ctx: <$ctx: tt>,
            get:  $cronet_get: ident,
            set:  $cronet_set: ident,
        )?
    ) => {
        // define
        pub struct $struct_name $(<$ctx>)? {
            ptr: $ptr,
            $(_ctx: std::marker::PhantomData<$ctx>,)?
        }

        // impl drop
        impl $(<$ctx>)? Drop for $struct_name $(<$ctx>)? {
            fn drop(&mut self) {
                $(unsafe{
                    let void_ptr = $cronet_get(self.ptr);
                    if !void_ptr.is_null() {
                        let ctx = Box::from_raw(void_ptr as *mut $ctx);
                        drop(ctx);
                    }
                })?

                unsafe { $drop_fn(self.ptr) }
            }
        }

        // impl common
        impl $(<$ctx>)? $struct_name $(<$ctx>)? {
            pub(crate) unsafe fn borrow_from(ptr: $ptr) -> crate::util::Borrowed<Self> {
                assert!(!ptr.is_null());
                let borrowed = $struct_name { ptr, $(_ctx: PhantomData::<$ctx>)?};
                let ptr = Box::into_raw(Box::new(borrowed));
                crate::util::Borrowed::new(ptr)
            }
            pub(crate) unsafe fn from_ptr<'a>(ptr: $ptr) -> &'a mut Self {
                assert!(!ptr.is_null());
                let borrowed = $struct_name { ptr, $(_ctx: PhantomData::<$ctx>)?};
                let ptr = Box::into_raw(Box::new(borrowed));
                &mut *ptr
            }
            pub(crate) unsafe fn from_raw(ptr: $ptr) -> Self {
                $struct_name { ptr, $(_ctx: PhantomData::<$ctx>)?}
            }
            pub(crate) fn into_raw(self) -> $ptr {
                let ptr = self.ptr;
                let _ = std::mem::ManuallyDrop::new(self);
                ptr
            }
            pub(crate) fn as_ptr(&self) -> $ptr {
                self.ptr
            }
        }

        // impl cronet method
        impl $(<$ctx>)? $struct_name $(<$ctx>)? {
        $(
            pub(crate) fn $func_name $(<$($gen_param),*>)?(self: $self_ $(,$arg_name: $arg_type )*) $( -> $return_type)? {
                unsafe {
                    let ret =  $cronet_func(
                        self.ptr $(,{
                            $(let $arg_name = $arg_trans_func($arg_name);)?
                            $arg_name
                        })*
                    );
                    $($(let ret = $return_trans_func(ret);)?)?
                    ret
                }
            }
        )*
        }

        // impl ctx
        $(
            impl <$ctx> $struct_name <$ctx> {
                pub(crate) fn get_client_context_mut<'a>(&mut self) -> &'a mut $ctx {
                    let void_ptr = unsafe { $cronet_get(self.ptr) };
                    assert!(!void_ptr.is_null());
                    let ctx_ptr = void_ptr as *mut Ctx;
                    unsafe{&mut  *ctx_ptr}
                }
                pub(crate) fn get_client_context<'a>(&self) -> &'a $ctx {
                    let void_ptr = unsafe { $cronet_get(self.ptr) };
                    assert!(!void_ptr.is_null());
                    let ctx_ptr = void_ptr as *mut Ctx;
                    unsafe{& *ctx_ptr}
                }
                pub(crate) fn set_client_context(&mut self, client_context: $ctx) {
                    let ptr = Box::into_raw(Box::new(client_context));
                    // todo: may leak previous ctx
                    unsafe { $cronet_set(self.ptr, ptr as crate::bindings::Cronet_ClientContext) }
                }
            }
        )?
    };
}
pub(crate) use define_impl;
