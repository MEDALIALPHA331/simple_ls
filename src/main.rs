use std::{env, fs};
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
        .map(|x| x.unwrap().path().to_string_lossy().to_string())
        .filter(|entry| {
              if hidden {
                  entry.starts_with(".")
              } else {
                  true
              }
        })
        .collect();

    if sorted {
         entries.sort()
    }

    for entry in entries {
         println!("{}", entry);
     }
}
