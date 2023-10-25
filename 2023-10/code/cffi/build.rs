fn main() {
    cc::Build::new().file("src/fib.c").compile("fib");
}
