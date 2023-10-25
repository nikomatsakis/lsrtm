#[no_mangle] // <-- note this attribute
pub extern "C" fn factorial(n: i32) -> i32 {
    if n == 0 {
        // panic!("Uh oh");
        1
    } else {
        n * factorial(n - 1)
    }
}
