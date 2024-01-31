use std::{fs, cmp};

#[derive(Clone)]
struct RGB {
	red: i32,
	green: i32, 
	blue: i32,
}

fn main() {
	let input = fs::read_to_string("./src/input").unwrap();

	let mut sum = 0;
	let mut power_sum = 0;

	for line in input.lines() {
		let (game_id, game_info) = line.split_once(": ").unwrap();

		let (_, game_id_num) = game_id.split_once(' ').unwrap();

		let id = game_id_num.parse::<i32>().unwrap();

		let ( red_max, green_max, blue_max, ) = (12, 13, 14);

		let results = game_info.split("; ").map(|game| {
			let ( mut red, mut green, mut blue, ) = (0, 0, 0);
	
			for cube in game.split(", ") {
				match cube {
					cube if cube.ends_with(" red") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						red = number.parse::<i32>().unwrap();
					}
					cube if cube.ends_with(" blue") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						blue = number.parse::<i32>().unwrap();
					}
					cube if cube.ends_with(" green") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						green = number.parse::<i32>().unwrap();
					}
					_ => panic!("game not handled")
				}
			}
	
			if red <= red_max && green <= green_max && blue <= blue_max {
				// print!("\tdraw with red {} green {} blue {} is possible!\n", red, green, blue);
				true
			} else {
				// print!("\tdraw with red {} green {} blue {} is NOT possible!\n", red, green, blue);
				false
			}
		}).collect::<Vec<bool>>();


		let rgb: Vec<RGB> = game_info.split("; ").map(|game| {
			let ( mut red, mut green, mut blue, ) = (0, 0, 0);
	
			for cube in game.split(", ") {
				match cube {
					cube if cube.ends_with(" red") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						red = number.parse::<i32>().unwrap();
					}
					cube if cube.ends_with(" blue") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						blue = number.parse::<i32>().unwrap();
					}
					cube if cube.ends_with(" green") => {
						let ( number, _) = cube.split_once(' ').unwrap();
						green = number.parse::<i32>().unwrap();
					}
					_ => panic!("game not handled")
				}
			}
			
			RGB { red, green, blue, }
		}).collect();

		let minimal_dice = rgb.into_iter().reduce(|left, right| RGB { 
			red: cmp::max(left.red, right.red), 
			green: cmp::max(left.green, right.green), 
			blue: cmp::max(left.blue, right.blue), 
		}).unwrap();

		let power = minimal_dice.blue * minimal_dice.red * minimal_dice.green;

		let legal_game = results.iter().all(|result| *result);

		if legal_game {
			sum += id;
		}

		power_sum += power;
		// print!("game with id {} is legal? {}\n", id, legal_game);
	}

	print!("total sum of legal games: {}\nTotal power is {}\n", sum, power_sum);
}
