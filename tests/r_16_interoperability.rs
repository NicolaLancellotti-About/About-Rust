#![allow(dead_code)]
#![allow(unused_variables)]

use std::panic::catch_unwind;

// The type representation is the representation of its only field
#[repr(transparent)]
struct TransparentNewType(f64);

// src/clib
mod clib {
    pub enum NLOpaqueType {}

    #[repr(C)]
    pub struct NLOpaqueTypeData {
        pub integer: i32,
        pub boolean: bool,
    }

    #[repr(C)]
    pub struct RustObject {
        pub value: i32,
    }

    #[link(name = "clib")]
    extern "C" {
        pub fn NLOpaqueTypeCreate(value: NLOpaqueTypeData) -> *mut NLOpaqueType;
        pub fn NLOpaqueTypeDelete(instance: *mut NLOpaqueType);
        pub fn NLOpaqueTypeGetValue(instance: *const NLOpaqueType) -> NLOpaqueTypeData;

        pub fn NLOpaqueTypeSetValueToSumOfValues(instance: *mut NLOpaqueType, count: i64, ...);

        pub fn NLOpaqueTypeRegisterCallback(
            instance: *mut NLOpaqueType,
            target: *mut RustObject,
            action: Option<extern "C" fn(*mut RustObject, i32)>,
        );
        pub fn NLOpaqueTypeTriggerCallback(instance: *const NLOpaqueType);

        pub fn NLInitVectorIncrementer(p: *const i64, count: usize);
    }
}

#[test]
fn test_clib() {
    use clib::*;

    extern "C" fn callback(target: *mut RustObject, new_value: i32) {
        let result = catch_unwind(|| {
            // panic!("Oops!");
        });

        match result {
            Ok(_) => (),
            Err(_) => (),
        }
        unsafe {
            (*target).value = new_value;
        }
    }

    unsafe {
        let instance = NLOpaqueTypeCreate(NLOpaqueTypeData {
            integer: 10,
            boolean: false,
        });

        let value = NLOpaqueTypeGetValue(instance);
        assert_eq!(value.integer, 10);

        NLOpaqueTypeSetValueToSumOfValues(instance, 2, 10, 20);
        let value = NLOpaqueTypeGetValue(instance);
        assert_eq!(value.integer, 30);

        let mut rust_object = Box::new(RustObject { value: 5 });
        NLOpaqueTypeRegisterCallback(instance, &mut *rust_object, Some(callback));
        NLOpaqueTypeTriggerCallback(instance);

        NLOpaqueTypeRegisterCallback(instance, &mut *rust_object, None);
        NLOpaqueTypeTriggerCallback(instance);
        assert_eq!(rust_object.value, NLOpaqueTypeGetValue(instance).integer);

        NLOpaqueTypeDelete(instance);
    }

    unsafe {
        let count = 3;
        let mut vector = Vec::with_capacity(count);
        let vector_p = vector.as_mut_ptr();
        NLInitVectorIncrementer(vector_p, count);
        vector.set_len(count);
        assert_eq!(vector, [0, 1, 2]);
    }
}
