#![allow(dead_code)]
#![allow(unused_variables)]

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

        #[cfg_attr(feature = "nightly", feature(core, std_misc))]
        pub fn foo_baz() {}

        // #[cfg_attr(a, b)]
        // Will be the same as #[b] if a is set by cfg attribute, and nothing otherwise.
    }

    // You need to compile with cargo build --features "foo"
    // assert_eq!(foo::bar(), 11);

    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {}
}

#[test]
fn local_crates() {
    let value = local_crate::get10();
    assert_eq!(value, 10);
}
