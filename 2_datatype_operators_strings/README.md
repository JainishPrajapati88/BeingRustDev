# Rust Basics: Data Types and Operators

This repository contains a Rust code snippet demonstrating fundamental data types and basic operations in Rust programming language.

## Data Types

### Integers
Rust supports both signed and unsigned integers. Examples include:
- Default creation of a signed integer of size 32 bits: `let a = 10;`
- Specifying sizes: `let age: i64 = 17; // signed int sized 64bits`
- CPU architecture-dependent sizes: `let marks: isize = 100;`

### Floats
Default is `f64` for floating-point numbers. Example: `let rank: f32 = 1.11;`

### Boolean
Boolean types represent `true` or `false`. Example: `let you_got_admission: bool = true;`

### Character
Characters are enclosed in single quotes. Example: `let first_name_char: char = 'J';`

### Strings
Strings can be defined with `&str` or using `String` objects. Examples:
- `let first_name: &str = "Jainish";`
- `let str = String::new();` and `let str2 = String::from("initialized at declaration of string");`

## Type Casting
Conversion between different types such as float to int, char to ASCII, bool to int is demonstrated.

## Operators

### Arithmetic Operators
Basic arithmetic operations like addition, subtraction, multiplication, division, and modulus are demonstrated.

### Assignment Operators
Assignment operators such as `+=`, `-=`, etc., are showcased.

### Comparison Operators
Operators like `>`, `<`, `>=`, `<=`, `!=`, and `==` are explained (will be implemented in control statements).

### Logical Operators
Logical operators like `&&`, `||`, `!` are introduced (will be implemented in control statements).

The code provides examples and explanations for these concepts. For further details and examples, refer to the code snippet itself.

Feel free to explore and experiment with the provided code!

