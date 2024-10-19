# Overview

This project was learning more about functional programming with the Rust language.
I wanted to get a grasp on the syntax used for declareing variable, functions, and
getting user input/output. I learned about different data types and how you can type 
cast safely. 

As a fun little project to experiment with, I decided to solve a problem I encountered
in one of my hobbies. I recently put much larger tires on my off road Suzuki Samurai.
I knew that doing this would make my speedometer inaccurate. I wanted to create a 
calculator that would give an accurate speed reading taking the engine RPM, tire 
height, and gear selection. 

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

This program was written in rustc 1.81.0 with cargo 1.81.0.
I used Rust's standard tools to make it, with a few modules declared at the top:
- std::io 
- - this let me read and write values from the terminal.
- std::f32::consts
- - This let me use some math constants in 32 bit floating point numbers.

For my development enviroment I used VS Code with the following extensions:
- Rust Analyzer
- CodeLLDB
- Cargo
- Even Better TOML
- Dependi
- Rust Mod Generator

# Useful Websites

- [Tutorialspoint Rust](https://www.tutorialspoint.com/rust/index.htm)
- [The Rust Programming Language](https://doc.rust-lang.org/book/#the-rust-programming-language)
- [Lighten up a bit](https://www.youtube.com/watch?v=xvFZjo5PgG0&ab_channel=Duran)

# Future Work

- Better UI
- More functions (find_rpm, duplicate function calls with more paramaters for different scenarios, ect..)
- Introduce OOP principles (A drivetrain class that holds all the final drive info, tire sizes, meathods to switch tire size/axle gears, ect..)
