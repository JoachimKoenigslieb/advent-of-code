use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;


fn split_half(content: &str) -> (&str, &str) {
	let length = content.len();
	let half_length = length / 2;
	
	let (first, second, ) = content.split_at(half_length);

	(first, second)
}

fn part1() {
	let contents = fs::read_to_string("./src/input").unwrap();
	let mut score_map: HashMap<char, usize> = HashMap::new();
	let mut score: usize = 0;

	for (value, char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
		score_map.insert(char, value + 1);
	}

	for line in contents.lines() {
		let (first, second) = split_half(line); 

		let mut first_set: HashSet<char> = HashSet::new();
		let mut second_set: HashSet<char> = HashSet::new();

		for c in first.chars() {
			first_set.insert(c);
		}

		for c in second.chars() {
			second_set.insert(c);
		}

		let mut intersection = first_set.intersection(&second_set);

		let char = intersection.next().unwrap();
		score += score_map.get(char).unwrap();
	}

	println!("{}", score);
}

fn part2() {
	let contents = fs::read_to_string("./src/input").unwrap();
	let mut score_map: HashMap<char, usize> = HashMap::new();
	let mut score: usize = 0;

	for (value, char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
		score_map.insert(char, value + 1);
	}

	let mut lines = contents.lines();

	loop {
		let mut first_set: HashSet<char> = HashSet::new();
		let mut second_set: HashSet<char> = HashSet::new();
		let mut third_set: HashSet<char> = HashSet::new();

		let first = lines.next();
		let second = lines.next();
		let third = lines.next();

		if first.is_none() {
			break
		}

		for c in first.unwrap().chars() {
			first_set.insert(c);
		}
		for c in second.unwrap().chars() {	
			second_set.insert(c);
		}
		for c in third.unwrap().chars() {
			third_set.insert(c);
		}

		let mut intersection = first_set.intersection(&second_set);
		let mut intersecting_char: char;

		for ele in intersection {
			if third_set.contains(ele) {
				score += score_map.get(ele).unwrap();
			}
		}
	}

	println!("{}", score)
}

fn main() {
	part2();
}
