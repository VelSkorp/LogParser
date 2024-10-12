use crate::log_config::load_config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
pub struct LogCounter {
    pub log_count: HashMap<String, usize>,
}

impl LogCounter {
    pub fn count_logs() -> Result<Self, io::Error> {
        let config = load_config();
        let file = File::open(&config.log_file)?;
        let reader = io::BufReader::new(file);
        let chunk_size = 1000;
        let mut lines: Vec<String> = Vec::new();
        let mut log_count: HashMap<String, usize> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            lines.push(line);

            if lines.len() >= chunk_size {
                let chunk = lines.split_off(0);
                log_count = process_log_lines(&chunk, &config.log_levels, log_count);
            }
        }

        if !lines.is_empty() {
            log_count = process_log_lines(&lines, &config.log_levels, log_count);
        }

        Ok(LogCounter { log_count })
    }
}

fn process_log_lines(
    lines: &Vec<String>,
    log_levels: &Vec<String>,
    mut log_count: HashMap<String, usize>
) -> HashMap<String, usize> {
    for level in log_levels {
        let regex = Regex::new(&level).unwrap();
        let count = lines.par_iter()
            .filter(|line| regex.is_match(line)) 
            .count();
        
        log_count.entry(level.to_string())
            .and_modify(|value| *value += count)
            .or_insert(count);
    }
    log_count
}