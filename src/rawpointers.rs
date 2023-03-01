fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;

    // Safe because r1 and r2 were obtained from references and so are guaranteed to be non-null and
    // properly aligned, the objects underlying the references from which they were obtained are
    // live throughout the whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
    }
}
