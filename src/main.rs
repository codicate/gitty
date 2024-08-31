use clap::{Parser, Subcommand};

mod cat_file;
mod hash_object;
mod init;
mod reset;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {},
    // DEBUG
    Reset {},

    /// Compute object ID and optionally create an object from a file
    HashObject {
        path: String,

        #[arg(short)]
        write: bool,
    },

    /// Provide contents or details of repository objects
    CatFile {
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

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
