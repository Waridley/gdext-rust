use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;

use gdext_sys::{self as sys, GDNativeTypePtr, GDNativeVariantPtr, interface_fn};
use crate::PtrCallArg;
use crate::string::GodotString;

#[cfg(not(feature = "real_is_double"))]
const SIZE_IN_BYTES: u64 = 24;
#[cfg(feature = "real_is_double")]
const SIZE_IN_BYTES: u64 = 40;

#[repr(C, align(8))]
pub struct Variant(MaybeUninit<[u8; SIZE_IN_BYTES as usize]>);

impl Variant {
    #[doc(hidden)]
    pub fn uninit() -> Self {
        Self(MaybeUninit::uninit())
    }

    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> sys::GDNativeVariantPtr {
        self.0.as_mut_ptr() as _
    }

    #[doc(hidden)]
    pub fn as_ptr(&self) -> sys::GDNativeVariantPtr {
        self.0.as_ptr() as _
    }

    pub fn nil() -> Self {
        unsafe {
            let mut v = Self::uninit();
            interface_fn!(variant_new_nil)(v.as_mut_ptr());
            v
        }
    }
    
    #[doc(hidden)]
    pub unsafe fn clone_from_ptr(ptr: sys::GDNativeVariantPtr) -> Self {
        let mut ret = Self::uninit();
        ret.0.write(std::ptr::read(ptr as *const _));
        ret
    }
    
    #[doc(hidden)]
    pub unsafe fn clone_into_ptr(&self, ptr: sys::GDNativeVariantPtr) {
        std::ptr::copy_nonoverlapping(self.as_ptr(), ptr, SIZE_IN_BYTES as usize)
    }
}

impl PtrCallArg for Variant {
    unsafe fn from_ptr_call_arg(arg: *const GDNativeTypePtr) -> Self {
        Variant::clone_from_ptr((*arg) as GDNativeVariantPtr)
    }
    
    unsafe fn to_ptr_call_arg(self, arg: GDNativeTypePtr) {
        Variant::clone_into_ptr(&self, arg as GDNativeVariantPtr)
    }
}

impl Debug for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = GodotString::new();
        unsafe {
            interface_fn!(variant_stringify)(self.as_ptr(), string.as_mut_ptr());
        }
        write!(f, "{string}")
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        unsafe {
            let mut v = Self::uninit();
            interface_fn!(variant_new_copy)(v.as_mut_ptr(), self.as_ptr());
            v
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            interface_fn!(variant_destroy)(self.as_ptr());
        }
    }
}

mod conversions {
    use std::mem::MaybeUninit;
    use gdext_sys::{self as sys, GDNativeBool, interface_fn};
    use once_cell::sync::Lazy;

    use crate::{string::GodotString, vector2::Vector2, vector3::Vector3};

    use super::Variant;

    impl From<bool> for Variant {
        fn from(b: bool) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_BOOL,
                    )
                    .unwrap()
                });
                let b = b as GDNativeBool;
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), &b as *const _ as _);
                v
            }
        }
    }

    impl From<&Variant> for bool {
        fn from(v: &Variant) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeTypePtr, sys::GDNativeVariantPtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_to_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_BOOL,
                    )
                    .unwrap()
                });
                let mut res = MaybeUninit::<GDNativeBool>::uninit();
                CONSTR(res.as_mut_ptr() as *mut _, v.as_ptr());
                let res = res.assume_init() != 0;
                res
            }
        }
    }

    impl From<i64> for Variant {
        fn from(i: i64) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_INT,
                    )
                    .unwrap()
                });
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), &i as *const _ as *mut _);
                v
            }
        }
    }

    impl From<&Variant> for i64 {
        fn from(v: &Variant) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeTypePtr, sys::GDNativeVariantPtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_to_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_INT,
                    )
                    .unwrap()
                });
                let mut res = 0;
                CONSTR(&mut res as *mut _ as *mut _, v.as_ptr());
                res
            }
        }
    }

    macro_rules! from_int {
        ($name:ty) => {
            impl From<$name> for Variant {
                fn from(i: $name) -> Self {
                    Variant::from(i as i64)
                }
            }

            impl From<&Variant> for $name {
                fn from(i: &Variant) -> Self {
                    i64::from(i) as $name
                }
            }
        };
    }

    from_int!(u8);
    from_int!(u16);
    from_int!(u32);
    from_int!(u64);

    from_int!(i8);
    from_int!(i16);
    from_int!(i32);

    impl From<Vector2> for Variant {
        fn from(vec: Vector2) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_VECTOR2,
                    )
                    .unwrap()
                });
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), &vec as *const _ as *mut _);
                v
            }
        }
    }

    impl From<&Variant> for Vector2 {
        fn from(v: &Variant) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeTypePtr, sys::GDNativeVariantPtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_to_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_VECTOR2,
                    )
                    .unwrap()
                });
                let mut vec = Vector2::ZERO;
                CONSTR(&mut vec as *mut _ as *mut _, v.as_ptr());
                vec
            }
        }
    }

    impl From<Vector3> for Variant {
        fn from(vec: Vector3) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_VECTOR3,
                    )
                    .unwrap()
                });
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), &vec as *const _ as *mut _);
                v
            }
        }
    }

    impl From<&Variant> for Vector3 {
        fn from(v: &Variant) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeTypePtr, sys::GDNativeVariantPtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_to_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_VECTOR3,
                    )
                    .unwrap()
                });
                let mut vec = Vector3::ZERO;
                CONSTR(&mut vec as *mut _ as *mut _, v.as_ptr());
                vec
            }
        }
    }

    impl From<GodotString> for Variant {
        fn from(mut s: GodotString) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_STRING,
                    )
                    .unwrap()
                });
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), s.as_mut_ptr());
                v
            }
        }
    }

    impl From<&GodotString> for Variant {
        fn from(s: &GodotString) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeVariantPtr, sys::GDNativeTypePtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_from_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_STRING,
                    )
                    .unwrap()
                });
                let mut v = Variant::uninit();
                CONSTR(v.as_mut_ptr(), s.as_ptr());
                v
            }
        }
    }

    impl From<&Variant> for GodotString {
        fn from(v: &Variant) -> Self {
            unsafe {
                static CONSTR: Lazy<
                    unsafe extern "C" fn(sys::GDNativeTypePtr, sys::GDNativeVariantPtr),
                > = Lazy::new(|| unsafe {
                    interface_fn!(get_variant_to_type_constructor)(
                        sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_STRING,
                    )
                    .unwrap()
                });
                let mut vec = GodotString::new();
                CONSTR(&mut vec as *mut _ as *mut _, v.as_ptr());
                vec
            }
        }
    }
}
