use std::{fs};

struct Monkey {
	items: Vec<i64>,
	operation: Box<dyn Fn(i64) -> i64>,
	test_modulus: i64,
	false_target: i64,
	true_target: i64,
	index: usize,
	num_inspects: usize,
}

#[derive(Debug)]
struct ThrownItem {
	target: i64,
	worry: i64,
}

impl Monkey {
	fn inspect_items(&mut self, is_part_1: bool) -> Vec<ThrownItem> {
		let thrown_items: Vec<ThrownItem> = (&self.items).into_iter().map(|worry| {
			let udpated_worry = (self.operation)(*worry);
			let worry_after_bored = {
				if is_part_1 {
					udpated_worry / 3
				} else {
					udpated_worry
				}
			}; // i think integer division takes care of roudning down actually..

			let target = match worry_after_bored % self.test_modulus {
				0 => self.true_target,
				_ => self.false_target
			};

			ThrownItem { target, worry: worry_after_bored }
		}).collect();

		self.items = Vec::new();
		self.num_inspects += thrown_items.len();
		thrown_items
	}
}

fn parse_monkey(monke: &str, index: usize) -> Monkey {
	let mut lines = monke.lines();

	lines.next();
	let mut items_line = lines.next().unwrap().split(": ");

	items_line.next();
	let items: Vec<i64> = items_line
		.next()
		.unwrap()
		.split(", ")
		.map(|item| item.parse().unwrap())
		.collect();

	let mut operation_line = lines.next().unwrap().split("= ");
	operation_line.next();

	let mut formula = operation_line.next().unwrap().split_whitespace();

	let operators: (&str, &str, &str) = ( formula.next().unwrap(), formula.next().unwrap(), formula.next().unwrap(), );

	let operation: Box<dyn Fn(i64) -> i64> = match operators {
		("old", "*", "old") => Box::new(|val| val.pow(2)),
		("old", "*", arg) => {
			let owned_arg = arg.to_owned().parse::<i64>().unwrap();
			Box::new(move |val| val * owned_arg)
		},
		("old", "+", arg) => {
			let owned_arg = arg.to_owned().parse::<i64>().unwrap();
			Box::new(move |val| val + owned_arg)
		},
		err => panic!("{err:?} panic"),
	};

	let test_modulus: i64 = lines.next().unwrap().split_whitespace().into_iter().rev().next().unwrap().parse().unwrap();
	let true_target: i64 = lines.next().unwrap().split_whitespace().into_iter().rev().next().unwrap().parse().unwrap();
	let false_target: i64 = lines.next().unwrap().split_whitespace().into_iter().rev().next().unwrap().parse().unwrap();

	return Monkey { items, operation, test_modulus, false_target, true_target, index, num_inspects: 0, }
}

fn debug_monkeys(monkeys: &Vec<Monkey>) {
	for monkey in monkeys {
		println!("Monkey {}: {:?}. Total throws: {}", monkey.index, monkey.items, monkey.num_inspects);
	}
}

fn main() {
	let content = fs::read_to_string("./src/input").unwrap();

	let mut monkeys: Vec<Monkey> = content.split("\n\n").enumerate().map(|(i, monke)| parse_monkey(monke, i)).collect();

	for round in 0..10000 {
		for i in 0..monkeys.len() {
			let thrown_items = monkeys[i].inspect_items(false);

			// println!("monkey {} throws {:?}", i, thrown_items);
			for item in thrown_items {
				let target_index: usize = item.target.try_into().unwrap();
				
				// println!("{target_index}");
				let target_monkey = &mut monkeys[target_index];
				// super magic number. we need to work in mod least common denominator. That way we are always agreeing on the results of the divisions tests!
				target_monkey.items.push(item.worry % 9699690); 
				// target_monkey.items.push(item.worry % 96577);
			}
		}

	}
	
	let sorted = &mut monkeys.into_iter().map(|monk| monk.num_inspects).collect::<Vec<usize>>();
	sorted.sort();

	println!("{:?}", sorted);
}
