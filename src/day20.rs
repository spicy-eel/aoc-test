use std::{iter, marker::PhantomData, num::NonZero, ptr::NonNull, slice};

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
			self.as_slice_mut().get_unchecked_mut(prev_len..)
		}
	}
	
	
	pub fn as_slice(&self) -> &[T] {
		&self.grid
	}
	
	pub fn as_slice_mut(&mut self) -> &mut [T] {
		&mut self.grid
	}
	
	
	pub fn get(&self, row: usize, col: usize) -> Option<&T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice().get_unchecked(i) })
	}
	
	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice_mut().get_unchecked_mut(i) })
	}
	
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
		self.as_slice().get_unchecked(self.to_index_unchecked(row, col))
	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
		let index = self.to_index_unchecked(row, col);
		self.as_slice_mut().get_unchecked_mut(index)
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
	
	#[allow(unused)]
	pub fn to_row_col_unchecked(&self, index: usize) -> (usize, usize) {
		(index / self.row_length(), index % self.row_length())
	}
	
	
	#[allow(unused)]
	pub fn get_row(&self, row: usize) -> Option<&[T]> {
		let row_start = row.checked_mul(self.row_length().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_slice().get(row_start..row_end)
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

const WALL: u32 = 0;
const EMPTY: u32 = u32::MAX;

fn count_cheats(grid: &mut Grid<u32>, goal: (usize, usize), minimum_skip: u32) -> Result<u32, ((usize, usize), u32)> {
	let Some(end) = grid.get_mut(goal.0, goal.1) else {
		return Err((goal, 0));
	};
	
	*end = 1;
	let mut next = Some(goal);
	
	let mut count = 0;
	while let Some((row, col)) = next.take() {
		let moves = *unsafe { grid.get_unchecked(row, col) };
		for (offset_r, offset_c) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let to = (row.wrapping_add_signed(offset_r), col.wrapping_add_signed(offset_c));
			if let Some(slot) = grid.get_mut(to.0, to.1) {
				if *slot == EMPTY {
					*slot = moves + 1;
					if next.replace(to).is_some() {
						return Err(((row, col), count));
					}
				} else if *slot == WALL {
					let skip_to = (row.wrapping_add_signed(offset_r * 2), col.wrapping_add_signed(offset_c * 2));
					if let Some(&skip_to) = grid.get(skip_to.0, skip_to.1).filter(|&m| !matches!(*m, EMPTY | WALL)) {
						if moves.checked_sub(skip_to + 2).is_some_and(|difference| difference >= minimum_skip) {
							count += 1;
						}
					}
				}
			}
		}
	}
	
	Ok(count)
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
	let data = unsafe { as_byte_grid(input).unwrap_unchecked() };
	let (width, height, width_ln) = (data.row_length().get(), data.row_count().get(), data.row_length_with_ln().get());
	
	let mut grid = vec![WALL; width * height];
	let mut end = None;
	for (i, (slot, &c)) in iter::zip(&mut grid, input.as_bytes().chunks(width_ln).flat_map(|r| unsafe { r.get_unchecked(..width) })).enumerate() {
		if c != b'#' {
			*slot = EMPTY;
			if c == b'E' {
				if end.replace((i / data.row_length(), i % data.row_length())).is_some() {
					unsafe { std::hint::unreachable_unchecked(); }
				}
			}
		}
	}
	
	unsafe { count_cheats(&mut Grid::from_vec_unchecked(grid, data.row_length()), end.unwrap_unchecked(), 100).unwrap_unchecked() }
}

struct GridRefWithLn<'a, T> {
	grid_ptr: NonNull<T>,
	total_length_with_ln: NonZero<usize>,
	row_length_with_ln: NonZero<usize>,
	marker: PhantomData<&'a [T]>
}

impl<'a, T> Copy for GridRefWithLn<'a, T> {}

