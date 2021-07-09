#![allow(dead_code)]
#![allow(unused_variables)]

use std::borrow::Cow;
use std::ops::*;
use std::rc::*;

fn take_immutable(value: &String) {}
fn take_mutable(value: &mut String) {}

#[test]
fn box_smart_pointer() {
    {
        let mut p = Box::new(String::from("a"));
        *p = String::from("b");
        assert_eq!(*p, String::from("b"));
    }

    {
        let p = Box::new(String::from("a"));
        let value: String = *p; // moved
    }

    {
        let mut p = Box::new(String::from("a"));
        {
            let ref_p = &mut p;
            **ref_p = String::from("a");
        }
    }

    {
        let mut p = Box::new(String::from("a"));
        {
            let _mutable_ref: &mut String = p.deref_mut();
            *_mutable_ref = String::from("b");
        }
        {
            let immutable_ref: &String = p.deref();
            assert_eq!(*immutable_ref, String::from("b"));
            //let s = *immutable_ref; // cannot move out of borrowed content
        }
    }

    {
        // Implicit Deref Coercions
        let mut p = Box::new(String::from("a"));
        take_immutable(&mut p);
        take_mutable(&mut p);
    }
}

#[test]
fn rc_smart_pointer() {
    // Automatic Reference Count
    {
        let mut strong_ref = Rc::new(6);
        let _value = *strong_ref; // only for copy type
        strong_ref = Rc::new(7);
        assert_eq!(*strong_ref, 7);

        // clone
        let another_strong_ref = strong_ref.clone();
        let another_strong_ref2 = Rc::clone(&strong_ref);

        // weak
        let weak_ref = Rc::downgrade(&strong_ref);
        let option_strong_ref = weak_ref.upgrade();
        if let Some(_) = option_strong_ref {}
    }

    {
        // Equatable
        let five = Rc::new(5);
        let same_five = Rc::clone(&five);
        let other_five = Rc::new(5);

        // Value pointer equal
        assert!(Rc::ptr_eq(&five, &same_five));
        assert!(!Rc::ptr_eq(&five, &other_five));

        // Value equal
        assert!(five == same_five);
        assert!(five == other_five);
    }

    {
        // Returns the contained value, if the Rc has exactly one strong reference.
        let x = Rc::new(3);
        let result: Result<i32, Rc<i32>> = Rc::try_unwrap(x);
        assert_eq!(result, Ok(3));
    }

    {
        // Returns a mutable reference to the inner value, if there are no other Rc or
        //  Weak pointers to the same value.
        let mut x = Rc::new(3);
        *Rc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);
    }
    {
        //clone-on-write
        let mut data = Rc::new(5);
        let other_data = Rc::clone(&data); // Won't clone inner data
        *Rc::make_mut(&mut data) += 1; // Clones inner data
        assert_eq!(*data, 6);
        assert_eq!(*other_data, 5);
    }
}

#[test]
fn cow() {
    //The type Cow is a smart pointer providing clone-on-write functionality
    let data = "1".to_string();
    let mut x = Cow::Borrowed(&data);
    *x.to_mut() = "2".to_string();
    assert_eq!(*x, "2");
}
