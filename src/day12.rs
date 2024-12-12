use rustc_hash::{FxHashMap as HashMap};

use std::collections::{/* HashMap, */ BTreeSet};
use std::{iter, marker::PhantomData, mem, num::NonZero, ops, ptr::NonNull, slice};

struct GridRefMutWithLn<'a, T> {
	grid_ptr: NonNull<T>,
	total_length_with_ln: NonZero<usize>,
	row_length_with_ln: NonZero<usize>,
	marker: PhantomData<&'a mut [T]>
}

impl<'a, T> GridRefMutWithLn<'a, T> {
	pub fn from_slice(grid: &'a mut [T], row_length: NonZero<usize>) -> Option<Self> {
		let row_length_with_ln = row_length.checked_add(1)?;
		let total_length_with_ln = NonZero::new(
			if grid.len() % row_length_with_ln == 0 {
				grid.len()
			} else {
				grid.len() + 1
			}
		)?;
		
		(total_length_with_ln.get() % row_length_with_ln == 0).then(||
			Self {
				grid_ptr: unsafe { NonNull::new_unchecked(grid.as_ptr() as *mut _) },
				total_length_with_ln,
				row_length_with_ln,
				marker: PhantomData
			}
		)
	}
	
	#[allow(unused)]
	pub unsafe fn from_raw_parts(grid_ptr: NonNull<T>, total_length_with_ln: NonZero<usize>, row_length_with_ln: NonZero<usize>) -> Self {
		Self { grid_ptr, total_length_with_ln, row_length_with_ln, marker: PhantomData }
	}
	
	
	pub fn as_slice(&self) -> &'a [T] { // Does not include the final possibly-uninitialized '\n'
		unsafe { slice::from_raw_parts(self.grid_ptr.as_ptr(), self.total_length_with_ln().get() - 1) }
	}
	
	pub fn as_mut_slice(&mut self) -> &'a mut [T] { // Does not include the final possibly-uninitialized '\n'
		unsafe { slice::from_raw_parts_mut(self.grid_ptr.as_ptr(), self.total_length_with_ln().get() - 1) }
	}
	
	#[allow(unused)]
	pub fn as_mut_ptr(&mut self) -> NonNull<T> {
		self.grid_ptr
	}
	
	#[allow(unused)]
	pub fn as_ptr(&self) -> NonNull<T> {
		self.grid_ptr
	}
	
	
	#[allow(unused)]
	pub fn get(&self, row: usize, col: usize) -> Option<&'a T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice().get_unchecked(i) })
	}
	
	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&'a mut T> {
		self.to_index(row, col).map(|i| unsafe { self.as_mut_slice().get_unchecked_mut(i) })
	}
	
	
	#[allow(unused)]
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &'a T {
		unsafe { self.as_slice().get_unchecked(self.to_index_unchecked(row, col)) }
	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &'a mut T {
		let index = self.to_index_unchecked(row, col);
		unsafe { self.as_mut_slice().get_unchecked_mut(index) }
	}
	
	
	pub fn to_index(&self, row: usize, col: usize) -> Option<usize> {
		(row < self.row_count().get() && col < self.row_length().get()).then(|| self.to_index_unchecked(row, col))
	}
	
	pub fn to_index_unchecked(&self, row: usize, col: usize) -> usize {
		row * self.row_length_with_ln().get() + col
	}
	
	
	#[allow(unused)]
	pub fn get_row(&self, row: usize) -> Option<&'a [T]> {
		let row_start = row.checked_mul(self.row_length_with_ln().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_slice().get(row_start..row_end)
	}
	
	#[allow(unused)]
	pub fn get_row_mut(&mut self, row: usize) -> Option<&'a mut [T]> {
		let row_start = row.checked_mul(self.row_length_with_ln().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_mut_slice().get_mut(row_start..row_end)
	}
	
//	fn rows(&self) -> Rows<T> {
//		Rows::over(*self)
//	}
	
	
	pub fn row_count(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.total_length_with_ln().get() / self.row_length_with_ln()) }
	}
	
	pub fn row_length(&self) -> NonZero<usize> {
		unsafe { NonZero::new_unchecked(self.row_length_with_ln().get() - 1) }
	}
	
	pub fn row_length_with_ln(&self) -> NonZero<usize> {
		self.row_length_with_ln
	}
	
	pub fn total_length_with_ln(&self) -> NonZero<usize> {
		self.total_length_with_ln
	}
}

const FLAG: u8 = 0b1000_0000u8;

