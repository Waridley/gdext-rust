use crate::{util::*, *};
use gdext_sys as sys;
use std::ops::{Index, IndexMut};
use sys::{get_cache, interface_fn, types::*, GodotFfi};

impl_builtin_stub!(
    Array,
    OpaqueArray,
    array_construct_default,
    array_construct_copy,
    array_destroy
);
impl_builtin_stub!(
    ByteArray,
    OpaquePackedByteArray,
    packed_byte_array_construct_default,
    packed_byte_array_construct_copy,
    packed_byte_array_destroy
);
impl_builtin_stub!(
    ColorArray,
    OpaquePackedColorArray,
    packed_color_array_construct_default,
    packed_color_array_construct_copy,
    packed_color_array_destroy
);
impl_builtin_stub!(
    Float32Array,
    OpaquePackedFloat32Array,
    packed_float32_array_construct_default,
    packed_float32_array_construct_copy,
    packed_float32_array_destroy
);
impl_builtin_stub!(
    Float64Array,
    OpaquePackedFloat64Array,
    packed_float64_array_construct_default,
    packed_float64_array_construct_copy,
    packed_float64_array_destroy
);
impl_builtin_stub!(
    Int32Array,
    OpaquePackedInt32Array,
    packed_int32_array_construct_default,
    packed_int32_array_construct_copy,
    packed_int32_array_destroy
);
impl_builtin_stub!(
    Int64Array,
    OpaquePackedInt64Array,
    packed_int64_array_construct_default,
    packed_int64_array_construct_copy,
    packed_int64_array_destroy
);
impl_builtin_stub!(
    StringArray,
    OpaquePackedStringArray,
    packed_string_array_construct_default,
    packed_string_array_construct_copy,
    packed_string_array_destroy
);
impl_builtin_stub!(
    Vector2Array,
    OpaquePackedVector2Array,
    packed_vector2_array_construct_default,
    packed_vector2_array_construct_copy,
    packed_vector2_array_destroy
);
impl_builtin_stub!(
    Vector3Array,
    OpaquePackedVector3Array,
    packed_vector3_array_construct_default,
    packed_vector3_array_construct_copy,
    packed_vector3_array_destroy
);

impl_builtin_froms!(Array;
    ByteArray => array_from_packed_byte_array,
    ColorArray => array_from_packed_color_array,
    Float32Array => array_from_packed_float_32_array,
    Float64Array => array_from_packed_float_64_array,
    Int32Array => array_from_packed_int_32_array,
    Int64Array => array_from_packed_int_64_array,
    StringArray => array_from_packed_string_array,
    Vector2Array => array_from_packed_vector_2_array,
    Vector3Array => array_from_packed_vector_3_array,
);

impl_builtin_froms!(ByteArray; Array => packed_byte_array_from_array);
impl_builtin_froms!(ColorArray; Array => packed_color_array_from_array);
impl_builtin_froms!(Float32Array; Array => packed_float32_array_from_array);
impl_builtin_froms!(Float64Array; Array => packed_float64_array_from_array);
impl_builtin_froms!(Int32Array; Array => packed_int32_array_from_array);
impl_builtin_froms!(Int64Array; Array => packed_int64_array_from_array);
impl_builtin_froms!(StringArray; Array => packed_string_array_from_array);
impl_builtin_froms!(Vector2Array; Array => packed_vector2_array_from_array);
impl_builtin_froms!(Vector3Array; Array => packed_vector3_array_from_array);

impl Array {
    pub fn size(&self) -> sys::GDNativeInt {
        unsafe {
            <sys::GDNativeInt as sys::PtrCall>::ptrcall_read_init(|ret_ptr| {
                (get_cache().array_size)(self.sys(), [].as_ptr(), ret_ptr, 0)
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            <bool as sys::PtrCall>::ptrcall_read_init(|ret_ptr| {
                (get_cache().array_is_empty)(self.sys(), [].as_ptr(), ret_ptr, 0)
            })
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            <() as sys::PtrCall>::ptrcall_read_init(|ret_ptr| {
                (get_cache().array_clear)(self.sys(), [].as_ptr(), ret_ptr, 0)
            })
        }
    }

    pub fn get(&self, index: i64) -> Option<&Variant> {
        unsafe {
            let ptr = (interface_fn!(array_operator_index))(self.sys(), index) as *mut Variant;
            if ptr.is_null() {
                return None;
            }
            Some(&*ptr)
        }
    }

    pub fn get_mut(&mut self, index: i64) -> Option<&mut Variant> {
        unsafe {
            let ptr = (interface_fn!(array_operator_index))(self.sys(), index) as *mut Variant;
            if ptr.is_null() {
                return None;
            }
            Some(&mut *ptr)
        }
    }
}

impl Index<i64> for Array {
    type Output = Variant;

    fn index(&self, index: i64) -> &Self::Output {
        self.get(index).unwrap() // Godot will print error if index is OOB
    }
}

impl IndexMut<i64> for Array {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        self.get_mut(index).unwrap() // Godot will print error if index is OOB
    }
}
