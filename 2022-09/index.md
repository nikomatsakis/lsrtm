class: center
name: title
count: false

# Interior mutability in Rust

.p60[![Ferris](./images/ferris.svg)]

.me[.grey[*by* **Nicholas Matsakis**]]
.left[.citation[View slides at `https://nikomatsakis.github.io/lsrtm-2022-09/`]]

---

# sharing xor mutability

Rust's key insight is that

* sharing
* mutability

don't work well together.

---

# Question

Is the data in `Vec<String>` "mutable"?

--

Answer:

It depends on the context.

---

```rust
let mut v = vec![];
v.push("Hello".to_string());
v[0].push_str(", world!");
```

???

Here, `v` is declared mutable, and so the vector is mutable, as are its contents.

---

```rust
let mut v = vec!["Hello".to_string()];
v.push("Hello".to_string());
v[0].push_str(", world!");

let p = &v[0];
v.push("Goodbye, world!".to_string());
println!("{p}");
```

???

Of course, if we borrow `v[0]`...

---

```
+---+----------+ 
| v | data     | -----> +-----+----------+
|   | len      |        | [0] | data     | ---> "Hello, world!"
|   | capacity |        |     | len      |
+---+----------+        |     | capacity |
                        +-----+----------+
```

```rust
let mut v: Vec<String> = ...;
```

---

```
+---+----------+ 
| v | data     | -----> +-----+----------+
|   | len      |    +-> | [0] | data     | ---> "Hello, world!"
|   | capacity |    |   |     | len      |
|   +----------+    |   |     | capacity |
|   |               |   +-----+----------+
| p | --------------+
+---+
```

```rust
let mut v: Vec<String> = ...;
let p = &v[0];
```

---

```
+---+----------+ 
| v | data     | ----------> (new copy of buffer)
|   | len      |    +-> ??
|   | capacity |    |   
|   +----------+    |   
|   |               |   
| p | --------------+
+---+
```

```rust
let mut v: Vec<String> = ...;
let p = &v[0];
v.push(...);
```

---

# Inherited mutability in Rust

We say that the mutability of the data in `Vec<String>` is *inherited* -- it comes from the context.

Another word: it is "external".

---

# Inherited from local variable

```rust
let v: Vec<String> = vec!["Hello".to_string()];
v.push("World".to_string());
```

--

.line1[![Arrow](./images/Arrow.png)]

---

# `&` references

```rust
let mut v: Vec<String> = vec!["Hello".to_string()];
let mut w = &v;
v.push("World".to_string());
for s in w { println!("{s}"); }
```

Even if value is mutable, it can be temporarily frozen via borrows.

--

.line2[![Arrow](./images/Arrow.png)]

--

.line4[![Arrow](./images/Arrow.png)]

---

# Sharing xor mutability

--

