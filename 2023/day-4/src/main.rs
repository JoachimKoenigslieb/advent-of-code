use std::{fs, cmp};

fn main() {
	let input = fs::read_to_string("./src/input").unwrap();

	let mut points = 0;
	let num_lines = input.lines().count();

	for line in input.lines() {
		let (_, card_info) = line.split_once(": ").unwrap();
		let (winners, yours) = card_info.split_once(" | ").unwrap();

		let winning_nums: Vec<u32> = winners.split(' ').filter_map(|num| num.parse::<u32>().ok()).collect();
		let your_nums: Vec<u32> = yours.split(' ').filter_map(|num| num.parse::<u32>().ok()).collect();

		let mut num_winners = 0;

		for num in your_nums {
			if winning_nums.contains(&num) {
				num_winners += 1;
			}
		}

		if num_winners > 0 {
			points += 2_u32.pow(num_winners - 1)
		}
	}

	let mut num_cards: Vec<u32> = vec![1; num_lines];

	for (i, line) in input.lines().enumerate() {
		let (_, card_info) = line.split_once(": ").unwrap();
		let (winners, yours) = card_info.split_once(" | ").unwrap();

		let winning_nums: Vec<u32> = winners.split(' ').filter_map(|num| num.parse::<u32>().ok()).collect();
		let your_nums: Vec<u32> = yours.split(' ').filter_map(|num| num.parse::<u32>().ok()).collect();

		let mut num_winners = 0;

		for num in your_nums {
			if winning_nums.contains(&num) {
				num_winners += 1;
			}
		}

		for index in (i + 1)..(i + num_winners + 1) {
			num_cards[index] += 1 * num_cards[i];
		}
	}

	let total_cards = num_cards.into_iter().sum::<u32>();

	println!("points: {}", points);
	println!("total cards: {}", total_cards)
}
