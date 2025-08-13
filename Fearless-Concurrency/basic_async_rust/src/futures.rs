use std::{future::Future, result, task::Poll, time::Duration};

// futures

pub struct CounterFuture {
    counter: u32,
}

impl CounterFuture {
    pub fn new(counter: u32) -> Self {
        Self { counter }
    }
}

impl Future for CounterFuture {
    type Output = u32;
//      Pin - value can't be moved into a different memory location.
//                    |#######################|
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//                                              |#############################|
//                                                    basically, a waker
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


pub async fn counter_example() {
    let counter_one = CounterFuture::new(0);
    let counter_two = CounterFuture::new(0);

    let handle_one = tokio::task::spawn(async move {
        counter_one.await
    });
    let handle_two = tokio::task::spawn(async move {
        counter_two.await
    });

    tokio::join!(handle_one, handle_two);
}


async fn slow_task() -> &'static str {
    tokio::time::sleep(Duration::from_secs(4)).await;
    "Slow Task Completed!"
}

pub async fn slow_task_example() {
    let duration = Duration::from_secs(3);
    let result = tokio::time::timeout(duration, slow_task()).await;

    match result {
        Ok(value) => println!("Task completed successfully: {}", value),
        Err(_) => println!("Task timed out"),
    }
}
