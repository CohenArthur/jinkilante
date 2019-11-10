use crate::Arg;
use crate::process_usage::ProcessUsage;

pub fn strike(args: Arg,
              process: &mut ProcessUsage) {
    if process.strikes > args.strikes {
        process.notify();
    }

    if process.cpu_usage > args.cpu ||
       process.mem_usage > args.mem {
        process.strikes += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_none() {
        let options = Arg {
            cpu: 90,
            mem: 90,
            strikes: 10,
            timeout: 256,
            process: 1,
        };

        let mut process = ProcessUsage::new(options.process);

        strike(options, &mut process);

        assert_eq!(process.strikes, 0);
    }

    #[test]
    fn test_strike_plus_one() {
        let options = Arg {
            cpu: 50,
            mem: 50,
            strikes: 10,
            timeout: 256,
            process: 1,
        };

        let mut process = ProcessUsage::new(options.process);

        process.cpu_usage = 80;
        process.mem_usage = 80;

        strike(options, &mut process);

        assert_eq!(process.strikes, 1);
    }
}
