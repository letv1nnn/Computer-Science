use std::thread;
use std::time::{Duration, Instant};

// understanding tasks
// A task is an asynchronous computation or operation that is managed and driven by
// an executor to completion. It represents the execution of a future, and it may involve
// multiple futures being composed or chained together.

// making a cup of coffee and yoast example

async fn prep_coffee_mug() {
    tokio::time::sleep(Duration::from_millis(100)).await; // cooperative multitasking
    println!("Pouring milk...");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured.");
    println!("Putting instant coffee...");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put.");
}

async fn make_coffee() {
    println!("boiling kettle...");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("kettle boiled.");
    println!("pouring boiled water...");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured.");
}

async fn make_toast() {
    println!("putting bread in toaster...");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("bread toasted.");
    println!("buttering toasted bread...");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered.");
}

pub async fn breakfast_example() {
    let start = Instant::now();
    let person_one = tokio::task::spawn(async {
        let prep_coffee_mug = prep_coffee_mug();
        let make_coffee = make_coffee();
        let make_toast= make_toast();
        tokio::join!(prep_coffee_mug, make_coffee, make_toast);
    });

    let person_two = tokio::task::spawn(async {
        let prep_coffee_mug = prep_coffee_mug();
        let make_coffee = make_coffee();
        let make_toast= make_toast();
        tokio::join!(prep_coffee_mug, make_coffee, make_toast);
    });

    let _ = tokio::join!(person_one, person_two);

    let elapsed_time = start.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}
