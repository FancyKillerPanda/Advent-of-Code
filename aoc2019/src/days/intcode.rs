use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::convert::TryInto;

pub struct Intcode {
	pub program: Vec<i32>,
	pub original_program: Vec<i32>
}

impl Intcode {
	pub fn new() -> Self {
		Intcode {
			program: vec![],
			original_program: vec![],
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
			match self.program[index] {
				1 => {
					let storage_location_index = self.program[index + 3] as usize;
					let first_value_index = self.program[index + 1] as usize;
					let second_value_index = self.program[index + 2] as usize;
					
					assert!(storage_location_index < self.program.len().try_into().unwrap());
					self.program[storage_location_index] = self.program[first_value_index] + self.program[second_value_index];

					index += 4;
				}
				2 => {
					let storage_location_index = self.program[index + 3] as usize;
					let first_value_index = self.program[index + 1] as usize;
					let second_value_index = self.program[index + 2] as usize;
					
					assert!(storage_location_index < self.program.len().try_into().unwrap());
					self.program[storage_location_index] = self.program[first_value_index] * self.program[second_value_index];

					index += 4;
				}
				99 | _ => {
					break;
				}
			}
		}
	}
}
