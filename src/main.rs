

use std::env;
use std::process;

use rcat;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (arg_switches, file_paths) = rcat::parse_args(&args);
    // dbg!(&arg_switches);
    // dbg!(&file_paths);

    rcat::print_all_content(&file_paths, &arg_switches);
    process::exit(0);
}

