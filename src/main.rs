use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, Subcommand};
use dirs;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

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

    return op.trim_end().to_string();
}

fn encrypt_password<'a>(key: &'a str, plaintext: &'a str) -> Vec<u8> {
    let iv = [0u8; 16];

    let buf_len = (plaintext.len() + 16) - (plaintext.len() % 16);

    let mut buf = vec![0; buf_len];
    let pt_len = plaintext.len();
    buf[..pt_len].copy_from_slice(plaintext.as_bytes());
    let ct = Aes128CbcEnc::new(key.as_bytes().into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();

    return ct.to_owned();
}
fn decrypt_password(mut buf: Vec<u8>, key: &str) -> String {
    let iv = [0u8; 16];

    let pt = Aes128CbcDec::new(key.as_bytes().into(), &iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .unwrap();
    let b = String::from_utf8(pt.to_vec()).unwrap();
    return b;
}

fn main() {
    let mut home_folder = dirs::home_dir().unwrap();
    home_folder.push(".passwords");
    let mut passphrase_path = home_folder.clone();
    passphrase_path.push("passphrase.txt");
    let args = PasswordManager::parse();

    match args.command {
        Commands::Init {} => {
            create_dir(home_folder.clone());
            let passphrase = read_input("Enter a passphrase key");
            fs::write(passphrase_path, passphrase)
                .expect("Could not save passphrase please check permissions");
            println!("Passphrase saved successfully");
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

                    let mut passphrase_file =
                        fs::File::open(passphrase_path).expect("Could not find passphrase path");
                    let mut passphrase = String::new();
                    passphrase_file
                        .read_to_string(&mut passphrase)
                        .expect("Could not read file, please check permissions");

                    let encrypted = encrypt_password(&passphrase[..16], &password);

                    let b64e = general_purpose::STANDARD.encode(encrypted.clone());

                    fs::write(path, b64e).expect("could not write to file");
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

            if let Ok(mut p) = fs::File::open(passphrase_path) {
                let mut passphrase = String::new();
                p.read_to_string(&mut passphrase)
                    .expect("Could not read passphrase");
                if let Ok(mut f) = fs::File::open(path) {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("Could not read the file");

                    let b64d = general_purpose::STANDARD
                        .decode(contents)
                        .expect("Could not convert b64 to string");
                    let password = decrypt_password(b64d, &passphrase[..16]);
                    println!("{:?}", password);
                } else {
                    eprintln!("Could not find the website");
                }
            } else {
                eprintln!("Could not find passphrase, please make sure to set one");
            }
        }
        Commands::List {} => {
            display_tree(home_folder, 0);
        }
    }
}
