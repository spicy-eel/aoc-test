use std::num::NonZero;
use std::{marker::PhantomData, ptr::NonNull, slice};

#[derive(Copy, Clone)]
struct GridRefWithLn<'a, T> {
	grid_ptr: NonNull<T>,
	total_length_with_ln: NonZero<usize>,
	row_length_with_ln: NonZero<usize>,
	marker: PhantomData<&'a [T]>
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
	
//	pub unsafe fn from_slice_unchecked(grid: &'a [T], row_length: NonZero<usize>) -> Self {
//		Self { grid, row_length }
//	}
//	
//	pub unsafe fn as_vec_mut(&mut self) -> &mut Vec<T> {
//		&mut self.grid
//	}
	
	
	pub fn as_slice(&self) -> &'a [T] { // Does not include the final possibly-uninitialized '\n'
		unsafe { slice::from_raw_parts(self.grid_ptr.as_ptr(), self.total_length_with_ln().get() - 1) }
	}
	
//	pub fn slice_mut(&mut self) -> &mut [T] {
//		self.grid
//	}
	
	
	#[allow(unused)]
	pub fn get(&self, row: usize, col: usize) -> Option<&'a T> {
		self.to_index(row, col).map(|i| unsafe { self.as_slice().get_unchecked(i) })
	}
	
//	#[allow(unused)]
//	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
//		self.to_index(row, col).map(|i| unsafe { self.slice_mut().get_unchecked_mut(i) })
//	}
	
	#[allow(unused)]
	pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &'a T {
		unsafe { self.as_slice().get_unchecked(self.to_index_unchecked(row, col)) }
	}
	
//	#[allow(unused)]
//	pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
//		let index = self.to_index_unchecked(row, col);
//		self.slice_mut().get_unchecked_mut(index)
//	}
	
	
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
	
//	fn rows(&self) -> Rows<T> {
//		Rows::over(self)
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
		self.total_length_with_ln // unsafe { NonZero::new_unchecked(self.slice().len()) }
	}
}

 // - Part One - //

fn check_matching_from(search: GridRefWithLn<u8>, from: (usize, usize), offset: (isize, isize), word_remainder: &[u8]) -> bool {
//	let Some((&letter, rest)) = word_remainder.split_first() else {
//		return true;
//	};
	let (from_row, from_col) = from;
	let (off_row, off_col) = offset;
	
	let offset_mult = word_remainder.len() as isize;
	if from_row.checked_add_signed(off_row * offset_mult).is_none_or(|r| r >= search.row_count().get()) ||
			from_col.checked_add_signed(off_col * offset_mult).is_none_or(|c| c >= search.row_length().get()/* - 1 */) {
		return false; // Check and exit early if the word would go off the grid.
	}
	
	if let Some((&letter, rest)) = word_remainder.split_first() {
		unsafe { check_matching_inner(search, from, offset, letter, rest) }
	} else {
		true
	}
//	let Some((row, col, at)) = from_row.checked_add_signed(off_row)
//			.and_then(|row| (row, from_col.checked_add_signed(off_col)?).into())
//			.and_then(|(r, c)| (r, c, search.get(r, c).copied()?).into()) else {
//		return false;
//	};
//	
//	at == letter && check_matching_from_with_ln(search, (row, col), offset, rest)
}

unsafe fn check_matching_inner(search: GridRefWithLn<u8>, from: (usize, usize), offset: (isize, isize), letter: u8, word_remainder: &[u8]) -> bool {
	let (from_row, from_col) = from;
	let (off_row, off_col) = offset;
	
	let (row, col) = (from_row.wrapping_add_signed(off_row), from_col.wrapping_add_signed(off_col));
	
	unsafe {
		*search.get_unchecked(row, col) == letter && if let Some((&next, rest)) = word_remainder.split_first() {
			check_matching_inner(search, (row, col), offset, next, rest)
		} else {
			true
		}
	}
}

const DIRECTIONS: [(isize, isize); 8] = [
	( -1, -1 ), ( -1,  0 ), ( -1,  1 ),
	(  0, -1 ),             (  0,  1 ),
	(  1, -1 ), (  1,  0 ), (  1,  1 )
];

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 { // This -relies- used to rely on all lines, including the last, being '\n'-terminated. (still relies on no "\r\n")
	// println!("{}", input.len());
	// let grid = GridRefWithLn::from_slice(input.as_bytes(), NonZero::new(input.find('\n').unwrap()/* + 1 */).unwrap()).unwrap();
	let grid = unsafe { GridRefWithLn::from_slice(input.as_bytes(), NonZero::new_unchecked(input.find('\n').unwrap_unchecked())).unwrap_unchecked() };
	
	let mut count = 0u32;
	for row in 0..grid.row_count().get() {
		for col in 0..grid.row_length().get() {
			if *unsafe { grid.get_unchecked(row, col) } == b'X' {
				for dir in DIRECTIONS {
					if check_matching_from(grid, (row, col), dir, b"MAS") {
						count += 1;
					}
				}
			}
		}
	}
			
	count
}

 // - Part Two - //

#[allow(unused)]
fn check_mas(search: GridRefWithLn<u8>, center: (usize, usize)) -> bool {
	assert!(0 < center.0 && center.0 < search.row_count().get() - 1);
	assert!(0 < center.1 && center.1 < search.row_length().get() - 1 /* 2 */);
	
	unsafe { check_mas_unchecked(search, center) }
//	let (row, col) = center;
//	
//	if *search.get(row, col).unwrap() != b'A' {
//		return false;
//	}
//	
//	let backslash = (*search.get(row - 1, col - 1).unwrap(), *search.get(row + 1, col + 1).unwrap());
//	let slash = (*search.get(row - 1, col + 1).unwrap(), *search.get(row + 1, col - 1).unwrap());
//	
//	matches!(backslash, (b'M', b'S') | (b'S', b'M')) && matches!(slash, (b'M', b'S') | (b'S', b'M'))
}

unsafe fn check_mas_unchecked(search: GridRefWithLn<u8>, center: (usize, usize)) -> bool {
	let (row, col) = center;
	
	if unsafe { *search.get_unchecked(row, col) != b'A' } {
		return false;
	}
	
	let (backslash, slash) = unsafe { (
		(*search.get_unchecked(row - 1, col - 1), *search.get_unchecked(row + 1, col + 1)),
		(*search.get_unchecked(row - 1, col + 1), *search.get_unchecked(row + 1, col - 1))
	) };
	
	matches!(backslash, (b'M', b'S') | (b'S', b'M')) && matches!(slash, (b'M', b'S') | (b'S', b'M'))
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
	// let grid = GridRefWithLn::from_slice(input.as_bytes(), NonZero::new(input.find('\n').unwrap()/* + 1 */).unwrap()).unwrap();
	let grid = unsafe { GridRefWithLn::from_slice(input.as_bytes(), NonZero::new_unchecked(input.find('\n').unwrap_unchecked())).unwrap_unchecked() };

	let mut count = 0;
	for row in 1..(grid.row_count().get() - 1) {
		for col in 1..(grid.row_length().get() - 1 /* 2 */) {
			count += unsafe { check_mas_unchecked(grid, (row, col)) } as u32;
		}
	}
	
	count
}
