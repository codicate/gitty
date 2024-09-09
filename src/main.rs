use clap::{Parser, Subcommand};
mod cmd;

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

    /// Store the current working directory to the object database
    WriteTree {
        #[arg(short)]
        print: bool,
    },

    /// Retrieve a stored directory from the object database using tree hash
    ReadTree {
        hash: String,
    },

    /// Record changes to the repository
    Commit {
        #[arg(short, required = true)]
        message: String,
    },

    // Show commit logs
    Log {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => cmd::init::main(),
        Some(Commands::Reset {}) => cmd::reset::main(),
        Some(Commands::HashObject { path, write }) => cmd::hash_object::main(path, write),
        Some(Commands::CatFile { hash }) => cmd::cat_file::main(hash),
        Some(Commands::WriteTree { print }) => cmd::write_tree::main(print),
        Some(Commands::ReadTree { hash }) => cmd::read_tree::main(hash),
        Some(Commands::Commit { message }) => cmd::commit::main(message),
        Some(Commands::Log {}) => cmd::log::main(),
        None => println!("Welcome to gyat. Use -h to see usage."),
    };
}
