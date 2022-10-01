use gdext_sys as sys;
use sys::real;
use sys::{impl_ffi_as_value, GodotFfi};

#[cfg(not(feature = "real_is_double"))]
type Inner = glam::f32::Vec2;
#[cfg(feature = "real_is_double")]
type Inner = glam::f64::DVec2;

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2 {
    inner: Inner,
}

impl Vector2 {
    pub fn new(x: real, y: real) -> Self {
        Self {
            inner: Inner::new(x, y),
        }
    }

    pub fn from_inner(inner: Inner) -> Self {
        Self { inner }
    }

    /// only for testing
    pub fn inner(self) -> Inner {
        self.inner
    }
}

impl GodotFfi for Vector2 {
    impl_ffi_as_value!();
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

type IInner = glam::IVec2;

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2i {
    inner: IInner,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            inner: IInner::new(x, y),
        }
    }

    pub fn from_inner(inner: IInner) -> Self {
        Self { inner }
    }

    /// only for testing
    pub fn inner(self) -> IInner {
        self.inner
    }
}

impl GodotFfi for Vector2i {
    impl_ffi_as_value!();
}

impl std::fmt::Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
