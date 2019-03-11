mod scanner;
use scanner::Scanner;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);

    let scanner = match Scanner::new(&args[1]) {
        Ok(scanner) => scanner,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };

    for x in scanner {
        println!("{}", x);
    }
}
