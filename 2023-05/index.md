class: center
name: title
count: false

# LSRTM 2023-05: Error handling

.p60[![Ferris](./images/ferris.svg)]

.me[.grey[*by* **Nicholas Matsakis**]]
.left[.citation[View slides at `https://nikomatsakis.github.io/lsrtm-2023-05/`]]

---

# Error handling is hard

???

Gonna start by saying I'm not proud of Rust's error handling.

But neither am I ashamed.

Error handling is hard. No language really gets it right. 

Rust, I think, gets closer than most, but we still need work.

---

# Overview

* What not to do
* Error handling in libraries
* Error handling in applications
* Case study: cargo and semantic backtraces
* Case study: lock poisoning in libstd
* Case study: network service error recovery

---

# What not to do

Where error handling can go wrong

---

# C

```c
int main() {
    do_something();
    return 0;
}

int do_something() { ... }
```

* Wait, is this `int` an error code?
* If so, does `0` indicate "no failure"? Or does it mean "false, operation failed"?
* When things fail, how can I find out something more than "failed"?

---

# Java in theory

* Exceptions are awesome! Checked exceptions tell you exactly how things can go wrong!

---

# Java in practice

```java
void doSomething() throws ThisError, ThatError, TheOtherError
{
    ...
}
    
interface Transformer {
    void map() throws TransformerException; // ??
}
```

omg just use RuntimeException everywhere

---

# See any potential problems with this code?

```java
void doSomething() {
    activeConnections += 1;
    sendMessage();
    activeConnections -= 1;
}
```

(Answer on slack)

---

# Rust error handling mechanisms

* `panic!`
* `Result`

---

# Panics

Specifically meant for **irrecoverable** errors

To create a panic:

```rust
panic!("message");
```

---

# What happens when you panic

What happens when you panic?

--

![Trick question](./images/trick-question.gif)

.footnote[
    Ah, "My Cousin Vinny"! Best movie ever.
]

---

# What happens when you panic

It depends! Two panic modes:

* Abort ==>
    * Process ends. Immediately. Do not pass go, do not collect $200.

.center[.p40[
    ![Do not pass go](./images/do-not-pass-go.gif)
]]

---

# What happens when you panic

It depends! Two panic modes:

* Abort ==>
    * Process ends. Immediately. Do not pass go, do not collect $200.
* Unwind ==> 
    * Walk up the stack, running all destructors.
    * Once stack is unwound, terminate the current thread.

--

Choice of panic mode is determined by the final application
(typically).

---

# Intention

> panic == irrecoverable error

---

# Implication

```rust
impl Buffer {
    pub fn new() -> Self {
        let file_descriptor = unsafe { open() };
        if file_descriptor < 0 {
            panic!("could not create buffer");
        }
        ...
    }
}
```

.line5[![Arrow](./images/Arrow.png)]

What happens if library authors write code like this?

---

# Intention

> panic == irrecoverable error

If you are in a library, though, how do you decide what is irrecoverable?

---

# Sources of panics

Apart from `panic!` macro, other sources of panics:

* `assert!` failures
* `unreachable!` macro
* `unwrap` method
* `vec[i]` or `map[i]` indexing, when `i` is not a valid key

What do you see in common with all of those?

---

# Pattern 1: Bugs

* `assert!` failures
* `unreachable!` macro

Internal bugs in the library. Shouldn't happen.

---

# Pattern 2: Failed preconditions

* `unwrap` method
* `vec[i]` or `map[i]` indexing, when `i` is not a valid key

This then corresponds to bugs in the caller's code. Also shouldn't happen.

Where possible, offer a checked version:

* e.g., `vec.get(i)` that returns `Option<&T>`

---

# Rules for error handling

* Authoring a library:
    * panic for bugs in your library
        * for failed preconditions, can panic but offer a checked version

---

# Rust error handling mechanisms

* `panic!` == irrecoverable error
* `Result` == ?

---

# Rust error handling mechanisms

