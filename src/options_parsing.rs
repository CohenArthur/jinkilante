use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Jinkilante", about, author)]
struct Options {
    #[structopt(short, long,
                help = "Specific process to keep track of")]
    process: u32,

    #[structopt(short, long,
                help = "Number of strikes before a process is terminated")]
    strikes: u32,

    #[structopt(short, long,
                help = "Number of seconds before a process is terminated")]
    timeout: u32,

    #[structopt(short, long,
                help = "Percentage of CPU usage required for a 
                        process to get striked")]
    cpu: u8,

    #[structopt(short, long,
                help = "Percentage of RAM usage required for a 
                        process to get striked")]
    mem: u8,
}
