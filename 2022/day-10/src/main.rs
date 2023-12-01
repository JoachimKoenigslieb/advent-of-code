use std::{fs, collections::HashSet};

fn main() {
	let content = fs::read_to_string("./src/input").unwrap();
	let mut stack: Vec<i32> = Vec::new();
	stack.push(1); // start of first cycle...

	for line in content.lines() {
		let mut splitted = line.split_whitespace();	
		let instruction = splitted.next().unwrap();

		match instruction {
			"noop" => {
				stack.push(*stack.get(stack.len() - 1).unwrap()); 
			},
			"addx" => {
				let value: i32 = splitted.next().unwrap().parse().unwrap();
				let stack_value = stack.get(stack.len() - 1).unwrap().to_owned();

				stack.push(stack_value); 
				stack.push(stack_value + value); 
			},
			err => panic!("got {err} not sure wat do"),
		}
	}

	let cyc_20 = stack.get(19).unwrap();
	let cyc_60 = stack.get(59).unwrap();
	let cyc_100 = stack.get(99).unwrap();
	let cyc_140 = stack.get(139).unwrap();
	let cyc_180 = stack.get(179).unwrap();
	let cyc_220 = stack.get(219).unwrap();

	println!("20: {} 60: {} 100: {} 140: {} 180: {} 220: {}", cyc_20, cyc_60, cyc_100, cyc_140, cyc_180, cyc_220);
	println!("{}", cyc_20 * 20 + cyc_60 * 60 + cyc_100 * 100 + cyc_140 * 140 + cyc_180 * 180 + cyc_220 * 220);

	for (cycle, pos) in stack.into_iter().enumerate() {
		let pixel: i32 = (cycle % 40).try_into().unwrap();
		
		match pos - pixel {
			-1 => print!("#"),
			1 => print!("#"),
			0 => print!("#"),
			_ => print!("."),
		}

		if pixel == 39 {
			print!("\n");
		}
	}
}
