// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::GodotString;
use gdext_sys as sys;
use gdext_sys::types::*;
use sys::{impl_ffi_as_opaque_pointer, GodotFfi};

macro_rules! impl_builtin_stub {
    ($Class:ident, $OpaqueTy:ident, $ctor:ident, $dtor:ident) => {
        #[repr(C)]
        pub struct $Class {
            opaque: sys::types::$OpaqueTy,
        }

        impl $Class {
            pub fn new() -> Self {
                unsafe {
                    Self {
                        opaque: $OpaqueTy::with_init(|ptr| {
                            (sys::get_cache().$ctor)(ptr, std::ptr::null_mut())
                        }),
                    }
                }
            }

            fn from_opaque(opaque: sys::types::$OpaqueTy) -> Self {
                Self { opaque }
            }
        }

        impl Default for $Class {
            fn default() -> Self {
                Self::new()
            }
        }

        impl GodotFfi for $Class {
            impl_ffi_as_opaque_pointer!(sys::GDNativeTypePtr);
        }

        impl Drop for $Class {
            fn drop(&mut self) {
                unsafe {
                    let destructor = sys::get_cache().$dtor;
                    // destructor(self.sys_mut()); // TODO: temporarily leaking to avoid segfaults
                }
            }
        }
    };
}

impl_builtin_stub!(Array, OpaqueArray, array_construct_default, array_destroy);
impl_builtin_stub!(
    Dictionary,
    OpaqueDictionary,
    dictionary_construct_default,
    dictionary_destroy
);
impl_builtin_stub!(
    StringName,
    OpaqueStringName,
    string_name_construct_default,
    string_name_destroy
);
impl_builtin_stub!(
    Transform2D,
    OpaqueTransform2D,
    transform2d_construct_default,
    transform2d_destroy
);
impl_builtin_stub!(
    Transform3D,
    OpaqueTransform3D,
    transform3d_construct_default,
    transform3d_destroy
);
impl_builtin_stub!(
    NodePath,
    OpaqueNodePath,
    node_path_construct_default,
    node_path_destroy
);
impl_builtin_stub!(
    StringArray,
    OpaquePackedStringArray,
    packed_string_array_construct_default,
    packed_string_array_destroy
);

macro_rules! impl_builtin_from {
    ($From:ty => $To:ty, $from_fn:ident) => {
        impl From<$From> for $To {
            fn from(other: $From) -> Self {
                unsafe {
                    Self::from_sys_init(|ptr| {
                        let converter = sys::get_cache().$from_fn;
                        converter(ptr, &other.sys() as _);
                        ::std::mem::forget(other);
                    })
                }
            }
        }
    };
}

impl_builtin_from!(StringName => GodotString, string_from_string_name);
impl_builtin_from!(GodotString => StringName, string_name_from_string);
impl_builtin_from!(NodePath => GodotString, node_path_from_string);
impl_builtin_from!(GodotString => NodePath, string_from_node_path);
impl_builtin_from!(StringArray => Array, packed_string_array_from_array);
impl_builtin_from!(Array => StringArray, array_from_packed_string_array);
