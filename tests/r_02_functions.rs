#![allow(dead_code)]
#![allow(unused_variables)]

#[test]
fn functions() {
    fn bar(x: &mut i32) {
        *x = 20;
    }
    let mut x = 10;
    bar(&mut x);
    assert_eq!(x, 20);

    // Functions pointer
    let bar_pointer: fn(&mut i32) -> () = bar;
    fn baz(f: fn(&mut i32) -> ()) {
        let mut x = 10;
        f(&mut x);
        assert_eq!(x, 20);
    }
    baz(bar_pointer);
    baz(bar);
}

#[test]
fn closures() {
    let sum = |x: i32, y: i32| -> i32 { x + y };
    let sum = |x, y| x + y;
    assert_eq!(sum(1, 2), 3);

    // __________________________________________________
    //  Closures are allowed to capture values from the scope in which they
    //  are called.

    let s = String::from("Hello");
    let fn_closure = |x: &str| x == s;

    // __________________________________________________
    // If we want to force the closure to take ownership of the values it use in
    // the environment, we can use the move keyword before the parameter list.

    let s = String::from("abc");
    let equal = move |x| x == s;
    // let s = s; // error
    assert_eq!(equal("abc".to_string()), true);

    // __________________________________________________
    // Input closures

    fn bar_ref(f: &dyn Fn(i32, i32) -> i32) -> i32 {
        f(1, 2)
    }
    assert_eq!(bar_ref(&sum), 3);

    fn bar_generic<T>(f: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        f(1, 2)
    }
    assert_eq!(bar_generic(sum), 3);

    // __________________________________________________
    // Higher-Ranked Trait

    fn baz_ref(t: &dyn for<'a> Fn(&'a i32) -> i32) -> i32 {
        let value = 10;
        t(&value)
    }
    assert_eq!(baz_ref(&(|&x| x + 1)), 11);

    fn baz_generic<T>(t: T) -> i32
    where
        T: for<'a> Fn(&'a i32) -> i32,
    {
        let value = 10;
        t(&value)
    }
    assert_eq!(baz_generic(|&x| x + 1), 11);

    // __________________________________________________
    // - FnOnce consumes the variables it captures from its enclosing scope
    //   the closure can’t take ownership of the same variables more than once,
    //   so it can only be called one time.
    // - FnMut can change the environment since it mutably borrows values.
    // - Fn borrows values from the environment immutably.

    // trait FnOnce<Args>
    // trait FnMut<Args>: FnOnce<Args>
    // trait Fn<Args>: FnMut<Args>

    // Fn: FnMut: FnOnce

    // Function pointers implement all three of the closure traits so we can
    // always pass a function pointer as an argument when calling a function
    // that expects a closure.

    {
        fn fn_fn_once<T>(f: T) -> usize
        where
            T: FnOnce() -> usize,
        {
            f()
        }
        // fn fn_fn_once_ref(f: &mut dyn FnOnce() -> usize) -> usize { f() }

        fn fn_fn_mut<T>(mut f: T) -> usize
        where
            T: FnMut() -> usize,
        {
            f()
        }
        fn fn_fn_mut_ref(f: &mut dyn FnMut() -> usize) -> usize {
            f()
        }

        fn fn_fn<T>(f: T) -> usize
        where
            T: Fn() -> usize,
        {
            f()
        }
        fn fn_fn_ref(f: &dyn Fn() -> usize) -> usize {
            f()
        }

        {
            // Fn
            let s = String::from("Hello");

            let mut x = || s.len();
            fn_fn_once(x);
            //
            fn_fn_mut(x);
            fn_fn_mut_ref(&mut x);

            fn_fn(x);
            fn_fn_ref(&x);
        }
        {
            // FnMut
            let mut s = String::from("Hello");

            let x = || {
                s.push_str("w");
                s.len()
            };
            fn_fn_once(x);

            let x = || {
                s.push_str("w");
                s.len()
            };
            fn_fn_mut(x);

            let mut x = || {
                s.push_str("w");
                s.len()
            };
            fn_fn_mut_ref(&mut x);

            let x = || {
                s.push_str("w");
                s.len()
            };
            // fn_fn(x);
            // fn_fn_ref(&x);
        }

        {
            // FnOnce
            let s = String::from("Hello");
            let x = || {
                let x = s.len();
                drop(s);
                x
            };
            fn_fn_once(x);

            // let mut s = String::from("Hello");
            // let x = || { let x = s.len(); drop(s); x};
            // fn_fn_mut(x);

            // let mut x = || {s.push_str("w"); s.len()};
            // fn_fn_mut_ref(&mut x);

            // let x = || {s.push_str("w"); s.len()};
            // fn_fn(x);
            // fn_fn_ref(&x);
        }
    }

    {
        // Closures with generic parameters

        let mut a_box = Box::new(10);

        struct BoxIncrementer<T>
        where
            T: FnMut(i32) -> (),
        {
            increment: T,
        }

        {
            let mut incrementer = BoxIncrementer {
                increment: |x| *a_box += x,
            };
            (incrementer.increment)(1);
            (incrementer.increment)(1);
        }

        assert_eq!(*a_box, 12);
    }
}

#[test]
fn closure_box() {
    {
        fn create() -> Box<dyn FnOnce()> {
            Box::new(move || {
                println!("Hello, world");
            })
        }

        let a_fn = create();
        a_fn();
    }

    {
        // Prior versions
        fn create() -> Box<dyn FnBox> {
            Box::new(move || {
                println!("Hello, world");
            })
        }

        let a_box = create();

        // cannot move a value of type std::ops::FnOnce():
        // the size of std::ops::FnOnce() cannot be statically determined
        // a_box();

        a_box.call_box();

        trait FnBox {
            fn call_box(self: Box<Self>);
        }

        impl<F: FnOnce()> FnBox for F {
            fn call_box(self: Box<F>) {
                (*self)()
            }
        }
    }
}

#[test]
fn raw_identifiers() {
    fn r#match(x: i32, y: i32) -> bool {
        x == y
    }
    assert!(r#match(1, 1));
}

#[test]
fn must_use() {
    #[must_use]
    fn double(x: i32) -> i32 {
        2 * x
    }

    let x = double(1);
}

// Const functions
const SIX: i32 = const_foo(5);
const fn const_foo(x: i32) -> i32 {
    x + 1
}
