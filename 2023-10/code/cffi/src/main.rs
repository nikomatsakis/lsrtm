extern "C" {
    fn fibonacci(x: i32) -> i32;
    fn call_rust(x: i32);
}

#[no_mangle]
extern "C" fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let r = unsafe { fibonacci(22) };
    println!("fibonacci(22) = {r}");

    unsafe {
        call_rust(4);
    }
}
