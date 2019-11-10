use std::error::Error;
use std::process::Command;

#[derive(Copy, Clone)]
pub struct ProcessUsage {
    pub pid: u32,
    pub strikes: u32,

    pub cpu_usage: u8,
    pub mem_usage: u8,
}

impl ProcessUsage {
    pub fn new(pid: u32) -> ProcessUsage {
        let new_proc_usage = ProcessUsage {
            pid: pid,
            strikes: 0,
            cpu_usage: 0,
            mem_usage: 0,
        };

        new_proc_usage
    }

    pub fn notify(&mut self) {
        let _child = match Command::new("notify-send")
                                    .args(&[
                                          "processs",
                                          &self.pid.to_string(),
                                          "striked !\n",
                                          "CPU Usage: ",
                                          &self.cpu_usage.to_string(),
                                          "\n",
                                          "MEM Usage: ",
                                          &self.mem_usage.to_string(),
                                           ])
                                    .spawn() {
            Err(why) => panic!("{}", why.description()),
            Ok(_child) => _child,
        };

        self.strikes = 0;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_process_init() {
        let new_p = ProcessUsage::new(12);

        assert_eq!(new_p.pid, 12);
        assert_eq!(new_p.cpu_usage, 0);
        assert_eq!(new_p.mem_usage, 0);
    }
}
