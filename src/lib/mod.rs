mod incrementer;
pub use incrementer::increment;
use std::io;
use std::io::Write;

pub fn run() {
    print!("Insert a number: ");
    io::stdout().flush().unwrap();
    let mut value = String::new();

    if let Some((value, value_plus_1)) = io::stdin()
        .read_line(&mut value)
        .ok()
        .and_then(|_| value.trim().parse().ok())
        .map(|value| (value, incrementer::increment(value)))
    {
        println!("{} + 1 = {}", value, value_plus_1);
    } else {
        println!("Failed to read a number");
    }
}
