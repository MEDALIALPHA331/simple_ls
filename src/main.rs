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
    let mut hidden = false;
    let mut sorted = false;




    // ? capture Arguments Safely
    let arguments: Vec<_> = env::args().into_iter().collect();

    match arguments.get(1) {
        Some(x) => match x.as_str() {
            "all" | "-l" => {
                hidden = true;
                //todo: add other filters
            },
            "-a" => hidden = true,
            "-s" => sorted = true,
            "-h" | "help" => todo!("Implement Help paragraph"), 
            _ => panic!("Argument '{x}' does not exist!"),
        },
        None => (),
    }

    if let Some(second_argument) = arguments.get(2) {
        panic!("There is no second Parameter, remove {second_argument}");
    }

    //todo: add better error handling
    let mut entries: Vec<_> = fs::read_dir("")
        .unwrap()
        .map(|x| x.unwrap().path())
        .filter(|entry| {
              if hidden {
                  is_hidden(entry).unwrap() //? windows specific
                  || entry.starts_with(".")
              } else {
                  true
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
