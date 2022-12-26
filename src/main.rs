use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executable_path = &args[0];
    let query_string = &args[1];
    let file_path = &args[2];

    dbg!(executable_path);

    println!("Searching for {query_string}");
    println!("In file {file_path}");
}
