#![allow(dead_code)]
#![allow(unused_variables)]

#[test]
fn unsafe_superpowers() {
    // _________________________________________________
    // 1) Dereferencing a raw pointer
    {
        let mut num = 5;

        // Explicit cast
        let r1: *const i32 = &num as *const i32;
        let r2: *mut i32 = &mut num as *mut i32;
        let r3: *mut i32 = 0 as *mut i32;

        // Implicit coercion
        let p_mut: *mut i32 = &mut num;

        unsafe {
            let num1 = *r1;
            let num1 = *r2;
            // let num1 = *r3;
        }

        // Conversion
        unsafe {
            let ref_imm: &i32 = &*r1;
            let ref_mut: &mut i32 = &mut *r2;
        }
    }

    // _________________________________________________
    // 2) Calling an unsafe function or method
    {
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
    }

    // _________________________________________________
    // 3) Accessing or modifying a mutable static variable
    {
        static mut COUNTER: u32 = 0;

        unsafe {
            COUNTER += 1;
        }
    }
    // _________________________________________________
    // Implementing an unsafe trait
    {
        unsafe trait Foo {}

        unsafe impl Foo for i32 {}
    }

    // _________________________________________________
    // 4) Access fields of unions
    union MyUnion {
        f1: u32,
        f2: f32,
    }
    let mut u = MyUnion { f1: 1 };
    u.f1 = 2;
    unsafe {
        assert_eq!(u.f1, 2);
    }
}
