use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.get(1).unwrap_or(&String::new()) == "--server" {

    } else {

    }

    println!("Hello, world!");
}

