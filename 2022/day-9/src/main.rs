use std::{fs, collections::HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
	x: i32,
	y: i32,
}

fn position_difference(head_position: &Position, tail_position: &Position) -> Position {
	Position { x: head_position.x - tail_position.x, y: head_position.y - tail_position.y, }
}

fn move_head(head_position: &mut Position, direction: &str) {
	match direction {
		"R" => head_position.x += 1,
		"L" => head_position.x -= 1,
		"U" => head_position.y += 1,
		"D" => head_position.y -= 1,
		err => panic!("Something read {err} we cant handle :)"),
	};
}

fn move_tail(head_position: &Position, tail_position: &mut Position) {
	let difference = position_difference(&head_position, tail_position);
	let squared_distance = difference.x.pow(2) + difference.y.pow(2); 
	// if squared distance is 1, we dont do anything, because tail and head are dirrectly adjacent each other
	// if squared distance is 2, they are diagonal to each other 

	if squared_distance == 1 || squared_distance == 2 || squared_distance == 0 {
		return;
	}

	match difference {
		Position { x: 0, y: 2 } => tail_position.y += 1,
		Position { x: 0, y: -2 } => tail_position.y -= 1,
		Position { x: 2, y: 0 } => tail_position.x += 1,
		Position { x: -2, y: 0 } => tail_position.x -= 1,
		
		Position { x: -1, y: 2 } => {tail_position.x -= 1; tail_position.y += 1},
		Position { x: -2, y: 1 } => {tail_position.x -= 1; tail_position.y += 1},
		Position { x: -2, y: 2 } => {tail_position.x -= 1; tail_position.y += 1},
		
		Position { x: -1, y: -2 } => {tail_position.x -= 1; tail_position.y -= 1},
		Position { x: -2, y: -1 } => {tail_position.x -= 1; tail_position.y -= 1},
		Position { x: -2, y: -2 } => {tail_position.x -= 1; tail_position.y -= 1},

		Position { x: 1, y: 2 } => {tail_position.x += 1; tail_position.y += 1},
		Position { x: 2, y: 1 } => {tail_position.x += 1; tail_position.y += 1},
		Position { x: 2, y: 2 } => {tail_position.x += 1; tail_position.y += 1},

		Position { x: 1, y: -2 } => {tail_position.x += 1; tail_position.y -= 1},
		Position { x: 2, y: -1 } => {tail_position.x += 1; tail_position.y -= 1},
		Position { x: 2, y: -2 } => {tail_position.x += 1; tail_position.y -= 1},

		delta_pos => panic!("PANIC: diff in position {:?}", delta_pos),
	};
}

fn part1() {
	let content = fs::read_to_string("./src/input").unwrap();
	
	let mut head_position = Position { x: 0, y: 0 };
	let mut tail_position = Position { x: 0, y: 0 };
	
	let mut total_tail_positions: HashSet<Position> = HashSet::new();
	total_tail_positions.insert(tail_position);
	
	for line in content.lines() {
		let mut split = line.split_whitespace();
		
		let direction = split.next().unwrap();
		let amount: i32 = split.next().unwrap().parse().unwrap();
		
		match direction {
			d => {
				for _ in 0..amount {
					move_head(&mut head_position, direction);
					move_tail(&head_position, &mut tail_position);
					total_tail_positions.insert(tail_position);
				}
			},
		}
	}
	
	println!("{}", total_tail_positions.len());
}

fn main() {
	let content = fs::read_to_string("./src/input").unwrap();

	let mut head_position = Position { x: 0, y: 0 };
	let mut tail_1 = Position { x: 0, y: 0 };
	let mut tail_2 = Position { x: 0, y: 0 };
	let mut tail_3 = Position { x: 0, y: 0 };
	let mut tail_4 = Position { x: 0, y: 0 };
	let mut tail_5 = Position { x: 0, y: 0 };
	let mut tail_6 = Position { x: 0, y: 0 };
	let mut tail_7 = Position { x: 0, y: 0 };
	let mut tail_8 = Position { x: 0, y: 0 };
	let mut tail_9 = Position { x: 0, y: 0 };

	let mut total_tail_positions: HashSet<Position> = HashSet::new();
	total_tail_positions.insert(tail_9);

		
	for line in content.lines() {
		let mut split = line.split_whitespace();
		
		let direction = split.next().unwrap();
		let amount: i32 = split.next().unwrap().parse().unwrap();
		
		match direction {
			d => {
				for _ in 0..amount {
					move_head(&mut head_position, direction);

					move_tail(&head_position, &mut tail_1);
					move_tail(&tail_1, &mut tail_2);
					move_tail(&tail_2, &mut tail_3);
					move_tail(&tail_3, &mut tail_4);
					move_tail(&tail_4, &mut tail_5);
					move_tail(&tail_5, &mut tail_6);
					move_tail(&tail_6, &mut tail_7);
					move_tail(&tail_7, &mut tail_8);
					move_tail(&tail_8, &mut tail_9);

					total_tail_positions.insert(tail_9);
				}
			},
		}
	}
	
	println!("{}", total_tail_positions.len());
}
