#![allow(dead_code)]
#![allow(unused_variables)]

/*
 Only traits that are object-safe can be made into trait objects.

A trait is object-safe if
- the trait does not require that Self: Sized
- the trait does not contain associated consts or associated functions
- all of its methods are object-safe (see below)

 */

// ____________________________________________________________
// Sized Trait

#[test]
fn sized_trait() {
    {
        // A trait does not have an implicit Sized bound as this is incompatible
        // with trait objects where, by definition, the trait needs to work with
        // all possible implementors, and thus could be any size.
        trait Foo {}
        trait Bar: Sized {}

        struct Impl;
        impl Foo for Impl {}
        impl Bar for Impl {}

        let x: &dyn Foo = &Impl;
        // let y: &dyn Bar = &Impl; // Error: object unsafe
    }

    {
        trait TraitSized: Sized {
            fn baz(&self) -> &Self;

            fn foo(self) {
                let x = self;
            }

            fn bar()
            where
                Self: Default,
            {
                let x: Self = Default::default();
            }
        }

        // Error
        // impl TraitSized for str {
        //     fn baz(&self) -> &Self {
        //         self
        //     }
        // }
    }

    {
        trait TraitUnsized /* Default */ {
            fn baz(&self) -> &Self;

            fn foo(self)
            where
                Self: Sized,
            {
                let x: Self = self;
            }

            fn bar()
            where
                Self: Default, /* Default: Sized */
            {
                let x: Self = Default::default();
            }
        }

        impl TraitUnsized for str {
            fn baz(&self) -> &Self {
                self
            }
        }
    }
}

// ____________________________________________________________
// Associated consts or functions

fn associated_consts() {
    trait Trait {
        const ZERO: i32;
        fn foo() {}
    }

    impl Trait for i32 {
        const ZERO: i32 = 0;
    }

    // let f: &dyn Trait = &10; // Error
}

// ____________________________________________________________
// A method is object-safe if:
// - if it uses Self then it requires that Self: Sized
// - if it uses a generic type parameter then it requires that Self: Sized
// - otherwise it is object-safe

fn object_safe_methods() {
    trait Trait {
        fn foo(&self) {}

        // The following methods cannot be invoked on a trait-object

        fn bar1(&self, x: Self)
        where
            Self: Sized,
        {
        }
        // fn bar2(&self, x: Self) {}

        fn baz1<T>(&self, x: T)
        where
            Self: Sized,
        {
        }
        // fn baz2<T>(&self, x: T) {}

        fn consume(self)
        where
            Self: Sized,
        {
        }
    }

    impl Trait for i32 {}
    let f: &dyn Trait = &10; // Error
    f.foo();
}
