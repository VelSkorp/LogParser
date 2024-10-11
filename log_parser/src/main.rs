mod log_config;
mod log_counter;
mod log_printer;

use log_counter::LogCounter;
use log_printer::print_log;
use std::io;

fn main() -> io::Result<()> {
    let counter = LogCounter::count_logs()?;
    print_log()?;
    println!("Total number of {}: {}", &counter.log_levels.join(", "), &counter.count);

    Ok(())
}