#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// __________________________________________

struct MyType {
    x: i32,
}

impl Hash for MyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
    }
}

fn compute_hash<T: Hash, H: Hasher>(value: &T, hasher: &mut H) -> u64 {
    value.hash(hasher);
    hasher.finish()
}

// __________________________________________

#[test]
fn simple() {
    let a = MyType { x: 10 };
    let mut hasher = DefaultHasher::new();
    let hash = compute_hash(&a, &mut hasher);
    println!("{}", hash);
}

// __________________________________________
// Custom hasher

#[derive(Default)]
struct CustomHasher {}

impl CustomHasher {
    fn new() -> CustomHasher {
        CustomHasher {}
    }
}

impl Hasher for CustomHasher {
    fn finish(&self) -> u64 {
        10
    }

    fn write(&mut self, bytes: &[u8]) {}
}

#[test]
fn custom_hash() {
    let a = MyType { x: 10 };
    let mut hasher = CustomHasher::new();
    let hash = compute_hash(&a, &mut hasher);
    println!("{}", hash);
}
