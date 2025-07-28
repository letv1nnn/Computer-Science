#![allow(unused)]

#![feature(coroutines)]
#![feature(coroutine_trait)]

use std::fs::{File, OpenOptions};
use std::io::{Write, self, BufRead, BufReader};
use std::time::Instant;
use rand::Rng;

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() -> io::Result<()> {
    // test_straightforward_approach()?; // brute force approach
    // test_coroutine_approach()?; // coroutine approach
    // test_generator_coroutine()?;
    // test_coroutine_stack();
    
    test_symmetric_coroutine()?;

    Ok(())
}

// employing a coroutine to handle the writing of the integers
struct WriteCoroutine {
    pub file_handle: File,
}

impl WriteCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file_handle = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self {file_handle} )
    }
}

// implementing the Coroutine trait
impl Coroutine<i32> for WriteCoroutine {
    type Return = ();
    type Yield = ();

    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

fn test_coroutine_approach() -> io::Result<()> {
    let mut rng = rand::rng();
    let numbers: Vec<i32> = (0..20000).map(|_| rng.random()).collect();

    let start = Instant::now();
    let mut coroutine = WriteCoroutine::new("numbers.txt")?;
    for &number in &numbers {
        Pin::new(&mut coroutine).resume(number);
    }
    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);
    Ok(())
}

// straightforward approach
fn append_number_to_file(num: i32) -> io::Result<()> {
    let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("numbers.txt")?;
    writeln!(file, "{}", num)?;
    Ok(())
}

fn test_straightforward_approach() -> io::Result<()> {
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
    Ok(())
}

// implementation of a simple generator
struct ReadCoroutine {
    lines: io::Lines<BufReader<File>>,
}

impl ReadCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Ok(Self { lines })
    }
}

impl Coroutine for ReadCoroutine {
    type Yield = i32;
    type Return = ();

    fn resume(mut self: Pin<&mut Self>, arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        match self.lines.next() {
            Some(Ok(line)) => {
                if let Ok(number) = line.parse::<i32>() {
                    CoroutineState::Yielded(number)
                } else {
                    CoroutineState::Complete(())
                }
            }
            Some(Err(_)) | None => CoroutineState::Complete(())
        }
    }
}

fn test_generator_coroutine() -> io::Result<()>{
    let mut coroutine = ReadCoroutine::new("./data.txt")?;

    loop {
        match Pin::new(&mut coroutine).resume(()) {
            CoroutineState::Yielded(number) => println!("{:?}", number),
            CoroutineState::Complete(()) => break,
        }
    }

    Ok(())
}

// stacking coroutines
// one coroutine reads a file and yields values
// while another coroutine recievecs values and write them to a file.
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


fn test_coroutine_stack() {
    let start = Instant::now();
    let mut manager = CoroutineManager::new("numbers.txt", "output.txt").unwrap();
    manager.run();
    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);
}

// Calling coroutine from a coroutine
// Symmetric Coroutine
trait SymmetricCoroutine {
    type Input;
    type Output;

    fn resume_with_input(self: Pin<&mut Self>, input: Self::Input) -> Self::Output;
}

// implementing this trait for our ReadCoroutine.
impl SymmetricCoroutine for ReadCoroutine {
    type Input = ();
    type Output = Option<i32>;

    fn resume_with_input(mut self: Pin<&mut Self>, input: Self::Input) -> Self::Output {
        if let Some(Ok(line)) = self.lines.next() {
            line.parse::<i32>().ok()
        } else {
            None
        }
    }
}

// implement this trait for the WriteCoroutine as well
impl SymmetricCoroutine for WriteCoroutine {
    type Input = i32;
    type Output = ();

    fn resume_with_input(mut self: Pin<&mut Self>, input: Self::Input) -> Self::Output {
        writeln!(self.file_handle, "{}", input).unwrap();
    }
}

// put these together in the test function
fn test_symmetric_coroutine() -> io::Result<()> {
    let start = Instant::now();

    let mut reader = ReadCoroutine::new("numbers.txt")?;
    let mut writer = WriteCoroutine::new("output.txt")?;

    loop {
        let number = Pin::new(&mut reader).resume_with_input(());
        if let Some(num) = number {
            Pin::new(&mut writer).resume_with_input(num);
        } else {
            break;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);

    Ok(())
}
