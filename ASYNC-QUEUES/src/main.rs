
extern crate async_queues;

use std::{future::Future, task::{Context, Poll}, time::Duration};

use async_queues::{join, spawn_task, FutureType};

fn main() {
    // Initialize runtime once to spawn threads
    let rt = async_queues::Runtime::new();
    unsafe { rt.run(); }

    // Create and spawn high priority future
    let high_future = CounterFuture {
        count: 0,
        order: FutureType::High,
    };
    let high_task = spawn_task!(high_future, FutureType::High);

    // Create and spawn low priority future
    let low_future = CounterFuture {
        count: 0,
        order: FutureType::Low,
    };
    let low_task = spawn_task!(low_future, FutureType::Low);

    // Wait for both tasks to complete
    let results = join!(high_task, low_task);

    println!("Final results: {:?}", results);
}

#[allow(unused)]
trait FutureOrderLabel: Future {
    fn get_order(&self) -> FutureType;
}

pub struct CounterFuture {
    count: u32,
    order: FutureType
}

impl FutureOrderLabel for CounterFuture {
    fn get_order(&self) -> FutureType {
        self.order
    }
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {:?}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}
