use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::convert::TryInto;

pub struct Intcode {
	pub numbers: Vec<i32>,
}

impl Intcode {
	pub fn new() -> Self {
		Intcode {
			numbers: vec![],
		}
	}

	pub fn read_program_from_file(&mut self, filepath: &str) {
		let file = File::open(filepath).unwrap();
		let reader = BufReader::new(file);
		
		self.numbers = reader.lines().next().unwrap().unwrap()	// Gets next line
							 .trim().split(',')					// Splits on commas
							 .map(|s| s.parse().unwrap())		// Parses into an i32
							 .collect();						// Collects into a Vec<i32>
	}

	pub fn run_program(&mut self) {
		let mut index = 0;

		while index < self.numbers.len() {
			match self.numbers[index] {
				1 => {
					let storage_location_index = self.numbers[index + 3] as usize;
					let first_value_index = self.numbers[index + 1] as usize;
					let second_value_index = self.numbers[index + 2] as usize;
					
					assert!(storage_location_index < self.numbers.len().try_into().unwrap());
					self.numbers[storage_location_index] = self.numbers[first_value_index] + self.numbers[second_value_index];

					index += 4;
				}
				2 => {
					let storage_location_index = self.numbers[index + 3] as usize;
					let first_value_index = self.numbers[index + 1] as usize;
					let second_value_index = self.numbers[index + 2] as usize;
					
					assert!(storage_location_index < self.numbers.len().try_into().unwrap());
					self.numbers[storage_location_index] = self.numbers[first_value_index] * self.numbers[second_value_index];

					index += 4;
				}
				99 | _ => {
					break;
				}
			}
		}
	}
}
