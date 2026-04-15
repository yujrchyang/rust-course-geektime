use std::vec;

fn main() {
    test_reference_addr();
    test_reference_mut();
}

fn test_reference_mut() {
    let mut data = vec![1, 2, 3, 4];
    // let data_ref = vec![&data[0]];
    println!("data[0] addr: {:p}", &data[0]);

    for i in 0..100 {
        data.push(i);
    }
    println!("data[0] addr: {:p}", &data[0]);
    // println!("boxed: {:p}", &data_ref)
}

fn test_reference_addr() {
    let vec = vec![1, 2, 3, 4];
    let data = &vec;
    println!(
        "addr of value: {:p}({:p}), addr of vec {:p}, data {:p}",
        &vec, data, &&vec, &data
    );
    sum(data);
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &vec[0], &vec[1], &vec[2], &vec[3]
    );
}

fn sum(data: &Vec<u32>) {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
}
