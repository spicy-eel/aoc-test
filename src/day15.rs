use std::{iter, num::NonZero};

#[derive(Copy, Eq, Clone, PartialEq)]
enum Space {
	Empty,
	Wall,
	Box
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Direction {
	North,
	East,
	South,
	West
}

#[derive(Clone)]
struct Grid<T> {
	grid: Vec<T>,
	row_length: NonZero<usize>
}

impl<T> Grid<T> {
	#[allow(unused)]
	pub fn from_vec(grid: Vec<T>, row_length: NonZero<usize>) -> Self {
		assert_eq!(grid.len() % row_length, 0);
		assert!( !grid.is_empty() );
		
		unsafe { Self::from_vec_unchecked(grid, row_length) }
	}
	
	#[allow(unused)]
	pub fn from_first_row(grid: Vec<T>) -> Result<Self, Vec<T>> {
		if let Some(row_length) = NonZero::new(grid.len()) {
			Ok(unsafe { Self::from_vec_unchecked(grid, row_length) })
		} else {
			Err(grid)
		}
	}
	
	pub unsafe fn from_vec_unchecked(grid: Vec<T>, row_length: NonZero<usize>) -> Self {
		Self { grid, row_length }
	}
	
	#[allow(unused)]
	pub unsafe fn as_vec_mut(&mut self) -> &mut Vec<T> {
		&mut self.grid
	}
	
	#[allow(unused)]
	pub fn reserve_rows(&mut self, rows: usize) {
		let capacity = self.row_length().get() * rows;
		unsafe {
			self.as_vec_mut().reserve(capacity);
		}
	}
	
	#[allow(unused)]
	pub fn add_filled_row(&mut self, value: T) -> &mut [T] where T: Clone {
		let prev_len = self.total_length().get();
		let row_len = self.row_length().get();
		unsafe {
			self.as_vec_mut().resize(prev_len + row_len, value);
			self.as_mut_slice().get_unchecked_mut(prev_len..)
		}
	}
	
	
	pub fn as_slice(&self) -> &[T] {
		&self.grid
	}
	
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.grid
	}
	
	
	pub fn get(&self, row: usize, col: usize) -> Option<&T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice().get_unchecked(i) })
	}
	
	#[allow(unused)]
	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
		self.to_index(row, col).map(|i| unsafe { self.as_mut_slice().get_unchecked_mut(i) })
	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
		self.as_slice().get_unchecked(self.to_index_unchecked(row, col))
	}
	
	pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
		let index = self.to_index_unchecked(row, col);
		self.as_mut_slice().get_unchecked_mut(index)
	}
	
	
	pub fn to_index(&self, row: usize, col: usize) -> Option<usize> {
		(row < self.row_count().get() && col < self.row_length().get()).then(|| self.to_index_unchecked(row, col))
	}
	
	pub fn to_index_unchecked(&self, row: usize, col: usize) -> usize {
		row * self.row_length().get() + col
	}
	
	#[allow(unused)]
	pub fn to_row_col(&self, index: usize) -> Option<(usize, usize)> {
		(index < self.total_length().get()).then(|| self.to_row_col_unchecked(index))
	}
	
	pub fn to_row_col_unchecked(&self, index: usize) -> (usize, usize) {
		(index / self.row_length(), index % self.row_length())
	}
	
	
	pub fn get_row(&self, row: usize) -> Option<&[T]> {
		let row_start = row.checked_mul(self.row_length().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_slice().get(row_start..row_end)
	}
	
	pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
		let row_start = row.checked_mul(self.row_length().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_mut_slice().get_mut(row_start..row_end)
	}
	
	
	pub fn row_count(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.total_length().get() / self.row_length()) }
	}
	
	pub fn row_length(&self) -> NonZero<usize> {
		self.row_length
	}
	
	pub fn total_length(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.as_slice().len()) }
	}
}

fn wrapping_offset(position: (usize, usize), direction: Direction) -> (usize, usize) {
	let (row, col) = position;
	match direction {
		Direction::North => (row.wrapping_sub(1), col),
		Direction::East => (row, col.wrapping_add(1)),
		Direction::South => (row.wrapping_add(1), col),
		Direction::West => (row, col.wrapping_sub(1))
	}
}

impl Direction {
	fn from_char(c: char) -> Option<Self> {
		match c {
			'^' => Some(Self::North),
			'>' => Some(Self::East),
			'v' => Some(Self::South),
			'<' => Some(Self::West),
			_ => None
		}
	}
}

