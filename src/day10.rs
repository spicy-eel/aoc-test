use std::{iter, marker::PhantomData, num::NonZero, ptr::NonNull, slice};

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
	
	pub unsafe fn from_raw_parts(grid_ptr: NonNull<T>, total_length_with_ln: NonZero<usize>, row_length_with_ln: NonZero<usize>) -> Self {
		Self { grid_ptr, total_length_with_ln, row_length_with_ln, marker: PhantomData }
	}
	
	
	pub fn as_slice(&self) -> &'a [T] { // Does not include the final possibly-uninitialized '\n'
		unsafe { slice::from_raw_parts(self.grid_ptr.as_ptr(), self.total_length_with_ln().get() - 1) }
	}
	
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
	pub fn get_row(&self, row: usize) -> Option<&[T]> {
		let row_start = row.checked_mul(self.row_length_with_ln().get())?;
		let row_end = row_start + self.row_length().get();
		self.as_slice().get(row_start..row_end)
	}
	
	fn rows(&self) -> Rows<T> {
		Rows::over(*self)
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

struct Rows<'a, T> {
	src: Option<GridRefWithLn<'a, T>>,
}

impl<'a, T> Clone for Rows<'a, T> {
	fn clone(&self) -> Self {
		Self { src: self.as_grid_ref() }
	}
}

impl<'a, T> Rows<'a, T> {
	fn over(src: GridRefWithLn<'a, T>) -> Self {
		Self { src: src.into() }
	}
	
	fn as_grid_ref(&self) -> Option<GridRefWithLn<'a, T>> {
		self.src
	}
	
	#[allow(unused)]
	fn empty() -> Self {
		Self { src: None }
	}
}

impl<'a, T> Iterator for Rows<'a, T> {
	type Item = &'a [T];
	
	fn next(&mut self) -> Option<Self::Item> {
		self.src.map(|src| {
			let slice = unsafe { src.as_slice().get_unchecked(..src.row_length().get()) };
			
			let (ptr, total_ln, row_ln) = (src.as_ptr(), src.total_length_with_ln(), src.row_length_with_ln());
			
			self.src = NonZero::new(unsafe { total_ln.get().unchecked_sub(row_ln.get()) }).map(|new_total_ln| unsafe {
				let new_ptr = ptr.add(row_ln.get());
				GridRefWithLn::from_raw_parts(new_ptr, new_total_ln, row_ln)
			});
			
			slice
		})
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, len.into())
	}
}

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.src.map(|src| {
			let (ptr, total_ln, row_ln) = (src.as_ptr(), src.total_length_with_ln(), src.row_length_with_ln());
			
			let start = unsafe { total_ln.get().unchecked_sub(row_ln.get()) };
			let end = start + src.row_length().get();
			
			let slice = unsafe { src.as_slice().get_unchecked(start..end) };
			
			self.src = NonZero::new(start).map(|new_total_ln| unsafe {
				GridRefWithLn::from_raw_parts(ptr, new_total_ln, row_ln)
			});
			
			slice
		})
	}
}

impl<'a, T> ExactSizeIterator for Rows<'a, T> {
	fn len(&self) -> usize {
		self.src.map(|src| src.total_length_with_ln().get() / src.row_length_with_ln()).unwrap_or(0)
	}
}

