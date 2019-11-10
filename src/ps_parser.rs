use crate::process_usage::ProcessUsage;

pub fn parse_ps_output(mut process: ProcessUsage, ps_output: &str) {

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

        parse_ps_output(test_process, "%CPU %MEM\n 0.0  0.0");

        assert_eq!(test_process.pid, 12);
        assert_eq!(test_process.cpu_usage, 0);
        assert_eq!(test_process.mem_usage, 0);
    }
}
