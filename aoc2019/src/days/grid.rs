use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::cmp;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
	row: i32,
	col: i32
}

pub struct Grid {
	filepath: &'static str,

	grid: Vec<Vec<GridLocationState>>,
	starting_point: Point,
	
	intersection_points: Vec<Point>,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum GridLocationState {
	None,
	First,
	Second,
	Both
}

impl Grid {
	pub fn new(filepath: &'static str) -> Self {
		let (row_count, col_count, starting_point) = Grid::get_required_size(filepath);
		
		Grid {
			filepath,

			grid: vec![vec![GridLocationState::None; col_count]; row_count],
			starting_point,

			intersection_points: vec![],
		}
	}

	fn get_required_size(filepath: &str) -> (usize, usize, Point) {
		let file = File::open(filepath).unwrap();
		let reader = BufReader::new(file);

		let mut greatest_up_down_distance = 0;
		let mut greatest_left_right_distance = 0;

		for line in reader.lines() {
			let line = line.unwrap();
			let instructions: Vec<&str> = line.split(',').collect();

			let mut current_up_down = 0;
			let mut current_left_right = 0;

			for instruction in instructions {
				let direction = instruction.chars().next().unwrap();
				let distance: i32 = instruction.char_indices()
											   .next()
											   .and_then(|(i, _)| instruction.get(i + 1..))
											   .unwrap()
											   .parse().unwrap();

				match direction {
					'U' => {
						current_up_down -= distance;
					}
					'D' => {
						current_up_down += distance;
					}
					'L' => {
						current_left_right -= distance;
					}
					'R' => {
						current_left_right += distance;
					}
					_ => {
						panic!("Non-standard direction for wires.");
					}
				}

				greatest_up_down_distance = cmp::max(current_up_down.abs(), greatest_up_down_distance);
				greatest_left_right_distance = cmp::max(current_left_right.abs(), greatest_left_right_distance);
			}
		}

		((greatest_up_down_distance * 2 + 1) as usize, (greatest_left_right_distance * 2 + 1) as usize, Point { row: greatest_up_down_distance, col: greatest_left_right_distance })
	}

	pub fn trace_wire_paths(&mut self) {
		let file = File::open(self.filepath).unwrap();
		let reader = BufReader::new(file);
		let mut lines_iter = reader.lines();

		for wire_index in 0..2 {
			let mut wire_type = GridLocationState::First;
			
			if wire_index == 1 {
				wire_type = GridLocationState::Second;
			}

			let line = lines_iter.next().unwrap().unwrap();
			let instructions: Vec<&str> = line.split(',').collect();

			let mut current_point = self.starting_point;
			self.set_location_as_touched(current_point, wire_type);

			for instruction in instructions {
				let direction = instruction.chars().next().unwrap();
				let distance: i32 = instruction.char_indices()
											.next()
											.and_then(|(i, _)| instruction.get(i + 1..))
											.unwrap()
											.parse().unwrap();

				match direction {
					'U' => {
						for _ in 0..distance {
							self.set_location_as_touched(current_point, wire_type);
							current_point.row -= 1;
						}
					}
					'D' => {
						for _ in 0..distance {
							self.set_location_as_touched(current_point, wire_type);
							current_point.row += 1;
						}
					}
					'L' => {
						for _ in 0..distance {
							self.set_location_as_touched(current_point, wire_type);
							current_point.col -= 1;
						}
					}
					'R' => {
						for _ in 0..distance {
							self.set_location_as_touched(current_point, wire_type);
							current_point.col += 1;
						}
					}
					_ => {
						panic!("Non-standard direction for wires.");
					}
				}
			}
		}
	}

	#[inline]
	fn set_location_as_touched(&mut self, location: Point, wire_type: GridLocationState) {
		match self.grid[location.row as usize][location.col as usize] {
			GridLocationState::None => {
				self.grid[location.row as usize][location.col as usize] = wire_type;
			}
			GridLocationState::First | GridLocationState::Second => {
				if self.grid[location.row as usize][location.col as usize] != wire_type {
					self.grid[location.row as usize][location.col as usize] = GridLocationState::Both;
					self.add_location_to_intersections(location);
				}
			}
			GridLocationState::Both => {}
		}
	}

	#[inline]
	fn add_location_to_intersections(&mut self, location: Point) {
		if location != self.starting_point {
			self.intersection_points.push(location);
		}
	}

	pub fn get_closest_intersection_distance_manhattan(&self) -> i32 {
		let mut intersection_distances: Vec<i32> = vec![];

		for intersection in &self.intersection_points {
			let row_distance = (self.starting_point.row as i32 - intersection.row as i32).abs();
			let col_distance = (self.starting_point.col as i32 - intersection.col as i32).abs();

			intersection_distances.push(row_distance + col_distance);
		}
		
		intersection_distances.sort();
		intersection_distances[0]
	}

	pub fn get_closest_intersection_distance_steps(&mut self) -> i32 {
		let file = File::open(self.filepath).unwrap();
		let reader = BufReader::new(file);
		
		let mut intersection_and_steps: HashMap<Point, i32> = HashMap::new();

		for line in reader.lines() {
			let line = line.unwrap();
			let instructions: Vec<&str> = line.split(',').collect();

			let mut current_point = self.starting_point;
			let mut current_number_of_steps = 0;

			for instruction in instructions {
				let direction = instruction.chars().next().unwrap();
				let distance: i32 = instruction.char_indices()
											   .next()
											   .and_then(|(i, _)| instruction.get(i + 1..))
											   .unwrap()
											   .parse().unwrap();

				match direction {
					'U' => {
						for _ in 0..distance {
							current_point.row -= 1;
							current_number_of_steps += 1;

							if self.intersection_points.contains(&current_point) {
								if let Some(steps) = intersection_and_steps.get_mut(&current_point) {
									*steps += current_number_of_steps;
								} else {
									intersection_and_steps.insert(current_point, current_number_of_steps);
								}
							}
						}
					}
					'D' => {
						for _ in 0..distance {
							current_point.row += 1;
							current_number_of_steps += 1;

							if self.intersection_points.contains(&current_point) {
								if let Some(steps) = intersection_and_steps.get_mut(&current_point) {
									*steps += current_number_of_steps;
								} else {
									intersection_and_steps.insert(current_point, current_number_of_steps);
								}
							}
						}
					}
					'L' => {
						for _ in 0..distance {
							current_point.col -= 1;
							current_number_of_steps += 1;

							if self.intersection_points.contains(&current_point) {
								if let Some(steps) = intersection_and_steps.get_mut(&current_point) {
									*steps += current_number_of_steps;
								} else {
									intersection_and_steps.insert(current_point, current_number_of_steps);
								}
							}
						}
					}
					'R' => {
						for _ in 0..distance {
							current_point.col += 1;
							current_number_of_steps += 1;

							if self.intersection_points.contains(&current_point) {
								if let Some(steps) = intersection_and_steps.get_mut(&current_point) {
									*steps += current_number_of_steps;
								} else {
									intersection_and_steps.insert(current_point, current_number_of_steps);
								}
							}
						}
					}
					_ => {
						panic!("Non-standard direction for wires.");
					}
				}
			}
		}

		let mut steps: Vec<i32> = intersection_and_steps.values().cloned().collect();
		steps.sort();
		steps[0]
	}
}
