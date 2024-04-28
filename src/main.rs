use clap::{Parser, Subcommand};
use dirs;
use std::fs;
use std::path::PathBuf;

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

fn create_dir(folder_path: PathBuf) {
    if folder_path.exists() {
        println!("Directory already exists {:?}", folder_path);
    } else {
        fs::create_dir_all(folder_path)
            .expect("could not create folder, please check permissions.");
        println!("Passwords folder successfully created");
    }
}

fn main() {
    let mut home_folder = dirs::home_dir().unwrap();
    let args = PasswordManager::parse();

    match args.command {
        Commands::Store { folder_path } => {
            if folder_path.contains("~") {
                let (_, path) = folder_path.split_once("~/").unwrap();
                home_folder.push(&path);
                create_dir(home_folder);
            } else {
                create_dir(folder_path.into());
            }
        }
        Commands::Init {} => {
            home_folder.push(".passwords");
            create_dir(home_folder);
        }
    }
}
