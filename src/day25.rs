// #![feature(iter_next_chunk)] (v1 only)
use std::iter;

const A: usize = "#####\n#####\n#####\n#####\n#####\n#####".len();
const B: usize = "\n.....\n\n".len();

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
	let mut locks = Vec::with_capacity(250);
	let mut keys = Vec::with_capacity(250);
	
	let mut bytes = input.as_bytes();
	while let Some((&chunk, after)) = bytes.split_first_chunk::<A>() {
		let [kind, _, _, _, _, _, pins @ ..] = chunk;
		let pins = pins.chunks(6).flat_map(|c| unsafe { *c.split_first_chunk::<5>().unwrap_unchecked().0 })
				.fold(0, |acc, pin| (acc << 1) | (pin == b'#') as u32);
		
		(if kind == b'#' { &mut locks } else { &mut keys }).push(pins);
		
		bytes = after.get(B..).unwrap_or_default();
	}
	
	map(keys, |key| filter(&locks, |&&lock| key & lock == 0).count()).sum()
}

fn filter<I: IntoIterator, F: FnMut(&I::Item) -> bool>(i: I, p: F) -> iter::Filter<I::IntoIter, F> {
	i.into_iter().filter(p)
}

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::Map<I::IntoIter, F> {
	i.into_iter().map(f)
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum PinHeight {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five
}

#[derive(Copy, Eq, Clone, PartialEq)]
struct Lock {
	heights: [PinHeight; 5]
}

#[derive(Copy, Eq, Clone, PartialEq)]
struct Key {
	heights: [PinHeight; 5]
}

impl Key {
	fn fits_within(self, lock: Lock) -> bool {
		iter::zip(self.heights, lock.heights).all(|(a, b)| a as u8 + b as u8 <= 5)
	}
}

impl PinHeight {
	fn from_height(height: u8) -> Option<Self> {
		match height {
			0 => Some(Self::Zero),
			1 => Some(Self::One),
			2 => Some(Self::Two),
			3 => Some(Self::Three),
			4 => Some(Self::Four),
			5 => Some(Self::Five),
			_ => None
		}
	}
	
	fn invert(self) -> Self {
		match self {
			Self::Zero  => Self::Five,
			Self::One   => Self::Four,
			Self::Two   => Self::Three,
			Self::Three => Self::Two,
			Self::Four  => Self::One,
			Self::Five  => Self::Zero
		}
	}
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum ParseError {
	WrongLen{ line_index: u8 },
	InvalidChar{ line_index: u8, byte_index: u8 },
	Hole{ line_index: u8, byte_index: u8 },
	TopNotUniform,
	BottomNotUniform,
	BottomMatchesTop
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Parsed {
	Lock(Lock),
	Key(Key)
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum Kind { Lock, Key }

fn parse_lock_or_key<T: AsRef<[u8]>>(input_lines: &[T; 7]) -> Result<Parsed, ParseError> {
	let [top, middle @ .., bottom] = input_lines;
	
	let top_kind = match top.as_ref() {
		&[b'#', b'#', b'#', b'#', b'#'] => Kind::Lock,
		&[b'.', b'.', b'.', b'.', b'.'] => Kind::Key,
		&[_, _, _, _, _] => return Err(ParseError::TopNotUniform),
		_ => return Err(ParseError::WrongLen{ line_index: 0 })
	};
	
	let bottom_kind = match bottom.as_ref() {
		&[b'#', b'#', b'#', b'#', b'#'] => Kind::Key,
		&[b'.', b'.', b'.', b'.', b'.'] => Kind::Lock,
		&[_, _, _, _, _] => return Err(ParseError::BottomNotUniform),
		_ => return Err(ParseError::WrongLen{ line_index: 7 - 1 })
	};
	
	let kind = if top_kind == bottom_kind {
		top_kind
	} else {
		return Err(ParseError::BottomMatchesTop)
	};
	
	let mut heights = [PinHeight::Zero; 5];
	for (i, line) in enumerate(middle).map(|(i, line)| ((i + 1) as u8, line.as_ref())) {
		let Ok(line): Result<[u8; 5], _> = line.try_into() else {
			return Err(ParseError::WrongLen{ line_index: i });
		};
		
		let to_height = unsafe { PinHeight::from_height(i).unwrap_unchecked() };
		for (i2, (byte, height)) in iter::zip(line, &mut heights).enumerate() {
			let increase = match byte {
				b'#' => kind == Kind::Lock,
				b'.' => kind == Kind::Key,
				_ => return Err(ParseError::InvalidChar{ line_index: i, byte_index: i2 as u8 })
			};
			
			if increase {
				if *height as u8 == to_height as u8 - 1 {
					*height = to_height;
				} else {
					return Err(ParseError::Hole { line_index: i, byte_index: i2 as u8 });
				}
			}
		}
	}
	
	Ok(match kind {
		Kind::Lock => Parsed::Lock(Lock{ heights }),
		Kind::Key => Parsed::Key(Key { heights: heights.map(PinHeight::invert) })
	})
}

#[allow(unused)]
pub fn part1_v1(input: &str) -> usize {
	let mut locks = Vec::with_capacity(250);
	let mut keys = Vec::with_capacity(250);
	
	let mut lines = input.lines();
	while let Ok(chunk) = lines.next_chunk() {
		match unsafe { parse_lock_or_key(&chunk).unwrap_unchecked() } {
			Parsed::Lock(lock) => locks.push(lock),
			Parsed::Key(key) => keys.push(key)
		}
		
		lines.next();
	}
	
	map(keys, |key| {
		filter(&locks, |&&lock| key.fits_within(lock)).count()
	}).sum()
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}


#[allow(unused)]
pub fn part2(_: &str) -> &'static str { "" }
