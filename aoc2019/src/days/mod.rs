use std::fs::File;
use std::io::{ BufReader, BufRead };

pub fn day_1() {
	const FILEPATH : &str = "res/day_1.txt";
	
	let file = File::open(FILEPATH).unwrap();
	let reader = BufReader::new(file);

	let mut total_fuel_required = 0;
	
	for line in reader.lines() {
		let line = line.unwrap();
		let module_mass: i32 = line.parse().unwrap();
		
		total_fuel_required += calculate_module_fuel(module_mass);
	}

	println!("Day 1 (Part 1): Total Fuel Required = {}", total_fuel_required);
}

fn calculate_module_fuel(mass: i32) -> i32 {
	(mass / 3) - 2
}
