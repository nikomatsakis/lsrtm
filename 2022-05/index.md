class: center
name: title
count: false

# LSRTM 2022-05

.p60[![Ferris](./images/ferris.svg)]

.left[.citation[View slides at `https://nikomatsakis.github.io/lsrtm-2022-05/`]]

---

# Agenda

* Lecture: async
* Code snippet w/ questions
* Lecture: review
* Open floor

---

# JavaScript promises

```js
async function process() {
    let promise = sendRequest();
    let result = await promise;
}

async function sendRequest() { /* ... */ }
```

![JavaScript promise timeline](images/js-promise.drawio.svg)

???

---

# Rust futures

```rust
async fn process() {
    let future = sendRequest();
    let result = future.await;
}

async fn sendRequest() -> Result { /* ... */ }
```

![Rust promise timeline](images/rust-promise.drawio.svg)

---

# Rust futures 1

```rust
async fn process() {
    let future = sendRequest();
    let result = future.await;
}

async fn sendRequest() -> Result { /* ...1 */ }
```

.line2[![Arrow](./images/Arrow.png)]

![Rust promise timeline](images/rust-promise-step-1.drawio.svg)

---

# Rust futures 2

```rust
async fn process() {
    let future = sendRequest();
    let result = future.await;
}

async fn sendRequest() -> Result { /* ...2 */ }
```

.line3[![Arrow](./images/Arrow.png)]

![Rust promise timeline](images/rust-promise-step-2.drawio.svg)

---

name: combinators

# Rust future combinators

```rust
async fn process() {
    let future1 = sendRequest();
    let future2 = sendRequest();
    let future3 = futures::future::join(future1, future2);
    let (result1, result2) = future3.await;
}

async fn sendRequest() -> Result { /* ... */ }
```

---

template: combinators

![Rust combinator timeline](images/rust-combinator.drawio.svg)

---

template: combinators

.line2[![Arrow](./images/Arrow.png)]

![Rust combinator timeline](images/rust-combinator-step-1.drawio.svg)

---

template: combinators

.line3[![Arrow](./images/Arrow.png)]

![Rust combinator timeline](images/rust-combinator-step-2.drawio.svg)

---

template: combinators

.line4[![Arrow](./images/Arrow.png)]

![Rust combinator timeline](images/rust-combinator-step-3.drawio.svg)

---

template: combinators

.line5[![Arrow](./images/Arrow.png)]

![Rust combinator timeline](images/rust-combinator-step-4.drawio.svg)

---

# Polling (simplified)

```rust
trait Future {
    type Output;

    fn poll(&mut self) -> Ready<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

---

# Awaiting

```rust
let result = future.await;

// becomes

let result = loop {
    match future.poll() {
        // If the value is available,
        // break from loop and use 
        // `v` as the value of the loop.
        Poll::Ready(v) => break v,

        // Otherwise, suspend current
        // task. Will be reawoken when
        // data might be ready.
        Poll::Pending => yield,
    }
};
```

---

# Join combinator

```rust
struct Join<F1, F2>
where
    F1: Future,
    F2: Future<Output = F1::Output>,
{
    future1: Option<F1>,
    future2: Option<F2>,
    output1: Option<F1::Output>
    output2: Option<F1::Output>
}
```

Poll method:

* Polls `self.f1` and polls `self.f2` until both have returned.
* If one returns before the other, stash the output until both have returned.

---

name: f-u

# `FuturesUnordered`

```rust
let mut pool = FuturesUnordered::new();
pool.push(sendRequest());
pool.push(sendRequest());
let first = pool.next().await;  // yields Some(r) 
let second = pool.next().await; // yields Some(r)
let none = pool.next().await;   // none
```

---

template: f-u

![FuturesUnordered diagram](images/rust-fu.drawio.svg)

---

template: f-u

.line1[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-1.drawio.svg)

---

template: f-u

.line2[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-2.drawio.svg)

---

template: f-u

.line3[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-3.drawio.svg)

---

template: f-u

.line4[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-4.drawio.svg)

---

template: f-u

.line5[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-5.drawio.svg)

---

template: f-u

.line6[![Arrow](images/Arrow.png)]

![FuturesUnordered diagram](images/rust-fu-step-5.drawio.svg)

---

# Review

* A Rust future is inert unless you are awaiting it

--


* Rust futures introduce *concurrency* within a task (by default)
    * You can use `tokio::spawn` to start a new task


--


* `FuturesUnordered` feels like a thread-pool but is not
    * The futures pushed inside only make progress when awaited


---

# Snippet

[Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=66455d3e61217d86ec4839c926619bf0)

Based on a real-life bug

What goes wrong?

---

# Replica function

```rust
async fn replica(host: u32, mut receiver: tokio::sync::mpsc::Receiver<char>) -> (u32, usize) {
    let mut count = 0;
    while let Some(message) = receiver.recv().await {
        eprintln!("Host {host} received message {message:?}");
        if message == '\n' {
            break;
        } else {
            count += 1;
        }
    }
    (host, count)
}
```

---

name: sharding

# Sharding

```rust
let replicas = 3;
let mut host_futures = FuturesUnordered::new();
let mut host_senders = vec![];
for host in 0..replicas {
    let (sender, receiver) = channel(2);
    host_senders.push(sender);
    host_futures.push(replica(host, receiver));
}
```

---

template: sharding

.line2[![Arrow](images/Arrow.png)]

Stores the replicas

---

template: sharding

.line4[![Arrow](images/Arrow.png)]

Maximum channel capacity: 2

---

template: sharding

.line5[![Arrow](images/Arrow.png)]

Replica will drain from the channel (when it runs)


---

template: sharding

.line8[![Arrow](images/Arrow.png)]

![send](images/rust-snippet-send.drawio.svg)

---

name: send-data

# Sending data

```rust
for message in ['H', 'e', 'l', 'l', 'o', '\n'] {
    for sender in &host_senders {
        sender.send(message).await.unwrap();
    }
}
```

Now we send data to the replicas


---

# What is supposed to happen

```rust
while let Some((host, count)) = host_futures.next().await {
    eprintln!("Host {host} received {count} bytes.");
}
```    

Wait for replicas to finish sending data

But what actually happens?

---

template: send-data

.line1[![Arrow](images/Arrow.png)]

![send](images/rust-snippet-send.drawio.svg)

---

template: send-data

.line3[![Arrow](images/Arrow.png)]

![send-1](images/rust-snippet-send-step-1.drawio.svg)

---

template: send-data

.line3[![Arrow](images/Arrow.png)]

![send-2](images/rust-snippet-send-step-2.drawio.svg)


---

template: send-data

Deadlock!

---

# Brief plug: moro

I have been experimenting with a "structured concurrency" library called [moro](https://github.com/nikomatsakis/moro/). API does not permit this sort of deadlock.

[This example in moro](https://github.com/nikomatsakis/moro/blob/d6ab92c5d0f0799a0a68dcd1c3f41d6d3a517df2/examples/replicas.rs)

Also supports cancellation of the entire scope, as shown [here](https://github.com/nikomatsakis/moro/blob/d6ab92c5d0f0799a0a68dcd1c3f41d6d3a517df2/examples/monitor.rs).

If you want to experiment with it, it's on crates.io, happy to discuss more offline!

---

# Open floor

Opening up the floor for any announcements.
