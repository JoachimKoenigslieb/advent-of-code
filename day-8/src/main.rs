use std::{fs, collections::{HashMap, HashSet}};
use ndarray::{s, ViewRepr};
use ndarray::{Array2, Axis, Dim, ArrayBase, OwnedRepr};

fn score_from_slice(slice: &ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>, max: &i32) -> i32 {
	let res = slice
		.into_iter()
		.enumerate()
		.map(|opt| {
			let index_as_i32: i32 = opt.0.try_into().unwrap();
			(index_as_i32, opt.1)
		})
		.find(|(i, val)| val >= &max); // this is off by one
	let (score, _) = res.unwrap_or({
		let len_as_signed: i32 = slice.len().try_into().unwrap(); 
		(len_as_signed - 1, &-1)
	}); // we have to make this off by one too...

	score + 1
}

fn get_visible_trees(forest: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>, coords: (i32, i32)) -> i32 {
	let highest = forest.get(( coords.0 as usize, coords.1 as usize)).unwrap();
	
	let slice_right = forest.slice(s![coords.0, coords.1+1..]);
	let slice_left = forest.slice(s![coords.0, ..coords.1; -1]);
	
	let slice_up = forest.slice(s![..coords.0; -1, coords.1]);
	let slice_down = forest.slice(s![coords.0+1.., coords.1]);

	let left_score = score_from_slice(&slice_left, highest);
	let right_score = score_from_slice(&slice_right, highest);
	let up_score = score_from_slice(&slice_up, highest);
	let down_score = score_from_slice(&slice_down, highest);

	// println!("left {}", left_score);
	// println!("right {}", right_score);
	// println!("up {}", up_score);
	// println!("down {}", down_score);

	left_score * right_score * up_score * down_score
}

fn part1(forest: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>, dim: usize) {
	let mut visible: HashSet<( usize, usize, )> = HashSet::new();

	// left to right
	for i in 0..dim {
		let mut highest = &0;

		for j in 0..dim {
			let tree = forest.get((i, j)).unwrap();

			match (tree > highest) || (j == 0) {
				true => { highest = tree; visible.insert((i, j)); },
				false => (),
			}
		}
	} 

	// right to left
	for i in 0..dim {
		let mut highest = &0;

		for j in (0..dim).rev() {
			let tree = forest.get((i, j)).unwrap();

			match (tree > highest) || (j == dim - 1) {
				true => { highest = tree; visible.insert((i, j)); },
				false => (),
			}
		}
	} 

	// up to down
	for j in 0..dim {
		let mut highest = &0;

		for i in 0..dim {
			let tree = forest.get((i, j)).unwrap();

			match (tree > highest) || (i == 0) {
				true => { highest = tree; visible.insert((i, j)); },
				false => (),
			}
		}
	} 

	// down to up
	for j in 0..dim {
		let mut highest = &0;

		for i in (0..dim).rev() {
			let tree = forest.get((i, j)).unwrap();

			match (tree > highest) || (i == dim - 1) {
				true => { highest = tree; visible.insert((i, j)); },
				false => (),
			}
		}
	} 
}

fn main() {
	let content = fs::read_to_string("./src/input").unwrap();
	let dim: i32 = 99;

	let nums: Vec<i32> = content
		.lines()
		.flat_map(|line| {
			line.chars().map(|num| num.to_string().parse::<i32>().unwrap())
		})
		.collect();

	let forest = Array2::from_shape_vec((dim as usize, dim as usize), nums).unwrap();
	let mut score = 0;

	for i in 0..dim {
		for j in 0..dim {
			let this_score = get_visible_trees(&forest, (i, j));

			if this_score > score {
				score = this_score;
			} 
		}
	}
	


	println!("{}", score);
}
