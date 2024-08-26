use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn print_hello() {
    println!("Hello, world!");
}
