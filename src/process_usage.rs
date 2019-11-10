pub struct ProcessUsage {
    pid: u32,

    cpu_usage: u8,
    mem_usage: u8,
}

impl ProcessUsage {
    pub fn new(pid: u32) -> ProcessUsage {
        let new_proc_usage = ProcessUsage {
            pid: pid,
            cpu_usage: 0,
            mem_usage: 0,
        };

        new_proc_usage
    }

    pub fn kill(&mut self) {
        self.pid = 0; // FIXME: Remove
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
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
