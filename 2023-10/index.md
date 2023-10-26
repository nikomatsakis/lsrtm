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

name: outline

# Outline

* The bad old days
    * Authoring calls through the C ABI
* Modern practices
    * cxx
    * pyo3
    * duchess
* Experience report

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

---

# Bindgen

[rust-lang/rust-bindgen](https://rust-lang.github.io/rust-bindgen/)

Autogenerate external declarations from C header files.

---

# CBindgen

[mozilla/cbindgen](https://github.com/mozilla/cbindgen)

> cbindgen creates C/C++11 headers for Rust libraries which expose a public C API.

```bash
> cargo install cbindgen
> cbindgen
```

---

# Wrapping external libraries

Best practices:

* Use `pkg-config` crate to probe for the library on system
* If you want to *vendor* (package the sources):
    * Include an env variable to disable vendoring
    * Only do it *after* `pkg-config` has a chance to look for the library
* Create a `foo-sys` that *just* exports the C APIs, and a `foo` that is more Rust-like and nice.

---

template: outline

.arrow.abspos.top190.left10[![Arrow](./images/Arrow.png)]

---

name: cxx

# [dtolnay/cxx](https://github.com/dtolnay/cxx)

.row[
.bg1.column30[C++]
.bg3.column30[Bridge]
.bg5.column30[Rust]
]

---

template: cxx

.arrow.abspos.top150.left80.rotNW[![Arrow](./images/Arrow.png)]

---

template: cxx

.arrow.abspos.top150.left550.rotNW[![Arrow](./images/Arrow.png)]

---

template: cxx

.arrow.abspos.top150.left350.rotNW[![Arrow](./images/Arrow.png)]

--

You supply:

```rust
#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    struct BlobMetadata {...} // Shared structs visible to both languages
    extern "Rust" {...} // Rust types and signatures exposed to C++.
    unsafe extern "C++" {...} // C++ types and signatures exposed to Rust
}
```

--

* Macro generates:
    * a header file (`main.rs.h`) C++ code can include
    * Rust code to reflect the C++ declarations
    * Code to check your C++ signatures are correctly transcribed

---

# cxx demo

---

template: outline

.arrow.abspos.top260.left40[![Arrow](./images/Arrow.png)]

---

# py03

[pyo3](https://pyo3.rs/v0.20.0/getting_started)

* install `pyenv` and create an environment
* `pyenv activate pyo3`
* `pip install maturin`
* `maturin develop` to build the sources

---

# Part 1: basics of exposing Rust fns to Python

```rust
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {...}

#[pymodule]
fn pyffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
}
```

```bash
> python -q
>>> import pyffi
>>> pyffi.sum_as_string(22, 44)
'66'
```

---

# Part 2: more complex types

```rust
#[pyfunction]
pub fn comma_join(a: Vec<String>) -> PyResult<String> {
    Ok(a.join(", "))
}
```

.arrow.abspos.top190.left10[![Arrow](./images/Arrow.png)]


.footnote[
    [Full set of conversions](https://pyo3.rs/v0.20.0/conversions/tables)
]

---

# Part 3: exceptions and errors

```rust
#[pyfunction]
pub fn comma_join_nonempty(a: Vec<String>) -> PyResult<String> {
    if a.is_empty() {
        return Err(PyValueError::new_err("empty list"));
    }
    Ok(a.join(", "))
}
```

---

# Part 4: interacting with Python values

```rust
#[pyfunction]
pub fn comma_join_py(a: &PyList) -> PyResult<String> { }
```

---

# Part 5: deriving `FromPyObject` on structs

```rust
#[derive(FromPyObject)]
pub struct RustyStruct {
    my_string: String,
}

#[pyfunction]
pub fn make_struct(a: RustyStruct) -> PyResult<String> {}
```

* You get:
    * field-by-field "duck typed" conversion

---

# Part 6: deriving `FromPyObject` on enums

```rust
#[derive(FromPyObject, Debug)]
pub enum TypeTest {
    ...
}
```

* You get:
    * attempts to check against multiple types

---

# Part 7: defining Python classes

```rust
#[pyclass(frozen)]
pub struct Character {
    name: String,
    age: u32,
}

#[pymethods] impl Character { ... }
```

* Python is GC'd:
    * How does that map to Rust?
    * What does that mean for mutating fields?

---

# Part 8: persisting Python references


