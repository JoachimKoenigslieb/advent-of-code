use std::{fs, collections::HashMap};

fn main() {
	let contents = fs::read_to_string("./src/input").unwrap();
	let mut current_dir: String = "".to_string();
	let mut dir_size: HashMap<String, i32> = HashMap::new();
	let mut size = 0; 

	for line in contents.lines() {
		let mut split_line = line.split_whitespace();

		let (first, second) = (split_line.next(), split_line.next());
	
		match (first, second) {
			(Some("$"), Some("cd")) => {
				match split_line.next().unwrap() {
					"/" => { 
						current_dir = "".to_string() }
					".." => { 
						let splitted: Vec<&str> = current_dir.split("/").into_iter().collect();
						let (_last, rest) = splitted.split_last().unwrap();
						current_dir = rest.join("/").to_owned();
					},
					path => { current_dir = format!("{}/{}", current_dir, path)}
				}
			},
			(Some("$"), Some("ls")) => (),
			(Some("dir"), _) => (),
			(Some(size), Some(name)) => {
				// println!("found a file called {name} of size {size} cur dir is {current_dir}");
				let folder_split: Vec<&str>= current_dir.split("/").collect();

				let mut splitter = folder_split.split_last();
				let mut rest_vec: Vec<&str>;

				while let Some((last, rest)) = splitter {
					let my_rest = last.to_owned();
					rest_vec = rest.to_vec();

					let mut merged_again = rest_vec.clone();
					merged_again.push(my_rest);

					let key = merged_again.join("/");

					let file_size = size.parse::<i32>().unwrap();
					let updated_size = dir_size.get(&key).unwrap_or(&0) + file_size;
					dir_size.insert(key, updated_size);

					splitter = rest_vec.split_last();
				}
			}
 			_ => (),
		}
	}

	// // part one
	// for (_key, val) in dir_size.into_iter() {
	// 	if val < 100000 {
	// 		size += val;
	// 	}
	// }
	// println!("total size is {size}");


	// part two
	let unused_space = 70000000 - dir_size.get("").unwrap();
	let to_delete = 30000000 - unused_space;
	println!("unused space: {unused_space:?}. I need to delete {to_delete}");

	let mut file_sizes: Vec<&i32> = dir_size.values().collect();
	file_sizes.sort();

	for val in file_sizes {
		if val > &to_delete {
			println!("i can delete file of size {val}");
			break;	
		}
	}
}
