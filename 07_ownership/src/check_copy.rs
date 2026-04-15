fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // 数值类型都实现了 copy
    is_copy::<i8>();
    is_copy::<u16>();
    is_copy::<f32>();
    is_copy::<usize>();

    // 函数指针本身是一个指针，也实现了 copy
    is_copy::<fn()>();

    // 裸指针实现了 copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // 对于数组、元组，内部成员是 copy 的那么他们也是 copy 的
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // // DST 类型不实现 copy
    // is_copy::<str>();
    // is_copy::<[u8]>();
    // is_copy::<dyn Send>();
    //
    // // 有堆内存的类型不实现 copy
    // is_copy::<String>();
    // is_copy::<Vec<u8>>();
    //
    // // 可变引用不是 copy
    // is_copy::<&mut String>();
    //
    // // 对于数组、元组，内部成员不是 copy 的那么他们也不是 copy 的
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}
