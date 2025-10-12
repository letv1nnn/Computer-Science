# Foundation of Async

## Task
A task is an asynchronous operation or computation that is managed and driven by an executor to completion. It represents the execution of a future, and it may involve multiple futures being composed or chained together.

```rust
let task1 = tokio::task::spawn(async {
    let future1 = async { /* ... */ };
    let future2 = async { /* ... */ };
    tokio::join!(future1, future2);
}); // can be awaited.
```

## Futures
Future is a placeholder object that represents the result of an asynchronous operation that has not yet completed. Futures allow you to start a task and continue with iother operations while the task is being executed in the background.

When a future is created, it is idle. It has yet to be executed. Once the future is executed, it can either yield a value, resolve, or go to sleep because the future is pending (awaiting on a result). When the future is called again the Poll can return either a Pending or Ready. The future will continue to be polled until it is either resolved or cancelled. 

The following code is a simple implementation of a counter future, consider a CounterFuture struct that consists of one field, `counter: u32`.

```rust
impl Future for CounterFuture {
    type Output = u32;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.counter += 1;
        println!("polling with result: {}", self.counter);
        std::thread::sleep(Duration::from_secs(1));
        if self.counter < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.counter)
        }
    }
}
```

There are several unknown traits, they are `Pin` and `Context`. 

Pin disables data types to be moved around memory.

The following code showing some case where value is moved and cause a segfault. Struct that reference itsef. In consequence if we create a variable and move it around, we will get likey a segmentation fault.
```rust
struct SelfReferencial {
    data: String,
    self_pointer: *const String,
}

impl SelfReferencial {
    fn new(data: String) -> Self {
        let mut sr = SelfReferencial {
            data,
            self_pointer: ptr::null(),
        };
        sr.self_pointer = &sr.data as *const String;
        sr
    }
    fn print(&self) {
        unsafe {
            println!("{}", *self.self_pointer);
        }
    }
}
```

A context only serves to provide access to a waker to wake a task. A waker is a handle that notifies the executor when the task is ready to be run.
The waker is wrapped in the context and is only used when the result of the poll is going to be Pending. The waker is essentially waking up the future so it can be executed.

```rust
if self.counter < 5 {
    cx.waker().wake_by_ref();
    Poll::Pending
} else {
    Poll::Ready(self.counter)
}
```