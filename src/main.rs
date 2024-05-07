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

/// This function is used to provide a path to store the passwords as a file
fn create_password_file(name: &mut String, home_folder: PathBuf) -> PathBuf {
    name.push_str(".txt");

    let mut path = PathBuf::new();
    path.push(home_folder);
    path.push(Into::<std::path::PathBuf>::into(&name));

    return path;
}

/// This function displays a tree like structure for all the stored passwords
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

/// This function is used to read user input form the command line
fn read_input(message: &str) -> String {
    let mut op = String::new();
    println!("{message}");
    std::io::stdin()
        .read_line(&mut op)
        .expect("no user input provided");

    return op;
}

fn main() {
    let mut home_folder = dirs::home_dir().unwrap();
    home_folder.push(".passwords");
    let args = PasswordManager::parse();

    match args.command {
        Commands::Init {} => {
            create_dir(home_folder);
        }
        Commands::Add { mut website } => {
            let path = create_password_file(&mut website, home_folder);

            let f = fs::create_dir_all(&path.parent().unwrap());
            if f.is_ok() {
                println!("File created");

                let password = read_input("Please input your password");

                let password_retype = read_input("Please confirm password");

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
            let path = create_password_file(&mut website, home_folder);

            if fs::File::open(&path).is_ok() {
                let new_password = read_input("Please enter your new password");

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
            let path = create_password_file(&mut website, home_folder);

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
            let path = create_password_file(&mut website, home_folder);

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
