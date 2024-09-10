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

        #[arg(short, long)]
        print: bool,
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

    /// Switch branches or restore working tree files
    Checkout {
        name: String,

        #[arg(short, help = "Create a new branch named <BRANCHNAME>")]
        branch: bool,
    },

    /// Tag a commit with a named reference
    Tag {
        name: String,
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => cmd::init::main(),
        Some(Commands::Reset {}) => cmd::reset::main(),
        Some(Commands::HashObject { path, print }) => cmd::hash_object::main(path, print),
        Some(Commands::CatFile { hash }) => cmd::cat_file::main(hash),
        Some(Commands::WriteTree { print }) => cmd::write_tree::main(print),
        Some(Commands::ReadTree { hash }) => cmd::read_tree::main(hash),
        Some(Commands::Commit { message }) => cmd::commit::main(message),
        Some(Commands::Log {}) => cmd::log::main(),
        Some(Commands::Checkout { name, branch }) => cmd::checkout::main(name, branch),
        Some(Commands::Tag { name, hash }) => cmd::tag::main(name, hash),
        None => println!("Welcome to gyat. Use -h to see usage."),
    };
}
