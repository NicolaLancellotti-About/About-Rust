#![allow(dead_code)]
#![allow(unused_variables)]

//________________________________________________
// Trait with Associated types

pub trait Incrementer {
    type Item;
    fn increment(&self) -> Self::Item;
}

impl Incrementer for i32 {
    type Item = i32;

    fn increment(&self) -> i32 {
        self + 1
    }
}

#[test]
fn trait_objects() {
    //let x: Incrementer<Item = i32>; // Error

    let x: &dyn Incrementer<Item = i32> = &10;

    struct Struct<T: ?Sized> {
        data: T,
    }
    let x: &Struct<dyn Incrementer<Item = i32>> = &Struct { data: 1 };
    assert_eq!(x.data.increment(), 2);
}

#[test]
fn call_increment_generic() {
    fn increment_generic_1<I: Incrementer>(value: I) -> I::Item {
        value.increment()
    }
    fn increment_generic_2<I: Incrementer>(value: &I) -> I::Item {
        value.increment()
    }

    let x: i32 = increment_generic_1(10);
    assert_eq!(x, 11);
    let x: i32 = increment_generic_2(&10);
    assert_eq!(x, 11);

    // Constraints
    fn increment_generic_3<I: Incrementer>(value: I) -> I::Item
    where
        I::Item: Clone,
    {
        value.increment()
    }

    fn increment_generic_4<I: Incrementer<Item = i32>>(value: I) -> i32 {
        value.increment()
    }
}

#[test]
fn call_increment_trait_object() {
    fn increment_trait_object<T>(value: &dyn Incrementer<Item = T>) -> T {
        value.increment()
    }

    let x: i32 = increment_trait_object(&10);
    assert_eq!(x, 11);

    // Constraints

    fn increment_trait_object_2<T>(value: &dyn Incrementer<Item = T>) -> T
    where
        T: Clone,
    {
        value.increment()
    }

    fn increment_trait_object_3(value: &dyn Incrementer<Item = i32>) -> i32 {
        value.increment()
    }
}

//________________________________________________
// Trait with Associated const

#[test]
fn associated_const() {
    trait Trait {
        const VALUE: bool = false;

        type Item;
        const ZERO: Self::Item;
    }

    impl Trait for i32 {
        type Item = i32;
        const ZERO: Self::Item = 0;
    }

    impl Trait for f32 {
        type Item = f32;
        const ZERO: Self::Item = 0.0;
    }

    assert_eq!(i32::ZERO, 0);
    assert_eq!(f32::ZERO, 0.0);
    assert_eq!(i32::VALUE, false);
    assert_eq!(f32::VALUE, false);
}

// ________________________________________________
// Associated Types with a Generic Where Clause
trait P2
where
    Self::Item: Clone,
{
    type Item: Copy;
}

// ________________________________________________
// Inherited trait with Associated types
trait Baz1: Incrementer {}
trait Baz2: Incrementer
where
    Self::Item: Copy,
{
}
trait Baz3: Incrementer<Item = i32> {}

//________________________________________________
// Impl dyn trait adds inherent methods to the trait object.

trait K {
    type Item;
    fn value(&self) -> Self::Item;
}

impl<T> dyn K<Item = T> {
    fn same_value(&self) -> T {
        self.value()
    }
}

impl dyn K<Item = i32> {
    fn value_plus_one(&self) -> i32 {
        self.value() + 1
    }
}

impl K for i32 {
    type Item = i32;
    fn value(&self) -> i32 {
        *self
    }
}

#[test]
fn impl_dyn_trait() {
    let x: &dyn K<Item = i32> = &10;
    assert_eq!(x.value_plus_one(), 11);
    assert_eq!(x.same_value(), 10);
}
