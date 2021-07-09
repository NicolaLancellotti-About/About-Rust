#![allow(dead_code)]
#![allow(unused_variables)]

#[test]
fn control_flow() {
    let number = if true {
        5
    } else if false {
        4
    } else {
        1
    };

    let result = loop {
        break 4;
    };

    'label: while number < 0 {
        break 'label;
    }

    let mut a = vec![10, 20, 30, 40, 50];
    for element in a.iter() {}
    for element in a.iter_mut() {}
    for element in a.into_iter() {}

    fn early_returns() {
        let x = Some(10);
        let value = match x {
            Some(x) => x,
            None => return,
        };
    }
}

#[test]
fn patterns() {
    // ________________________________________________
    // Irrefutable patterns

    {
        //let statement
        let (x, y, z) = (1, 2, 3);
    }

    {
        // function parameter
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }

    {
        fn foo(ref mut x: i32) {
            *x = 20;
        }
        let x = 10;
        foo(x);
        assert_eq!(x, 10);
    }

    // ____________________________
    // Destructuring
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point { x: 0, y: 7, z: 1 };
    let Point { x, y, z } = p;
    let Point { x: a, y: b, z: c } = p;

    match p {
        Point { x, y: 0, z } => println!("On the x axis at {} {}", x, z),
        Point { x: 0, .. } => println!("x is {}", x),
        Point { x, y, z } => println!("On neither axis: ({}, {} {})", x, y, z),
    }

    match &p {
        &Point { x, y: 0, z } => println!("On the x axis at {} {}", x, z),
        _ => println!("On the x axis at {}", x),
    }

    // ________________________________________________
    // Refutable patterns

    {
        // if let
        let k = Some(10);
        if let Some(k) = k {}
    }

    {
        // While let
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);

        while let Some(_) = stack.pop() {}
    }

    {
        match Some(100) {
            Some(0) => println!("one or two"),         // literal
            Some(1 | 2) => println!("1 or 2"),         // multiple pattern
            Some(3..=4) => println!("3 or 4"),         // ranges
            Some(id @ 5..=7) => println!("{}", id),    // bindings
            Some(x) if x > 10 => println!("anything"), // named variable
            Some(_) => println!("anything"),           // ignoring value
            _ => (),
        }

        match (10, 11) {
            (10, x) | (x, 10) => println!("{}", x),
            _ => (),
        }

        // Subslice patterns
        let _ = match [1, 2, 3] {
            [1, ..] => "starts with one",
            [.., 1] => "ends with one",
            rest => "something else",
        };

        //  automatically reference or de-reference in match statements.
        let arg = &Some("test".to_string());
        match arg {
            &Some(ref name) => println!("Hello {}!", name),
            &None => println!("I don't know who you are."),
        };

        //  matches!
        assert_eq!(matches!('b', 'A'..='Z' | 'a'..='z'), true);
    }

    // ________________________________________________
    // Box patterns

    {
        fn foo(_value: &Box<i32>) {}

        let k = Some(Box::new(10));

        if let Some(a) = k.as_ref() {
            foo(a);
        }

        if let Some(ref k_ref) = k {
            foo(k_ref);
        }
    }
}

#[test]
pub fn conditional_compilation() {
    // Defined in cargo.toml
    #[cfg(feature = "foo")]
    mod foo {
        pub fn bar() -> i32 {
            10
        }

        #[cfg(any(unix, windows))]
        pub fn baz() {}

        fn len(
            #[cfg(windows)] slice: &[u16],     // This parameter is used on Windows.
            #[cfg(not(windows))] slice: &[u8], // Elsewhere, this one is used.
        ) -> usize {
            slice.len()
        }

        #[cfg_attr(feature = "nightly", feature(core, std_misc))]
        pub fn foo_baz() {}

        // #[cfg_attr(a, b)]
        // Will be the same as #[b] if a is set by cfg attribute, and nothing otherwise.
    }

    // You need to compile with cargo build --features "foo"
    // assert_eq!(foo::bar(), 11);

    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {}
}
