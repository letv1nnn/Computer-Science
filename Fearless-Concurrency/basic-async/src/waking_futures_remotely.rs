use std::clone;
use std::sync::{Mutex, Arc};
use std::task::{Waker, Context, Poll};

use tokio::sync::mpsc;
use tokio::task;


struct MyFututre {
    state: Arc<Mutex<MyFututreState>>
}

struct MyFututreState {
    data: Option<Vec<u8>>,
    waker: Option<Waker>,
}

impl MyFututre {
    fn new() -> (Self, Arc<Mutex<MyFututreState>>) {
        let state = Arc::new(Mutex::new(MyFututreState {
            data: None,
            waker: None,
        }));
        (
            MyFututre {
                state: state.clone(),
            },
            state,
        )
    }
}

impl Future for MyFututre {
    type Output = String;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling the future");
        let mut state = self.state.lock().unwrap();

        if state.data.is_some() {
            let data = state.data.take().unwrap();
            Poll::Ready(String::from_utf8(data).unwrap())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}


pub async fn test_future_remote_waking() {
    let (my_future, state) = MyFututre::new();
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let task_handle = task::spawn(async {
        my_future.await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("spawning trigger task");

    let trigger_task = task::spawn(async move {
        rx.recv().await;
        let mut state = state.lock().unwrap();
        state.data = Some(b"Hello from the outside".to_vec());
        loop {
            if let Some(waker) = state.waker.take() {
                waker.wake();
                break;
            }
        }
    });
    tx.send(()).await.unwrap();

    let outcome = task_handle.await.unwrap();
    println!("Task completed with outcome: {:?}", outcome);
    trigger_task.await.unwrap();
}
