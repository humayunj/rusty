use core::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        // println!("r: {}", *r);
    }
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(&mut v, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Abs value of -3 {}", abs(-3));
    }

    unsafe {
        COUNTER += 1;
    }
    unsafe {
        println!("Counter: {}", COUNTER);
    }
    let u = Color { hsv: 0xFF2233 };

    unsafe {
        println!("{:#X}", u.rgb);
    }
}

unsafe fn dangerous() {}

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);
//     (&mut values[..mid], &mut values[mid..])
// }

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Hello from Rust");
}

static mut COUNTER: u32 = 0;

unsafe trait Foo {}

unsafe impl Foo for i32 {}

#[repr(C)]
union Color {
    hsv: i32,
    rgb: i32,
}
