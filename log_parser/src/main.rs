use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    println!("Write path to log file: ");

    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"ERROR").unwrap();

    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        if re.is_match(&line) {
            error_count += 1;
            println!("{}", line);
        }
    }

    println!("Total number of errors: {}", error_count);

    Ok(())
}