![It's a lie](images/simpsons-grampa.gif)

---

# Sharing xor mutability

![Not a lie](images/simpsons-homer.gif)

---

# Sharing xor mutability

```rust
let c = &Cell::new(22);
let d = c;
c.set(23);
println!("{}", d.get());
```

--

.line2[![Sharing on line 2](./images/Arrow.png)]

--

.line3[![Sharing on line 3](./images/Arrow.png)]

--

.line4[![Sharing on line 3](./images/Arrow.png)]

---

```rust
let c = &Cell::new(22);
```

When you have `&<expr>`...

* if `<expr>` is a place in memory, you get a reference to that memory
    * e.g., `&v[0]` that we saw from before

--

* otherwise, creates a temporary and gives a reference to that

---

```rust
let _tmp = Cell::new(22);
let c = &_tmp;
```

---

```
+------+----+ 
| _tmp | 22 | <-+
|      +----+   |
|      |        |
| c    | -------+
+------+
```

```rust
let _tmp = Cell::new(22);
let c = &_tmp;
```

---

```
+------+----+ 
| _tmp | 22 | <-+
|      +----+   |
|      |        |
| c    | -------|
|      |        |
| d    | -------+
+------+
```

```rust
let _tmp = Cell::new(22);
let c = &_tmp;
let d = c;
```

---

```
+------+----+ 
| _tmp | 23 | <-+
|      +----+   |
|      |        |
| c    | -------|
|      |        |
| d    | -------+
+------+
```

```rust
let _tmp = Cell::new(22);
let c = &_tmp;
let d = c;
c.set(23);
```

---

# Some observations

* Important to understand memory layout

---

```rust
let c1 = Cell::new(22);
let c2 = c1.clone();
c1.set(23);
println!("{}", c1.get());
println!("{}", c2.get());
```

What does this print? (answer on slack)

---

```
+------+----+ 
| c1   | 22 |
|------+----+
| c2   | 22 |
+------+----+
```

```rust
let c1 = Cell::new(22);
let c2 = c1.clone();
```

---

```
+------+----+ 
| c1   | 22 |
|------+----+
| c2   | 22 |
+------+----+
```

```rust
let c1 = Cell::new(22);
let c2 = c1.clone();
c1.set(23);
println!("{}", c1.get());
println!("{}", c2.get());
```

---

# Off the stack

```rust
let c1 = Rc::new(Cell::new(22));
let c2 = c1.clone();
c1.set(23);
println!("{}", c1.get());
println!("{}", c2.get());
```

.line1[![Arrow at Rc](images/Arrow.png)]

---

# Off the stack

```
+-----+ 
| c1  | ----> +----+
+-----+  |    | 1  |
| c2  | -+    +----+
+-----+       | 22 |
              +----+
```

```rust
let c1 = Rc::new(Cell::new(22));
let c2 = c1.clone();
c1.set(23);
println!("{}", c1.get());
println!("{}", c2.get());
```

---

# "Sheep clone"

When you clone something, by default...

* You clone everything it *uniquely* owns
* But not what is shared

---

# Cell

Let's look more closely at `Cell<T>`...

* `set(&self, value: T)`
* `replace(&self, value: T) -> T`

And, if `T: Copy`...

* `get(&self) -> T`

---

# Observations on cell

* It provides a *small set of complete operations* you can do on `T`
* Great for counters etc

---

# Cell

```rust
let shared_vec: Rc<Cell<Vec<String>>> = ...;
```

How can I push onto this vector?

---

# Cell

```rust
let shared_vec: Rc<Cell<Vec<String>>> = ...;
let mut vec = shared_vec.replace(vec![]);
vec.push(...);
shared_vec.set(vec);
```

---

# RefCell

* Extends `Cell` with `borrow` and `borrow_mut` methods:

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard = shared_vec.borrow_mut();
guard.push(...);
```

---

# RefCell

```
+------------+      +----------+ 
| shared_vec | ---> | 1        | // reference count
|            |      +----------+
|            |      | 0        | // refcell flags
|            |      +----------+
+------------+      | data     | -----> [...]
                    | len      |        
                    | capacity |
                    +----------+        
```

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
```

---

# RefCell

```
+------------+      +----------+ 
| shared_vec | ---> | 1        |
|            |  |   +----------+
| guard      | -+   | writer   | // "lock" acquired
|            |      +----------+
+------------+      | data     | -----> [...]
                    | len      |        
                    | capacity |
                    +----------+        

```

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard = shared_vec.borrow_mut();
```

---

# RefCell

```
+------------+      +----------+ 
| shared_vec | ---> | 1        |
|            |  |   +----------+
| guard      | -+   | writer   |
|            |      +----------+
+------------+      | data     | -----> [...]
                    | len      | // incremented!
                    | capacity |
                    +----------+        

```

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard = shared_vec.borrow_mut();
guard.push(...);
```

---

# RefCell

```
+------------+      +----------+ 
| shared_vec | ---> | 1        |
|            |      +----------+
| ~~guard~~  |      | 0        | // flag released
|            |      +----------+
+------------+      | data     | -----> [...]
                    | len      |
                    | capacity |
                    +----------+        

```

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard = shared_vec.borrow_mut();
guard.push(...);
// destructor for guard runs
```

---

# RefCell double acquire

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard1 = shared_vec.borrow_mut();
let mut guard2 = shared_vec.borrow_mut(); 
```

What happens here?

---

# RefCell early release

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
let mut guard1 = shared_vec.borrow_mut();
drop(guard1);
let mut guard2 = shared_vec.borrow_mut(); 
```

What happens here?

---

# Temporary lifetimes

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
shared_vec.borrow_mut().push(...);
```

becomes something like

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;

{
    let mut _guard = shared_vec.borrow_mut();
    _guard.push(...);
}
```

---

# General rule for temporaries

* Temporaries are dropped at the end of a statement
* Unless they are assigned into a let (e.g., `let x = &vec![]`)

**But:** the statement may not be where you expect.

---

# General rule for temporaries

```rust
let shared_vec: Rc<RefCell<Vec<String>>> = ...;
match shared_vec.borrow_mut().is_empty() {
    false => {},
    true => shared_vec.borrow_mut().push(...),
}
```

The rules around matches are often particularly surprising.

What happens here? ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=48b9c9d4fed456e277c7c8069e5545dc))

.footnote[There is a clippy lint `significant_drop_in_scrutinee`, but it needs work (see [rust-clippy#8987](https://github.com/rust-lang/rust-clippy/issues/8987)).]

---

# Sharing and mutability

There are several types that enable mutability on shared data, with different capabilities

|                | Atomic operations  | Gives a reference     |
| -------------- | ------------------ | --------------------- |
| Not threadsafe | [`Cell`]           | [`RefCell`]           |
| Threadsafe     | [`AtomicU32`], etc (*) | [`Mutex`], [`RwLock`] |

[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`AtomicU32`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html

.footnote[(*) Instead of `AtomicU32`, crossbeam offers [`AtomicCell<T>`], which has a simpler API, though it degrades poorly if `T` is not word-sized]

[`AtomicCell<T>`]: https://docs.rs/crossbeam-utils/latest/crossbeam_utils/atomic/struct.AtomicCell.html

---

# RefCell vs Mutex

```rust
let shared_vec: Arc<Mutex<Vec<String>>>;
let mut guard = shared_vec.lock().unwrap();
//                                ^^^^^^^^
```

`Mutex` works much like RefCell but:

* Only `lock`, no `borrow` vs `borrow_mut` (but `RwLock` supports that)
* Deadlocks instead of panics
* Returns a `Result` from `lock` -- what's up with that?

---

# Lock poisoning

```rust
let shared_vec: Arc<Mutex<Vec<String>>>;
let mut guard = shared_vec.lock().unwrap();
guard.push(String::from("temporary_value"));
do_something(); // <-- what will other threads see if this panics?
guard.pop();
```

---

# How to use interior mutability?

Just follow the leadership principles

* **Ownership**
    * If a type makes use of interior mutability, it should offer methods that own the complete operation.
    * Avoid callbacks unless truly needed.
* **Think big**
    * If fields have entangled invariants, group together
* **Are right, a lot**
    * Rely on exterior mutability where you can

---

# Ownership

```rust
pub struct PortList {
    certificates: Atomic<Vec<Port>>
}

impl PortList {
    pub fn ports(&self) -> &Atomic<Vec<Port>> { ... }

    // vs

    pub fn ports(&self) -> Vec<Port> {
        self.ports.lock().unwrap().clone()
    }

    pub fn push_port(&self, port: Port)
        self.ports.lock().unwrap().push(port);
    }
}
```

---

# Think big

```rust
pub struct DayTripper {
    days: AtomicU32,
    weeks: AtomicU32,
}

