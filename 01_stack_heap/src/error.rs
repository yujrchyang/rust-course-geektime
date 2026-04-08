fn main() {
    let name = "Jared".to_string();
    std::thread::spawn(move || println!("hello {}", name));
    std::thread::sleep(std::time::Duration::from_millis(100));
}
