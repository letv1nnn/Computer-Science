use std::{io, ops::{Coroutine, CoroutineState}, pin::Pin, time::Instant};

use crate::{coroutine_basic_example::WriteCoroutine, simple_generator::ReadCoroutine};

struct CoroutineManager {
    reader: ReadCoroutine,
    writer: WriteCoroutine,
}

impl CoroutineManager {
    fn new(read_path: &str, write_path: &str) -> io::Result<Self> {
        let reader = ReadCoroutine::new(read_path)?;
        let writer = WriteCoroutine::new(write_path)?;

        Ok(Self {
            reader,
            writer,
        })
    }
    fn run(&mut self) {
        let mut read_pin = Pin::new(&mut self.reader);
        let mut write_pin = Pin::new(&mut self.writer);
    
        loop {
            match read_pin.as_mut().resume(()) {
                CoroutineState::Yielded(number) => {
                    write_pin.as_mut().resume(number);
                }
                CoroutineState::Complete(()) => break,
            }
        }
    }
}


pub fn coroutine_stack_example() {
    let start = Instant::now();
    let mut manager = CoroutineManager::new("numbers.txt", "output.txt").unwrap();
    manager.run();
    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);
}