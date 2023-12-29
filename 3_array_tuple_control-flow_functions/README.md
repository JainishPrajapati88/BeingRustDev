# Rust Basics: Arrays, Tuples, Control Flow, Functions, and User Input

This repository contains a Rust code snippet demonstrating fundamental concepts including arrays, tuples, control flow statements, functions, and user console input.

## Arrays and Tuples

### Tuples
Tuples in Rust are defined within `()` and can store multiple elements of different types.
- Example:
    ```rust
    let tup = (300, 200, 100);
    println!("{:?}", tup);
    ```

### Arrays
Arrays are defined within `[]` and have a fixed size.
- Example:
    ```rust
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("{:?}", arr);
    ```

## Control Flow: Loops and Conditionals

### Loops
Demonstrates the use of loops like `while` and `for` with arrays.
- Example:
    ```rust
    let mut index = 0;
    while index < 5 {
        println!("{index} element is {arr[index]}");
        index += 1;
    }
    ```

### Conditionals
Shows the use of `if`, `if-else`, and `if-else-if` ladder.
- Example:
    ```rust
    if num % 2 == 0 {
        0
    } else {
        1
    }
    ```

## User Input Handling

Uses `std::io` for user console input.
- Example:
    ```rust
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    ```

## Functions

### Function Implementation
Implements a function to check whether a number is odd or even.
- Example:
    ```rust
    fn odd_or_even(val: usize) -> usize {
        if val % 2 == 0 {
            0
        } else {
            1
        }
    }
    ```

### Function Usage
Demonstrates how the function is called and utilized within the code.
- Example:
    ```rust
    let res: usize = odd_or_even(num);
    if res == 0 {
        println!("{num} is even");
    } else {
        println!("{num} is odd");
    }
    ```

The code snippet serves as an introduction to these fundamental Rust concepts. It showcases practical examples and their usage within the Rust programming language. Feel free to explore and experiment further based on this code!

For more detailed explanations and examples, refer to the code itself.

