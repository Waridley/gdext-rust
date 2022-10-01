use std::ffi::CString;
use std::{convert::Infallible, mem::MaybeUninit, str::FromStr};

use crate::{types::*, util::*};
use gdext_sys as sys;
use sys::types::OpaqueString;
use sys::{get_cache, impl_ffi_as_opaque_pointer, interface_fn, GodotFfi};

#[repr(C, align(8))]
pub struct GodotString {
    opaque: OpaqueString,
}

impl GodotString {
    pub fn new() -> Self {
        unsafe {
            Self::from_sys_init(|opaque_ptr| {
                let ctor = get_cache().string_construct_default;
                ctor(opaque_ptr, std::ptr::null_mut());
            })
        }
    }

    fn from_opaque(opaque: OpaqueString) -> Self {
        Self { opaque }
    }

    pub fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }

    // TODO remove this method
    // it's currently used for _to_string(), which has a const char* return type,
    // however Godot devs already announced to change it to a GDNativeStringPtr parameter.
    #[doc(hidden)]
    pub fn leak_c_string(&self) -> *const std::os::raw::c_char {
        let s: String = self.into();

        let c = CString::new(s).unwrap();
        let ptr = c.as_ptr();
        std::mem::forget(c);
        ptr
    }

    #[doc(hidden)]
    pub fn string_sys(&self) -> sys::GDNativeStringPtr {
        self.sys() as _
    }

    #[doc(hidden)]
    pub fn string_sys_mut(&mut self) -> sys::GDNativeStringPtr {
        self.sys_mut() as _
    }

    #[doc(hidden)]
    pub unsafe fn write_string_sys(&self, dst: sys::GDNativeStringPtr) {
        std::ptr::write(dst as *mut _, self.opaque)
    }
}

impl Default for GodotString {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GodotString {
    fn clone(&self) -> Self {
        unsafe {
            Self::from_sys_init(|opaque_ptr| {
                let ctor = get_cache().string_construct_copy;
                ctor(opaque_ptr, &self.sys() as *const sys::GDNativeTypePtr);
            })
        }
    }
}

impl_builtin_froms!(GodotString;
    StringName => string_from_string_name,
    NodePath => string_from_node_path,
);

impl From<String> for GodotString {
    fn from(s: String) -> GodotString {
        GodotString::from(s.as_str())
    }
}

impl From<&str> for GodotString {
    fn from(val: &str) -> Self {
        GodotString::from(val)
    }
}

impl std::fmt::Display for GodotString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(self);
        f.write_str(s.as_str())
    }
}

impl From<&GodotString> for String {
    fn from(string: &GodotString) -> Self {
        unsafe {
            let len =
                interface_fn!(string_to_utf8_chars)(string.string_sys(), std::ptr::null_mut(), 0);

            assert!(len >= 0);
            let mut buf = vec![0u8; len as usize];

            interface_fn!(string_to_utf8_chars)(
                string.string_sys(),
                buf.as_mut_ptr() as *mut i8,
                len,
            );

            String::from_utf8_unchecked(buf)
        }
    }
}

impl FromStr for GodotString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut opaque = MaybeUninit::<OpaqueString>::uninit();

        let b = s.as_bytes();
        unsafe {
            interface_fn!(string_new_with_utf8_chars_and_len)(
                opaque.as_mut_ptr() as sys::GDNativeStringPtr,
                b.as_ptr() as *mut _,
                b.len() as i64,
            );

            Ok(Self {
                opaque: opaque.assume_init(),
            })
        }
    }
}

impl Drop for GodotString {
    fn drop(&mut self) {
        unsafe {
            let destructor = get_cache().string_destroy;
            destructor(self.sys_mut()); // TODO: temporarily leaking to prevent segfaults (strangely only sporadically at exit when destroying Vulkan context)
        }
    }
}

// FIXME: Should this only be implemented for &GodotString? This risks UB from dropping GodotStrings
impl GodotFfi for GodotString {
    impl_ffi_as_opaque_pointer!(sys::GDNativeTypePtr);
}

impl From<&Variant> for &GodotString {
    fn from(v: &Variant) -> Self {
        unsafe {
            let converter = sys::get_cache().string_from_variant;
            Self::from_sys_init(|ptr| converter(ptr, v.sys()))
        }
    }
}

impl GodotFfi for &GodotString {
    type SysPointer = GDNativeTypePtr;

    unsafe fn from_sys(opaque_ptr: Self::SysPointer) -> Self {
        unsafe { &*(opaque_ptr as *const GodotString) }
    }

    unsafe fn from_sys_init(init: impl FnOnce(Self::SysPointer)) -> Self {
        let mut ret = MaybeUninit::<OpaqueString>::zeroed();
        init(ret.assume_init_mut() as *mut _ as _);
        std::mem::transmute(ret.assume_init_ref())
    }

    fn sys(&self) -> Self::SysPointer {
        self.opaque.to_sys()
    }

    unsafe fn write_sys(&self, dst: Self::SysPointer) {
        std::ptr::write(dst as *mut _, self.opaque)
    }
}

// While this is a nice optimisation for ptrcalls, it's not easily possible
// to pass in &GodotString when doing varcalls.
/*
impl PtrCall for &GodotString {
    unsafe fn from_ptr_call_arg(arg: *const gdext_sys::GDNativeTypePtr) -> Self {
        &*(*arg as *const GodotString)
    }

    unsafe fn to_ptr_call_arg(self, arg: gdext_sys::GDNativeTypePtr) {
        std::ptr::write(arg as *mut GodotString, self.clone());
    }
}
*/
