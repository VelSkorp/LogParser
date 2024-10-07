use std::fs::File;
use std::io::{self, BufRead, Write};
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

    let re = Regex::new(log_level).unwrap();

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        if re.is_match(&line) {
            count += 1;
            println!("{}", line);
        }
    }

    println!("Total number of {log_level}: {count}");

    Ok(())
}
