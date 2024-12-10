use std::{iter, num::NonZero};

#[derive(Copy, Eq, Clone, PartialEq)]
enum Kind {
	Free,
	File{ id: u16 }
}

#[derive(Copy, Eq, Clone, PartialEq)]
struct FileChunk {
	kind: Kind,
	size: NonZero<u8>
}

//	fn append_chunk(to: &mut Vec<FileChunk>, new: FileChunk) {
//		let extra = if let Some(last) = to.last_mut().filter(|c| c.kind == new.kind) {
//			let (total_size, excess) = match new.size.checked_add(new.size.get()) {
//				Some(sum) => (sum, None),
//				None => (<NonZero<u8>>::MAX, NonZero::new(last.size.get().wrapping_add(new.size.get()).wrapping_sub(u8::MAX)))
//			};                                // last + new - MAX = new - (MAX - last)
//			
//			last.size = total_size;
//			excess.map(|size| FileChunk { size, ..new })
//		} else {
//			Some(new)
//		};
//		
//		to.extend(extra);
//	}

// Sum of N1..N2
// Sum of 0..N2 - Sum of 0..N1
// N2(N2 - 1) / 2 - N(N - 1) / 2
// (N2(N2 - 1) - N(N - 1)) / 2
// (N2^2 - N2 - (N^2 - N)) / 2

fn sum_between(start: u32, end: u32) -> u64 {
	let (start, end) = (start as u64, end as u64);
	
	((end * end - end) - (start * start - start)) / 2
}

#[derive(Copy, Eq, Clone, PartialEq)]
struct Occupied {
	id: u16,
	size: NonZero<u8>
}

impl Occupied {
	fn from_chunk(FileChunk{ kind, size }: FileChunk) -> Option<Self> {
		if let Kind::File{ id } = kind {
			Some(Self { id, size })
		} else {
			None
		}
	}
}

impl From<Occupied> for FileChunk {
	fn from(Occupied{ id, size }: Occupied) -> Self {
		Self { kind: Kind::File{ id }, size }
	}
}

fn file_advance(position: u32, file: Occupied) -> (u32, u64) {
	let next_position = position + file.size.get() as u32;
	(next_position, file.id as u64 * sum_between(position, next_position))
}

enum FillResult {
	Over{ excess: Occupied },
	Exact,
	Under{ remaining: NonZero<u8> }
}

fn try_fill(capacity: NonZero<u8>, file: Occupied) -> (Occupied, FillResult) {
	let Occupied { id, size } = file;
	
	use std::cmp::Ordering;
	match size.cmp(&capacity) {
		Ordering::Greater => (Occupied{ id, size: capacity }, FillResult::Over{ excess: Occupied{ id, size: NonZero::new(size.get() - capacity.get()).unwrap() } }),
		Ordering::Equal => (file, FillResult::Exact),
		Ordering::Less => (file, FillResult::Under{ remaining: NonZero::new(capacity.get() - size.get()).unwrap() })
	}
}

fn kind_from_index(index: usize) -> Kind {
	if index & 1 == 1 {
		Kind::Free
	} else {
		Kind::File{ id: ((index) / 2) as u16 }
	}
}

fn chunk_from_byte(index: usize, byte: u8) -> Option<FileChunk> {
	// if byte == b'0' && index & 1 == 0 { eprintln!("[!] Zero-size file at index {index}."); }
	NonZero::new(byte.wrapping_sub(b'0')).map(|size| FileChunk{ kind: kind_from_index(index), size })
}

fn compacted_checksum_bytes(files: &[u8]) -> u64 {
	let mut files = enumerate(files).filter_map(|(i, &b)| chunk_from_byte(i, b));
	let mut total = 0;
	
	let mut position = 0u32;
	let mut insertion_remainder = None;
	while let Some(FileChunk{ kind, size }) = files.next() {
		match kind {
			Kind::File{ id } => {
				let (next_position, add) = file_advance(position, Occupied{ id, size });
				total += add;
				position = next_position;
			},
			Kind::Free => {
				let mut size_left = size;
				while let Some(file) = insertion_remainder.take().or_else(|| files.by_ref().rev().find_map(Occupied::from_chunk)) {
					let (filled, result) = try_fill(size_left, file);
					
					let (next_position, add) = file_advance(position, filled);
					total += add;
					position = next_position;
					
					size_left = match result {
						FillResult::Over{ excess } => {
							insertion_remainder = Some(excess);
							break;
						}
						FillResult::Exact => { break; },
						FillResult::Under{ remaining } => remaining
					}
				}
			}
		}
	}
	
	if let Some(file) = insertion_remainder {
		total += file_advance(position, file).1;
	}
	
	total
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
	compacted_checksum_bytes(input.trim().as_bytes())
}

fn compact(files: &mut Vec<FileChunk>) {
	// This implementation assumes that the input vector does not contain consecutive 'free' chunks, but can leave it in a state where it does.
	// Luckily, the input doesn't seem to contain any 0-size files, and this only needs to run once, so it shouldn't actually matter in practice.
	if files.is_empty() { return; }
	let Some(mut index) = files.len().checked_sub(1) else { return };
	let Some(mut first_free) = enumerate(&*files).find_map(|(i, c)| matches!(c.kind, Kind::Free).then(|| i)) else { return };
	
	while index > first_free {
		let FileChunk{ kind, size } = files[index];
		
		match kind {
			Kind::File{ id } => {
				let file = Occupied{ id, size };
				let mut found_free = false;
				for (i, chunk) in take(&mut *files, index).enumerate().skip(first_free).filter(|(_, c)| matches!(c.kind, Kind::Free)) {
					let excess_capacity = match try_fill(chunk.size, file) {
						(_, FillResult::Over{ .. }) => {
							if !found_free {
								first_free = i;
								found_free = true;
							}
							continue;
						},
						(_, FillResult::Exact) => None,
						(_, FillResult::Under{ remaining }) => Some(remaining)
					};
					
					*chunk = file.into();
					if index == files.len() - 1 {
						files.pop();
					} else {
						files[index].kind = Kind::Free;
					}
					
					if !found_free {
						first_free = i + 1;
						found_free = true;
					}
					
					if let Some(size) = excess_capacity {
						files.insert(i + 1, FileChunk{ kind: Kind::Free, size });
						index += 1;
					}
					break;
				}
				
				if !found_free {
					break; // No future files can be moved, break outer loop.
				}
			},
			Kind::Free => {
				if index == files.len() - 1 {
					files.pop();
				}
			}
		}
		
		index -= 1; // Index must be at least > 0 at start of loop iteration and can only increase (by 1) throughout, so this shouldn't overflow.
	}
}

fn checksum(files: &[FileChunk]) -> u64 {
	let mut position = 0u32;
	let mut total = 0u64;
	
	for &file in files {
		let next_pos = position + file.size.get() as u32;
		
		if let Kind::File{ id } = file.kind {
			total += id as u64 * sum_between(position, next_pos);
		}
		
		position = next_pos;
	}
	
	total
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
	let input = input.trim().as_bytes();
	
	let mut files = Vec::with_capacity(input.len() + input.len() / 16);
	files.extend(input.iter().enumerate().filter_map(|(i, &b)| chunk_from_byte(i, b)));
	
	compact(&mut files);
	// eprintln!("[i] {} / {} (from {})", files.len(), files.capacity(), input.len());
	
	checksum(&files)
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}

fn take<I: IntoIterator>(i: I, n: usize) -> iter::Take<I::IntoIter> {
	i.into_iter().take(n)
}
