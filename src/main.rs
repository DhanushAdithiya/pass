use clap::{Parser, Subcommand};
use dirs;
use std::fs;
use std::io::prelude::*;
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

fn display_tree(home_dir: PathBuf, indent: usize) {
    let directory = fs::read_dir(home_dir).unwrap().filter_map(|file| file.ok());

    for entry in directory {
        let file_type = entry.file_type().unwrap();
        let mut line = String::new();

        for _ in 0..indent {
            line.push(' ');
            line.push(' ');
        }

        line.push_str(if file_type.is_dir() {
            "├───"
        } else {
            "└───"
        });
        line.push_str(entry.file_name().to_str().unwrap());

        println!("{}", line);

        if file_type.is_dir() {
            display_tree(entry.path(), indent + 2);
        }
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
        Commands::Modify { mut website } => {
            // TODO: Refactor this to a fn

            website.push_str(".txt");
            let mut path = PathBuf::new();
            path.push(home_folder);
            path.push(Into::<std::path::PathBuf>::into(&website));

            if fs::File::open(&path).is_ok() {
                let stdin = std::io::stdin();
                let mut new_password = String::new();
                println!("Please enter your new password");
                stdin
                    .read_line(&mut new_password)
                    .expect("No input was provided");

                if fs::write(path, new_password).is_ok() {
                    println!("Password sucessfully changed");
                } else {
                    eprintln!("There was an error while writing to file");
                }
            } else {
                println!("{:?}", website);
                eprintln!("Could not find the website");
            }
        }
        Commands::Delete { mut website } => {
            website.push_str(".txt");
            let mut path = PathBuf::new();
            path.push(home_folder);
            path.push(Into::<std::path::PathBuf>::into(&website));

            if fs::File::open(&path).is_ok() {
                if fs::remove_file(path).is_ok() {
                    println!("Password Deleted!");
                } else {
                    eprintln!("Could not delete file");
                }
            } else {
                eprintln!("Could not find the website");
            }
        }
        Commands::Get { mut website } => {
            website.push_str(".txt");
            let mut path = PathBuf::new();
            path.push(home_folder);
            path.push(Into::<std::path::PathBuf>::into(&website));

            if let Ok(mut f) = fs::File::open(path) {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("Could not read the file");
                println!("{}", contents);
            } else {
                eprintln!("Could not find the website");
            }
        }
        Commands::List {} => {
            display_tree(home_folder, 0);
        }
    }
}
