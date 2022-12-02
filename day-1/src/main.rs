use std::fs;

fn main() {
	let contents = fs::read_to_string("./src/input").unwrap();
	let groups = contents.split("\n\n");

	let mut sums: Vec<i32> = Vec::new();

	for group in groups {
		let mut s = 0;

		for line in group.lines() {
			let num = line.parse::<i32>();
			
			match num {
				Ok(res) => s = s + res,
				Err(e) => panic!("{}", e),
			}
		}

		sums.push(s);
	}

	let max = sums.iter().max().unwrap();

	print!("max ${}\n", max);

	{
		sums.sort();
		let max_three: i32 = sums.iter().rev().take(3).sum();
	
		print!("max three: {}\n", max_three)
	}

}
