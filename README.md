# Mandlebrot-Fractal-Generation

# Overview

This program generates a command line rendering of the mandlebrot fractal.

Using Rust and a [complex numbers implimentation](https://crates.io/crates/num-complex) I was able to recreate the famous fractal. The formula z = z^2 + c is used in a loop to calculate the escape value for each "pixel" on the screen and render a loose image of the fractal.

The purpose of creating this was to practice Rust and programming art.

[Software Demo Video](https://youtu.be/3E3tsIqDcHw)

# Development Environment

For this project I used VSCode with Rust-analyzer

# Useful Websites

- [Rust Docs](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [Mandlebrot Wiki](https://en.wikipedia.org/wiki/Mandelbrot_set)
- [Similar Project](https://github.com/ProgrammingRust/mandelbrot)

# Future Work

- Create a way to zoom, change viewport
- Move out of command line to different rendering tool
- Better documentation

