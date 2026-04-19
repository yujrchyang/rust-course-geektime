fn main() {
    test_type_derivation();
    // test_generaic();
}

fn test_type_derivation() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res: Vec<&i32> = a.iter().filter(|x| *x % 2 == 0).collect();
    println!("res: {:?}", res);
    println!("arr : {:?}", a);

    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res: Vec<i32> = b.into_iter().filter(|x| *x % 2 == 0).collect();
    println!("res: {:?}", res);
    // println!("arr : {:?}", b);

    let c = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = c.into_iter().filter(|x| *x % 2 == 0).collect::<Vec<_>>();
    println!("res: {:?}", res);
}

#[allow(unused)]
fn test_generaic() {
    use std::io::{BufWriter, Write};
    use std::net::TcpStream;

    #[derive(Debug)]
    struct MyWriter<W> {
        writer: W,
    }

    impl<W: Write> MyWriter<W> {
        pub fn new(writer: W) -> Self {
            Self { writer }
        }

        pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
            self.writer.write_all(buf.as_bytes())
        }
    }

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut writer = MyWriter::new(BufWriter::new(stream));
    writer.write("hello world!").unwrap();
}
