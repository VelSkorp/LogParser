use std::fs::File;
use std::io::{self, BufRead, Write};
use rayon::prelude::*;
use regex::Regex;

fn main() -> io::Result<()> {
    print!("Write path to log file: ");

    io::stdout().flush().unwrap();

    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read path");

    let path = path.trim();
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    print!("Write log level (ERROR, WARNING, INFO, DEBUG): ");

    io::stdout().flush().unwrap();

    let mut log_level = String::new();

    io::stdin()
        .read_line(&mut log_level)
        .expect("Failed to read log level");

    let log_level = log_level.trim();
    let regex = Regex::new(log_level).unwrap();
    let chunk_size = 1000;
    let mut lines: Vec<String> = Vec::new();
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        lines.push(line);

        if lines.len() >= chunk_size {
            let chunk = lines.split_off(0);
            count += process_log_lines(chunk, &regex);
        }
    }

    if !lines.is_empty() {
        count += process_log_lines(lines, &regex);
    }

    println!("Total number of {log_level}: {count}");

    Ok(())
}

fn process_log_lines(lines:Vec<String>, regex: &Regex) -> usize {
    lines.into_par_iter()
        .filter(|line| regex.is_match(line))
        .count()
}