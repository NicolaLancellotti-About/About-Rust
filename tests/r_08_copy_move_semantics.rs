#![allow(dead_code)]
#![allow(unused_variables)]

// __________________________________________
// Copy Semantic

#[derive(Copy, Clone)]
/*
impl Copy for CopiableValue {}
impl Clone for CopiableValue {
    fn clone(&self) -> CopiableValue {
        *self
    }
}
*/
struct CopiableValue {
    value: i32,
}

#[test]
fn copy() {
    let x = CopiableValue { value: 10 };
    let mut y = x;
    y.value += 1;
    assert_ne!(x.value, y.value)
}

// __________________________________________
// Move semantic

struct MovableValue {
    value: i32,
}

#[test]
fn test_move() {
    let x = MovableValue { value: 10 };
    let mut y = x;
    y.value += 1;
    // assert_ne!(x.value, y.value) // borrow of moved value: `x`
}

#[test]
fn move_and_borrowing() {
    let x = MovableValue { value: 10 };
    let ref_x1 = &x;
    // let y = x; // cannot move out of `x` because it is borrowed
    let ref_x2 = &*ref_x1;
}

#[test]
fn partially_moved() {
    struct Value {
        s1: String,
        s2: String,
    }

    let x = Value {
        s1: String::from("a"),
        s2: String::from("a"),
    };
    drop(x.s1);

    // let y = x; // use of partially moved value
}

#[test]
fn partially_moved_drop() {
    //It is not possible to partially move a type which implements the `Drop` trait
    struct Value {
        s: String,
    }
    impl Drop for Value {
        fn drop(&mut self) {}
    }

    let x = Value {
        s: String::from("a"),
    };
    // drop(x.s); // cannot move out of type
}

#[test]
fn values_match() {
    {
        let mut x = String::from("test");

        // discriminant: lvalue
        // x is not moved
        match x {
            ref mut y => *y = String::from("test1"),
        }
        assert_eq!(x, "test1");
    }

    {
        let x = String::from("test");

        // discriminant: rvalue
        // x is moved
        match x {
            y => y,
        };
        // let x = x; // use of moved value: `x`
    }

    {
        let mut x = String::from("test");
        {
            let ref_x = &mut x;
            // discriminant: rvalue
            // ref_x is moved
            match ref_x {
                y => *y = String::from("test1"),
            }
            // let ref_x = ref_x; // use of moved value: `ref_x`
        }
        assert_eq!(x, "test1");
    }

    {
        let mut x = String::from("test");
        {
            let mut ref_x = &mut x;
            // discriminant: lvalue
            // ref_x is not moved
            match ref_x {
                ref mut y => **y = String::from("test1"),
            }
            let ref_x = ref_x;
        }
        assert_eq!(x, "test1");
    }
}

#[test]
fn rvalue_in_lvalue_context() {
    let x = &10;
    assert_eq!(*x, 10);
}
