# Multithreading

There are generally two types of models used in multithreading, they are ***Message Passing Model*** and ***Shared Memory Model***.

***Message Passing Model***

The *Message Passing Model* allows threads to communicate by sending and receiving data through channels. 

```rust
// message-passing model code snippet
// creating a Sender and Receiver variables
let (tx, rx) = mpsc::channel();
// sending data to a channel inside of a thread
thread::spawn(move || {
    let a = 10;
    tx.send(a).unwrap();
});
// receiving data in the main thread
for el in rx {
    println!("{:?}", el);
}
```

***Shared Memory Model***

*Shared Memory Model* allows multiple threads to access and modify the same data structure in memory.
Fot the example, I've created a si mple program that returns a file stats (loc, comments, blank lines, etc.) that read a file with directories and then find all files in there and calculate the statistic, it allocates one thread per directory. 

```rust
// shared memory model code snippet
// arc for multiple owners and mutex for locking data for a thread
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..5 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.locks().unwrap();
        *num += 1;
    });
    handles.push(handle);
}
// joining threads to the main thread so they finish before the main thread
for handle in handles {
    handle.join().unwrap();
}
println!("Result: {}", *counter.lock().unwrap());
```