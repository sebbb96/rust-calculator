# Rust Command-Line Calculator

## Description

This project is a simple yet powerful command-line calculator implemented in Rust. It provides basic arithmetic operations with a user-friendly interface, demonstrating modular design and Rust programming concepts. The calculator now includes a history feature to track and review past calculations.

## Features

- Basic arithmetic operations: addition, subtraction, multiplication, and division
- Continuous calculation: apply operations to the previous result
- Clear function to reset the calculator
- Error handling for invalid inputs and division by zero
- Modular design for easy maintenance and extensibility
- History feature to track and review past calculations
- Option to clear calculation history

## Project Structure

The project is organized into several modules:

- `main.rs`: The entry point of the application, handling the main loop and user interaction
- `calculator.rs`: Implements the core calculation logic and manages calculation history
- `input.rs`: Manages user input handling
- `menu.rs`: Provides a menu interface for operation selection
- `history.rs`: Defines structures and methods for managing calculation history

## How to Use

1. Run the program
2. Select an operation from the menu
3. If there's no current result, enter two numbers
4. If there's a current result, enter one number to apply the operation
5. The result will be displayed, and you can continue with more operations
6. Select 'history' to view past calculations
7. Select 'clear history' to erase the calculation history
8. Select 'clear' to reset the calculator or 'exit' to quit the program

## Dependencies

- `dialoguer`: For creating an interactive command-line menu

## Building and Running

To build and run the project:

1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Navigate to the project directory
4. Run `cargo build` to build the project
5. Run `cargo run` to start the calculator

## Future Improvements

- Add more complex mathematical operations
- Implement the ability to recall and reuse specific calculations from history
- Add support for parentheses and order of operations
- Add the ability to save and load calculation history from a file

## License

[MIT](https://choosealicense.com/licenses/mit/)
