use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub struct ReadCoroutine {
    lines: io::Lines<BufReader<File>>,
}

impl ReadCoroutine {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        return Ok(Self {lines});
    }
}

impl Coroutine for ReadCoroutine {
    type Return = ();
    type Yield = i32;

    fn resume(mut self: Pin<&mut Self>, arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        match self.lines.next() {
            Some(Ok(line)) => {
                if let Ok(number) = line.parse::<i32>() {
                    CoroutineState::Yielded(number)
                } else {
                    CoroutineState::Complete(())
                }
            },
            Some(Err(_)) | None => CoroutineState::Complete(()) 
        }
    }
}
pub fn simple_generator_example() -> io::Result<()> {
    let mut coroutine = ReadCoroutine::new("./data.txt")?;
    loop {
        match Pin::new(&mut coroutine).resume(()) {
            CoroutineState::Yielded(number) => println!("{:?}", number),
            CoroutineState::Complete(()) => break,
        }
    }

    Ok(())
}

