use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::time::Instant;
use rand::Rng;

fn append_number_to_file(number: i32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("numbers.txt")?;
    writeln!(file, "{}", number)?;
    Ok(())
}

pub fn run_coroutine_example() -> io::Result<()> {
    let mut rng = rand::rng();
    let numbers: Vec<i32> = (0..20000).map(|_| rng.random()).collect();

    let start = Instant::now();
    for &number in &numbers {
        if let Err(e) = append_number_to_file(number) {
            eprintln!("Failed to write to file: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);
    
    // leveraging coroutines
    let mut coroutine = WriteCoroutine::new("numbers.txt")?;
    for &number in &numbers {
        Pin::new(&mut coroutine).resume(number);
    }
    
    Ok(())
}

// using coroutines for the same task
pub struct WriteCoroutine {
    pub file_handle: File,
}

impl WriteCoroutine {
    pub fn new(path: &str) -> io::Result<Self> {
        let file_handle = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        Ok(Self {file_handle})
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Yield = ();
    type Return = ();

    fn resume(mut self: std::pin::Pin<&mut Self>, arg: i32) -> std::ops::CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{:?}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

