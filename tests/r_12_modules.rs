#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/*
If an item is public, it can be accessed through any of its parent modules.
If an item is private, it can be accessed only by its immediate parent module
and any of the parent’s child modules.

- The crate keyword refers to the current crate.
- Absolute paths begin with a crate name, where the keyword crate refers to the
current crate.
- A foo.rs and foo/ subdirectory may coexist; mod.rs is no longer needed when
placing submodules in a subdirectory.

Our preference is to specify absolute paths because it’s more likely to move
code definitions and item calls independently of each other.
*/

pub mod foo1 {
    fn f() {
        let x: i32 = self::bar2();

        // let x: f32 = crate::foo2::bar2();
        let x: f32 = super::foo2::bar2();
        {
            use super::foo2;
            let x: f32 = foo2::bar2();
        }
        {
            use super::foo2 as ff;
            let x: f32 = ff::bar2();
        }
        {
            use super::foo2::*;
            let x: f32 = bar2();
        }
    }

    fn bar2() -> i32 {
        10
    }
}

pub mod foo2 {
    pub fn bar2() -> f32 {
        1.1
    }

    pub(crate) fn f1() {}
    pub(super) fn f2() {}
    //    pub(in a::b::c) fn f1() {}
}

// Nested import with use
use std::{
    fs::File,
    path::{Path, PathBuf},
};

// This allows you to import a trait's impls, and not have the name in the namespace
use std::io::Read as _;
// Allowed as there is only one `Read` in the module.
pub trait Read {}

mod common;
#[test]
fn t() {
    common::common_fn();
}
