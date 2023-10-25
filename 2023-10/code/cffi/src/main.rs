#![allow(warnings)]

// PART 1

extern "C" {
    fn fibonacci(x: i32) -> i32;
}

fn part_1() {
    let r = unsafe { fibonacci(22) };
    println!("fibonacci(22) = {r}");
}

#[no_mangle] // <-- note this attribute
extern "C" fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// PART 2

extern "C" {
    fn call_rust(x: i32);
}

fn part_2() {
    unsafe {
        call_rust(4);
    }
}

// PART 3

extern "C" {
    fn test_struct(x: *const TestStruct) -> TestStruct;
}

#[derive(Debug)]
struct TestStruct {
    a: u16,
    b: u64,
    c: u16,
}

fn part_3() {
    let ts = unsafe {
        test_struct(&TestStruct {
            a: 22,
            b: 44,
            c: 66,
        })
    };
    println!("ts = {ts:?}");
}

fn main() {
    part_1();
    part_2();
    part_3();
}
