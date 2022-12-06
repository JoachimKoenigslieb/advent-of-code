use std::{fs, iter::Enumerate, str::from_utf8};

fn parse_stacks(stacks_str: &str) -> Vec<Vec<char>>{
	let mut stacks: Vec<Vec<char>> = Vec::new();
	let mut rev_lines = stacks_str.lines().rev();

	let stack_counts = rev_lines.next().unwrap().split_whitespace().count();

	for _ in 0..stack_counts {
		let new_vec: Vec<char> = Vec::new();
		stacks.push(new_vec);
	}

	for line in rev_lines {	
		let chunks: Vec<&str> = line
			.as_bytes()
			.chunks(4)
			.map(|chunk| from_utf8(chunk).unwrap())
			.collect();
			
		for (index, crates) in chunks.into_iter().enumerate() {
			let mut chars = crates.chars();

			chars.next(); // throw away the first bracket :)))
			let crate_content = chars.next().unwrap();

			if crate_content != ' ' {
				stacks.get_mut(index).unwrap().push(crate_content);
			}
		}
	}

	stacks
}

#[derive(Debug)]
struct Move {
	amount: usize,
	from: usize,
	to: usize,
}

fn parse_procedure(procedure_str: &str) -> Vec<Move> {
	let moves: Vec<Move> = procedure_str.lines().map(|line| {
		let mut split_line = line.split_whitespace();
		split_line.next();
		let amount = split_line.next().unwrap().parse().unwrap();
		split_line.next();
		let from = split_line.next().unwrap().parse().unwrap();
		split_line.next();
		let to = split_line.next().unwrap().parse().unwrap();

		Move { amount, from: from, to }
	}).collect();

	moves
}

fn part1() {
	let contents = fs::read_to_string("./src/input").unwrap();

	let mut splitted = contents.split("\n\n");
	
	let stacks_str = splitted.next().unwrap();
	let procedure_str = splitted.next().unwrap();

	let mut stacks = parse_stacks(stacks_str);
	let procedure = parse_procedure(procedure_str);

	for step in procedure {
		let Move { to, from, amount, } = step;

		for _ in 0..amount {
			let moved_item = stacks.get_mut(from - 1).unwrap().pop(); // sub 1 to get indexing right

			match moved_item {
				None => (), // i guess we dont do anything?,
				Some(c) => stacks.get_mut(to - 1).unwrap().push(c),
			}
		}
	}

	for vec in stacks {
		let last = vec.last().unwrap();
		print!("{}", last);
	}

	println!("");
}

fn part2() {
	let contents = fs::read_to_string("./src/test_input").unwrap();

	let mut splitted = contents.split("\n\n");
	
	let stacks_str = splitted.next().unwrap();
	let procedure_str = splitted.next().unwrap();

	let mut stacks = parse_stacks(stacks_str);
	let procedure = parse_procedure(procedure_str);

	for step in procedure {
		let Move { to, from, amount, } = step;
		let mut moved_items_holder: Vec<char> = Vec::new();

		for _ in 0..amount {
			let moved_item = stacks.get_mut(from - 1).unwrap().pop(); // sub 1 to get indexing right

			match moved_item {
				None => (),
				Some(c) => moved_items_holder.push(c),
			}
		}

		for ele in moved_items_holder.into_iter().rev() {
			stacks.get_mut(to - 1).unwrap().push(ele);
		}
	}

	for vec in stacks {
		let last = vec.last().unwrap();
		print!("{}", last);
	}

	println!("");
}

fn main() {
	part2()	
}
