use alfred::{json, Item, ItemBuilder};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn read_file(file_path: PathBuf) -> Result<String, io::Error> {
    // we need do use a Vec<u8> since ~/.zsh_history
    // might not contain valid UTF-8 data
    let mut content = Vec::new();
    let mut file = File::open(&file_path)?;
    file.read_to_end(&mut content)?;
    Ok(String::from_utf8_lossy(&content).to_string())
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let pattern = args.get(0).unwrap();
    let zsh_history_path = Path::new(&dirs::home_dir().unwrap()).join(".zsh_history");
    let file_content = read_file(zsh_history_path).expect("Unable to read file");

    let mut matched_lines = file_content
        .lines()
        .filter(|line| line.contains(pattern))
        .map(|line| line[15..].to_string())
        .rev() // we want the most recent entry first
        .collect::<Vec<String>>();

    matched_lines.dedup();

    let matched_items = matched_lines
        .iter()
        .map(|line| ItemBuilder::new(line).into_item())
        .collect::<Vec<Item>>();

    json::write_items(io::stdout(), &matched_items).expect("cannot write to Alfred");
}
