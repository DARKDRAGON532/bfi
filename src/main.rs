use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid arguments.");
        process::exit(1);
    }

    let contents = bfi::read_file(args[1].clone()).unwrap_or_else(|err| {
        eprintln!("Application error : {}", err);
        process::exit(1);
    });

    bfi::interpret(contents);

}
