use monoglu::{
    controller::Controller,
    message::Message,
};
use std::{
    io,
    sync::mpsc,
    thread,
};

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(|| {
        let mut controller = Controller::new(rx);
        controller.init();
    });

    loop {
        print_commands();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn print_commands() {
    println!("");
}
