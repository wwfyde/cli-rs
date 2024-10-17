use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author,version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long)]
    quiet: bool,

    /// Display the author information
    #[arg(long)]
    author: bool,

    /// Print more information
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,

    },

    #[command(about = "Add two numbers", long_about = "This command will add two numbers together")]
    Add {
        /// The first number
        first: f64,

        /// The second number
        second: f64,
    },
    /// Print all input arguments
    #[command(about = "Print all input arguments")]
    Print(PrintArgs),
}

#[derive(Args)]
struct PrintArgs {
    /// The arguments to print
    #[arg(required = true, num_args = 1.., value_name = "ARGS")]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.author {
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        return;
    }

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Add { first, second }) => {
            let sum = first + second;
            if cli.quiet {
                println!("The sum is {}", sum);
            } else {
                println!("The sum of {} and {} is {}", first, second, sum);
            }
        }

        Some(Commands::Print(print_args)) => {
            println!("You Input : {}", print_args.args.join(" "));
        }

        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}