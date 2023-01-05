use monoglu::controller::Controller;
use std::io;

fn main() {
    let mut controller = Controller::new();
    loop {
        print_commands();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let (r, p, y) = {
            let rpy: Vec<f64> = input
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|i| i.parse::<f64>().unwrap())
                .collect();
            (rpy[0], rpy[1], rpy[2])
        };
        controller.pid(r, p, y);
    }
}

fn print_commands() {
    println!(
        "Enter target roll, pitch, yaw\ne.g. '90.0 -180.0 255.0'"
    );
}
