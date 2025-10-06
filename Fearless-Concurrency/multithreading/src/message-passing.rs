use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move ||{
        let vec: Vec<String> = vec!["one".into(), "two".into(), "three".into(), "four".into()];
        for el in vec {
            tx1.send(el).unwrap();
        }
    });

    thread::spawn(move || {
        let vec: Vec<String> = vec!["Five".into(), "Six".into(), "Seven".into(), "Eight".into()];
        for el in vec {
            tx2.send(el).unwrap();
        }
    });

    for recv in rx {
        println!("Received value: {:?}", recv);
    }

}