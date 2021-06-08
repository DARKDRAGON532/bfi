use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid arguments.");
        process::exit(1);
    }

    let contents = bfi::read_file(args[1].clone()).expect("Error reading file.");

    bfi::interpret(contents);

}
