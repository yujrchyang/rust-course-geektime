use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

fn main() {
    test_hashmap_basic();
    test_hashmap_student();
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}

fn test_hashmap_student() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("jared", 18);
    student.hash(&mut hasher);

    let mut map = HashMap::new();
    map.insert(student, vec!["Math", "Writing"]);
    println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
}

fn test_hashmap_basic() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    assert_eq!(map.get(&'a'), None);
    assert_eq!(map.contains_key(&'a'), false);
    explain("removed", &map);

    map.shrink_to_fit();
    explain("shrink", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, capacity: {}", name, map.len(), map.capacity());
}
