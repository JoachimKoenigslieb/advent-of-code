use std::fs;

fn overlaps_completely(line: &str) -> i32 {
	let mut pair_split = line.split(',');

	let first = pair_split.next().unwrap();
	let second = pair_split.next().unwrap();

	let mut first_sections_split = first.split('-');
	let mut second_sections_split = second.split('-');

	let first_low = first_sections_split.next().unwrap().parse::<i32>().unwrap();
	let first_high = first_sections_split.next().unwrap().parse::<i32>().unwrap();
	
	let second_low = second_sections_split.next().unwrap().parse::<i32>().unwrap();
	let second_high = second_sections_split.next().unwrap().parse::<i32>().unwrap();

	if first_low <= second_low && first_high >= second_high {
		// first completely overlaps second
		
		return 1
	}

	if first_low >= second_low && first_high <= second_high {
		return 1
	}

	return 0
}

fn overlaps_at_all(line: &str) -> i32 {
	let mut pair_split = line.split(',');

	let first = pair_split.next().unwrap();
	let second = pair_split.next().unwrap();

	let mut first_sections_split = first.split('-');
	let mut second_sections_split = second.split('-');

	let first_low = first_sections_split.next().unwrap().parse::<i32>().unwrap();
	let first_high = first_sections_split.next().unwrap().parse::<i32>().unwrap();
	
	let second_low = second_sections_split.next().unwrap().parse::<i32>().unwrap();
	let second_high = second_sections_split.next().unwrap().parse::<i32>().unwrap();

	if first_low >= second_low && first_low <= second_high {
		return 1
	}

	if second_low >= first_low && second_low <= first_high {
		return 1
	}

	if first_high >= second_low && first_high <= second_high {
		return 1
	}

	if second_high >= first_low && second_high <= first_high {
		return 1
	}

	return 0
}

fn main() {
	let contents = fs::read_to_string("./src/input").unwrap();
	let mut sum = 0;

	for line in contents.lines() {
		sum += overlaps_at_all(line);
	}

	println!("{}", sum);
}
