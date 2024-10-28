use csv::ReaderBuilder;
use reqwest;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt}; // Import sysinfo for resource tracking

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize system for memory usage
    let system = System::new_all();
    let process = system.process(sysinfo::get_current_pid().unwrap()).unwrap();
    let start_memory: u64 = process.memory(); // Memory usage in KB

    // Start time measurement
    let start_time = Instant::now();

    // Set up the CSV reader to read from the URL
    let url: &str = "https://github.com/fivethirtyeight/data/raw/refs/heads/master/most-common-name/surnames.csv";
    let response_text = reqwest::blocking::get(url)?.text()?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_reader(response_text.as_bytes());

    // Calculate the total count
    let mut total_count: i64 = 0;
    for result in rdr.records() {
        let record = result?;
        if let Some(count_str) = record.get(2) {
            if let Ok(count) = count_str.parse::<i64>() {
                total_count += count;
            }
        }
    }

    // End time measurement and memory usage
    let duration = start_time.elapsed();
    let end_memory: u64 = process.memory(); // Memory usage in KB

    // Save results to a markdown file with a specific name
    let mut file = File::create("rust_output.md")?;
    writeln!(file, "# Rust Analysis Results\n")?;
    writeln!(file, "**Total Count of People by Surname:** {}\n", total_count)?;
    writeln!(file, "**Rust Execution Time:** {:.6} seconds\n", duration.as_secs_f64())?;
    writeln!(file, "**Rust Memory Usage:** {} KB\n", end_memory - start_memory)?;

    Ok(())
}
