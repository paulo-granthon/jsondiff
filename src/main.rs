use serde_json::{Value, Error};
use std::env;
use std::fs;
use colored::*;

fn read_file_to_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(path).map_err(|e| e.into())
}

fn compare_json_objects(left: &Value, right: &Value, path: &str, file_identifier: &str) {
    match (left, right) {
        (Value::Array(left_arr), Value::Array(right_arr)) => {
            for (i, (left_item, right_item)) in left_arr.iter().zip(right_arr.iter()).enumerate() {
                let new_path = format!("{}[{}]", path, i);
                compare_json_objects(left_item, right_item, &new_path, file_identifier);
            }
        }
        (left, right) => {
            if left != right {
                println!("--------------------------");
                println!("Array: {} ({}: [{}])", path, file_identifier, path);
                println!("--------------------------");
                println!("Mismatch at '{}':", path);

                let left_str = serde_json::to_string_pretty(&left).unwrap_or("Invalid JSON".to_string());
                let right_str = serde_json::to_string_pretty(&right).unwrap_or("Invalid JSON".to_string());

                let left_lines = left_str.lines().collect::<Vec<&str>>();
                let right_lines = right_str.lines().collect::<Vec<&str>>();

                for (i, line) in left_lines.iter().enumerate() {
                    if line != right_lines.get(i).unwrap_or(&"") {
                        println!("L: {}", line.red().bold());
                        if i < right_lines.len() {
                            println!("R: {}", right_lines[i].green().bold());
                        }
                    } else {
                        println!("L: {}", line);
                    }
                }
                if left_lines.len() < right_lines.len() {
                    for &line in right_lines[left_lines.len()..].iter() {
                        println!("R: {}", line.green().bold());
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: command <file1.json> <file2.json>");
        std::process::exit(1);
    }

    let left_json = match read_file_to_string(&args[1]) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to read left file: {}", err);
            std::process::exit(1);
        }
    };

    let right_json = match read_file_to_string(&args[2]) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to read right file: {}", err);
            std::process::exit(1);
        }
    };

    println!("Json Diff | Comparing `{}` (L) with `{}` (R)", &args[1], &args[2]);

    let left_data: Value = serde_json::from_str(&left_json)?;
    let right_data: Value = serde_json::from_str(&right_json)?;

    compare_json_objects(&left_data["data"], &right_data["data"], "data", &args[1]);

    Ok(())
}

