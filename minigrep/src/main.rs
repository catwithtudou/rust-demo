use std::env;
use std::fs;

// $ cargo run searchstring example-filename.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    //    println!("{:?}", args)

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let file_ontent = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", file_ontent);
}
