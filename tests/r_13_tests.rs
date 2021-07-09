#![allow(dead_code)]
#![allow(unused_variables)]

/*
Test all crate
cargo test --all

Test main crate
cargo test

Test specific crate
cargo test -p crate_name
example: cargo test -p local-crate

Test pattern
cargo test <pattern>

Test ignored tests
cargo test -- --ignored

see printed value for passing test
cargo test -- --nocapture
*/

fn func() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(func());

        assert_eq!(4, 4);
        assert_ne!(4, 5);

        assert_eq!(1, 1, "error message");
    }

    #[test]
    #[should_panic(expected = "same text")]
    fn test_panic() {
        panic!("same text");
    }

    #[test]
    #[ignore]
    fn ignored() {
        assert!(true);
    }
}

use lib;

#[test]
fn test_lib() {
    assert_eq!(lib::increment(9), 10);
}
