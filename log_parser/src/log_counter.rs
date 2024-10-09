use crate::log_config::load_config;
use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
pub struct LogCounter {
    pub log_levels: Vec<String>,
    pub count: usize
}

impl LogCounter {
    pub fn count_logs() -> Result<Self, io::Error> {
        let config = load_config();
        let file = File::open(&config.log_file)?;
        let reader = io::BufReader::new(file);
        let chunk_size = 1000;
        let mut lines: Vec<String> = Vec::new();
        let mut count = 0;
        let regex: Vec<Regex> = config.log_levels.iter()
        .map(|level| Regex::new(&level).unwrap())
        .collect();

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

        Ok(LogCounter { log_levels: config.log_levels, count})
    }
}

fn process_log_lines(lines:Vec<String>, regex: &Vec<Regex>) -> usize {
    lines.into_par_iter()
        .filter(|line| regex.iter().any(|re| re.is_match(line)))
        .count()
}