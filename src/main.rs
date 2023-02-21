

use std::env;
use std::process;

use catr;


fn main() {
    let args: Vec<String> = env::args().collect();
    let (arg_switches, file_paths) = catr::parse_args(&args);
    catr::print_all_content(&file_paths, &arg_switches);
    process::exit(0);
}

