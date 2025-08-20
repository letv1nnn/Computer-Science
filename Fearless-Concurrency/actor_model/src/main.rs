#![allow(unused)]

use std::io;
use actix::prelude::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let addr = MyActor{handled: 0}.start();

    for _ in 0..4 {
        match addr.send(Ping).await {
            Ok(Ok(_)) => println!("Ping delivered and handled"),
            Ok(Err(e)) => eprintln!("Handler returned error: {e}"),
            Err(mailbox_err) => eprintln!("Actor not reachable: {mailbox_err}"),
        }
    }
    Ok(())
}

struct Ping;

// Messages must implement `Message`. They declare the reply type.
impl Message for Ping {
    type Result = Result<bool, io::Error>;
}

// define actor
struct MyActor {
    handled: u32
}

// provide actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("MyActor started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("MyAgent stopped");
    }
}

// define handler 'Ping' message
impl Handler<Ping> for MyActor {
    type Result = Result<bool, io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.handled += 1;
        println!("Ping received (handled = {})", self.handled);
        Ok(true)
    }
}

