use std::fs;

fn main() {
	let contents = fs::read_to_string("./src/test_input").unwrap();
	let mut score = 0;

	for line in contents.lines() {
		let mut split = line.split_whitespace();

		let opp = split.next().unwrap();
		let result = split.next().unwrap();

		// X is lose, Y is draw Z is win
		match (opp, result) {
			("A", "X") => score = score + 0 + 3, // lose: choose scissor
			("A", "Y") => score = score + 3 + 1, // draw: choose rock
			("A", "Z") => score = score + 6 + 2, // win: choose paper

			("B", "X") => score = score + 0 + 1, // lose: choose rock
			("B", "Y") => score = score + 3 + 2, // draw: choose paper
			("B", "Z") => score = score + 6 + 3, // win: choose scissor

			("C", "X") => score = score + 0 + 2, // lose: choose paper
			("C", "Y") => score = score + 3 + 3, // draw: choose scissor 
			("C", "Z") => score = score + 6 + 1, // win: choose rock
			(_m, _o) => print!("panic!"),
		}
	}

	print!("{}\n", score);
}
