use std::{ffi::OsStr, fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf, sync::{Arc, Mutex}, thread};


#[derive(Debug)]
struct Stats {
    pub number_of_files: u32,
    pub loc: u32,
    pub comments: u32,
    pub blanks: u32,
}

impl Stats {
    pub fn new() -> Self {
        return Self {
            number_of_files: 0,
            loc: 0,
            comments: 0,
            blanks: 0
        }
    }
}

fn main() {
    let stats = Stats::new();
    let stats_counter = Arc::new(Mutex::new(stats));

    let mut dir = match File::open("./dirs.txt") {
        Ok(dir) => dir,
        Err(e)  => {
            panic!("Failed opening the file: {}", e);
        },
    };
    let reader = BufReader::new(&mut dir);
    let dirs: Vec<_> = reader.lines().collect();

    let mut child_handles = vec![];
    for dir in dirs {
        let dir = dir.expect("Failed to parse this line!");
        let stats = Arc::clone(&stats_counter);

        let handle = thread::spawn(move || {
            let mut dir_entries = vec![PathBuf::from(dir)];
            let mut file_entries = vec![];
            while let Some(entry) = dir_entries.pop() {
                for inner_entry in fs::read_dir(&entry).unwrap() {
                    if let Ok(entry) = inner_entry {
                        if entry.path().is_dir() {
                            dir_entries.push(entry.path());
                        } else {
                            if entry.path().extension() == Some(OsStr::new("rs")) {
                                println!("File name processed is {:?}", entry);
                                file_entries.push(entry);
                            }
                        }
                    }
                }
            }
            for file in file_entries {
                let file_contents = fs::read_to_string(&file.path()).unwrap();

                let mut stats_pointer = stats.lock().unwrap();
                for line in file_contents.lines() {
                    if line.len() == 0 {
                        stats_pointer.blanks += 1;
                    } else if line.starts_with("//") {
                        stats_pointer.comments += 1;
                    } else {
                        stats_pointer.loc += 1;
                    }
                }

                stats_pointer.number_of_files += 1;
            }
        });

        child_handles.push(handle);
    }

    for handle in child_handles {
        handle.join().expect("Failed to join the handle!");
    }

    println!("\nSource stats: {:#?}", stats_counter.lock().unwrap());
}
