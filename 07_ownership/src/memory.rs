use std::collections::HashMap;

macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}    {}    {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4}    {:8}    {:12}",
            stringify!($t),
            std::mem::size_of::<$t>(),
            std::mem::size_of::<Option<$t>>(),
            std::mem::size_of::<Result<$t, std::io::Error>>()
        );
    };
}

fn main() {
    test_struct();
    test_enum();
}

fn test_enum() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}

#[allow(unused)]
enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

fn test_struct() {
    println!(
        "sizeof s1: {}, s2: {}, s3: {}",
        std::mem::size_of::<S1>(),
        std::mem::size_of::<S2>(),
        std::mem::size_of::<S3>()
    );
    println!(
        "alignof s1: {}, s2: {}, s3: {}",
        std::mem::align_of::<S1>(),
        std::mem::align_of::<S2>(),
        std::mem::align_of::<S3>()
    );
}

#[allow(unused)]
struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

#[allow(unused)]
struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

#[allow(unused)]
#[repr(C)]
struct S3 {
    a: u8,
    b: u16,
    c: u8,
}
