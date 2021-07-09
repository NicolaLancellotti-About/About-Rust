#![allow(dead_code)]
#![allow(unused_variables)]

use std::borrow::{Borrow, BorrowMut};
use std::num::ParseIntError;
use std::str::FromStr;

#[test]
fn from_into() {
    struct Type {
        x: i32,
    }

    impl From<i32> for Type {
        fn from(x: i32) -> Type {
            Type { x }
        }
    }

    impl FromStr for Type {
        type Err = ParseIntError;

        fn from_str(x: &str) -> Result<Self, Self::Err> {
            let x = x.parse::<i32>()?;
            return Ok(Type { x });
        }
    }

    let t = Type::from(10); // From
    assert_eq!(t.x, 10);

    let t: Type = 10.into(); // Into
    assert_eq!(t.x, 10);

    let t: Type = Type::from_str("10").unwrap(); // FromStr
    assert_eq!(t.x, 10);

    let flag = Type::from_str("-").is_err(); // FromStr
    assert_eq!(flag, true);
}

#[test]
fn from_str() {
    use std::str::FromStr;
    let bool_value: Option<bool> = FromStr::from_str("true").ok();
    assert_eq!(bool_value, Some(true));
}

#[test]
fn to_owned() {
    // The ToOwned trait generalizes Clone to construct owned data
    // from any borrow of a given type.
    let s: &str = "a";
    let s: String = s.to_owned();
}

#[test]
fn as_ref_borrow() {
    fn as_ref<T: AsRef<u64>>(num: &T) {
        assert_eq!(*num.as_ref(), 10);
    }
    fn borrow<T: Borrow<u64>>(num: &T) {
        assert_eq!(*num.borrow(), 10);
    }

    let boxed_num = Box::new(10);

    as_ref(&boxed_num);
    borrow(&boxed_num);
}

#[test]
fn as_mut_borrow_mut() {
    fn as_mut<T: AsMut<u64>>(num: &mut T) {
        *num.as_mut() += 1;
    }
    fn borrow_mut<T: BorrowMut<u64>>(num: &mut T) {
        *num.borrow_mut() += 1;
    }

    let mut boxed_num = Box::new(0);

    as_mut(&mut boxed_num);
    assert_eq!(*boxed_num, 1);

    borrow_mut(&mut boxed_num);
    assert_eq!(*boxed_num, 2);
}
