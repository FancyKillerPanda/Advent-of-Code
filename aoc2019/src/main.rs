use std::env;

mod days;
use days::*;

const DAYS_ARRAY: [fn(); 6] = [day_1, day_2, day_3, day_4, day_5, day_6];

fn main() {
	let mut args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		for day in &DAYS_ARRAY {
			day();
		}
	} else {
		// Removes executable
		args.remove(0);
		
		let mut indices_to_run = vec![];

		for arg in args {
			if arg.starts_with('-') {
				let index: usize = arg.char_indices()
									  .next()
									  .and_then(|(i, _)| arg.get(i + 1..))
									  .unwrap()
									  .parse().unwrap();

				let index = index - 1;

				if index >= DAYS_ARRAY.len() {
					println!("Error: argument out of range (day {} not yet implemented)", index + 1);
					return;
				}

				indices_to_run.push(index);
			} else {
				println!("Error: arguments must be passed in the format -1, -2, etc.");
				return;
			}
		}

		for index in indices_to_run {
			DAYS_ARRAY[index]();
		}
	}
}
