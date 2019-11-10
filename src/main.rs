mod process_usage;
mod exec_watcher;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Jinkilante", about, author)]
struct Arg {
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

    exec_watcher::exec_watcher();
}
