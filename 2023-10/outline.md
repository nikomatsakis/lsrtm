# Idea: 

We will work through how to connect via FFI to multiple languages.

* C code
    * bindgen
    * cbindgen
    * 
* C++ (via cxx)
* Python (via PyO3)
* Javascript (via nsapi.rs)
* Java (via duchess)

# Outline

* What is FFI?
    * Calling from one language to another
* Alternatives to FFI
    * HTTP, microservices
    * IPC
* Reasons to use FFI:
    * Vended libraries: Rust as portable code base
    * Large data structures to be shared
    * Testing code
* C was special
    * No runtime to speak of
    * Lingua franca
* Rust can play that role
* Example:
    * Compute Fibonnacci in C
    * Call it from Rust
    * Vice versa
* 
* Some gotchas to be 

* First question:
    * Is FFI what you want?
* Why use FFI?
    * Wrapping one language to access from another
    * Vended libraries
    * Testing code
* Dangers of FFI
    * Unsafe code (!)
    * Most bugs occur at the boundary
* A trend:
    * Libraries that make FFI easy **and safe**

# C / C++

* Explain what the C ABI is

```rust
extern "C" {
    ...
}
```

# the importance of `repr(C)`

# cxx

* show how cxx works

# Example

* 

# Python



