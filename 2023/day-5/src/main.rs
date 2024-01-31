use std::fs;

fn main() {
	let input = fs::read_to_string("./src/input").unwrap();

	let mut rough_split = input.split("\n\n");

	let (_, seeds_str) = rough_split.next().unwrap().split_once(": ").unwrap();

	let seeds: Vec<u32> = seeds_str.split(' ').filter_map(|s| s.parse::<u32>().ok()).collect();

	println!("seeds is: {:?}", seeds);

	for stuff in rough_split {
		let mut map_lines = stuff.lines();
		map_lines.next();

		// this is dumb
		let map_ranges: Vec<Vec<u32>> = map_lines.map(|map_line| {
			let map: Vec<u32> = map_line
				.split(' ')
				.filter_map(|s| s.parse::<u32>().ok())
				.collect();

			return map
		}).collect();
	

		
		println!("{}", stuff);
	}
}
