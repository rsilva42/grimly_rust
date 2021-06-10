use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
	maze:	Vec<String>,
	chars:	Chars,
}

fn build_maze(first_line: String, maze: Vec<String>) -> Maze {
	let map = Maze {
		height: 0,
		width: 0,
		maze: vec!["unsolved".to_string(), "maze".to_string()],
		chars: Chars {
			wall: 'a',
			empty: 's',
			path: 'd',
			start: 'f',
			end: 'g',
		},
	};
	map
}

fn parse(filename: &String) -> Maze {
	let mut file = BufReader::new(File::open(filename).unwrap());

	let mut first_line = String::new();
	file.read_line(&mut first_line).unwrap();

	println!("first line: {:?}", first_line);

	let maze: Vec<String> = file.lines()
		.map(|line| line.unwrap()).collect();
	println!("lines:\n{:?}", maze);

	let map: Maze = build_maze(first_line, maze);
	map
}

fn solve(map: Maze) -> Vec<String> {
	println!("solve the maze");
	vec!["solved".to_string(), "maze".to_string()]
}

fn print_map(solution: Vec<String>) {
	println!("printing solution:");
	for line in solution {
		println!("{}", line);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("args: {:?}", args);

	if args.len() > 1 {
		for i in 1..args.len() {
			let map = parse(&args[i]);
			let solution = solve(map);
			print_map(solution);
		}
	}
	else {
		println!("usage: ./grimly file.map [file2.map ...]");
	}
}
