use std::ops::{Index, IndexMut};
// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::{types::*, util::*};
use gdext_sys as sys;
use gdext_sys::types::*;
use gdext_sys::{get_cache, PtrCall};
use sys::GodotFfi;

impl_builtin_stub!(
    Dictionary,
    OpaqueDictionary,
    dictionary_construct_default,
    dictionary_construct_copy,
    dictionary_destroy,
);
impl_builtin_stub!(
    StringName,
    OpaqueStringName,
    string_name_construct_default,
    string_name_construct_copy,
    string_name_destroy,
);
impl_builtin_stub!(
    Transform2D,
    OpaqueTransform2D,
    transform2d_construct_default,
    transform2d_construct_copy,
    transform2d_destroy,
);
impl_builtin_stub!(
    Transform3D,
    OpaqueTransform3D,
    transform3d_construct_default,
    transform3d_construct_copy,
    transform3d_destroy,
);
impl_builtin_stub!(
    Projection,
    OpaqueProjection,
    projection_construct_default,
    projection_construct_copy,
);

impl_builtin_stub!(
    NodePath,
    OpaqueNodePath,
    node_path_construct_default,
    node_path_construct_copy,
    node_path_destroy,
);

impl_builtin_froms!(StringName; GodotString => string_name_from_string);
impl_builtin_froms!(Transform3D; Projection => transform3d_from_projection);
impl_builtin_froms!(NodePath; GodotString => node_path_from_string);

impl Transform2D {
    pub fn from_rot_pos(rotation: f32, position: Vector2) -> Self {
        let rotation = unsafe { rotation.ptrcall_write_return() };
        let position = position.sys();
        unsafe {
            Self {
                opaque: OpaqueTransform2D::with_init(|ptr| {
                    (get_cache().transform2d_from_rotation_position)(
                        ptr,
                        [rotation, position].as_ptr(),
                    )
                }),
            }
        }
    }

    pub fn from_rot_scale_skew_pos(
        rotation: f32,
        scale: Vector2,
        skew: f32,
        position: Vector2,
    ) -> Self {
        let rotation = unsafe { rotation.ptrcall_write_return() };
        let scale = scale.sys();
        let skew = unsafe { skew.ptrcall_write_return() };
        let position = position.sys();
        let args = [rotation, scale, skew, position];
        unsafe {
            Self {
                opaque: OpaqueTransform2D::with_init(|ptr| {
                    (get_cache().transform2d_from_rotation_scale_skew_position)(ptr, args.as_ptr())
                }),
            }
        }
    }

    pub fn from_axes_and_origin(x_axis: Vector2, y_axis: Vector2, origin: Vector2) -> Self {
        let args = [x_axis.sys(), y_axis.sys(), origin.sys()];
        unsafe {
            Self {
                opaque: OpaqueTransform2D::with_init(|ptr| {
                    (get_cache().transform2d_from_x_axis_y_axis_origin)(ptr, args.as_ptr())
                }),
            }
        }
    }
}
