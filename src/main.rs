use clap::{Command, Arg, ArgAction};
use std::string::String;

fn main() {
    let matches = Command::new("cli")
        .version("0.1.0")
        .author("Kayn")
        .about("A simple command line tool")
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .help("Don't print the sum")
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new("verbose")
            .long("verbose")
            .help("Print more information")
            .action(ArgAction::SetTrue)
        )
        .subcommand(Command::new("add")
            .about("Add two numbers")
            .arg(Arg::new("first")
                .help("The first number")
                .required(true)
                .index(1))
            .arg(Arg::new("second")
                .help("The second number")
                .required(true)
                .index(2))
        )
        .get_matches();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");
    if let Some(add_matches) = matches.subcommand_matches("add") {
        let first: f64 = add_matches.get_one::<String>("first").expect("required")
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("Error: First argument must be a number");
                std::process::exit(1);
            });
        let second: f64 = add_matches.get_one::<String>("second").expect("required")
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("Error: First argument must be a number");
                std::process::exit(1);
            });
        if matches.get_flag("quiet") {
            println!("The sum is {}", first + second);
        } else {
            println!("The sum of {} and {} is {}", first, second, first + second);
        }
    } else if quiet {
        println!("Nothing to do");
    } else if verbose {
        println!("Please use subcommand add");
    } else {
        println!("No subcommand was used");
    }
    // println!("Hello, world!");
}
