use std::borrow::Cow;

use serde::Deserialize;
use url::Url;

fn main() {
    test_cow1();
    test_cow2();
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed: {}", v),
        Cow::Owned(v) => format!("Owned: {}", v),
    }
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn test_cow1() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    println!("key: {}, value: {}", k, v);

    k.to_mut().push_str("_lala");
    print_pairs((k, v));
    print_pairs(pairs.next().unwrap());
    print_pairs(pairs.next().unwrap());
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn test_cow2() {
    let input = r#"{ "name": "Tyr", "age": 18 }"#;
    let user: User = serde_json::from_str(input).unwrap();
    match user.name {
        Cow::Borrowed(v) => println!("borrowed: {}", v),
        Cow::Owned(v) => println!("owned: {}", v),
    }
}
