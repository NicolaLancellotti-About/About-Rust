#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::num::Wrapping;
use std::path;

#[test]
fn bool() {
    let x: bool = true;
    use std::ops::Not;
    assert_eq!(x.not(), false);
}

#[test]
fn numeric_types() {
    println!("{}", std::f64::consts::SQRT_2);

    // Wrapping arithmetic
    {
        let zero = 0u32;
        let one = 1u32;

        // panic in debug mode
        // let difference = zero - one;
        // assert_eq!(std::u32::MAX, difference);

        let difference = zero.wrapping_sub(one);
        assert_eq!(std::u32::MAX, difference);
    }

    {
        let zero = Wrapping(0u32);
        let one = Wrapping(1u32);
        let difference = zero - one;
        assert_eq!(::std::u32::MAX, difference.0);
    }
}

#[test]
fn slices() {
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(&[10, 40], v.get(0..2).unwrap());
}

#[test]
fn array() {
    let array = [1, 2, 3];
    assert_eq!(array, [1, 2, 3]);
    let array = [10; 2];
    assert_eq!(array, [10, 10]);
}

#[test]
fn iterators() {
    let array = [1, 2];
    let mut iter = array.iter();
    assert_eq!(Some(&1), iter.next());
}

#[test]
fn recursion() {
    let array = [1, 2, 3, 4];

    fn foo(x: &[i32]) {
        match x.split_first() {
            Some((value, other)) => {
                println!("{}", value);
                foo(other)
            }
            None => return,
        }
    }

    foo(&array);
}

#[test]
fn default() {
    let array: [Option<i32>; 2];
    array = Default::default();
    assert_eq!(array, [None, None]);
}

#[test]
fn ord() {
    let result = [1, 2].lt(&[2, 1]);
    assert_eq!(result, true);

    use std::cmp::Ordering;
    let result = [1, 2].partial_cmp(&[1, 2]);
    assert_eq!(result, Some(Ordering::Equal));
}

#[test]
fn eq() {
    assert_eq!([1].eq(&[1]), true);
}

#[test]
fn print() {
    use std::fmt;
    struct Type1 {
        x: i32,
    }

    impl fmt::Display for Type1 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x = {}", self.x)
        }
    }

    let x = Type1 { x: 10 };
    assert_eq!(x.to_string(), "x = 10"); // ToString trait
    println!("{}", x); // Display trait

    {
        // fmt
        println!("{:|^+20.2}", 12.123456789);
        println!("{:|^+20.1$}", 12.123456789, 2);
        println!("{:|^+20.*}", 2, 12.123456789);
    }

    {
        println!("{:?}", [1, 2, 3]);
        // Pretty Printing
        println!("{:#?}", [1, 2, 3]);
    }
}

#[test]
fn read_file() {
    let file_name = "./files/file.txt";
    let mut f = File::open(file_name).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error");

    let dir = path::Path::new("./target/files");
    fs::create_dir_all(dir).unwrap();
    fs::copy(file_name, dir.join("file2.txt")).unwrap();
    let _ = fs::hard_link(file_name, dir.join("file3.txt"));
}