// vs

pub struct DayTripper {
    days_weeks: Mutex<(u32, u32)>
}

// vs

pub struct DayTripper {
    days_weeks: AtomicU64
}
```

???

Niko's thoughts:

* Separate atomics is fine if the counters are unrelated
* If they are linked, better to put under a single mutex
* Atomic U64 is a way to be cute and clever

Question: what if we were using `Cell` intead?

* With Cell, this becomes less relevant, because only one thread can use it at a time

---

# Are right, a lot

```rust
impl ModuleContext {
    fn test_modules(&self) {
        let module_table = self.module_table.borrow();
        for m in module_table {
            let contents = self.load_contents(m);
            self.test_contents(contents);
        }
    }

    fn test_contents(&self) {
        ...
    }
}
```

---

# Are right, a lot

```rust
impl ModuleContext {
    fn test_modules(&self) {
        let module_table = self.module_table.borrow();
        for m in module_table {
            let contents = self.load_contents(m);
            self.test_contents(contents);
        }
    }

    fn test_contents(&self, ...) {
        ...
        // if module is lazy:
        self.module_table.borrow_mut().push(new_module());
        ...
    }
}
```

---

# Are right, a lot

```rust
struct ModuleContext {
    data: RefCell<ModuleContextData>
}

impl ModuleContext {
    fn test_modules(&self) {
        self.data.borrow_mut().test_modules();
    }
}

impl ModuleContextData {
    ...
}
```

---

# Are right, a lot

```rust
let mut mutex = Mutex::new(vec![]);

// parallel scope in which mutex is shared
data.par_iter()
    .for_each(|datum| process(datum, &mutex));

// data no longer shared, can avoid locks
mutex.get_mut().unwrap().push(...);
//    ^^^^^^^^^ `&mut Mutex` is not shared
```

Many cell- and sharing-types have operations with names like `get_mut` or `into_inner`. Faster and more flexible, but also more correct.

---

# Are right, a lot

Do you need interior mutability at all?

Consider the actor pattern:

* Instead of sharing an `Arc<Mutex<Data>>` across many tasks.
* Create a task that owns `Data` and communicate with it via channels.

"Share via communicating, don't communicate via sharing"

Good reference: [Actors in Tokio blog post](https://ryhl.io/blog/actors-with-tokio/)

---

# Discussion questions

Prompts:

* Where does your team use interior mutability?
* Are there places you might benefit from actors instead?
* 


