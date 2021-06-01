use std::env;
use std::fs;

struct Chars {
	wall:	char,
	empty:	char,
	path:	char,
	start:	char,
	end:	char,
}

struct Maze {
	height: i32,
	width:	i32,
	maze:	String,
	chars:	Chars,
}

fn parse(filename: &String) {
	let contents = fs::read_to_string(filename)
		.unwrap_or_else(|err| {
			println!("Unable to read file: {}", err);
			String::from("error\n")
		});
	println!("contents in file:\n{}", contents);

	let mut first_line = contents.lines().next();
	println!("first line: {:?}", first_line);


	for line in contents.lines() {
		println!("{}\n", line);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("args: {:?}", args);

	if args.len() > 1 {
		for i in 1..args.len() {
			parse(&args[i]);
		}
	}
}
