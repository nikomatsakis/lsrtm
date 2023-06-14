## scheduler, runtime

* cooperative vs pre-emptive multithreading
    * pre-emptive: O/S level thread for each task
        * used by Java
    * cooperative: lightweight user-space data structure called a TASK
        * having a moment, used by JavaScript, Go, and Rust
        * Java supports via Future and upcoming Loom project
    * advantages:
        * cooperation is more efficient, can be tuned by application depending on its needs
        * can scale up more efficiently, don't have to allocate a big stack
    * disadvantages:
        * less transparent
        * caveat: Go does a great job of hiding it from you, as long as you stick to goroutines
* thread vs task
    * THREAD -- operating system level concept
    * TASK -- ...
    * THREADS run a set of TASKS
* how tokio works at a very (VERY) high level
    * example: pool of tasks, ask "done yet?"
        * yes? remove from list.
        * no? keep for later.
    * of course that's not what REALLY happens
        * more like: done yet?
            * blocked: put aside
    * how do you know when it's ready?
        * not going to go into detail of that
        * don't really need to know unless you are manually implementing futures
        * look here for details
        * but I'd also like to understand why you are manually implementing futures
* how can this go wrong?
    * one task can starve others
    * common reasons
        * synchronous I/O
        * big loops
        * calls to JNI functions (can trigger a GC)
        * locks (we'll cover these in detail later)
    * [demonstration](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cb3683b0be675229273367528630547b)
    * how can you resolve this?
        * `tokio::task::yield_now().await`
        * easy option `spawn_blocking`
            * starts up a new thread
* multithreaded tokio
    * `main(current_thread)` -- what is this?
    * multithreaded tokio behaves differently
    * change [demo](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8d3d84075d770cd0042fae3035897323) to use multi-threaded scheduler, what happens?
        * change `worker_threads = 10`, play around
        * guesses?
        * answer: trick question! depends how many cores.
    * work stealing?
* common distinction:
    * **concurrency** -- time-slicing; going back and forth between two things but only doing one at a time
    * **parallelism** -- truly doing two things at once
* review
    * each tokio thread runs one task at a time
        * when you spawn, you create a new TASK
    * if one task stalls, that will stall other tasks in the same thread
        * multithreaded scheduler can compensate sometimes via work stealing
        * avoid this by yielding or spawn-blocking
    * plug for [tokio metrics](https://docs.rs/tokio-metrics/latest/tokio_metrics/)
        * can help you understand behavior of scheduler at a lower level
        * has [awesome docs](https://docs.rs/tokio-metrics/latest/tokio_metrics/struct.TaskMonitor.html#why-are-my-tasks-slow) prepared by our very own Jack Wrenn
        * if you are having trouble tuning your application, talk to our team if you're not already
* question:
    * ok to use a mutex?
        * sort of:
            * it's blocking, introduces a dependency between threads
        * if you hold it across a join point: definitely bad
            *  deadlock

## individual tasks

* javascript promises
* rust futures
* [solution](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0597c6cb72dd07a8a0da4c3e3bbe8cda)
    * [but wait](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5d40d0885d0acdc96ebb9ab30706b96b)

## independent tasks

* spawned tasks can be joined but can also be detached
    * not tied to their parent
* parent can drop that buffer
* use Rc

## upcoming meetings

* preview:
    * next meeting is June 28
* July meeting topic?
    * FFI (PyO3 / JNI)?
    * Integration?
    * Other suggestions?
* August: 
    no meeting
* September: 
    celinval + zyadh on unsafe code
