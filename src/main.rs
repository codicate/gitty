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
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let res = match &cli.command {
        Some(Commands::Init {}) => cmd::init::main(),
        Some(Commands::Reset {}) => cmd::reset::main(),
        Some(Commands::HashObject { path, write }) => cmd::hash_object::main(path, write, &true),
        Some(Commands::CatFile { hash }) => cmd::cat_file::main(hash),
        Some(Commands::WriteTree { print }) => cmd::write_tree::main(print),
        None => Ok(println!("Welcome to gyat. Use -h to see usage.")),
    };

    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Unexpected Error: {}", e),
    }
}
