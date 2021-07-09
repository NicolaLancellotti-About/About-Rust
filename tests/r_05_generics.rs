#![allow(dead_code)]
#![allow(unused_variables)]

// ____________________________________________________________
// Generic Functions & Type Constraints

fn my_swap<T: Copy>(a: &mut T, b: &mut T) {
    let t = *a;
    *a = *b;
    *b = t;
}

fn foo_1<P: ToString + Clone, C: Default>(x: P, y: C) {}

// Generic Where Clauses
fn foo_2<P, C>(x: P, y: C)
where
    P: ToString + Clone,
    C: Default,
{
}

fn foo_3<P>(x: P)
where
    Option<P>: Default,
{
}

// ____________________________________________________________
// Generic Types

struct Value<T, U, const N: usize> {
    x: T,
    y: T,
    key: U,
    list: [T; N],
}

impl<T, U, const N: usize> Value<T, U, N>
where
    T: Copy,
{
    fn new(x: T, y: T, key: U) -> Value<T, U, N> {
        Value {
            x,
            y,
            key,
            list: [x; N],
        }
    }

    fn key(&self) -> &U {
        &self.key
    }
}

impl<T, U, const N: usize> Value<T, U, N>
where
    U: ToString,
{
    fn key_string(&self) -> String {
        self.key.to_string()
    }
}

// Equality constraints are not yet supported in where clauses
// impl<T> Value<T, T> where T == f32 {}
impl<U, const N: usize> Value<i32, U, N> {
    fn max(&self) -> i32 {
        self.x.max(self.y)
    }
}

impl<U, const N: usize> Default for Value<i32, U, N>
where
    U: Default,
{
    fn default() -> Self {
        let list: [i32; N];
        Value {
            x: 10,
            y: 10,
            key: Default::default(),
            list: [0; N],
        }
    }
}

#[test]
fn generic_types() {
    let value: Value<i32, char, 10> = Value::new(1, 0, 'a');
    assert_eq!(value.key(), &'a');
    assert_eq!(value.max(), 1);
    assert_eq!(value.key_string(), "a");
    let value: Value<i32, char, 10> = Default::default();
}

// ____________________________________________________________
// Turbo fish

#[test]
fn turbo_fish() {
    fn gen<T>() -> T
    where
        T: Default,
    {
        Default::default()
    }

    let x: i32 = gen();
    let x = gen::<i32>();
    assert_eq!(x, 0);
}

// ________________________________________________
// Generic Traits

trait Incrementer<T = i32 /* default type parameter */> /* default type parameter */> {
    fn increment(&self) -> T;
}

impl Incrementer for i32 {
    fn increment(&self) -> i32 {
        self + 1
    }
}

impl Incrementer<String> for i32 {
    fn increment(&self) -> String {
        (self + 1).to_string()
    }
}

#[test]
fn call_increment_generic() {
    fn increment_generic_1<T, I: Incrementer<T>>(value: I) -> T {
        value.increment()
    }

    fn increment_generic_2<T, I: Incrementer<T>>(value: &I) -> T {
        value.increment()
    }

    let x: i32 = increment_generic_1(10);
    assert_eq!(x, 11);
    let x: String = increment_generic_2(&10);
    assert_eq!(x, "11");
}

fn call_increment_trait_object() {
    fn increment_trait_object<T>(value: &dyn Incrementer<T>) -> T {
        value.increment()
    }

    let x: i32 = increment_trait_object(&10);
    assert_eq!(x, 11);
    let x: String = increment_trait_object(&10);
    assert_eq!(x, "11");
}

// ________________________________________________
// Inherited generic traits
trait Baz1<T>: Incrementer<T> {}
trait Baz2<T>: Incrementer<T>
where
    T: Copy,
{
}
trait Baz3: Incrementer<i32> {}

//________________________________________________
// Impl dyn trait adds inherent methods to the trait object.

trait K<Item> {
    fn value(&self) -> Item;
}

impl<T> dyn K<T> {
    fn same_value(&self) -> T {
        self.value()
    }
}

impl dyn K<i32> {
    fn value_plus_one(&self) -> i32 {
        self.value() + 1
    }
}

impl K<i32> for i32 {
    fn value(&self) -> i32 {
        *self
    }
}

#[test]
fn impl_dyn_trait() {
    let x: &dyn K<i32> = &10;
    assert_eq!(x.value_plus_one(), 11);
    assert_eq!(x.same_value(), 10);
}

// ____________________________________________________________
// By default, generic functions will only work on types that have a known size
// at compile time.
// All type parameters have an implicit bound of Sized
// The special syntax ?Sized can be used to remove this bound if it's not appropriate.

fn ref_sized<T: Sized /* Default */>(v: &T) {}
fn sized<T: Sized /* Default */>(v: T) {}
fn ref_dst<T: ?Sized>(v: &T) {} // Dynamically sized types

trait Foo {}
struct Struct {}
impl Foo for Struct {}

#[test]
fn test_sized() {
    let s = Struct {};
    ref_sized(&s);
    ref_dst(&s);
    sized(s);
}

#[test]
fn test_unsized() {
    let s: &str = "abc";
    // ref_sized(s);
    sized(s);
    ref_dst(s);

    let array = [1, 2, 3];
    let s = &array[..];
    // ref_sized(s);
    ref_dst(&s);
    sized(s);

    let obj = Box::new(Struct {}) as Box<dyn Foo>;
    ref_sized(&obj);
    ref_dst(&obj);
    sized(obj);

    let obj = Box::new(Struct {}) as Box<dyn Foo>;
    // ref_sized(&(*obj));
    ref_dst(&(*obj));
    // sized(*obj);

    struct S<T: ?Sized> {
        x: T,
    }
    let s = S { x: "123" };
}

// ____________________________________________________________
// Phantom Type

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Meter;
#[derive(Debug, Clone, Copy)]
struct Mile;

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

#[test]
fn phantom_data() {
    let one_meter: Length<Meter> = Length(1.0, PhantomData);
    let one_mile = Length(1.0, PhantomData::<Mile>);

    let two_miles = one_mile + one_mile;
    let two_meters = one_meter + one_meter;

    // let x = one_mile + one_meter; // Error
}
