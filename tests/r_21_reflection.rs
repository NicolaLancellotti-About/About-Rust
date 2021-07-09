#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::Any;
use std::any::TypeId;
use std::boxed::Box;

#[test]
fn reflection() {
    {
        let t = TypeId::of::<i32>();
        println!("{:?}", t);
    }

    {
        let mut value = 10;
        let any_value: &mut dyn Any = &mut value;
        assert!(any_value.is::<i32>());

        match any_value.downcast_ref::<i32>() {
            Some(v) => {
                assert_eq!(v, &10);
            }
            None => (),
        }

        match any_value.downcast_mut::<i32>() {
            Some(v) => {
                *v = 11;
            }
            None => (),
        }
        assert_eq!(value, 11);
    }

    {
        let a_box: Box<dyn Any> = Box::new(10);
        match a_box.downcast::<i32>() {
            Ok(v) => {
                assert_eq!(*v, 10);
            }
            Err(_) => (),
        }
    }

    {
        fn type_name<T>() {
            println!("Instance of {}", std::any::type_name::<T>());
        }

        type_name::<i32>();
    }
}
