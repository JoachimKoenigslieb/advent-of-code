use std::{fs, collections::HashSet};

fn has_duplicate(vec: &Vec<char>) -> bool {
	let mut seen: HashSet<char> = HashSet::new();

	for ele in vec {
		if seen.contains(ele) {
			return true
		}

		seen.insert(*ele);
	}
	
	return false
}

fn main() {
	let contents = fs::read_to_string("./src/input").unwrap();

	let mut scanned: Vec<char> = Vec::new();
	let mut chars = contents.chars();

	let scan_size = 14;

	for _ in 0..scan_size {
		scanned.push(chars.next().unwrap());
	}

	for (ind, char) in chars.enumerate() {
		if !has_duplicate(&scanned) {
			println!("{}", ind + scan_size);
			break;
		} else {
			let (_first, rest) = scanned.split_first().unwrap();
			scanned = rest.into();
			scanned.push(char);
		}
	}
}
