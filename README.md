Rust CICD: [![Rust_CICD](https://github.com/nogibjj/Nzarama_Kouadio_DE_Mini_Project8_test1/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Nzarama_Kouadio_DE_Mini_Project8_test1/actions/workflows/rust.yml)

Python CICD: [![Python_CICD](https://github.com/nogibjj/Nzarama_Kouadio_DE_Mini_Project8_test1/actions/workflows/python.yml/badge.svg)](https://github.com/nogibjj/Nzarama_Kouadio_DE_Mini_Project8_test1/actions/workflows/python.yml)

Python CICD: 

## Mini Project 8: Python vs. Rust Performance Comparison for Data Processing

This project demonstrates how to perform a data-processing task using both Python and Rust. The goal is to highlight performance differences between Python and Rust, focusing on speed and resource usage improvements when rewriting an existing Python script in Rust.

# Project Overview
This project implements a simple data-processing task that calculates the total count of people by surname from a CSV dataset. The task is performed in both Python and Rust, with the following objectives:

- **Functionality**: Ensure both scripts deliver the same results.

- **Performance**: Highlight improvements in speed and resource efficiency of the Rust implementation compared to Python.


# Set Up Instructions

You can run the project from source code using either the Python or Rust implementations, or by using the pre-built Rust binary executable. Follow the instructions below for each method.

> 1. Running the Python Script

- Install dependencies: `pip3 install -r requirements.txt`

- Run the python script: `python main_python.py`

> 2. Running the Rust Script

- Run the scrip: `cargo build --release`


# Test Set Up

- **Dataset**: [Most Common Surnames CSV Dataset](https://github.com/fivethirtyeight/data/raw/refs/heads/master/most-common-name/surnames.csv)

- **Task**: Calculate the total count of people by surname.

- **Environment**: GitHub Codespaces, Ubuntu 20.04

- **Python Version**: 3.11.2

- **Rust Version**: 1.66.0


# Important File Elements

- `main_python.py`: Python script that processes the CSV data.

- `main.rs`: Rust script that performs the same data-processing task.

- `requirements.txt`: Python dependencies for running the Python script.

- `Cargo.toml`: Rust dependencies for running the Rust project.


# Time Comparison

For detailed results, see [Rust Analysis Results](rust_project/rust_output.md) or [Python Analysis Results](python_project/python_output.md).

# Conclusion

> The Rust version of the script demonstrated significant improvements in execution time due to Rust’s memory safety and system-level performance optimizations. 

> Python, while versatile and easy to write, had slightly higher memory usage and was slower due to its interpreted nature.




