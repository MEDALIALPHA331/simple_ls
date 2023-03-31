use std::fs;
fn main() {
    //? add better error handling
    let files: Vec<_> = fs::read_dir("")
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();



    println!("files {:?}", files );
}
