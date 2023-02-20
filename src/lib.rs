
use std::process;
use std::fs;

use colored::*;

#[derive(Debug)]
pub struct ArgSwitch {
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub number: bool,
    pub squeeze_blank: bool,
    pub show_tabs: bool,
    pub u: bool,
    pub show_nonprinting:bool,
}

impl ArgSwitch {
    pub fn new() -> Self {
        ArgSwitch { 
            number_nonblank: false, 
            show_ends: false, 
            number: false, 
            squeeze_blank: false, 
            show_tabs: false, 
            u: false, 
            show_nonprinting: false, 
        }
    }
}


pub fn print_version() {

    let title = "rcat (RNGNU coreutils) 0.1".bright_cyan();

    let version = 
"Copyright (C) 2023 Frank Pereny
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Frank Pereny.";

    println!();
    println!("{title}");
    println!("{version}");
    println!();
}


pub fn print_help() {

    println!();
    print!("Usage: ");
    print!("{}", "rcat ".bright_cyan());
    print!("{}", "[OPTION...] ".bright_red());
    println!("{}", "[FILE]... ".bright_green());

    let help_menu = 

"Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help        display this help and exit
      --version     output version information and exit

Examples:
  rcat f - g  Output f's contents, then standard input, then g's contents.
  rcat        Copy standard input to standard output.

RNGNU online help: <https://www.frankpereny.com/rcat>
Full documentation <https://www.frankpereny.com/rcat/docs>
or available locally via: info '(coreutils) cat invocation'";

    println!("{help_menu}")
}


pub fn parse_args(args: &[String]) -> (ArgSwitch, Vec<&String>) {
    let mut file_paths: Vec<&String> = Vec::new();
    let mut arg_switch = ArgSwitch::new();
    for arg in args {
        if arg == &args[0] {continue;}

        let arg_str = arg.as_str();
        if arg_str.len() > 2 && arg_str[0..2].contains("--") {
            match arg_str {
                "--version" => {
                    print_version();
                    process::exit(0);
                },
                "--help" => {
                    print_help();
                    process::exit(0);
                },
                "--show-all" => {
                    arg_switch.show_ends = true;
                    arg_switch.show_nonprinting = true;
                    arg_switch.show_tabs = true;
                }
                "--number-nonblank" => arg_switch.number_nonblank = true,
                "--show-ends" => arg_switch.show_ends = true,
                "--number" => arg_switch.number = true,
                "--squeeze-blank" => arg_switch.squeeze_blank = true,
                "--show-tabs" => arg_switch.show_tabs = true,
                "--show-nonprinting" => arg_switch.show_nonprinting = true,
                _ => {
                    println!("Invalid switch parameter");
                    print_help();
                    process::exit(1);
                }
            };
        } else if arg_str.len() > 1 && arg_str[0..1].contains("-") {
            let chars = arg_str[1..].chars();
            for c in chars {
                match c {
                    'A' => {
                        arg_switch.show_ends = true;
                        arg_switch.show_nonprinting = true;
                        arg_switch.show_tabs = true;
                    }
                    'b' => arg_switch.number_nonblank = true,
                    'e' => { 
                        arg_switch.show_ends = true;
                        arg_switch.show_nonprinting = true;
                    },
                    'E' => arg_switch.show_ends = true,
                    'n' => arg_switch.number = true,
                    's' => arg_switch.squeeze_blank = true,
                    't' => {
                        arg_switch.show_tabs = true;
                        arg_switch.show_nonprinting = true;
                    },
                    'T' => arg_switch.show_tabs = true,
                    'u' => continue,
                    'v' => arg_switch.show_nonprinting = true,
                    _ => {
                        println!("Invalid switch parameter.\n");
                        print_help();
                        process::exit(1);
                    }                        
                }
            }
        } else {
            file_paths.push(&arg);
        }
    }
    (arg_switch, file_paths)
}


pub fn get_content(file_path: &str, arg_switch: &ArgSwitch) -> String {
    let content = fs::read(file_path).unwrap();
    let mut result = String::new();
    let mut i = 1;
    let mut prev_prev_char = '\0';
    let mut prev_char = '\n';
    for c in content {
        let c = c as char;

        if arg_switch.show_tabs && c == '\t' {
            result.push_str("^I");
            continue;
        }

        if arg_switch.squeeze_blank {
            match c {
                '\n' => {
                    if prev_char == '\n' && prev_prev_char == '\n' {
                        prev_prev_char = prev_char;
                        prev_char = c;
                        continue;
                    }
                }
                _ => {}
            };
        }

        if arg_switch.number_nonblank && prev_char == '\n' {
            if c != '\n' {
                let num = &i.to_string();
                let space_count = 6 - num.len();
                result.push_str(&" ".repeat(space_count));
                result.push_str(&num);
                result.push_str("  ");
                i += 1;
            }
        } else if arg_switch.number && prev_char == '\n' {
            let num = &i.to_string();
            let space_count = 6 - num.len();
            result.push_str(&" ".repeat(space_count));
            result.push_str(&num);
            result.push_str("  ");
            i += 1;
        } 
        
        if arg_switch.squeeze_blank {
            match c {
                '\n' => {
                    if prev_char == '\n' && prev_prev_char == '\n' {
                        prev_prev_char = prev_char;
                        prev_char = c;
                        continue;
                    }
                }
                _ => {}
            };
        }
        
        if arg_switch.show_ends  && c == '\n' {
            result.push('$');
        }

        result.push(c);
        
        prev_prev_char = prev_char;
        prev_char = c;
    }
    result
}


pub fn print_all_content(file_paths: &Vec<&String>, arg_switch: &ArgSwitch) {
    for file in file_paths {
        let content = get_content(file, arg_switch);
        println!("{content}");
    }
}