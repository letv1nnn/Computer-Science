#![allow(unused)]

use crate::{futures::{counter_example, slow_task_example}, putting_all_together::loggin_test, sharing_data_between_futures::sharing_data_test, tasks::breakfast_example, waking_futures_remotely::test_future_remote_waking};

mod tasks;
mod futures;
mod waking_futures_remotely;
mod sharing_data_between_futures;
mod putting_all_together;
mod pinning_in_futures;

#[tokio::main]
async fn main() {
    // Tasks
    // breakfast_example().await;

    // Futures - introduction
    // counter_example().await;
    // pinning_in_futures::pinning_in_futures();
    // slow_task_example().await;

    // Futures - waking futures remotely
    // test_future_remote_waking().await;

    // Futures - sharing data between futures
    // sharing_data_test().await;

    // Putting all together
    // loggin_test().await;
}
