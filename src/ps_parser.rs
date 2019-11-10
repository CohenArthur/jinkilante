use crate::ProcessUsage;

pub fn parse_ps_output(mut process: ProcessUsage, ps_output: str) {
    let mut ps_lines = ps_output.lines();

    let to_parse = lines.next().next();

    let mut ps_word = to_parse.words();

    process.cpu_usage = to_parse.next().parse();
    process.mem_usage = to_parse.next().parse();
}
