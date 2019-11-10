mod exec_watcher;
mod process_usage;
mod ps_parser;
mod striker;

use structopt::StructOpt;
use std::{thread, time};

use process_usage::ProcessUsage;
use exec_watcher::exec_watcher;
use striker::strike;

#[derive(Debug, StructOpt, Copy, Clone)]
#[structopt(name = "Jinkilante", about, author)]
pub struct Arg {
    #[structopt(short,
                long,
                help = "Specific process to keep track of",
                default_value = "0")]
    process: u32,

    #[structopt(short,
                long,
                help = "Number of strikes before a process is terminated",
                default_value = "10")]
    strikes: u32,

    #[structopt(short,
                long,
                help = "Number of seconds before a process is terminated",
                default_value = "100")]
    timeout: u32,

    #[structopt(short,
                long,
                help = "Percentage of CPU usage required for a
process to get striked",
                default_value = "90")]
    cpu: u8,

    #[structopt(short,
                long,
                help = "Percentage of RAM usage required for a
process to get striked",
                default_value = "90")]
    mem: u8,
}

fn main() {
    let options = Arg::from_args();
    let mut process_test = ProcessUsage::new(options.process);
    let wait_time = time::Duration::new(5, 0);

    loop {
        exec_watcher(&mut process_test);
        strike(options, &mut process_test);
        thread::sleep(wait_time);
    }
}
