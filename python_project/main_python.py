import pandas as pd
import time
import psutil

# Start time and memory usage
process = psutil.Process()
start_time = time.time()
start_memory = process.memory_info().rss  # in bytes

# Load the data directly from the URL
data_surnames = pd.read_csv("https://github.com/fivethirtyeight/data/raw/refs/heads/master/most-common-name/surnames.csv")

# Clean the data by dropping rows with any missing values
data_surnames.dropna(inplace=True)

# Calculate the total count of people by surname
total_count = data_surnames['count'].sum()

# End time and memory usage
end_time = time.time()
end_memory = process.memory_info().rss  # in bytes

# Save results to a markdown file with a specific name
with open("python_output.md", "w") as f:
    f.write("# Python Analysis Results\n\n")
    f.write(f"**Total Count of People by Surname**: {total_count}\n\n")
    f.write(f"**Python Execution Time**: {end_time - start_time:.6f} seconds\n\n")
    f.write(f"**Python Memory Usage**: {end_memory - start_memory} bytes\n")
