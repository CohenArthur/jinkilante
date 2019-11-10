use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use crate::process_usage::ProcessUsage;
use crate::ps_parser::parse_ps_output;

pub fn exec_watcher(process: &mut ProcessUsage) {
    let pid = process.pid.to_string();
    let child = match Command::new("ps")
                                .args(&["-p", &pid, "-o", "%cpu,%mem"])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn ps: {}", why.description()),
        Ok(child) => child,
    };

    let mut output = String::new();
    match child.stdout.unwrap().read_to_string(&mut output) {
        Err(why) => panic!("couldn't read ps stdout: {}",
                           why.description()),
        Ok(_) => (),
    }

    parse_ps_output(process, &output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_watcher_null() {
        let mut process = ProcessUsage::new(1);

        exec_watcher(&mut process);

        assert_eq!(process.cpu_usage, 0);
        assert_eq!(process.mem_usage, 0);
    }
}
