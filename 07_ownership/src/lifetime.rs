use std::str::Chars;

fn main() {
    test_strtok();
    test_lifetime();
}

fn test_lifetime() {
    let name = String::from("hello");
    let _ = lifetime3(&name);
}

// 返回临时变量的引用
// fn lifetime1() -> &str {
//     let s = String::from("hello");
//     &s
// }

// name 所有权被转移，还是返回了临时变量的引用
// fn lifetime2(name: String) -> &str {
//     &name
// }

fn lifetime3<'a>(name: &'a str) -> Chars<'a> {
    name.chars()
}

fn test_strtok() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is : {}, s1 is {}, s is {}", hello, s1, s);
}

fn strtok<'a>(s: &mut &'a str, delim: char) -> &'a str {
    if let Some(i) = s.find(delim) {
        let prefix = &s[..i];
        let suffix = &s[(i + delim.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
