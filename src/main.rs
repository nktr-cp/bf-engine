mod interpreter;

use std::env;

use crate::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    if args.len() == 1 {
        filename = "test/hello.bf";
    } else if args.len() == 2 {
        filename = &args[1];
    } else {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mut interpreter = Interpreter::new(filename);
    interpreter.run();
}
