#![allow(unused)]

#![feature(coroutines)]
#![feature(coroutine_trait)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    use std::future::Future;
    use std::task::{Context, Poll};
    use std::time::Duration;

    // sync testing interface
    fn check_yield(coroutine: &mut MutexCoroutine) -> bool {
        match Pin::new(coroutine).resume(()) {
            CoroutineState::Yielded(()) => true,
            CoroutineState::Complete(()) => false,
        }
    }

    // async runtime interface
    impl Future for MutexCoroutine {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match Pin::new(&mut self).resume(()) {
                CoroutineState::Complete(_) => Poll::Ready(()),
                CoroutineState::Yielded(_) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                },
            }
        }
    }

    #[test]
    fn basic_test() {
        let handle = Arc::new(Mutex::new(0));
        let mut first_coroutine = MutexCoroutine {
            handle: handle.clone(),
            treshold: 2,
        };
        let mut second_coroutine = MutexCoroutine {
            handle: handle.clone(),
            treshold: 2,
        };

        let lock = handle.lock().unwrap();
        for _ in 0..3 {
            assert_eq!(check_yield(&mut first_coroutine), true);
            assert_eq!(check_yield(&mut second_coroutine), true);
        }
        assert_eq!(*lock, 0);
        std::mem::drop(lock);

        assert_eq!(check_yield(&mut first_coroutine), true);
        assert_eq!(*handle.lock().unwrap(), 1);
        assert_eq!(check_yield(&mut second_coroutine), true);
        assert_eq!(*handle.lock().unwrap(), 2);
        assert_eq!(check_yield(&mut first_coroutine), false);
        assert_eq!(*handle.lock().unwrap(), 3);
        assert_eq!(check_yield(&mut second_coroutine), false);
        assert_eq!(*handle.lock().unwrap(), 4);
    }

    #[tokio::test]
    async fn async_test() {
        let handle = Arc::new(Mutex::new(0));
        let mut first_coroutine = MutexCoroutine {
            handle: handle.clone(),
            treshold: 2,
        };
        let mut second_coroutine = MutexCoroutine {
            handle: handle.clone(),
            treshold: 2,
        };

        // Run first coroutine to completion
        while let CoroutineState::Yielded(()) = Pin::new(&mut first_coroutine).resume(()) {
            tokio::task::yield_now().await;
        }

        // Run second coroutine to completion
        while let CoroutineState::Yielded(()) = Pin::new(&mut second_coroutine).resume(()) {
            tokio::task::yield_now().await;
        }
        assert_eq!(*handle.lock().unwrap(), 4);
    }
}

// start with a struct that handle mutex and a treshold where
// the coroutine will be complete after the treshold is reached
pub struct MutexCoroutine {
    pub handle: Arc<Mutex<u8>>,
    pub treshold: u8,
}

impl Coroutine<()> for MutexCoroutine {
    type Return = ();
    type Yield = ();

    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        let locked = {
            match self.handle.try_lock() {
                Ok(mut handle) => {
                    *handle += 1;
                    true
                }
                Err(_) => return CoroutineState::Yielded(()),
            }
        };

        if locked {
            self.treshold -= 1;
            if self.treshold == 0 {
                CoroutineState::Complete(())
            } else {
                CoroutineState::Yielded(())
            }
        } else {
            CoroutineState::Yielded(())
        }
    }
}
