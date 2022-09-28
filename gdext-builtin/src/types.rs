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
