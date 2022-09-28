use crate::util::*;
use crate::BuiltinInner;
use gdext_sys as sys;
use glam::Quat;
use std::ops::{Deref, DerefMut};
use sys::{get_cache, types::OpaqueQuaternion, GodotFfi};

impl_builtin_stub!(
    Quaternion,
    Quat,
    quaternion_construct_default,
    quaternion_construct_copy,
);

impl BuiltinInner for Quat {
    type Opaque = OpaqueQuaternion;

    unsafe fn with_init(init: impl FnOnce(sys::GDNativeTypePtr)) -> Self {
        let mut raw = std::mem::MaybeUninit::<Self>::uninit();
        init(raw.as_mut_ptr() as _);
        raw.assume_init()
    }

    fn opaque(&self) -> &Self::Opaque {
        unsafe { std::mem::transmute(self) }
    }

    fn from_opaque(other: &Self::Opaque) -> &Self {
        unsafe { std::mem::transmute(other) }
    }
}

impl Deref for Quaternion {
    type Target = Quat;

    fn deref(&self) -> &Self::Target {
        &self.opaque
    }
}

impl DerefMut for Quaternion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.opaque
    }
}
