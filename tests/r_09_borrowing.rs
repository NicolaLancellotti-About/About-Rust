#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

/*
    Borrowing
    At any given time, you can have either but not both of:
    * One mutable reference.
    * Any number of immutable references.
    References must always be valid.
*/

#[test]
fn aliasing() {
    {
        struct Test {
            a: String,
            b: String,
        }

        let mut x = Test {
            a: "a".to_string(),
            b: "b".to_string(),
        };

        let ref_a = &x.a;
        let ref_b = &mut x.b;

        // let ref_x = &x; // Error
        // let x = x; // Error

        assert_eq!(ref_a, "a");
        assert_eq!(ref_b, "b");
    }

    {
        // Slices
        let mut array = [1, 2, 3, 4, 5];
        let slice1 = &array[1..3];
        // let slice2 = &mut array[3..4];
        assert_eq!(slice1, &[2, 3]);
    }

    {
        // Iterator invalidation
        let v = vec![1, 2, 3];

        for _i in &v {
            // v.push(34); // Error
        }
    }
}

#[test]
fn lifetime() {
    {
        // Dangling reference
        let mut ref1: &i32 = &10;
        {
            let x = 10;
            // ref1 = &x; // `x` does not live long enough
        }
        assert_eq!(*ref1, 10);
    }

    {
        // Function lifetime
        fn greater<'a>(value1: &'a i32, value2: &'a i32) -> &'a i32 {
            if value1 > value2 {
                value1
            } else {
                value2
            }
        }

        {
            let value1 = 10;
            let value2 = 11;
            let result = greater(&value1, &value2);
            assert_eq!(*result, 11);
        }

        {
            let mut result = 0;
            {
                let value1 = 10;
                let value2 = 11;
                //result = greater(&value1, &value2); // Error
            }
            assert_eq!(result, 0);
        }
    }

    {
        // Extended scope
        fn foo<'a>(x: &'a String) -> &'a str {
            &x[0..1]
        }

        let mut s = String::from("123");

        let x: &str = foo(&s);

        let ref_s = &s;
        // let ref_mut_s = &mut s; // Error

        assert_eq!(ref_s, "123");
    }

    {
        // Static lifetime
        static NUM: i32 = 10;
        fn get_num() -> &'static i32 {
            &NUM
        }
        let num = get_num();
        assert_eq!(*num, 10);
    }

    {
        struct AStruct<'a> {
            a_ref: &'a i32,
        }

        impl<'a> AStruct<'a> {}

        let a = 10;
        let mut s = AStruct { a_ref: &a };

        {
            let a = 10;
            // s = AStruct{a_ref: &a}; // Error
        }
        assert_eq!(*s.a_ref, 10);
    }

    {
        // Lifetime bound
        struct Ref<'a, T: 'a>(&'a T); // :'a can be inferred

        let x = 10;
        let ref_x = Ref(&x);
    }

    {
        // Function Lifetime subtyping
        fn reference2<'a: 'b, 'b>(a: &'a i32, mut b: &'b i32) {
            b = a;
            // a = b; // Error
        }
    }

    {
        // Struct Lifetime subtyping
        struct Car<'o> {
            owner: &'o str,
        }

        // struct WorkShop<'c> {
        //     car: &'c Car<'c>,
        // }
        //
        // impl<'c> WorkShop<'c> {
        //     fn fix(&self) -> &'c str {
        //         &((*self.car).owner)
        //     }
        // }

        struct WorkShop<'c, 'o: 'c> {
            car: &'c Car<'o>,
        }

        impl<'c, 'o> WorkShop<'c, 'o> {
            fn fix(&self) -> &'o str {
                &((*self.car).owner)
            }
        }

        fn fix<'o>(car: Car<'o>) -> &'o str {
            let work_shop = WorkShop { car: &car };
            let owner = work_shop.fix();
            // car.owner
            owner
        }

        let owner = "owner";
        let car = Car { owner: &owner };
        let owner_ref = fix(car);
        assert_eq!(owner_ref, owner);
    }

    {
        /*
            Elision Rules
            - Each elided lifetime in input position becomes a distinct lifetime parameter.
            - If there is exactly one input lifetime position (elided or not), that lifetime
                is assigned to all elided output lifetimes.
            - If there are multiple input lifetime positions, but one of them is &self or
                &mut self, the lifetime of self is assigned to all elided output lifetimes.

            Otherwise, it is an error to elide an output lifetime.
        */
        fn foo1(v1: &i32, v2: &mut i32) {}

        fn foo1_explicit<'a, 'b>(v1: &'a i32, v2: &'b mut i32) {}

        fn foo2(v1: &i32) -> (&i32, &i32) {
            (v1, v1)
        }
        fn foo2_explicit<'a>(v1: &'a i32) -> (&'a i32, &'a i32) {
            (v1, v1)
        }

        struct A {
            value: i32,
        }
        impl A {
            fn foo3(&self, v1: &i32) -> &i32 {
                &self.value
            }

            fn foo3_explicit<'a, 'b>(&'a self, v1: &'b i32) -> &'a i32 {
                &self.value
            }
        }
    }
}

#[test]
fn trait_object_lifetimes() {
    /*
        The default lifetime of a trait object is 'static.
        If we have &'a X or &'a mut X, then the default is 'a.
        If we have a single T: 'a clause, then the default is 'a.
        If we have multiple T: 'a-like clauses, then there is no default; we must be explicit.
    */

    trait Trait {}

    struct Struct1<'a> {
        x: &'a i32,
    }

    impl<'a> Trait for Struct1<'a> {}

    struct Value<'a> {
        value: &'a i32,
    }

    struct Struct2<'a, T: 'a> {
        x: &'a i32,
        y: T,
    }

    impl<'a, T> Trait for Struct2<'a, T> {}

    struct Struct3<'a, 'b: 'a, T: 'a, U: 'b> {
        x1: &'a i32,
        x2: &'b i32,
        y1: T,
        y2: U,
    }

    impl<'a, 'b, T, U> Trait for Struct3<'a, 'b, T, U> {}

    fn trait_object_lifetime<'a: 'b, 'b: 'a, 'c>(
        num: &'a i32,
        num2: &'b i32,
        value1: Value<'a>,
        value2: Value<'a>,
        value3: Value<'a>,
        value4: Value<'b>,
        value5: Value<'a>,
        value6: Value<'b>,
    ) -> Box<dyn Trait + 'a> {
        let trait_object: Box<dyn Trait> = Box::new(Struct1 { x: &10 });
        let trait_object_explicit: Box<dyn Trait + 'static> = Box::new(Struct1 { x: &10 });

        let trait_object: Box<dyn Trait> = Box::new(Struct1 { x: &num });
        let trait_object_explicit: Box<dyn Trait + 'a> = Box::new(Struct1 { x: &num });

        let trait_object: Box<dyn Trait> = Box::new(Struct2 { x: &num, y: value1 });
        let trait_object_explicit: Box<dyn Trait + 'a> = Box::new(Struct2 { x: &num, y: value2 });

        let trait_object: Box<dyn Trait> = Box::new(Struct3 {
            x1: &num,
            x2: &num2,
            y1: value3,
            y2: value4,
        });
        let trait_object_explicit: Box<dyn Trait + 'a> = Box::new(Struct3 {
            x1: &num,
            x2: &num2,
            y1: value5,
            y2: value6,
        });
        return trait_object_explicit;
    }
}
