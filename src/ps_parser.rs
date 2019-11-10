use crate::process_usage::ProcessUsage;

pub fn parse_ps_output(process: &mut ProcessUsage, ps_output: &str) {

    let split = ps_output.split_whitespace();
    let words = split.collect::<Vec<&str>>();

    process.cpu_usage = words[2].parse::<f32>().unwrap() as u8;
    process.mem_usage = words[3].parse::<f32>().unwrap() as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_zeros() {
        let mut test_process = ProcessUsage::new(12);

        parse_ps_output(&mut test_process, "%CPU %MEM\n 0.0  0.0");

        assert_eq!(test_process.pid, 12);
        assert_eq!(test_process.cpu_usage, 0);
        assert_eq!(test_process.mem_usage, 0);
    }

    #[test]
    fn test_parse_middle() {
        let mut test_process = ProcessUsage::new(12);

        parse_ps_output(&mut test_process, "%CPU %MEM\n47.8 23.4");

        assert_eq!(test_process.pid, 12);
        assert_eq!(test_process.cpu_usage, 47);
        assert_eq!(test_process.mem_usage, 23);
    }

    #[test]
    fn test_parse_max() {
        let mut test_process = ProcessUsage::new(12);

        parse_ps_output(&mut test_process, "%CPU %MEM\n100 100");

        assert_eq!(test_process.pid, 12);
        assert_eq!(test_process.cpu_usage, 100);
        assert_eq!(test_process.mem_usage, 100);
    }
}
