use std::error::Error;
use std::fs;
use std::process;

use rlp::rlp_transform;

fn main() {
    let filename = "data.txt".to_string();

    if let Err(e) = run(&filename) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(filename: &String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    // println!("With text:\n{}", contents);

    println!("Output:");

    let lines = contents.split("\n").collect::<Vec<&str>>();

    for line in lines {
        // println!("{}", line);
        println!("{}", rlp_transform(line));
    }
    Ok(())
}
