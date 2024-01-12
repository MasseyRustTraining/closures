fn inc_f(a: i32) -> impl Fn(i32) -> i32 {
    move |x| x + a
    // struct ClosureType { a: &i32 }
}

#[test]
fn test_inc_f() {
    let f = inc_f(3);
    assert_eq!(4, f(1));
}

fn append_f(a: &str) -> impl Fn(String) -> String + '_ {
    |mut x| {
        x.push_str(a);
        x
    }
}

/*
#[test]
fn test_lifetimes() {
    let s = "!".to_string();
    let f = append_f(&s);
    drop(s);
    println!("{}", f("x".to_string()));
}
*/

#[test]
fn test_append_f() {
    let s = "!".to_string();
    let f = append_f(&s);
    assert_eq!("hello!", f("hello".to_string()));
}

fn lengthen_f(mut a: String) -> impl FnOnce() -> String {
    move || {
        a.push('!');
        a
    }
}

/*
#[test]
fn test_lifetimes() {
    let s = "!".to_string();
    let f = append_f(&s);
    drop(s);
    println!("{}", f("x".to_string()));
}
*/

#[test]
fn test_lengthen_f() {
    let s = "hello".to_string();
    let f = lengthen_f(s);
    assert_eq!("hello!", f());
}

fn make_id<T>() -> Box<dyn Fn(T) -> T> {
    Box::new(|x| x)
}

fn make_add<T, U, V>() -> impl Fn(U, V) -> T
where
    U: std::ops::Add<V, Output=T>
{
    |x, y| x + y
}

fn main() {
    let f = inc_f(1);
    println!("{}", f(1));

    let s = "!".to_string();
    let f = append_f(&s);
    println!("{}", f("hello".to_string()));

    let s = "hello".to_string();
    let f = lengthen_f(s);
    println!("{}", f());

    let f = make_id();
    println!("{}", f(3u8));

    let f = make_add();
    println!("{}", f(3u8, 5));
}
