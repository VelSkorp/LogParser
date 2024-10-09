mod log_config;
mod log_counter;

use log_counter::LogCounter;
use std::io;

fn main() -> io::Result<()> {
    let counter = LogCounter::count_logs()?;

    println!("Total number of {}: {}", &counter.log_levels.join(", "), &counter.count);

    Ok(())
}