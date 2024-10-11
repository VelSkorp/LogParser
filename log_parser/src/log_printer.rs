use crate::log_config::load_config;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub fn print_log() -> io::Result<()> {
    let config = load_config();
    let file = File::open(&config.log_file)?;
    let reader = io::BufReader::new(file);
    let regex: Vec<Regex> = config.log_levels.iter()
        .map(|level| Regex::new(&level).unwrap())
        .collect();

    reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| regex.iter().any(|re| re.is_match(&line)))
        .for_each(|line| println!("{line}"));
    
    Ok(())
}