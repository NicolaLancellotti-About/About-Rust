#![allow(dead_code)]
#![allow(unused_variables)]

#[test]
fn check() {
    fn foo(x: Option<i32>) -> Option<i32> {
        let y = x?;
        Some(y + 1)
    }

    let x: Option<i32> = foo(Some(2));
    assert_eq!(x.is_some(), true);
}
#[test]
fn unwrap() {
    assert_eq!(Some(1).unwrap(), 1);
    assert_eq!(None.unwrap_or(1), 1);
    assert_eq!(None.unwrap_or_else(|| 2 * 10), 20);
    assert_eq!("abc".parse::<i32>().ok().unwrap_or_default(), 0);
}

#[test]
fn take() {
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));
}

#[test]
fn insert_if_none() {
    let mut x = None;
    let y: &mut u32 = x.get_or_insert(1);
    assert_eq!(y, &1);
}

#[test]
fn map() {
    let x = Some(String::from("Hello, World!"));
    assert_eq!(x.map(|s| s.len()), Some(13));

    let x: Option<&str> = None;
    assert_eq!(x.map_or(1, |v| v.len()), 1);

    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * 10, |v| v.len()), 20);
}

#[test]
fn or_and() {
    assert_eq!(Some(1).or(Some(2)), Some(1));
    assert_eq!(None.or(Some(2)), Some(2));
    assert_eq!(None.or_else(|| Some(2)), Some(2));

    assert_eq!(Some(2).and(Some('a')), Some('a'));
    assert_eq!(Some(2).and_then(|v| { Some(v + 1) }), Some(3));
    assert_eq!(Option::<i32>::None.and(Some('a')), None);
}

#[test]
fn to_result() {
    assert_eq!(Option::<&str>::None.ok_or(0), Err(0));
    assert_eq!(Option::<&str>::None.ok_or_else(|| 0), Err(0));

    assert_eq!(Some(1).ok_or(0), Ok(1));
}
