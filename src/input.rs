use std::io;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_number(&self, prompt: &str) -> f64 {
        loop {
            let mut input = String::new();
            println!("{}", prompt);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => return num,
                Err(_) => println!("Please enter a valid number."),
            }
        }
    }
}
