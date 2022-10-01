mod arrays;
mod color;
mod others;
mod quat;
mod string;
mod variant;
mod vector2;
mod vector3;

pub use arrays::*;
pub use color::*;
pub use others::*;
pub use quat::*;
pub use string::*;
pub use variant::*;
pub use vector2::*;
pub use vector3::*;

use gdext_sys as sys;

pub trait BuiltinInner {
    type Opaque;

    unsafe fn with_init(init: impl FnOnce(sys::GDNativeTypePtr)) -> Self;

    fn opaque(&self) -> &Self::Opaque;

    fn from_opaque(other: &Self::Opaque) -> &Self;
}

impl<const N: usize> BuiltinInner for sys::opaque::Opaque<N> {
    type Opaque = Self;

    unsafe fn with_init(init: impl FnOnce(sys::GDNativeTypePtr)) -> Self {
        Self::with_init(init)
    }

    fn opaque(&self) -> &Self::Opaque {
        self
    }

    fn from_opaque(other: &Self::Opaque) -> &Self {
        other
    }
}

macro_rules! impl_builtin_stub {
    ($Class:ident, $OpaqueTy:ident, $ctor:ident, $copy:ident $(, $dtor:ident)? $(,)?) => {
        #[repr(C)]
        pub struct $Class {
            opaque: $OpaqueTy,
        }

        impl $Class {
            pub fn new() -> Self {
                unsafe {
                    Self {
                        opaque: $OpaqueTy::with_init(|ptr| {
                            (gdext_sys::get_cache().$ctor)(ptr, ::std::ptr::null_mut())
                        }),
                    }
                }
            }

            fn from_opaque(opaque: $OpaqueTy) -> Self {
                Self { opaque }
            }
        }

        impl Default for $Class {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Clone for $Class {
            fn clone(&self) -> Self {
                unsafe {
                    Self::from_sys_init(|opaque_ptr| {
                        let ctor = get_cache().$copy;
                        ctor(opaque_ptr, &self.sys() as *const sys::GDNativeTypePtr);
                    })
                }
            }
        }

        impl gdext_sys::GodotFfi for $Class {
            gdext_sys::impl_ffi_as_opaque_pointer!(gdext_sys::GDNativeTypePtr);
        }

        $(impl Drop for $Class {
            fn drop(&mut self) {
                unsafe {
                    let destructor = sys::get_cache().$dtor;
                    destructor(self.sys_mut());
                }
            }
        })?
    };
}
use impl_builtin_stub;

macro_rules! impl_builtin_froms {
    ($To:ty; $($From:ty => $from_fn:ident),* $(,)?) => {
        $(impl From<&$From> for $To {
            fn from(other: &$From) -> Self {
                unsafe {
                    Self::from_sys_init(|ptr| {
                        let converter = sys::get_cache().$from_fn;
                        converter(ptr, &[other.sys()] as _);
                    })
                }
            }
        })*
    };
}
use impl_builtin_froms;
