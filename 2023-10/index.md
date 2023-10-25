class: center
name: title
count: false

# LSRTM 2023-10: FFI

.p60[![Ferris](./images/ferris.svg)]

.me[.grey[*by* **Nicholas Matsakis**]]
.left[.citation[View slides at `https://nikomatsakis.github.io/lsrtm/2023-10`]]

---

# How to participate

.p50[
    ![Moira Rose, I have questions](./images/i-have-questions.gif)
]

---

# How to participate

## Have a question?

* Do not use chime chat!
    * Start a 🧵 in slack channel `#learn-some-rust-this-month-interest`!
    * Better yet, come on video!

---

# How to participate

## Have a question?

## All questions are welcome

.p40[
    ![no deprecatory mocking](./images/no-deprecatory-mocking.gif)
]

---

# FFI

FFI = "Foreign Function Interface"

Ability to call *in-process* between languages

---

# Do you really *want* FFI?

Alternatives:

* HTTP
* Microservices
* IPC -- see [JSII](https://github.com/aws/jsii)

---

# Reasons to use FFI

* Vended libraries

--
* Sharing large data structures, high perf requirements

--
* Testing code

---

# Cautionary tales

![Corro](./images/corro.svg)

---

# Special role for C

* C has historically been the "lingua franca"
* Rust can do the same!

---

# PART 1: Calling C from Rust, easy case

---

# C FFI PART 1: Key points

* `extern "C" { .. }`
    * Unsafe to call
* Linking
    * If you are calling a system library, reach out and follow best practices

---

# C FFI PART 2: Calling Rust from C

---

# C FFI PART 2: Key points

* An `extern "C"` Rust function uses C ABI
* 
* Things to watch out for
    * Unhandled panic
    * setjmp, longjmp, C++ exceptions
        * Unwinding across a "C" boundary is 

---

# C FFI PART 3: Structures

---

# C FFI PART 3: Key points

* Struct layout: `#[repr(C)]`
* 

---

# Bindgen

[rust-lang/rust-bindgen](https://rust-lang.github.io/rust-bindgen/)

---

# CBindgen

https://github.com/mozilla/cbindgen