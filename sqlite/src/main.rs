use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;


// Define a struct to hold the table data
#[derive(Serialize, Deserialize, Debug)]
struct Table {
   columns: Vec<String>,
   rows: Vec<HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Database {
   tables: HashMap<String, Table>,
}


impl Database {
   fn new() -> Self {
       Self {
           tables: HashMap::new(),
       }
   }


   // Load data from file
   fn load_from_file() -> Self {
       let file_content = fs::read_to_string("db.json").unwrap_or_else(|_| "{}".to_string());
       serde_json::from_str(&file_content).unwrap_or_else(|_| Database::new())
   }


   // Save data to file
   fn save_to_file(&self) {
       let data = serde_json::to_string(self).expect("Failed to serialize data");
       fs::write("db.json", data).expect("Failed to write data to file");
   }
}


fn main() {
   let mut db = Database::load_from_file(); // Load the database at the start


   let matches = Command::new("sqlite")
       .version("1.0")
       .author("Your Name")
       .about("Simulates SQLite CRUD operations")
       .subcommand(
           Command::new("create")
               .about("Creates a table")
               .arg(Arg::new("tablename").required(true).index(1))
               .arg(Arg::new("columns").required(true).num_args(1..).index(2)),
       )
       .subcommand(
           Command::new("insert")
               .about("Inserts data into a table")
               .arg(Arg::new("tablename").required(true).index(1))
               .arg(Arg::new("values").required(true).num_args(1..).index(2)),
       )
       .subcommand(
           Command::new("query")
               .about("Queries data from a table")
               .arg(Arg::new("tablename").required(true).index(1)),
       )
       .subcommand(
           Command::new("delete")
               .about("Deletes a table")
               .arg(Arg::new("tablename").required(true).index(1)),
       )
       .get_matches();


   // Handle "create" command
   if let Some(matches) = matches.subcommand_matches("create") {
       let tablename = matches.get_one::<String>("tablename").unwrap();
       let columns: Vec<String> = matches
           .get_many::<String>("columns")
           .unwrap()
           .map(|s| s.to_string())
           .collect();


       if db.tables.contains_key(tablename) {
           println!("Table '{}' already exists!", tablename);
       } else {
           db.tables.insert(
               tablename.to_string(),
               Table {
                   columns,
                   rows: Vec::new(),
               },
           );
           println!("Table '{}' created.", tablename);
       }
       db.save_to_file(); // Save the database after creating the table
   }


   // Handle "insert" command
   if let Some(matches) = matches.subcommand_matches("insert") {
       let tablename = matches.get_one::<String>("tablename").unwrap();
       let values: Vec<String> = matches
           .get_many::<String>("values")
           .unwrap()
           .map(|s| s.to_string())
           .collect();


       if let Some(table) = db.tables.get_mut(tablename) {
           if values.len() == table.columns.len() {
               let mut row = HashMap::new();
               for (i, col) in table.columns.iter().enumerate() {
                   row.insert(col.to_string(), values[i].to_string());
               }
               table.rows.push(row);
               println!("Data inserted into '{}'.", tablename);
           } else {
               println!(
                   "Error: Expected {} values, but got {}.",
                   table.columns.len(),
                   values.len()
               );
           }
           db.save_to_file(); // Save the database after inserting data
       } else {
           println!("Table '{}' not found!", tablename);
       }
   }


   // Handle "query" command
   if let Some(matches) = matches.subcommand_matches("query") {
       let tablename = matches.get_one::<String>("tablename").unwrap();
       if let Some(table) = db.tables.get(tablename) {
           println!("Table '{}' contents:", tablename);
           for row in &table.rows {
               for col in &table.columns {
                   print!("{}: {}, ", col, row.get(col).unwrap());
               }
               println!();
           }
       } else {
           println!("Table '{}' not found!", tablename);
       }
   }


   // Handle "delete" command
   if let Some(matches) = matches.subcommand_matches("delete") {
       let tablename = matches.get_one::<String>("tablename").unwrap();
       if db.tables.remove(tablename).is_some() {
           println!("Table '{}' deleted.", tablename);
           db.save_to_file(); // Save the database after deleting the table
       } else {
           println!("Table '{}' not found!", tablename);
       }
   }
}

