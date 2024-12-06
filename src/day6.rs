// #![feature(ascii_char, new_zeroed_alloc)]
use std::{ascii, collections::HashSet, mem, num::NonZero};

#[derive(Copy, Eq, Clone, PartialEq)]
enum Space {
	Empty = 0,
	Wall,
	Visited
}

#[derive(Copy, Eq, Hash, Clone, PartialEq)]
enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {
	fn turn_right(&mut self) {
		*self = self.turned_right();
	}
	
	fn turned_right(self) -> Self {
		match self {
			Self::North => Self::East,
			Self::East => Self::South,
			Self::South => Self::West,
			Self::West => Self::North
		}
	}
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
			self.slice_mut().get_unchecked_mut(prev_len..)
		}
	}
	
	
	pub fn slice(&self) -> &[T] {
		&self.grid
	}
	
	pub fn slice_mut(&mut self) -> &mut [T] {
		&mut self.grid
	}
	
	
	#[allow(unused)]
	pub fn get(&self, row: usize, col: usize) -> Option<&T> {
		self.to_index(row, col).map(|i| unsafe { self.slice().get_unchecked(i) })
	}
	
	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
		self.to_index(row, col).map(|i| unsafe { self.slice_mut().get_unchecked_mut(i) })
	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
		self.slice().get_unchecked(self.to_index_unchecked(row, col))
	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
		let index = self.to_index_unchecked(row, col);
		self.slice_mut().get_unchecked_mut(index)
	}
	
	
	pub fn to_index(&self, row: usize, col: usize) -> Option<usize> {
		(row < self.row_count().get() && col < self.row_length().get()).then(|| self.to_index_unchecked(row, col))
	}
	
	pub fn to_index_unchecked(&self, row: usize, col: usize) -> usize {
		row * self.row_length().get() + col
	}
	
	
	#[allow(unused)]
	pub fn get_row(&self, row: usize) -> Option<&[T]> {
		let row_start = row.checked_mul(self.row_length().get())?;
		let row_end = row_start + self.row_length().get();
		self.slice().get(row_start..row_end)
	}
	
	
	pub fn row_count(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.total_length().get() / self.row_length()) }
	}
	
	pub fn row_length(&self) -> NonZero<usize> {
		self.row_length
	}
	
	pub fn total_length(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.slice().len()) }
	}
}

fn parse_to_grid(s: &[ascii::Char]) -> Option<(Grid<Space>, Option<(usize, usize)>)> {
	let row_length = NonZero::new(s.as_str().find('\n').unwrap_or(s.len()))?; // s.as_str().find('\n').and_then(NonZero::new)?;
	let row_length_with_ln = row_length.checked_add(1)?;
	let total_length = if (s.len() + 1) % row_length_with_ln == 0 {
		s.len() + 1 - (s.len() + 1) / row_length_with_ln
	} else if s.len() % row_length_with_ln == 0 {
		s.len() - s.len() / row_length_with_ln
	} else {
		return None
	};
	
	let mut grid = unsafe {
		let vec: Vec<Space> = Box::new_zeroed_slice(total_length).assume_init().into();
		Grid::from_vec_unchecked(vec, row_length)
	};
	
	let mut start = None;
	
	for (row, contents) in s.chunks(row_length_with_ln.get()).map(|r| unsafe { r.get_unchecked(..row_length.get()) }.as_str()).enumerate() {
		if start.is_none() {
			if let Some(col) = contents.find('^') {
				start = Some((row, col));
			}
		}
			
		for (col, _) in contents.match_indices('#') {
			*unsafe { grid.get_unchecked_mut(row, col) } = Space::Wall;
		}
	}
	
	Some((grid, start))
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

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
	// let (mut grid, start) = parse_to_grid(input.as_ascii().unwrap()).unwrap();
	let (mut grid, start) = unsafe { parse_to_grid(input.as_bytes().as_ascii_unchecked()).unwrap_unchecked() };
	
	let mut position = unsafe { start.unwrap_unchecked() }; // start.unwrap();
	let mut direction = Direction::North;
	
	*unsafe { grid.get_unchecked_mut(position.0, position.1) } = Space::Visited;
	let mut visited = 1u32;
	
	loop {
		let to = wrapping_offset(position, direction);
		match grid.get_mut(to.0, to.1) {
			Some(&mut Space::Wall) => direction.turn_right(),
			Some(free) => {
				if mem::replace(free, Space::Visited) == Space::Empty {
					visited += 1;
				}
				
				position = to;
			},
			None => break
		}
	}
			
	visited
}

fn check_loop_with_wall(grid: &Grid<Space>, wall_at: (usize, usize), from: (usize, usize), from_dir: Direction,
		outer_wall_hits: &HashSet<(usize, usize)>, inner_wall_hits_buffer: &mut HashSet<(usize, usize)>) -> bool {
	inner_wall_hits_buffer.clear();
	inner_wall_hits_buffer.insert(from); // inner_wall_hits_buffer.insert((wall_at, from_dir));
	
	let mut position = from;
	let mut direction = from_dir.turned_right();
	
	let mut prev_wall_hit = from;
	let mut same_hit_streak = 0u8;
	loop {
		let to = wrapping_offset(position, direction);
		match (to == wall_at).then(|| &Space::Wall).or_else(|| grid.get(to.0, to.1)) {
			Some(&Space::Wall) => {
				if position == prev_wall_hit {
					same_hit_streak += 1;       // Probably pointless in practice,      ..X..
					                           // but this should avoid an infinite  :  .#^#.
					if same_hit_streak >= 4 { // loop with the following arrangement    ..#..
						break true;
					}
				} else if outer_wall_hits.contains(&position) || !inner_wall_hits_buffer.insert(position) {
					break true;
				} else {
					same_hit_streak = 0;
				}
				prev_wall_hit = position;
				direction.turn_right();
			},
			Some(_) => position = to,
			None => break false
		}
	}
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
	// let (mut grid, start) = parse_to_grid(input.as_ascii().unwrap()).unwrap();
	let (mut grid, start) = unsafe { parse_to_grid(input.as_bytes().as_ascii_unchecked()).unwrap_unchecked() };
	
	let mut position = unsafe { start.unwrap_unchecked() }; // start.unwrap();
	let mut direction = Direction::North;
	
	*unsafe { grid.get_unchecked_mut(position.0, position.1) } = Space::Visited;
	let mut hit_walls_at = HashSet::with_capacity(64);
	let mut hit_walls_inner_buffer = HashSet::with_capacity(64);
	
	let mut possibilities = 0;
	loop {
		let to = wrapping_offset(position, direction);
		match grid.get_mut(to.0, to.1) {
			Some(&mut Space::Wall) => {
			//	if !hit_walls.insert((to, direction)) {
			//		eprintln!("[!] Loop encountered without adding any blocks.");
			//		break;
			//	}
				hit_walls_at.insert(position);
				direction.turn_right();
			},
			Some(empty @ &mut Space::Empty) => {
				*empty = Space::Visited;
				possibilities += check_loop_with_wall(&grid, to, position, direction, &hit_walls_at, &mut hit_walls_inner_buffer) as u32;
				
				position = to;
			},
			Some(&mut Space::Visited) => position = to,
			None => break
		}
	}
			
	possibilities
}
