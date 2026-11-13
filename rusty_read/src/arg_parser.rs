use crate::flags::Flags;
use std::{collections::HashMap, process::exit, str::FromStr};

pub fn parser(args: Vec<String>) {
    let mut parsed_args: HashMap<Flags, Vec<String>> = HashMap::new();
    let mut current_arg: Option<Flags> = None;
    for arg in args {
        if arg.starts_with("-") {
            let flag = match Flags::from_str(&arg) {
                Ok(f) => f, 
                Err(e) => {
                    eprintln!("{}", e);
                    Flags::show_help();
                    exit(1);
                }
            };
            parsed_args.entry(flag).or_default();
            current_arg = Some(flag);
        } else {
            let current_arg = match current_arg {
                Some(f) => f,
                None => {
                    eprintln!("No valid flag found");
                    Flags::show_help();
                    exit(1);
                }
                
            };
            parsed_args.entry(current_arg).or_default().push(arg);
        }
    }
}
