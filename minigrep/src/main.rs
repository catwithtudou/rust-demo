use std::env;

// $ cargo run searchstring example-filename.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    //    println!("{:?}", args)

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
