use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::cmp;

#[derive(Copy, Clone)]
pub struct Point {
	row: i32,
	col: i32
}

pub struct Wire {
	path: Vec<Vec<u8>>,
	row_count: usize,
	col_count: usize,
	starting_point: Point,
}

impl Wire {
	pub fn new(row_count: usize, col_count: usize, starting_point: Point) -> Self {
		Wire {
			path: vec![vec![0; col_count]; row_count],
			row_count,
			col_count,
			starting_point,
		}
	}

	pub fn get_required_grid_size(filepath: &str) -> (usize, usize, Point) {
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

	pub fn trace_path(&mut self, filepath: &str, starting_point: Point, is_first_path: bool) {
		let file = File::open(filepath).unwrap();
		let reader = BufReader::new(file);
		let mut lines_iter = reader.lines();


		if !is_first_path {
			lines_iter.next();
		}

		let line = lines_iter.next().unwrap().unwrap();
		let instructions: Vec<&str> = line.split(',').collect();

		let mut current_point = starting_point;
		self.path[current_point.row as usize][current_point.col as usize] = 1;

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
						self.path[current_point.row as usize][current_point.col as usize] = 1;
						current_point.row -= 1;
					}
				}
				'D' => {
					for _ in 0..distance {
						self.path[current_point.row as usize][current_point.col as usize] = 1;
						current_point.row += 1;
					}
				}
				'L' => {
					for _ in 0..distance {
						self.path[current_point.row as usize][current_point.col as usize] = 1;
						current_point.col -= 1;
					}
				}
				'R' => {
					for _ in 0..distance {
						self.path[current_point.row as usize][current_point.col as usize] = 1;
						current_point.col += 1;
					}
				}
				_ => {
					panic!("Non-standard direction for wires.");
				}
			}
		}
	}

	pub fn compare_intersections(&self, other_wire: &Wire) -> i32 {
		let mut intersection_distances: Vec<i32> = vec![];
		
		for row in 0..self.row_count {
			for col in 0..self.col_count {
				if (self.path[row][col] == 1) && (other_wire.path[row][col] == 1) {
					let row_distance = (self.starting_point.row as i32 - row as i32).abs();
					let col_distance = (self.starting_point.col as i32 - col as i32).abs();
					let distance = row_distance + col_distance;

					if distance != 0 {
						// Starting point doesn't count
						intersection_distances.push(row_distance + col_distance);
					}	
				}
			}
		}

		intersection_distances.sort();
		intersection_distances[0]
	}
}