const OFFSETS: [[isize; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn flood_fill_ascii_price(grid: &mut GridRefMutWithLn<u8>, start: (usize, usize), stack_buffer: &mut Vec<(usize, usize)>) -> u64 {
	let (ch, replacement) = match grid.get_mut(start.0, start.1) {
		Some(prev) if *prev & FLAG == 0 => {
			let replacement = *prev | FLAG;
			(mem::replace(prev, replacement), replacement)
		},
		_ => return 0
	};
	
	enum FillState {
		New((usize, usize)),
		Filled
	}
	
	let mut perimeter = 0;
	let mut area = 1;
	
	stack_buffer.clear();
	stack_buffer.push(start);
	while let Some((row, col)) = stack_buffer.pop() {
		stack_buffer.extend(
			map(OFFSETS, |[off_r, off_c]| {
				let to_r = row.checked_add_signed(off_r)?;
				let to_c = col.checked_add_signed(off_c)?;
				
				grid.get_mut(to_r, to_c).and_then(|space| {
					if *space == ch {
						*space = replacement;
						area += 1;
						Some(FillState::New((to_r, to_c)))
					} else if *space == replacement{
						Some(FillState::Filled)
					} else {
						None
					}
				})
			}).filter_map(|state| {
				if state.is_none() {
					perimeter += 1;
				}
				state.and_then(|state| if let FillState::New(to) = state { Some(to) } else { None })
			})
		)
	}
	
	perimeter * area
}


#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
	let mut bytes = Box::from(input.as_bytes());
	
	let line_length = enumerate(&bytes).find_map(|(i, &b)| (b == b'\n').then(|| i)).unwrap_or(bytes.len());
	let Some(mut grid) = NonZero::new(line_length).and_then(|line_length| GridRefMutWithLn::from_slice(&mut bytes, line_length)) else {
		unreachable!()
	};
	
	let mut stack = Vec::with_capacity(256);
	
	let mut total = 0;
	for row in 0..grid.row_count().get() {
		for col in 0..grid.row_length().get() {
			total += flood_fill_ascii_price(&mut grid, (row, col), &mut stack);
		}
	}
	
	total
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Direction {
	Right = 0, Down, Left, Up
}

impl Direction {
	pub const ALL: [Self; 4] = [Self::Right, Self::Down, Self::Left, Self::Up];
	
	pub fn offset_once(self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
		match self {
			Self::Right => Some((row, col.checked_add(1)?)),
			Self::Down =>  Some((row.checked_add(1)?, col)),
			Self::Left  => Some((row, col.checked_sub(1)?)),
			Self::Up   =>  Some((row.checked_sub(1)?, col))
		}
	}
	
	pub fn is_horizontal(self) -> bool {
		matches!(self, Self::Right | Self::Left)
	}
	
	pub fn is_vertical(self) -> bool {
		!self.is_horizontal()
	}
}

#[derive(Copy, Default, Eq, Clone, PartialEq)]
struct DirMap<T> {
	pub map: [T; 4]
}

impl<T> From<[T; 4]> for DirMap<T> {
	fn from(map: [T; 4]) -> Self {
		Self { map }
	}
}

impl<T> ops::Index<Direction> for DirMap<T> {
	type Output = T;
	
	fn index(&self, i: Direction) -> &T {
		unsafe { self.map.get_unchecked(i as usize) }
	}
}

impl<T> ops::IndexMut<Direction> for DirMap<T> {
	fn index_mut(&mut self, i: Direction) -> &mut T {
		unsafe { self.map.get_unchecked_mut(i as usize) }
	}
}

fn add_fence(fence_tracker: &mut DirMap<HashMap<usize, BTreeSet<usize>>>, from: (usize, usize), dir: Direction) -> bool {
	let (row, col) = from;
	let (outer, inner) = if dir.is_vertical() { (row, col) } else { (col, row) };
	
	fence_tracker[dir].entry(outer).or_default().insert(inner)
}

fn flood_fill_ascii_bulk_price(grid: &mut GridRefMutWithLn<u8>, start: (usize, usize),
		stack_buffer: &mut Vec<(usize, usize)>, fence_buffer: &mut DirMap<HashMap<usize, BTreeSet<usize>>>) -> u64 {
	let (ch, replacement) = match grid.get_mut(start.0, start.1) {
		Some(prev) if *prev & FLAG == 0 => {
			let replacement = *prev | FLAG;
			(mem::replace(prev, replacement), replacement)
		},
		_ => return 0
	};
	
	enum FillState {
		Expand((usize, usize)),
		Filled,
		Fence(Direction)
	}
	
	let mut area = 1;
	
	stack_buffer.push(start);
	while let Some((row, col)) = stack_buffer.pop() {
		stack_buffer.extend(
			map(Direction::ALL, |dir| {
				dir.offset_once((row, col)).and_then(|to|
					grid.get_mut(to.0, to.1).and_then(|space|
						if *space == ch {
							*space = replacement;
							area += 1;
							Some(FillState::Expand(to))
						} else if *space == replacement{
							Some(FillState::Filled)
						} else {
							None
						}
					)
				).unwrap_or_else(|| FillState::Fence(dir))
			}).filter_map(|state| match state {
				FillState::Expand(to) => Some(to),
				FillState::Fence(dir) => { add_fence(fence_buffer, (row, col), dir); None },
				FillState::Filled => None
			})
		);
	}
	
	// eprintln!("[i] For '{}' — Up: {:#?}", ch.escape_ascii(), fence_buffer[Direction::Up]);
	
	let sides: u64 = flat_map(&mut fence_buffer.map, |map| map.drain()).map(|(_, set)| {
		let mut count = 0;
		let mut prev = usize::MAX - 1;
		for next in set {
			if next != prev + 1 {
				count += 1;
			}
			prev = next;
		}
		count
	}).sum();
	
	sides * area
}


#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
	let mut bytes = Box::from(input.as_bytes());
	
	let line_length = enumerate(&bytes).find_map(|(i, &b)| (b == b'\n').then(|| i)).unwrap_or(bytes.len());
	let Some(mut grid) = NonZero::new(line_length).and_then(|line_length| GridRefMutWithLn::from_slice(&mut bytes, line_length)) else {
		unreachable!()
	};
	
	let mut stack = Vec::with_capacity(256);
	let mut fence_tracker = DirMap::default();
	
	let mut total = 0;
	for row in 0..grid.row_count().get() {
		for col in 0..grid.row_length().get() {
			total += flood_fill_ascii_bulk_price(&mut grid, (row, col), &mut stack, &mut fence_tracker);
		}
	}
	
	total
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}

fn flat_map<I: IntoIterator, O: IntoIterator, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::FlatMap<I::IntoIter, O, F> {
	i.into_iter().flat_map(f)
}

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::Map<I::IntoIter, F> {
	i.into_iter().map(f)
}
