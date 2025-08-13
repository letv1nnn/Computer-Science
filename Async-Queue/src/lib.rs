use std::future::Future;
use std::panic::catch_unwind;
use std::thread;
use std::time::Duration;
use std::sync::LazyLock;

use async_task::{Runnable, Task};
use flume::{Sender, Receiver};

#[derive(Debug, Clone, Copy)]
pub enum FutureType {
    High,
    Low
}

// implementing a mini runtime
pub struct Runtime {
    high_num: usize,
    low_num: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let num_cores = std::thread::available_parallelism().unwrap().get();
        Self {
            high_num: num_cores,
            low_num: 1,
        }
    }

    pub fn with_high_num(mut self, num: usize) -> Self {
        self.high_num = num;
        self
    }
    
    pub fn with_low_num(mut self, num: usize) -> Self {
        self.low_num = num;
        self
    }
    
    pub unsafe fn run(&self) {
            unsafe { std::env::set_var("HIGH_NUM", self.high_num.to_string());
            std::env::set_var("LOW_NUM", self.low_num.to_string());
            };
            // Warm-up dummy futures (delay to ensure threads spawn)
            let high = spawn_task!(
                async {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                },
                FutureType::High
            );
            let low = spawn_task!(
                async {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                },
                FutureType::Low
            );
        
            join!(high, low);
        }
    
}


// task spawning
pub fn spawn_task<F, T>(future: F, order: FutureType) -> Task<T>
where
F: Future<Output = T> + Send + 'static,
T: Send + 'static,
{


    static HIGH_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(|| flume::unbounded::<Runnable>());

    static LOW_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(|| flume::unbounded::<Runnable>());

    static HIGH_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        for _ in 0..2 {
            let high_reciever = HIGH_CHANNEL.1.clone();
            let low_reciever = LOW_CHANNEL.1.clone();

            thread::spawn(move || {
                loop {
                    match high_reciever.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        },
                        Err(_) => {
                            match low_reciever.try_recv() {
                                Ok(runnable) => {
                                    let _ = catch_unwind(|| runnable.run());
                                }
                                Err(_) => {
                                    thread::sleep(Duration::from_millis(100));
                                }
                            }
                        },
                    }
                }
            });
        }
        HIGH_CHANNEL.0.clone()
    });

    static LOW_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        for _ in 0..2 {
            let low_receiver = LOW_CHANNEL.1.clone();
            let high_receiver = HIGH_CHANNEL.1.clone();
            thread::spawn(move || loop {
                match low_receiver.try_recv() {
                    Ok(runnable) => {
                        let _ = catch_unwind(|| runnable.run());
                    }
                    Err(_) => match high_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_millis(100));
                        }
                    },
                }
            });
        }
        LOW_CHANNEL.0.clone()
    });

    let scheduler_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
    let scheduler_low = |runnable| LOW_QUEUE.send(runnable).unwrap();

    let schedule= match order {
        FutureType::High => scheduler_high,
        FutureType::Low => scheduler_low,
    };

    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    return task;
}

// Creating spawn_task, join and try_join macros, just like in tokio.
#[macro_export]
macro_rules! spawn_task {
    ($future:expr) => {
        spawn_task!($future. FutureType::Low)
    };
    ($future:expr, $order:expr) => {
        spawn_task($future, $order)
    };
}

#[macro_export]
macro_rules! join {
    ($($future:expr), *) => {
        {
            let mut results = Vec::new();
            $(
                results.push(futures_lite::future::block_on($future));
            )*
            results
        }
    };
}

#[macro_export]
macro_rules! try_join {
    ($($future:expr), *) => {
        {
            let mut results = Vec::new();
            $(
                let result = catch_unwind(|| future::block_on($future));
                results.push(result);
            )*
            results
        }
    };
}
