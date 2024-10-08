use crate::log_config::load_config;
use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
pub struct LogCounter {
    pub log_level: String,
    pub count: usize
}

impl LogCounter {
    pub fn count_logs() -> Result<Self, io::Error> {
        let config = load_config();
        let log_level = &config.log_level;
        let file = File::open(&config.log_file)?;
        let reader = io::BufReader::new(file);
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

        Ok(LogCounter { log_level: log_level.to_string(), count})
    }
}

fn process_log_lines(lines:Vec<String>, regex: &Regex) -> usize {
    lines.into_par_iter()
        .filter(|line| regex.is_match(line))
        .count()
}