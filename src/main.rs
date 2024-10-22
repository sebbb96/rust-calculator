mod calculator;
mod history;
mod input;
mod menu;
use calculator::Calculator;
use input::InputHandler;
use menu::Menu;
fn main() {
    let mut calculator = Calculator::new();
    let input_handler = InputHandler::new();
    let menu = Menu::new(vec![
        "+",
        "-",
        "*",
        "/",
        "clear",
        "history",
        "clear history",
        "exit",
    ]);

    loop {
        match calculator.get_result() {
            Some(result) => println!("Current result: {}", result),
            None => println!("No current result"),
        }
        let selection = menu.get_selection();
        match selection.as_str() {
            "+" | "-" | "*" | "/" => {
                if calculator.get_result().is_none() {
                    let a = input_handler.get_number("Enter the first number: ");
                    let b = input_handler.get_number("Enter the second number: ");
                    calculator.calculate(&selection, a, b);
                } else {
                    let value = input_handler.get_number("Enter a number: ");
                    calculator.apply(&selection, value);
                }
            }
            "history" => {
                println!("Calculation History:");
                for (i, entry) in calculator.get_history().iter().enumerate() {
                    println!(
                        "{}. {} {} {} = {}",
                        i + 1,
                        entry.operands[0],
                        entry.operation,
                        entry.operands[1],
                        entry.result
                    );
                }
            }
            "clear history" => {
                calculator.clear_history();
                println!("History cleared.");
            }
            "clear" => calculator.clear(),
            "exit" => break,
            _ => println!("Invalid option"),
        }
    }
}
