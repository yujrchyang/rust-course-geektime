fn main() {
    test_closure_mut();
    test_closure();
}

fn test_closure() {
    let v1 = vec![0u8; 1024];
    let v2 = vec![0u8; 1023];

    let mut c1 = |x: u64| v1.len() as u64 * x;
    let mut c2 = move |x: u64| v2.len() as u64 * x;

    println!("direct call: {}", c1(2));
    println!("direct call: {}", c2(2));

    println!("call: {}", call(3, &c1));
    println!("call: {}", call(3, &c2));

    println!("call_mut_new: {}", call_mut_new(4, &mut c1));
    println!("call_mut_new: {}", call_mut_new(4, &mut c2));

    println!("call_once_new: {}", call_once_new(5, c1));
    println!("call_once_new: {}", call_once_new(5, c2));
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut_new(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once_new(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

fn test_closure_mut() {
    let mut name1 = String::from("hello");
    let mut name2 = String::from("world");

    let mut c1 = || {
        name1.push_str(" Jared");
        println!("c1: {}", name1);
    };

    let mut c2 = move || {
        name2.push_str(" Jared");
        println!("c2: {}", name2);
    };

    c1();
    c2();

    call_mut(&mut c1);
    call_mut(&mut c2);

    call_once(c1);
    call_once(c2);
}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}
