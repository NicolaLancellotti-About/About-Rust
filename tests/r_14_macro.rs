#![allow(dead_code)]
#![allow(unused_variables)]

// Declarative Macros

/*
    Designators:

    block
    expr is used for expressions
    ident is used for variable/function names
    item
    pat (pattern)
    path
    stmt (statement)
    tt (token tree)
    ty (type)
*/

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}
create_function!(foo);

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };

    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! find_min {

    ($x:expr) => ($x);

    ($x:expr, $($y:expr),+) => (
        ::std::cmp::min($x, find_min!($($y),+))
    )
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    (eval $e:expr, $(eval $es:expr),+) => (
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    );
}

#[test]
fn test_macro() {
    say_hello!();
    foo();
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

// Procedural Macros

// 1. Custom #[derive] macros
#[derive(Debug)]
struct AStruct {
    name: String,
}
// 2. Attribute-like macros
// 3. Function-like macros