* `panic!` == irrecoverable error
* `Result` == recoverable error!

.footnote[
    I know, I know. You didn't see *that* one coming.
]

---

# Result

```rust
enum Result<O, E> {
    Ok(O),
    Err(E),
}
```

Result itself is nothing special.

---

# Using Result — fallible functions

```rust
impl Buffer {
    pub fn new() -> Result<Self, IoError> {
        let d: *const u8 = unsafe { load_data() };
        if d.is_null() {
            return Err(IoError);
        }
        ...
        Ok(buffer)
    }
}
```

--

.line2[![Arrow](./images/Arrow.png)]

--

.line5[![Arrow](./images/Arrow.png)]

--

.line8[![Arrow](./images/Arrow.png)]

---

# Calling functions that return result 

```rust
fn load_data() -> Vec<u8> {
    let buffer = Buffer::new();
    buffer.read() // ❌ 
}
```

Can't forget to check for errors!

---

# Calling functions that return result 

```rust
fn load_data() -> Vec<u8> {
    let buffer = match Buffer::new() {
        Ok(s) => s,
        Err(e) => panic!("could not create buffer {e}"),
    };
    buffer.into_data() // ✅
}
```

Is it ok to panic here?

---

# Other ways to write that code?

--

```rust
fn load_data() -> Vec<u8> {
    let buffer = Buffer::new().unwrap();
    buffer.into_data()
}
```

--

```rust
fn load_data() -> Vec<u8> {
    let Ok(buffer) = Buffer::new() else {
        panic!("could not create buffer")
    };
    buffer.into_data()
}
```

---

# Propagating the exception

```rust
fn load_data() -> Result<Vec<u8>, IoError> {
    let buffer = match Buffer::new() {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok(buffer.into_data())
}
```

.line4[![Arrow](./images/Arrow.png)]

--

.line6[![Arrow](./images/Arrow.png)]

---

# Other ways to write that code?

--

```rust
fn load_data() -> Result<Vec<u8>, IoError> {
    let buffer = Buffer::new()?;
    Ok(buffer.into_data())
}
```

.line2[![Arrow](./images/Arrow.png)]

Yay! Question mark!

--

```rust
Buffer::new()
    .map(|buffer| buffer.into_data()) // (infallible closure)

Buffer::new()
    .and_then(|buffer| Ok(buffer.into_data())) // (fallible closure)
```

I personally avoid these methods.<sup>1</sup>

.footnote[<sup>1</sup> Except when I don't.]

---

# The 'try' (or '?') operator

```rust
fn load_data() -> Result<Vec<u8>, IoError> {
    let mut buffer = Buffer::new()?;
    buffer.load_data()?;
    Ok(buffer.into_data())
}
```

Goal:

* lightweight way to signal "error may occur here"
* permits auditing

---

# Remember this?

```java
void doSomething() {
    activeConnections += 1;
    sendMessage();
    activeConnections -= 1;
}
```

---

# In Rust

```rust
fn do_something(&mut self) -> Result<()> {
    self.activeConnections += 1;
    self.sendMessage()?;
    self.activeConnections -= 1;
    Ok(())
}
```

.line3[![Arrow](./images/Arrow.png)]

--

.line5[![Arrow](./images/Arrow.png)]

---

# How to fix?

```rust
fn do_something(&mut self) -> Result<()> {
    self.activeConnections += 1;
    let result = self.sendMessage();
    self.activeConnections -= 1;
    result
}
```

---

# The 'try' (or '?') operator

```rust
foo?
```

expands to

```rust
match foo {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
    //                   --------
}
```

What is the purpose of this `into`?

---

# Remember the "checked exception" problem?

```java
void doSomething() throws ThisError, ThatError, TheOtherError
{
    ...
}
```

---

# Common Java pattern: wrapped exceptions

```java
void myFunction() throws MyLibraryException
{
    try {
        otherFunction();
    } catch (OtherLibraryException e) {
        throw new MyLibraryException(e);
    }
}
```

---

# Common Rust pattern (in libraries)

```java
enum MyError { .. }

impl std::error::Error for MyError { ... }

impl From<other_crate::Error> for Error { ... }

fn my_function() -> Result<(), Error> {
    other_crate::function()?;
    Ok(())
}
```

* Each library defines an `Error` type
* It implements the `std::error::Error` trait
* It also implements `From` to convert from other errors
* Then you can just use `?` 

--

.line1[![Arrow](./images/Arrow.png)]

--

.line3[![Arrow](./images/Arrow.png)]

--

.line5b[![Arrow](./images/Arrow.png)]

--

.line8b[![Arrow](./images/Arrow.png)]

---

# The Error trait

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
}
```

* Use `Debug` or `Display` to pretty print
* Use `source` to inspect underlying error (if any)

.footnote[
    Actually the stable, non-deprecated subset
]

---

# Removing boilerplate: this error

```rust
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("interal error: {source}")]
    OtherError {
        #[from]
        source: other_crate::Error
    },
}

