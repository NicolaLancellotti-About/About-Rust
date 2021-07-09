#![allow(dead_code)]
#![allow(unused_variables)]

trait ATrait {
    fn foo(&self) -> i32 {
        1
    }
}

// derived trait
#[derive(Debug)]
struct Value {
    x: i32,
    y: i32,
}

impl ATrait for Value {
    fn foo(&self) -> i32 {
        self.x + self.y
    }
}

// ____________________________________________________________

#[test]
fn trait_objects() {
    let value = Value { x: 10, y: 10 };
    let x: &dyn ATrait = &value;
    assert_eq!(x.foo(), 20);
}

// ____________________________________________________________
// Super trait

trait ASuperTrait1 {}
trait ASuperTrait2 {}
trait AnotherTrait3: ASuperTrait1 + ASuperTrait2 {}

struct Name;
impl AnotherTrait3 for Name {}
impl ASuperTrait1 for Name {}
impl ASuperTrait2 for Name {}

// ____________________________________________________________
// Fully Qualified Syntax for Disambiguation

trait Foo {
    fn f(&self) -> String;
    fn foo_bar() -> String;
}

trait Bar {
    fn f(&self) -> String;
    fn foo_bar() -> String;
}

struct Baz;

impl Foo for Baz {
    fn f(&self) -> String {
        "Foo".to_string()
    }
    fn foo_bar() -> String {
        "Foo".to_string()
    }
}

impl Bar for Baz {
    fn f(&self) -> String {
        "Bar".to_string()
    }
    fn foo_bar() -> String {
        "Bar".to_string()
    }
}

impl Baz {
    fn f(&self) -> String {
        "Baz".to_string()
    }
    fn foo_bar() -> String {
        "Baz".to_string()
    }
}

#[test]
fn fully_qualified_syntax_for_disambiguation() {
    let b = Baz;
    assert_eq!(b.f(), "Baz");

    // Angle bracket form
    assert_eq!(<Baz as Foo>::f(&b), "Foo");
    assert_eq!(<Baz as Bar>::f(&b), "Bar");
    assert_eq!(<Baz>::f(&b), "Baz");

    assert_eq!(<Baz as Foo>::foo_bar(), "Foo");
    assert_eq!(<Baz as Bar>::foo_bar(), "Bar");
    assert_eq!(<Baz>::foo_bar(), "Baz");

    // Universal function call syntax
    assert_eq!(Foo::f(&b), "Foo");
    assert_eq!(Bar::f(&b), "Bar");
    assert_eq!(Baz::f(&b), "Baz");
}

// ____________________________________________________________
// Sealed trait

#[test]
fn sealed() {
    mod a {
        /// This trait is sealed and cannot be implemented for types outside this crate.
        pub trait TheTrait: private::Sealed {
            fn foo(&self);
        }

        mod private {
            pub trait Sealed {}
        }

        impl TheTrait for usize {
            fn foo(&self) {}
        }
        impl private::Sealed for usize {}
    }

    let x: &dyn a::TheTrait = &10;
    x.foo();
}

// ____________________________________________________________
// Conforming all types, which conform to a trait, to a trait

trait Hello {
    fn hello_string(&self) -> String;
}

impl<T: ToString> Hello for T {
    fn hello_string(&self) -> String {
        let mut hello = "Hello ".to_string();
        hello.push_str(&(self.to_string()));
        return hello;
    }
}

#[test]
fn conforming_all_types() {
    assert_eq!(10.hello_string(), "Hello 10");
}

//________________________________________________
// Impl dyn trait adds inherent methods to the trait object.

trait K {
    fn value(&self) -> i32;
}

impl dyn K {
    fn value_plus_one(&self) -> i32 {
        self.value() + 1
    }
}

impl K for i32 {
    fn value(&self) -> i32 {
        *self
    }
}

#[test]
fn impl_dyn_trait() {
    let x: &dyn K = &10;
    assert_eq!(x.value_plus_one(), 11);
}

// ____________________________________________________________
// Impl trait is existential in an output position

#[test]
fn impl_trait_output() {
    fn foo<T: Copy + PartialEq + std::fmt::Debug>(x: T) -> impl Copy + PartialEq + std::fmt::Debug {
        x
    }

    let x1 = foo(1);
    let x2 = foo(1);
    assert_eq!(x1, x2);

    let y = foo('a');
    // assert_eq!(x1, y);

    fn foo1() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let x = foo1();
}

// ____________________________________________________________
// Impl trait is universal in an input position

#[test]
fn impl_trait_input() {
    //  syntax sugar for 'trait bound'
    fn foo(_x: impl Copy, y: impl Copy) {}
    foo(10, true);
}
