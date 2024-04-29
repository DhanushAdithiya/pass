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
    //Store { folder_path: String },
    Init {},
    Add { website: String },
    Modify { website: String },
    Delete { website: String },
    Get { website: String },
    List {},
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
    home_folder.push(".passwords");
    let args = PasswordManager::parse();

    match args.command {
        Commands::Init {} => {
            // TODO: add a "." flag to initialize in the current dir
            create_dir(home_folder);
        }
        Commands::Add { mut website } => {
            website.push_str(".txt");

            let mut path = PathBuf::new();
            path.push(home_folder);
            path.push(Into::<std::path::PathBuf>::into(website));

            let f = fs::create_dir_all(&path.parent().unwrap());
            if f.is_ok() {
                println!("File created");

                let mut password = String::new();
                println!("Please input your password");
                std::io::stdin()
                    .read_line(&mut password)
                    .expect("no user input provided");

                let mut password_retype = String::new();
                println!("Please confirm password");
                std::io::stdin()
                    .read_line(&mut password_retype)
                    .expect("no user input provided");

                if password == password_retype {
                    println!("{path:?}");
                    fs::write(path, password).expect("could not write to file");
                    println!("Password Saved Sucessfully");
                } else {
                    eprintln!("Passwords do not match!");
                }
            } else {
                println!("Could not create a file, pleas check folder permissions");
            }
        }
        Commands::Modify { website } => todo!(),
        Commands::Delete { website } => todo!(),
        Commands::Get { website } => todo!(),
        Commands::List {} => todo!(),
    }
}
