use gdext_sys as sys;
use sys::{impl_ffi_as_value, real, GodotFfi};

#[cfg(not(feature = "real_is_double"))]
type Inner = glam::f32::Vec4;
#[cfg(feature = "real_is_double")]
type Inner = glam::f64::DVec3;

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector4 {
    inner: Inner,
}

impl Vector4 {
    pub fn new(x: real, y: real, z: real, w: real) -> Self {
        Self {
            inner: Inner::new(x, y, z, w),
        }
    }
}

impl GodotFfi for Vector4 {
    impl_ffi_as_value!();
}

impl std::fmt::Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let Inner {x, y, z} = self.inner;
        //write!(f, "({x}, {y}, {z})")
        self.inner.fmt(f)
    }
}