impl<'a, T> Clone for GridRefWithLn<'a, T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, T> GridRefWithLn<'a, T> {
	pub fn from_slice(grid: &'a [T], row_length: NonZero<usize>) -> Option<Self> {
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
	
	#[allow(unused)]
	pub fn as_ptr(&self) -> NonNull<T> {
		self.grid_ptr
	}
	
	
	#[allow(unused)]
	pub fn get(&self, row: usize, col: usize) -> Option<&'a T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice().get_unchecked(i) })
	}
	
	
	#[allow(unused)]
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &'a T {
		unsafe { self.as_slice().get_unchecked(self.to_index_unchecked(row, col)) }
	}
	
	
	pub fn to_index(&self, row: usize, col: usize) -> Option<usize> {
		(row < self.row_count().get() && col < self.row_length().get()).then(|| self.to_index_unchecked(row, col))
	}
	
	pub fn to_index_unchecked(&self, row: usize, col: usize) -> usize {
		row * self.row_length_with_ln().get() + col
	}
	
	#[allow(unused)]
	pub fn to_row_col(&self, index: usize) -> Option<(usize, usize)> {
		(index < self.total_length_with_ln().get()).then(|| self.to_row_col_unchecked(index)).filter(|&(_, c)| c < self.row_length().get())
	}
	
	pub fn to_row_col_unchecked(&self, index: usize) -> (usize, usize) {
		(index / self.row_length_with_ln(), index % self.row_length_with_ln())
	}
	
	
	#[allow(unused)]
	pub fn get_row(&self, row: usize) -> Option<&[T]> {
		let row_start = row.checked_mul(self.row_length_with_ln().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_slice().get(row_start..row_end)
	}
	
	
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
		self.total_length_with_ln // unsafe { NonZero::new_unchecked(self.slice().len()) }
	}
}

fn as_byte_grid(input: &str) -> Option<GridRefWithLn<u8>> {
	let row_length = input.find('\n').unwrap_or(input.len());
	NonZero::new(row_length).and_then(|row_length| GridRefWithLn::from_slice(input.as_bytes(), row_length))
}

fn count_cheats_dx<T, F: FnMut(&T) -> bool>(grid: GridRefWithLn<T>, goal: (usize, usize), minimum_skip: usize, cheat_duration: usize,
		path_buffer: &mut Vec<usize>, mut is_passable: F) -> Result<usize, ((usize, usize), usize)> {
	debug_assert!(grid.to_index(goal.0, goal.1).is_some());
	path_buffer.clear();
	
	let mut current = goal;
	let mut count = 0;
	loop {
		let (row, col) = current;
		let moves = path_buffer.len();
		count += enumerate(&path_buffer[..moves.saturating_sub(minimum_skip).saturating_sub(1)]).filter(
			|(prev_moves, &prev_at)| {
				let (prev_row, prev_col) = grid.to_row_col_unchecked(prev_at);
				let moved = moves - prev_moves;
				let distance = row.abs_diff(prev_row) + col.abs_diff(prev_col);
				distance <= cheat_duration && moved - distance >= minimum_skip
			}
		).count();
		
		let from = path_buffer.last().map(|&from| grid.to_row_col_unchecked(from));
		let mut next = None;
		for (offset_r, offset_c) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let to = (row.wrapping_add_signed(offset_r), col.wrapping_add_signed(offset_c));
			if from.is_none_or(|from| to != from) && grid.get(to.0, to.1).is_some_and(&mut is_passable) {
				if next.replace(to).is_some() {
					return Err(((row, col), count));
				}
			}
		}
		
		if let Some(next) = next {
			path_buffer.push(grid.to_index_unchecked(row, col));
			current = next;
		} else {
			break Ok(count);
		}
	}
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
	let grid = unsafe { as_byte_grid(input).unwrap_unchecked() };
	
	let end = unsafe { input.find('E').map(|i| (i / grid.row_length_with_ln(), i % grid.row_length_with_ln())).unwrap_unchecked() };
	
	unsafe { count_cheats_dx(grid, end, 100, 20, &mut Vec::with_capacity(10_000), |&b| b != b'#').unwrap_unchecked() }
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}
