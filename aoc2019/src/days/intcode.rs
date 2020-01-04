use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::convert::TryInto;

pub struct Intcode {
	pub program: Vec<i32>,
	pub original_program: Vec<i32>,

	pub input_value: i32,
	pub output_value: i32,
}

impl Intcode {
	pub fn new() -> Self {
		Intcode {
			program: vec![],
			original_program: vec![],

			input_value: 0,
			output_value: 0,
		}
	}

	pub fn read_program_from_file(&mut self, filepath: &str) {
		let file = File::open(filepath).unwrap();
		let reader = BufReader::new(file);
		
		self.original_program = reader.lines().next().unwrap().unwrap()	// Gets next line
									  .trim().split(',')				// Splits on commas
									  .map(|s| s.parse().unwrap())		// Parses into an i32
							 		  .collect();						// Collects into a Vec<i32>
		self.program = self.original_program.to_vec();
	}

	pub fn run_program(&mut self) {
		let mut index = 0;

		while index < self.program.len() {
			let instruction = self.program[index];
			
			let instruction_type = instruction % 100; // Gets the rightmost two digits
			let parameters = instruction / 100;
			let parameters: Vec<i32> = Intcode::get_parameter_modes(instruction_type, parameters);
			
			match instruction_type {
				1 | 2 => {
					let storage_location_index = self.program[index + 3] as usize;
					let first_value: i32;
					let second_value: i32;

					if parameters[0] == 0 {
						let first_value_index = self.program[index + 1] as usize;
						first_value = self.program[first_value_index];
					} else {
						first_value = self.program[index + 1];
					}

					if parameters[1] == 0 {
						let second_value_index = self.program[index + 2] as usize;
						second_value = self.program[second_value_index];
					} else {
						second_value = self.program[index + 2];
					}
					
					assert!(storage_location_index < self.program.len().try_into().unwrap());
					
					if instruction_type == 1 {
						// Instruction 1 is add
						self.program[storage_location_index] = first_value + second_value;
					} else {
						// Instruction 2 is multiply
						self.program[storage_location_index] = first_value * second_value;
					}

					index += 4;
				}
				3 => {
					let storage_location_index = self.program[index + 1] as usize;
					assert!(storage_location_index < self.program.len().try_into().unwrap());

					self.program[storage_location_index] = self.input_value;
					index += 2;
				}
				4 => {
					let value = self.program[index + 1];
					
					if parameters[0] == 0 {
						self.output_value = self.program[value as usize];
					} else {
						self.output_value = value;
					}

					index += 2;
				}
				5 | 6 => {
					let value = if parameters[0] == 0 {
						let value_index = self.program[index + 1] as usize;
						self.program[value_index]
					} else {
						self.program[index + 1]
					};

					if (instruction_type == 5 && value != 0) || (instruction_type == 6 && value == 0) {
						// Should jump
						let value: usize = if parameters[1] == 0 {
							let value_index = self.program[index + 2] as usize;
							self.program[value_index] as usize
						} else {
							self.program[index + 2] as usize
						};

						index = value;
					} else {
						index += 3;
					}
				}
				7 | 8 => {
					let first_value: i32;
					let second_value: i32;

					if parameters[0] == 0 {
						let value_index = self.program[index + 1] as usize;
						first_value = self.program[value_index];
					} else {
						first_value = self.program[index + 1];
					}

					if parameters[1] == 0 {
						let value_index = self.program[index + 2] as usize;
						second_value = self.program[value_index];
					} else {
						second_value = self.program[index + 2];
					}

					let storage_location_index = self.program[index + 3] as usize;

					if (instruction_type == 7 && first_value < second_value) || (instruction_type == 8 && first_value == second_value) {
						self.program[storage_location_index] = 1;
					} else {
						self.program[storage_location_index] = 0;
					}

					index += 4;
				}
				99 | _ => {
					break;
				}
			}
		}
	}

	fn get_parameter_modes(instruction_type: i32, parameters_number: i32) -> Vec<i32> {
		let mut number_of_parameters_left;

		match instruction_type {
			1 | 2 => number_of_parameters_left = 3,
			3 | 4 => number_of_parameters_left = 1,
			5 | 6 => number_of_parameters_left = 2,
			7 | 8 => number_of_parameters_left = 3,
			99 => number_of_parameters_left = 0,
			_ => {
				panic!("Incorrect program, unknown instruction {}.", instruction_type);
			}
		}

		let mut parameter_modes = vec![];
		let mut parameters = parameters_number;

		while number_of_parameters_left > 0 {
			parameter_modes.push(parameters % 10);
			parameters /= 10;

			number_of_parameters_left -= 1;
		}

		parameter_modes
	}
}
