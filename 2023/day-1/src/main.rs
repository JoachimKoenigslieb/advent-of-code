use std::{fs, i32};

fn main() {
	let input = fs::read_to_string("./src/input").unwrap();
	let lines = input.lines();
	let mut sum: u32 = 0;

	for line in lines {
		let mut numbers: Vec<u32> = Vec::new();

		let line_replaced = line
			.replace("oneight", "1")
			.replace("twone", "2")
			.replace("threeight", "3")
			.replace("fiveight", "5")
			.replace("sevenine", "7")
			.replace("nineight", "9")
			.replace("eightwo", "8")
			.replace("eighthree", "8")

			.replace("one", "1")
			.replace("two", "2")
			.replace("three", "3")
			.replace("four", "4")
			.replace("five", "5")
			.replace("six", "6")
			.replace("seven", "7")
			.replace("eight", "8")
			.replace("nine", "9")
			;

		print!("old: {} new: {}\n", line, line_replaced);

		for char in line_replaced.chars() {
			if char.is_ascii_digit() {
				numbers.push(char.to_digit(10).unwrap());
			}
		}

		let first_last_concat = numbers.first().unwrap() * 10 + numbers.last().unwrap();

		print!("found digit: {}\n", first_last_concat);

		sum += first_last_concat;
	}

	print!("sum is: {}", sum)
}