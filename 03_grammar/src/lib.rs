const PI: f64 = 3.1415926;
static V: Vec<u8> = Vec::new();

#[allow(unused)]
fn where_is_pi() {
    let r = 10f64;
    println!("addr(r): {:p}, addr(PI): {:p}, addr(V): {:p}", &r, &PI, &V);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_where_is_pi() {
        where_is_pi();
    }

    #[test]
    fn test_struct() {
        #[derive(Debug)]
        struct Marker;

        #[derive(Debug)]
        #[allow(unused)]
        struct Color(u8, u8, u8);

        #[derive(Debug)]
        #[allow(unused)]
        struct Person {
            name: String,
            age: u8,
        }

        let marker = Marker {};
        let color = Color(128, 128, 128);
        let person = Person {
            name: "Jared".into(),
            age: 18,
        };
        println!(
            "marker: {:?}, color: {:?}, person: {:?}",
            marker, color, person
        );
    }

    #[test]
    fn test_enum() {
        #[derive(Debug)]
        #[allow(unused)]
        enum MyOption<T> {
            Some(T),
            None,
        }

        #[derive(Debug)]
        #[allow(unused)]
        enum Status {
            Ok = 0,
            BadName = 1,
            NotFound = 2,
            Internal = 3,
        }

        let opt = MyOption::Some("hello");
        let status = Status::NotFound;
        println!("opt is {:?}, status is: {:?}", opt, status);
    }
}
