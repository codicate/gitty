use clap::{Parser, Subcommand};

mod cat_file;
mod hash_object;
mod init;
mod reset;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {},
    Reset {},

    /// Compute object ID and optionally create an object from a file
    HashObject {
        path: String,

        #[arg(short)]
        write: bool,
    },

    CatFile {
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

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
        Some(Commands::Init {}) => init::main(),
        Some(Commands::Reset {}) => reset::main(),
        Some(Commands::HashObject { path, write }) => hash_object::main(path, write),
        Some(Commands::CatFile { hash }) => cat_file::main(hash),
        None => println!("Welcome to gyat. Use -h to see usage."),
    }
}