// --------------v-
//>99.....989...989  0 -8
// ......98789.....  1 -7
// .....9876789....  2 -6
// ....987656789...  3 -5
// ...98765456789..  4 -4
// ..9876543456789.  5 -3
// .987654322345678  6 -2
// 9876543212345678> 7 -1
// 8765432112345678  8  0
// 9876454212345678> 9 +1
// .987654323456789 10 +2
// ..9876543456789. 11 +3
// ...98765456789.. 12 +4
// ....987656789... 13 +5
// .....9876789.... 14 +6
// ......98789..... 15 +7
//         v
fn mark_visited(visited: &mut [u16; 16], offset: (isize, isize)) -> bool {
	// The outermost reachable '9's don't need to be tracked since there's only one path that reaches them.
	// Offset (0, 0) also doesn't need to be tracked as it will always have been already visited.
	debug_assert!(offset.0.unsigned_abs().saturating_add(offset.1.unsigned_abs()) <= 9);
	let (row, col_bit) = match offset {
		(0, 0) => return false,
		(-1, 8) => (&mut visited[0], 1 << 15),
		( 1, 8) => (&mut visited[0], 1 << 14),
		( 8, col_off @ -1..1) => {
			let shift = (col_off + 1) as u32;
			(&mut visited[0], 1 << shift)
		},
		(row_off, col_off) => {
			let Some(row) = usize::try_from(row_off + 8).ok().and_then(|i| visited.get_mut(i)) else {
				return true;
			};
			
			let shift = if row_off == 0 && col_off > 0 {
				col_off - 1
			} else {
				col_off
			} + 8;
			
			let Some(bit) = u32::try_from(shift).ok().and_then(|shift| 1u16.checked_shl(shift)) else {
				return true;
			};
			
			(row, bit)
		}
	};
	
	let vacant = *row & col_bit == 0;
	
	if vacant {
		*row |= col_bit;
	}
	
	vacant
}

fn reachable_nines(grid: &GridRefWithLn<u8>, at: (usize, usize), at_char: u8, visited: &mut [u16; 16], offset: (isize, isize)) -> u32 {
	if at_char >= b'9' {
		1
	} else {
		let next_char = at_char + 1;
		let mut total = 0;
		
		for extra_offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
			if let Some((row, col)) = Option::zip(at.0.checked_add_signed(extra_offset.0), at.1.checked_add_signed(extra_offset.1)) {
				let new_offset = (offset.0 + extra_offset.0, offset.1 + extra_offset.1);
				
				if grid.get(row, col).is_some_and(|&b| b == next_char) && mark_visited(visited, new_offset) {
					total += reachable_nines(grid, (row, col), next_char, visited, new_offset);
				}
			}
		}
		
		total
	}
}

fn trailhead_score(grid: &GridRefWithLn<u8>, row: usize, col: usize) -> u32 {
	reachable_nines(grid, (row, col), b'0', &mut [0u16; 16], (0, 0))
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
	let line_length = input.find('\n').unwrap_or(input.len());
	let Some(grid) = NonZero::new(line_length).and_then(|line_length| GridRefWithLn::from_slice(input.as_bytes(), line_length)) else {
		unreachable!();
	};
	
	let mut total = 0u32;
	for (row, contents) in grid.rows().enumerate() {
		for col in enumerate(contents).filter_map(|(i, &b)| (b == b'0').then(|| i)) {
			total += trailhead_score(&grid, row, col);
		}
	}
			
	total
}

fn paths_to_nine(grid: &GridRefWithLn<u8>, at: (usize, usize), at_char: u8) -> u32 {
	if at_char >= b'9' {
		1
	} else {
		let next_char = at_char + 1;
		let mut total = 0;
		
		for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
			if let Some(to) = Option::zip(at.0.checked_add_signed(offset.0), at.1.checked_add_signed(offset.1)) {
				if grid.get(to.0, to.1).is_some_and(|&b| b == next_char) {
					total += paths_to_nine(grid, to, next_char);
				}
			}
		}
		
		total
	}
}

fn trailhead_rating(grid: &GridRefWithLn<u8>, row: usize, col: usize) -> u32 {
	paths_to_nine(grid, (row, col), b'0')
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u32 {
	let line_length = input.find('\n').unwrap_or(input.len());
	let Some(grid) = NonZero::new(line_length).and_then(|line_length| GridRefWithLn::from_slice(input.as_bytes(), line_length)) else {
		unreachable!();
	};
	
	let mut total = 0u32;
	for (row, contents) in grid.rows().enumerate() {
		for col in enumerate(contents).filter_map(|(i, &b)| (b == b'0').then(|| i)) {
			total += trailhead_rating(&grid, row, col);
		}
	}
			
	total
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}
