
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

    let title = "catr 0.1".bright_cyan();

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
    print!("{}", "catr ".bright_cyan());
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
  catr f - g  Output f's contents, then standard input, then g's contents.
  catr        Copy standard input to standard output.

Git Repository: <https://github.com/fjpereny/rust-cat>
Crate.io <https://crates.io/crates/catr>";

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
    let content = fs::read(file_path).unwrap_or_else(|err| {
        println!();
        print!("{}", "WARNING:".on_bright_yellow());
        println!(" Unable to read file.");
        println!("({err})");
        process::exit(1);
    });
    let mut result = String::new();
    let mut i = 1;
    let mut prev_prev_char = '\0';
    let mut prev_char = '\n';
    for c in content {
        if arg_switch.show_nonprinting {
            if c <= 31 || c == 127 {
                result.push('^');
                match c {
                    0 => result.push('@'),
                    1 => result.push('A'),
                    2 => result.push('B'),
                    3 => result.push('C'),
                    4 => result.push('D'),
                    5 => result.push('E'),
                    6 => result.push('F'),
                    7 => result.push('G'),
                    8 => result.push('H'),
                    9 => result.push('I'),
                    10 => result.push('J'),
                    11 => result.push('K'),
                    12 => result.push('L'),
                    13 => result.push('M'),
                    14 => result.push('N'),
                    15 => result.push('O'),
                    16 => result.push('P'),
                    17 => result.push('Q'),
                    18 => result.push('R'),
                    19 => result.push('S'),
                    20 => result.push('T'),
                    21 => result.push('U'),
                    22 => result.push('V'),
                    23 => result.push('W'),
                    24 => result.push('X'),
                    25 => result.push('Y'),
                    26 => result.push('Z'),
                    27 => result.push('['),
                    28 => result.push('\\'),
                    29 => result.push(']'),
                    30 => result.push('^'),
                    31 => result.push('_'),
                    127 => result.push('?'),
                    _ => {
                        let c = c as char;
                        result.push(c);
                    }
                }
                continue;
            }            
        }

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

        if is_printable(&c) || arg_switch.show_nonprinting {
            result.push(c);
        }
        
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


fn is_printable(char: &char) -> bool {

    if char.is_ascii_alphanumeric() {
        return true;
    }

    if char.is_whitespace() {
        return true;
    }

    if char.is_ascii_punctuation() {
        return true;
    }

    if char.is_ascii_control() {
        return true;
    }

    if char.is_ascii_graphic() {
        return true;
    }

    false    
}