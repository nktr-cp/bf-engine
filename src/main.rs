mod interpreter;

use clap::Parser;
use crate::interpreter::Interpreter;

#[derive(Parser)]
struct Cli {
	filename: Option<String>,

	#[arg(long)]
	gen: bool,
}

fn main() {
    let args = Cli::parse();

		let filename = args.filename.as_deref().unwrap_or("test/hello.bf");

		let mut interpreter = Interpreter::new(filename);

		if args.gen {
			println!("{}", interpreter.gen());
		} else {
			interpreter.run();
		}
}
