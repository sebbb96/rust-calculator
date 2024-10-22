pub struct Calculator {
    result: f64,
    has_result: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            result: 0.0,
            has_result: false,
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
        self.result = match operation {
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
        self.has_result = true;
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
}
