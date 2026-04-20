use std::alloc::{GlobalAlloc, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe {
            let data = System.alloc(layout);
            eprintln!("Alloc: {:p}, Size: {}", data, layout.size());
            data
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
        eprintln!("Free: {:p}, Size: {}", ptr, layout.size());
    }
}

#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

fn main() {
    let data = Box::new(Matrix::default());
    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );
}
