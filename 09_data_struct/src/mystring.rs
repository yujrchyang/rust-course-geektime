use std::{fmt, ops::Deref};

const MINI_STRING_MAN_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAN_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAN_LEN];
        data[..len].copy_from_slice(bytes);
        MiniString {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let slice = &self.data[..self.len as usize];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(s) => s.deref(),
            MyString::Standard(s) => s.deref(),
        }
    }
}

impl<T> From<T> for MyString
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref().len() > MINI_STRING_MAN_LEN {
            true => MyString::Standard(value.as_ref().to_string()),
            false => MyString::Inline(MiniString::new(value)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Inline(ref mut v) => {
                let len = v.len as usize;
                let len1 = s.len();
                if len + len1 > MINI_STRING_MAN_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    v.data[len..len + len1].copy_from_slice(s.as_bytes());
                    v.len = (len + len1) as u8;
                }
            }
            MyString::Standard(ref mut v) => v.push_str(s),
        }
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("len1: {}, len2: {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    println!("{:?} - {:?}", s1, s2);
    println!(
        "s1: {} ({} bytes, {} chars), s2: {} ({} bytes, {} chars)",
        s1,
        len1,
        s1.len(),
        s2,
        len2,
        s2.len()
    );

    // MyString 可以使用一切 &str 接口
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));

    let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
    println!("s: {:?}", &*s);
    let s3: MyString = s.into();
    println!("s3: {:?}", &*s3);

    let mut s4: MyString = "Hello Try!".into();
    println!("s4: {:?}", s4);
    s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
    println!("s4: {:?}", s4);
}
