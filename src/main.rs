use std::{env, fs};

#[cfg(windows)]
use std::os::windows::prelude::*;

pub fn is_hidden(file_path: &std::path::PathBuf) -> std::io::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();

    if (attributes & 0x2) > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn main() {
    //? prepare flags
    let mut hidden_only = false;
    let mut all = false;
    let mut sorted = false;

    //? capture Arguments Safely
    let arguments: Vec<_> = env::args().into_iter().collect();

    match arguments.get(1) {
        Some(x) => match x.as_str() {
            "-l" | "all" => all = true,
            "-a" | "hidden" => hidden_only = true,
            "-s" | "sorted" => sorted = true,
            "-h" | "help" => todo!("Implement Help paragraph"),
            //todo: add multiple arguments capability
            _ => panic!("Argument '{x}' does not exist!"),
        },
        None => (),
    }

    if let Some(second_argument) = arguments.get(2) {
        panic!("There is no second Parameter, remove {second_argument}");
    }

    //? create list and print it
    //todo: add better error handling
    let mut entries: Vec<_> = fs::read_dir("")
        .unwrap()
        .map(|x| x.unwrap().path())
        .filter(|entry| {
            if hidden_only {
                is_hidden(entry).unwrap() || entry.starts_with(".")
            } else if all {
                true
            } else {
                !is_hidden(entry).unwrap()
            }
        })
        .map(|buff| buff.to_string_lossy().to_string())
        .collect();

    if sorted {
        entries.sort()
    }

    for entry in entries {
        println!("{}", entry);
    }
}
