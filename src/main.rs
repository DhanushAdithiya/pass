use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "Password Manger")]
#[command(about="A CLI password manager", long_about = None)]
struct PasswordManager {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Store { folder_path: String },
    Init {},
}

fn main() {
    let mut folder_path = "~/.passwords";
    let args = PasswordManager::parse();

    match args.command {
        Commands::Store { folder_path } => todo!(),
        Commands::Init {} => todo!(),
    }
}
