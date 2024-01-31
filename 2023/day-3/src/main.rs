use std::{fs, cmp};

#[derive(Debug)]
struct Num {
	value: u32,
	start: (usize, usize),
	end: (usize, usize),
}

fn resolve_matched_digits(is_matching_digits: &mut bool, matched_digit: &mut Vec<u32>, numbers: &mut Vec<Num>, i: &usize, j: &usize) {
	if *is_matching_digits {
		let len = matched_digit.len();
		let value: u32 = matched_digit.iter().rev().enumerate().fold(0, |left, (index, digit)| {
			// quite shady: we reduce the numbers into their digit. We need to account for powers of ten
			10_u32.pow(index as u32) * digit + left
		});

		matched_digit.clear();

		numbers.push(Num { value, start: (*i, j - len), end: (*i, j - 1), });
		*is_matching_digits = false;
	}
}

fn main() {
	let input = fs::read_to_string("./src/input").unwrap();

	let mut numbers: Vec<Num> = Vec::new();
	let mut symbols: Vec<( usize, usize, char )> = Vec::new();

	let mut touches_symbol_sum = 0;
	let mut gear_ratio_sum = 0;

	for (i, line) in input.lines().enumerate() {
		let mut is_matching_digits = false;
		let mut matched_digit: Vec<u32> = Vec::new();
		let line_len = line.len();

		for (j, char) in line.chars().enumerate() {
			match char {
				'.' => {
					resolve_matched_digits(&mut is_matching_digits, &mut matched_digit, &mut numbers, &i, &j);
					
					print!(".")
				},
				char if char.is_digit(10) => {
					is_matching_digits = true;
					matched_digit.push(char.to_digit(10).unwrap());
					print!("{}", char)
				}, 
				symbol => {
					resolve_matched_digits(&mut is_matching_digits, &mut matched_digit, &mut numbers, &i, &j);

					symbols.push((i, j, symbol));
				
					print!("{}", symbol)
				},
			}
		}

		// also need to resolve a matched digit after end of line!
		resolve_matched_digits(&mut is_matching_digits, &mut matched_digit, &mut numbers, &i, &line_len);

		print!("\n");
	}
	
	for (i, j, symbol) in symbols.as_slice() {
		let mut touches_number: Vec<&Num> = Vec::new();
 
		for number in numbers.as_slice() {
			let Num { start, end, .. } = number;

			let larger_than_j: usize = match start.1.checked_sub(1) {
				Some(value) => value,
				None => 0,
			};
			
			let smaller_than_j = end.1 + 1;
			
			let larger_than_i = match start.0.checked_sub(1) {
				Some(value) => value,
				None => 0,
			};
		
			let smaller_than_i = start.0 + 1;

			if *j >= larger_than_j && *j <= smaller_than_j && *i >= larger_than_i && *i <= smaller_than_i {
				touches_number.push(number);
			}
		}

		if (*symbol == '*' && touches_number.len() == 2) {
			let ratio = touches_number[0].value * touches_number[1].value;
			gear_ratio_sum += ratio;
		}
	}

	for number in numbers {
		let Num { value, start, end } = number;
		// handle underflow
		let larger_than_j: usize = match start.1.checked_sub(1) {
			Some(value) => value,
			None => 0,
		};
		
		let smaller_than_j = end.1 + 1;
		
		let larger_than_i = match start.0.checked_sub(1) {
			Some(value) => value,
			None => 0,
		};
	
		let smaller_than_i = start.0 + 1;

		let mut touches_symbol = false;

		for (i, j, _) in symbols.as_slice() {
			if (*j >= larger_than_j && *j <= smaller_than_j && *i >= larger_than_i && *i <= smaller_than_i) {
				touches_symbol = true;
				continue;
			}
		}

		if touches_symbol {
			touches_symbol_sum += value;
			// println!("number overlaps {}", value);
		} else {
			// println!("number has no overlaps {}", value);
		}
	}

	println!("touches symbol sum: {}", touches_symbol_sum);
	println!("gear ratio sum: {}", gear_ratio_sum);
}
