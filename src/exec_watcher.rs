use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use crate::process_usage::ProcessUsage;

pub fn exec_watcher(mut process: ProcessUsage) {
    let pid = process.pid.to_string();
    let process = match Command::new("ps")
                                .args(&["-p", &pid, "-o", "%cpu,%mem"])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn ps: {}", why.description()),
        Ok(process) => process,
    };

    let mut output = String::new();
    match process.stdout.unwrap().read_to_string(&mut output) {
        Err(why) => panic!("couldn't read ps stdout: {}",
                           why.description()),
        Ok(_) => print!("ps responded with:\n{}", output),
    }
}
