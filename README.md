# calc_cli

`calc_cli` is a simple command-line calculator program written in Rust. It performs basic arithmetic operations like addition, subtraction, multiplication, and division.

## Overview

This program prompts the user to input two numbers and choose an operation to perform on them. The result is then displayed in the console.

### Features

- **Add** two numbers.
- **Subtract** one number from another.
- **Multiply** two numbers.
- **Divide** two numbers (with error handling for division by zero).

## Usage

Since this project only contains the `main.rs` file, you'll need to manually set up a Rust project to use it. Follow these steps:

### 1. Set Up Your Rust Project

If you don't already have Rust installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust.

Once you have Rust installed, follow these steps to set up the project:

#### a. Create a New Rust Project:

In your terminal or command prompt, run the following command to create a new project:

`cargo init calc_cli --bin`

This will create a new folder called calc_cli with the necessary files for a Rust project, including a Cargo.toml and a src folder.

#### b. Replace main.rs:
After running cargo init, navigate to the src folder of your newly created project (calc_cli/src/). Replace the automatically generated main.rs file with the main.rs file from this repository.

To do this, download or copy the contents of main.rs from this repository into the src/main.rs file of your project.

#### 2. Run the Program
Once you've set up the project with the correct main.rs file:

Navigate to your project directory (if you aren't already there):
`cd calc_cli`
Build and run the program using Cargo:
`cargo run`
Follow the prompts to interact with the calculator:

Enter the first and second numbers.
Choose an operation (Add, Subtract, Multiply, Divide).
See the result displayed in the console.
```Example Output: 
WELCOME TO THE SIMPLE CALCULATOR!

Please enter the first number: 5
Please enter the second number: 3

OPERATION LIST:
1. Add
2. Subtract
3. Multiply
4. Divide

Please choose an operation number from the list (1-4): 1

ADD: 5 + 3 = 8.00
