use std::fs::File;
use std::io::{ BufReader, BufRead };

mod intcode;
use intcode::Intcode;

// NOTE(fkp): Day 1 - The Tyranny of the Rocket Equation
pub fn day_1() {
	const FILEPATH: &str = "res/day_1.txt";
	
	let file = File::open(FILEPATH).unwrap();
	let reader = BufReader::new(file);
	
	let mut fuel_required_for_modules_only = 0; // Part 1
	let mut total_fuel_required = 0; // Part 2
	
	// NOTE(fkp): Part 1
	for line in reader.lines() {
		let line = line.unwrap();
		let module_mass: i32 = line.parse().unwrap();
		
		fuel_required_for_modules_only += calculate_fuel_for_mass(module_mass);
		total_fuel_required += calculate_module_fuel(module_mass);
	}
	
	println!("Day 1 (Part 1): Fuel Required (Modules Only) = {}", fuel_required_for_modules_only);
	println!("      (Part 2): Total Fuel Required = {}", total_fuel_required);
}

fn calculate_fuel_for_mass(mass: i32) -> i32 {
	(mass / 3) - 2
}

fn calculate_module_fuel(module_mass: i32) -> i32 {
	let mut total_mass = 0;
	let mut mass = calculate_fuel_for_mass(module_mass);
	
	while mass > 0 {
		total_mass += mass;
		mass = calculate_fuel_for_mass(mass);
	}
	
	total_mass
}

// NOTE(fkp): Day 2 - 1202 Program Alarm
pub fn day_2() {
	const FILEPATH: &str = "res/day_2.txt";
	
	let mut intcode_computer = Intcode::new();
	intcode_computer.read_program_from_file(FILEPATH);
	intcode_computer.run_program();
	
	println!("Day 2 (Part 1): First Index of Resulting Program = {}", intcode_computer.numbers[0]);
}