fn my_function() -> Result<(), Error> {
    other_crate::function()?;
    Ok(())
}
```

---

# Playground time!

[Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=22e9e80cb79d9cd27a2fdd0008556cfa)

Goal(s):

* Modify `main` to print a "nicely formatted" error with `eprintln!`
* Modify `main` to print out the `source` field from error
* Include some other details

---

# How much detail to provide in error enums?

It's not necessarily helpful to create a library error enum that enumerates every possible error scenario differently.

Error should always be expressed in the domain of your library, and not some specifics of how the code works.
If you refactor your implementation to work completely differently, the errors should not change.

You can always add more error variants, but it's hard to take them away.

---

# Rules for error handling

* Authoring a library:
    * panic for bugs in your library
        * for failed preconditions, can panic but offer a checked version
    * result for everything else with a single error type
        * use `thiserror` to implement it nicely

---

# Error handling in applications

Same basic tools:

* panic for irrecoverable actions.
* result for (typically) recoverable failures.

...but the application is in a better position to make that distinction.

---

# What is "recovery"?

Sometimes, recovery means "keep going with the operation, but adapt".

For example, a file not found error might mean "create the file and try again"

--

In a CLI, it might mean "print out a nice error message about what went wrong".

--

In a server, it might mean "tear down the connection and be ready to receive a new one".

---

# Panic gives you very limited options

One thing to keep in mind is that panic and unwind give you limited options to react.

You can run destructors, but can't do async actions, etc.

---

# The anyhow crate

```rust
// short for `-> Result<T, anyhow::Error>`
fn fallible() -> anyhow::Result<T> {
    something()?
}
```

Often it's enough to know that "some error happened".

Every type implementing `Error` can be coerced into the `anyhow::Error` type.

So you can use `anyhow::Result` and then just use `?` without thinking too hard.

[Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9aad0ca851a6fee8c173f07a6d7dda0c)

---

# Recovering information

Even with `anyhow::Result`, it is *possible* to recover the original error using `downcast`.

[Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=547761312927f7482f679165f03679b3)

---

# Policy error enum

```rust
#[derive(thiserror::Error, Debug)]
enum AppError {
    /// An error that permits resuming
    #[error(transparent)]
    Resume(#[from] anyhow::Error),

    /// An error that requires aborting
    #[error(transparent)]
    Abort(#[from] anyhow::Error),
}
```

---

# Case study: Lock poisoning

---

# Case study: server error handling, take 1

Proposal:

* panic for irrecoverable errors that occur
* use `anyhow::Error` for recoverable errors
    * catch at the top event loop and discard

Pros? Cons?

---

# Case study: server error handling, take 2

Proposal:

* panic only for internal bugs (assertions, etc)
* policy enum with two (or potentially more) cases:
    * irrecoverable
    * recoverable
* catch at top level and abort loop if it is irrecoverable

Pros? Cons?

