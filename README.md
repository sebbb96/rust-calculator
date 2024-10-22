# Rust Command-Line Calculator

## Description

This project is a simple yet powerful command-line calculator implemented in Rust. It provides basic arithmetic operations with a user-friendly interface, demonstrating modular design and Rust programming concepts. The calculator includes a history feature to track and review past calculations, with the ability to download this history to a file.

## Features

- Basic arithmetic operations: addition, subtraction, multiplication, and division
- Continuous calculation: apply operations to the previous result
- Clear function to reset the calculator
- Error handling for invalid inputs and division by zero
- Modular design for easy maintenance and extensibility
- History feature to track and review past calculations
- Option to clear calculation history
- Download calculation history to a file

## Installation

### From crates.io

The easiest way to install the Rust Calculator is via crates.io:

Make sure you have Rust and Cargo installed on your system. If not, you can install them from [rustup.rs](https://rustup.rs/).

### From source

To build and run the project from source:

1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Navigate to the project directory
4. Run `cargo build` to build the project
5. Run `cargo run` to start the calculator

## How to Use

1. After installation, run the program by typing `calc` in your terminal
2. Select an operation from the menu
3. If there's no current result, enter two numbers
4. If there's a current result, enter one number to apply the operation
5. The result will be displayed, and you can continue with more operations
6. Select 'history' to view past calculations
7. Select 'download history' to save the calculation history to a file
8. Select 'clear history' to erase the calculation history
9. Select 'clear' to reset the calculator or 'exit' to quit the program

### Downloading History

When you use the 'download history' command, a file named `operations_history.txt` will be created in your current directory. This file contains a record of all calculations performed in the current session.

## Project Structure

The project is organized into several modules:

- `main.rs`: The entry point of the application, handling the main loop and user interaction
- `calculator.rs`: Implements the core calculation logic and manages calculation history
- `input.rs`: Manages user input handling
- `menu.rs`: Provides a menu interface for operation selection
- `history.rs`: Defines structures and methods for managing calculation history

## Dependencies

- `dialoguer`: For creating an interactive command-line menu

## Future Improvements

- Add more complex mathematical operations
- Implement the ability to recall and reuse specific calculations from history
- Add support for parentheses and order of operations

## License

This project is licensed under the [MIT License](LICENSE).
