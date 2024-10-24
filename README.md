## Mini Project 7: Rust CLI Project: Database Operations with Simple CRUD

This project demonstrates how to perform basic CRUD (Create, Read, Update, Delete) operations using a Command Line Interface (CLI) built with Rust. The CLI simulates database-like operations, allowing you to create tables, insert data, query the contents, and delete tables.

# Project Overview
The CLI supports the following operations:

- **Create**: Create a table with specified columns.

- **Insert**: Insert rows of data into a specified table.

- **Query**: Display the contents of a table.

- **Delete**: Remove a table from the database.

The project uses JSON to store table data, allowing persistence between runs. Data is saved in the db.json file.

# Set Up Instructions

- Install Rust on your machine

- Clone the repository: git clone https://github.com/nogibjj/Nzarama_Kouadio_DE_Mini_Project7.git

- Navigate to the project directory: cd sqlite

- Run the following to build and run the project: `cargo build` and `cargo run`

# Important File Elements

- `main.rs`: The main Rust file containing the CLI logic and CRUD operations.

- `db.json`: The file where table data is stored.

- `Cargo.toml`: Rust package file with dependencies.

# Explanation of JSON Database

The CLI tool uses a simple JSON file (db.json) to store all table data. Each table consists of a set of columns and rows. The database is saved after each operation to ensure persistence between program executions.

# Command and Usage: Create Table

- Create a new table with a specified name and columns:

`cargo run -- create <tablename> <column1> <column2> ...`

- Example: cargo run -- create users id name age

This will create a table called users with columns id, name, and age.

# Command and Usage: Insert Data into table

`cargo run -- insert <table_name> <value1> <value2> ... <valueN>` 

- Example: cargo run -- insert users 1 "Alice" 30

This inserts a row into the users table with the values 1, "Alice", and 30.


# Command and Usage: Query Table

- Retrieve and display the data from a specific table.

` cargo run -- query <table_name> `

- Example: cargo run -- query users

This will display all rows of the users table


# Command and Usage: Delete Table

`cargo run -- delete <table_name>`

- Example: cargo run -- delete users

This will delete the users table from the database.