fn try_push_at(grid: &mut Grid<Space>, at: (usize, usize), in_dir: Direction, with: Space) -> bool {
	if let Some(&space) = grid.get(at.0, at.1) {
		let can_move = match space {
			Space::Empty => true,
			Space::Wall => false,
			Space::Box => try_push_at(grid, wrapping_offset(at, in_dir), in_dir, Space::Box)
		};
		
		if can_move && with != space {
			*unsafe { grid.get_unchecked_mut(at.0, at.1) } = with;
		}
		
		can_move
	} else {
		false
	}
}

fn box_sum(grid: &Grid<Space>) -> u64 {
	enumerate(grid.as_slice()).filter_map(|(i, &s)| matches!(s, Space::Box).then(|| grid.to_row_col_unchecked(i)))
			.map(|(row, col)| row as u64 * 100 + col as u64).sum()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
	let (grid, directions) = input.split_once("\n\n").unwrap();
	
	let row_length = NonZero::new(grid.find('\n').unwrap_or(grid.len())).unwrap();
	let mut vec = Vec::with_capacity({
		let total_len_with_ln = grid.len() + 1;
		let row_count = total_len_with_ln / unsafe { NonZero::new_unchecked(row_length.get() + 1) };
		
		total_len_with_ln - row_count
	});
	
	let mut position = (0, 0);
	for (i, c) in grid.bytes().filter(|&b| b != b'\n').enumerate() {
		vec.push(
			match c {
				b'#' => Space::Wall,
				b'O' => Space::Box,
				b'.' => Space::Empty,
				b'@' => {
					position = (i / row_length, (i % row_length) * 2);
					Space::Empty
				},
				_ => unreachable!()
			}
		);
	}
	
	let mut grid = Grid::from_vec(vec, row_length);
	
	for dir in directions.bytes() {
		if let Some(dir) = Direction::from_char(dir as char) {
			let to = wrapping_offset(position, dir);
			if try_push_at(&mut grid, to, dir, Space::Empty) {
				position = to;
			}
		}
	}
			
	box_sum(&grid)
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Space2 {
	Empty,
	Wall,
	BoxLeft,
	BoxRight
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Vertical {
	North,
	South
}

impl Direction {
	fn as_vertical(self) -> Option<Vertical> {
		match self {
			Self::North => Some(Vertical::North),
			Self::South => Some(Vertical::South),
			Self::East | Self::West => None
		}
	}
}

fn wrapping_offset_row(row: usize, dir: Vertical) -> usize {
	match dir {
		Vertical::North => row.wrapping_sub(1),
		Vertical::South => row.wrapping_add(1)
	}
}

fn try_push_big_box_vertically(grid: &mut Grid<Space2>, to_row: usize, box_left: usize, dir: Vertical, buffer: &mut Vec<usize>) -> bool {
	// safety: start_i ≤ box_lefts.len() and all entries of box_lefts must be ≤ grid.row_length() - 2
	unsafe fn inner(grid: &mut Grid<Space2>, to_row: usize, dir: Vertical, box_lefts: &mut Vec<usize>, start_i: usize) -> bool {
		let end_i = box_lefts.len();
		if start_i == end_i {
			true
		} else if let Some(row) = grid.get_row(to_row) {
			for i in start_i..end_i {
				let left = box_lefts[i];
				for pushing in [left, left + 1] {
					match *unsafe { row.get_unchecked(pushing) } {
						Space2::Wall => return false,
						Space2::Empty => {},
						b @ (Space2::BoxLeft | Space2::BoxRight) => {
							let left = if b == Space2::BoxLeft { pushing } else { pushing - 1 };
							if left > row.len() - 2 {
								return false;
							} else if box_lefts[end_i..].last() != Some(&left) {
								box_lefts.push(left);
							}
						}
					}
				}
			}
			
			let next_end_i = box_lefts.len();
			let success = unsafe { inner(grid, wrapping_offset_row(to_row, dir), dir, box_lefts, end_i) };
			if success {
				let row = unsafe { grid.get_row_mut(to_row).unwrap_unchecked() };
				for &moved_out in unsafe { box_lefts.get_unchecked(end_i..next_end_i) } {
					for i in [moved_out, moved_out + 1] {
						*unsafe { row.get_unchecked_mut(i) } = Space2::Empty;
					}
				}
				for &moved_in in unsafe { box_lefts.get_unchecked(start_i..end_i) } {
					*unsafe { row.get_unchecked_mut(moved_in) } = Space2::BoxLeft;
					*unsafe { row.get_unchecked_mut(moved_in + 1) } = Space2::BoxRight;
				}
			}
			success
		} else {
			false
		}
	}
	
	// -
	
	if box_left <= grid.row_length().get() - 2 {
		buffer.clear();
		buffer.push(box_left);
		unsafe {
			inner(grid, to_row, dir, buffer, 0)
		}
	} else {
		false
	}
}

fn try_push_at2(grid: &mut Grid<Space2>, at: (usize, usize), in_dir: Direction, with: Space2, buffer: &mut Vec<usize>) -> bool {
	if let Some(&space) = grid.get(at.0, at.1) {
		let can_move = match space {
			Space2::Empty => true,
			Space2::Wall => false,
			Space2::BoxLeft | Space2::BoxRight => if let Some(vert) = in_dir.as_vertical() {
				let (left, other) = if space == Space2::BoxLeft {
					(at.1, at.1 + 1)
				} else {
					(at.1 - 1, at.1 - 1)
				};
				let success = try_push_big_box_vertically(grid, wrapping_offset_row(at.0, vert), left, vert, buffer);
				if success {
					*unsafe { grid.get_unchecked_mut(at.0, other) } = Space2::Empty;
				}
				success
			} else {
				try_push_at2(grid, wrapping_offset(at, in_dir), in_dir, space, buffer)
			}
		};
		
		if can_move && with != space {
			*unsafe { grid.get_unchecked_mut(at.0, at.1) } = with;
		}
		
		can_move
	} else {
		false
	}
}

fn box_sum2(grid: &Grid<Space2>) -> u64 {
	enumerate(grid.as_slice()).filter_map(|(i, &s)| matches!(s, Space2::BoxLeft).then(|| grid.to_row_col_unchecked(i)))
			.map(|(row, col)| row as u64 * 100 + col as u64).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
	let (grid, directions) = input.split_once("\n\n").unwrap();
	
	let row_length = NonZero::new(grid.find('\n').unwrap_or(grid.len())).unwrap();
	let mut vec = Vec::with_capacity({
		let total_len_with_ln = grid.len() + 1;
		let row_count = total_len_with_ln / unsafe { NonZero::new_unchecked(row_length.get() + 1) };
		
		total_len_with_ln - row_count
	} * 2);
	
	const EMPTY: [Space2; 2] = [Space2::Empty; 2];
	const WALL: [Space2; 2] = [Space2::Wall; 2];
	const BOX: [Space2; 2] = [Space2::BoxLeft, Space2::BoxRight];
	
	let mut position = (0, 0);
	for (i, c) in grid.bytes().filter(|&b| b != b'\n').enumerate() {
		vec.extend(
			match c {
				b'#' => WALL,
				b'O' => BOX,
				b'.' => EMPTY,
				b'@' => {
					position = (i / row_length, (i % row_length) * 2);
					EMPTY
				},
				_ => unreachable!()
			}
		);
	}
	
	let mut grid = Grid::from_vec(vec, row_length.checked_mul(NonZero::new(2).unwrap()).unwrap());
	
	let mut buffer = Vec::with_capacity(16);
	for dir in directions.bytes() {
		if let Some(dir) = Direction::from_char(dir as char) {
			let to = wrapping_offset(position, dir);
			if try_push_at2(&mut grid, to, dir, Space2::Empty, &mut buffer) {
				position = to;
			}
		}
	}
			
	box_sum2(&grid)
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}

//	struct Print<'a>(&'a Grid<Space2>, (usize, usize));
//	
//	impl<'a> fmt::Display for Print<'a> {
//		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//			use fmt::Write as _;
//			let last = self.0.row_count().get() - 1;
//			for (row, contents) in self.0.as_slice().chunks_exact(self.0.row_length().get()).enumerate() {
//				for (col, &s) in enumerate(contents) {
//					let player = (row, col) == self.1;
//					f.write_char(
//						match s {
//							Space2::Wall => if player { '!' } else { '#' },
//							Space2::Empty => if player { '@' } else { '.' },
//							Space2::BoxLeft => if player { '<' } else { '[' },
//							Space2::BoxRight => if player { '>' } else { ']' }
//						}
//					)?;
//				}
//				if row < last {
//					writeln!(f)?;
//				}
//			}
//			Ok(())
//		}
//	}
