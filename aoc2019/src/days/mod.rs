use std::fs::File;
use std::io::{ BufReader, BufRead };

mod intcode;
use intcode::Intcode;
mod wire;
use wire::Wire;

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
	intcode_computer.run_program();
	
	// NOTE(fkp): Part 2
	const REQUIRED_OUTPUT: i32 = 19690720;
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
	
	println!("Day 2 (Part 1): First Index of Resulting Program = {}", intcode_computer.program[0]);
	println!("      (Part 2): 100 * noun + verb = {}", output_value);
}

// NOTE(fkp): Day 3 - Crossed Wires
pub fn day_3() {
	const FILEPATH: &str = "res/day_3.txt";

	let (row_count, col_count, starting_point) = Wire::get_required_grid_size(FILEPATH);

	let mut first_wire = Wire::new(row_count, col_count, starting_point);
	let mut second_wire = Wire::new(row_count, col_count, starting_point);

	first_wire.trace_path(FILEPATH, starting_point, true);
	second_wire.trace_path(FILEPATH, starting_point, false);

	println!("Day 3 (Part 1): Manhattan Distance to Nearest Intersection = {}", first_wire.compare_intersections(&second_wire));
}
