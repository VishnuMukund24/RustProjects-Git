use std::fs::{self, OpenOptions};
use std::collections::HashMap;
use std::io::{self, Write};

// Reads the file and loads key-value pairs into a HashMap, Each line in the file is expect to be in the format: key=value
fn load_file(file_path: &str) -> HashMap<String, String> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let mut map = HashMap::new();

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            // Convert &str to String to own the data.
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

// Appends a new key-value pair to the file. Before appending, it checks if the file ends with a newline.

fn add_word_to_file(file_path: &str, key: String, value: String) {
    // Read current contents of the file
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file for newline check");

    // Open the file in append mode.
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    // Check if the file ends with a newline.
    if let Some(last_char) = contents.chars().last() {
        if last_char != '\n' {
            writeln!(file).expect("Error writing newline to file");
        }
    } else {
        // The file is empty; no need to insert an extra newline.
    }

    // Now append the new key-value pair, followed by a newline.
    writeln!(file, "{}={}", key, value)
        .expect("Error writing key-value pair to file");
}

fn main() {
    let file_path = "src/data.txt";

    // Load and print the file's key-value pairs.
    let map = load_file(file_path);
    println!("Current data:");
    for (key, value) in &map {
        println!("{} = {}", key, value);
    }

    // New key-value pair to add.
    let key = String::from("language");
    let value = String::from("Rust");

    // Append the new pair.
    add_word_to_file(file_path, key, value);

    // For verification, you could read and print the file again.
    let updated_contents = fs::read_to_string(file_path)
        .expect("Failed to read file after appending");
    println!("\nUpdated file contents:\n{}", updated_contents);
}