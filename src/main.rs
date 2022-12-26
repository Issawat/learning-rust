use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let executable_path = &args[0];
    let query_string = &args[1];
    let file_path = &args[2];

    dbg!(executable_path);

    println!("Searching for {query_string}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("with text content:\n {contents}");
}
