use std::fs::File;
use std::io::{ BufReader, BufRead };

mod intcode;
use intcode::Intcode;
mod grid;
use grid::Grid;

// NOTE(fkp): Day 1 - The Tyranny of the Rocket Equation
pub fn day_1() {
	const FILEPATH: &str = "res/day_1.txt";
	
	let file = File::open(FILEPATH).unwrap();
	let reader = BufReader::new(file);
	
	let mut fuel_required_for_modules_only = 0; // Part 1
	let mut total_fuel_required = 0; // Part 2
	
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

	// NOTE(fkp): Part 1
	intcode_computer.program[1] = 12;
	intcode_computer.program[2] = 2;

	intcode_computer.run_program();
	let part_1_result = intcode_computer.program[0];
	
	// NOTE(fkp): Part 2
	const REQUIRED_OUTPUT: i32 = 19_690_720;
	let mut output_value = 0;
	
	'outer: for noun in 0..100 {
		for verb in 0..100 {
			intcode_computer.program = intcode_computer.original_program.to_vec();
			
			// Sets the required input values
			intcode_computer.program[1] = noun;
			intcode_computer.program[2] = verb;

			intcode_computer.run_program();

			if intcode_computer.program[0] == REQUIRED_OUTPUT {
				output_value = 100 * noun + verb;
				break 'outer;
			}
		}
	}
	
	println!("Day 2 (Part 1): First Index of Resulting Program = {}", part_1_result);
	println!("      (Part 2): 100 * noun + verb = {}", output_value);
}

// NOTE(fkp): Day 3 - Crossed Wires
pub fn day_3() {
	const FILEPATH: &str = "res/day_3.txt";

	let mut grid = Grid::new(FILEPATH);
	grid.trace_wire_paths();

	println!("Day 3 (Part 1): Manhattan Distance to Nearest Intersection = {}", grid.get_closest_intersection_distance_manhattan());
	println!("      (Part 2): Closest Intersection (Number of Steps) = {}", grid.get_closest_intersection_distance_steps());
}

// NOTE(fkp): Day 4 - Secure Container
pub fn day_4() {
	// From input data
	const MIN_NUMBER: i32 = 264_793;
	const MAX_NUMBER: i32 = 803_935;

	let mut part_1_number_of_valid_passwords = 0;
	let mut part_2_number_of_valid_passwords = 0;
	
	for number in MIN_NUMBER..=MAX_NUMBER {
		if part_1_is_valid_password(number) {
			part_1_number_of_valid_passwords += 1;
		}

		if part_2_is_valid_password(number) {
			part_2_number_of_valid_passwords += 1;
		}
	}

	println!("Day 4 (Part 1): Number of Valid Passwords = {}", part_1_number_of_valid_passwords);
	println!("      (Part 2): Number of Valid Passwords = {}", part_2_number_of_valid_passwords);
}

fn part_1_is_valid_password(number: i32) -> bool {
	let number_as_string = number.to_string();
	let mut has_adjacent_same_digits = false;
	
	for i in 1..number_as_string.len() {
		if number_as_string.as_bytes()[i] < number_as_string.as_bytes()[i - 1] {
			return false;
		}
		
		if number_as_string.as_bytes()[i] == number_as_string.as_bytes()[i - 1] {
			has_adjacent_same_digits = true;
		}
	}

	has_adjacent_same_digits
}

fn part_2_is_valid_password(number: i32) -> bool {
	let number_as_string = number.to_string();
	let number_as_bytes = number_as_string.as_bytes();
	let mut has_adjacent_same_digits = false;
	
	for i in 1..number_as_bytes.len() {
		if number_as_bytes[i] < number_as_bytes[i - 1] {
			return false;
		}
		
		if number_as_bytes[i] == number_as_bytes[i - 1] {
			let mut same_digits_allowed = true;
			
			// Three in a row is not allowed
			if i < number_as_bytes.len() - 1 && number_as_bytes[i + 1] == number_as_bytes[i] {
				same_digits_allowed = false;
			}

			if i > 1 && number_as_bytes[i - 2] == number_as_bytes[i] {
				same_digits_allowed = false;
			}

			if same_digits_allowed {
				has_adjacent_same_digits = true;
			}
		}
	}

	has_adjacent_same_digits
}

// NOTE(fkp): Day 5 - Sunny With a Chance of Asteroids
pub fn day_5() {
	const FILEPATH: &str = "res/day_5.txt";
	
	let mut intcode_computer = Intcode::new();
	intcode_computer.read_program_from_file(FILEPATH);

	// NOTE(fkp): Part 1
	intcode_computer.input_value = 1;
	intcode_computer.run_program();
	let part_1_output_value = intcode_computer.output_value;
	
	// NOTE(fkp): Part 2
	intcode_computer.program = intcode_computer.original_program.to_vec();
	intcode_computer.input_value = 5;
	intcode_computer.run_program();
	let part_2_output_value = intcode_computer.output_value;

	
	println!("Day 5 (Part 1): Output Diagnostic Code = {}", part_1_output_value);
	println!("      (Part 2): Output Diagnostic Code = {}", part_2_output_value);
}

// NOTE(fkp): Day 6 - Universal Orbit Map
pub fn day_6() {
	const FILEPATH: &str = "res/day_6.txt";

	let orbital_pairs: Vec<(String, String)> = read_file_into_orbital_pairs(FILEPATH);
	let number_of_orbits = recursive_count_orbits(orbital_pairs[0].1.as_str(),	&orbital_pairs);

	println!("Day 6 (Part 1): Number of Orbits = {}", number_of_orbits);
}

// Returns Vec<(orbitee, orbiter)>
fn read_file_into_orbital_pairs(filepath: &str) -> Vec<(String, String)> {
	let file = File::open(filepath).unwrap();
	let reader = BufReader::new(file);
	let mut result: Vec<(String, String)> = vec![];
	
	for line in reader.lines() {
		let line = line.unwrap();
		let values: Vec<&str> = line.trim().split(',').collect();

		result.push((values[0].to_string(), values[1].to_string()));
	}

	result
}

// Returns number of orbits
fn recursive_count_orbits(target: &str, orbital_pairs: &Vec<(String, String)>, orbits_already_added: &mut Vec<(&String, i32)>) -> i32 {
	if target == "COM" {
		return 1;
	}

	for orbit in orbital_pairs {
		if orbit.1 == target {
			return 1 + recursive_count_orbits(&orbit.0, orbital_pairs);
		}
	}
		
	panic!("Targer not in orbit.");
}
