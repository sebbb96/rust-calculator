use crate::history::{History, HistoryEntry};
use std::fs;
use std::io::Write;
pub struct Calculator {
    result: f64,
    has_result: bool,
    history: History,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            result: 0.0,
            has_result: false,
            history: History::new(),
        }
    }

    pub fn get_result(&self) -> Option<f64> {
        if self.has_result {
            Some(self.result)
        } else {
            None
        }
    }

    pub fn calculate(&mut self, operation: &str, a: f64, b: f64) {
        let result = match operation {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b != 0.0 {
                    a / b
                } else {
                    println!("Error: Cannot divide by zero");
                    return;
                }
            }
            _ => {
                println!("Invalid operation");
                return;
            }
        };
        self.result = result;
        self.has_result = true;
        self.history
            .add_entry(operation.to_string(), vec![a, b], result);
    }

    pub fn apply(&mut self, operation: &str, value: f64) {
        if !self.has_result {
            println!("Error: No current result to operate on");
            return;
        }
        self.calculate(operation, self.result, value);
    }

    pub fn clear(&mut self) {
        self.result = 0.0;
        self.has_result = false;
    }
    pub fn get_history(&self) -> &[HistoryEntry] {
        self.history.get_entries()
    }
    pub fn create_file_history(&self) -> std::io::Result<()> {
        let mut file = fs::File::create("operations_history.txt")?;
        for (i, entry) in self.history.get_entries().iter().enumerate() {
            writeln!(
                file,
                "{}. {} {} {} = {}",
                i + 1,
                entry.operands[0],
                entry.operation,
                entry.operands[1],
                entry.result
            )?;
        }
        Ok(())
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
