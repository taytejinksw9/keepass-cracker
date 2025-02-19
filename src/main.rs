use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use rpassword::read_password;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct KeePassEntry {
    title: String,
    username: String,
    password: String,
}

fn main() {
    let matches = App::new("KeePass Cracker")
        .version("1.0")
        .author("taytejinksw9")
        .about("Cracks KeePass safes")
        .arg(Arg::new("file")
            .about("Path to the KeePass file")
            .required(true)
            .index(1))
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let password = read_password().unwrap();
    let entries = crack_keepass(file_path, &password);
    display_entries(entries);
}

fn crack_keepass(file_path: &str, password: &str) -> Vec<KeePassEntry> {
    let path = Path::new(file_path);
    if !path.exists() {
        panic!("File does not exist");
    }

    let mut file = File::open(path).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read file");

    let decrypted_data = decrypt_data(&data, password);
    parse_entries(&decrypted_data)
}

fn decrypt_data(data: &[u8], password: &str) -> Vec<u8> {
    let key = generate_key(password);
    let cipher = aes::Aes256::new(&key);
    let mut buffer = data.to_vec();
    let mode = block_modes::Cbc::new(cipher, Default::default());
    mode.decrypt(&mut buffer).expect("Decryption failed");
    buffer
}

fn generate_key(password: &str) -> Vec<u8> {
    let mut key = vec![0u8; 32];
    key.copy_from_slice(&password.as_bytes()[..32]);
    key
}

fn parse_entries(data: &[u8]) -> Vec<KeePassEntry> {
    let entries: Vec<KeePassEntry> = serde_json::from_slice(data).expect("Failed to parse entries");
    entries
}

fn display_entries(entries: Vec<KeePassEntry>) {
    for entry in entries {
        println!("Title: {}", entry.title);
        println!("Username: {}", entry.username);
        println!("Password: {}", entry.password);
        println!("-------------------");
    }
}