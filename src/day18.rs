use std::{collections::VecDeque, iter, mem};

const BOUND: u8 = 70;
const DIM: usize = BOUND as usize + 1;

const START: (u8, u8) = (0, 0);

fn parse_coord_from_ascii(bytes: &[u8]) -> Option<(u8, u8)> {
	fn v(b: u8) -> Option<u8> {
		matches!(b, b'0'..=b'9').then(|| b - b'0')
	}
	
	Some(match bytes {
		&[    a , b',',     b ] => (              v(a)? ,               v(b)? ),
		&[a1, a2, b',',     b ] => (v(a1)? * 10 + v(a2)?,               v(b)? ),
		&[    a , b',', b1, b2] => (              v(a)? , v(b1)? * 10 + v(b2)?),
		&[a1, a2, b',', b1, b2] => (v(a1)? * 10 + v(a2)?, v(b1)? * 10 + v(b2)?),
		_ => return None
	})
}

fn index_of(pos: (u8, u8), bound: u8) -> usize {
	let dim = bound as usize + 1;
	pos.0 as usize * dim + pos.1 as usize
}

fn surrounds_within(center: (u8, u8), bound: u8) -> impl Iterator<Item = (u8, u8)> {
	const OFFSETS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
	
	let (row, col) = center;
	filter_map(OFFSETS, move |(off_r, off_c)|
		Option::zip(row.checked_add_signed(off_r), col.checked_add_signed(off_c)).filter(|&(r, c)| r <= bound && c <= bound)
	)
}

unsafe fn do_thing(impassable: &mut [bool], bound: u8, buffer: &mut VecDeque<((u8, u8), u32)>) -> Option<u32> {
	debug_assert!(impassable.len() > index_of((bound, bound), bound));
	let goal = (bound, bound);
	
	if START == goal { return Some(0); }
	
	buffer.clear();
	buffer.push_back((START, 0));
	*unsafe { impassable.get_unchecked_mut(index_of(START, bound)) } = true;
	
	while let Some((at, moves)) = buffer.pop_front() {
	//	if at == goal {
	//		return Some(moves);
	//	}
		for to in surrounds_within(at, bound) {
			if !mem::replace(unsafe { impassable.get_unchecked_mut(index_of(to, bound)) }, true) {
				if to == goal {
					// eprintln!("[i] Queue capacity: {}", buffer.capacity());
					return Some(moves + 1);
				} else {
					buffer.push_back((to, moves + 1));
				}
			}
		}
	}
	
	None
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
	let mut map = [[false; DIM]; DIM];
	
	for line in input.lines().take(1024) {
		let (r, c) = unsafe { parse_coord_from_ascii(line.as_bytes()).filter(|&(r, c)| r <= BOUND && c <= BOUND).unwrap_unchecked() };
		map[r as usize][c as usize] = true;
		// eprintln!("[i] Parsed: {r},{c}");
	}
	
	unsafe { do_thing(map.as_flattened_mut(), BOUND, &mut VecDeque::with_capacity(128)).unwrap_unchecked() }
}

const WALL: u8 = u8::MAX;
const VISITED: u8 = 1u8;
const EMPTY: u8 = 0u8;

unsafe fn try_fill_to_end(map: &mut [u8], bound: u8, start: (u8, u8), buffer: &mut VecDeque<(u8, u8)>) -> bool {
	debug_assert!(map.len() > index_of((bound, bound), bound));
	debug_assert!(start.0 <= bound && start.1 <= bound);
	
	buffer.clear();
	
	let goal = (bound, bound);
	if start == goal {
		return true;
	}
	
	*unsafe { map.get_unchecked_mut(index_of(start, bound)) } = VISITED;
	buffer.push_back(start);
	
	while let Some(at) = buffer.pop_front() {
		for to in surrounds_within(at, bound) {
			let slot = unsafe { map.get_unchecked_mut(index_of(to, bound)) };
			if *slot == EMPTY {
				if to == goal {
					return true;
				} else {
					*slot = VISITED;
					buffer.push_back(to);
				}
			}
		}
	}
	
	false
}

unsafe fn remove_wall(map: &mut [u8], bound: u8, at: (u8, u8), buffer: &mut VecDeque<(u8, u8)>) -> bool {
	debug_assert!(map.len() > index_of((bound, bound), bound));
	debug_assert!(at.0 <= bound && at.1 <= bound);
	debug_assert_eq!(map[index_of(at, bound)], WALL);
	
	if at == START {
		try_fill_to_end(map, bound, START, buffer)
	} else if surrounds_within(at, bound).any(|check| *unsafe{ map.get_unchecked(index_of(check, bound)) } == VISITED) {
		try_fill_to_end(map, bound, at, buffer)
	} else {
		*unsafe{ map.get_unchecked_mut(index_of(at, bound)) } = EMPTY;
		false
	}
}

unsafe fn initial_fill(map: &mut [u8], bound: u8, buffer: &mut VecDeque<(u8, u8)>) -> bool {
	debug_assert!(map.len() > index_of((bound, bound), bound));
	
	unsafe {
		if *map.get_unchecked(index_of(START, bound)) == WALL {
			false
		} else {
			try_fill_to_end(map, bound, START, buffer)
		}
	}
}

#[aoc(day18, part2)]
pub fn part2_outer(input: &str) -> String {
	part2(input).into()
}

pub fn part2(input: &str) -> &str {
	let mut map = [[EMPTY; DIM]; DIM];
	
	for line in input.lines() {
		let (r, c) = unsafe { parse_coord_from_ascii(line.as_bytes()).filter(|&(r, c)| r <= BOUND && c <= BOUND).unwrap_unchecked() };
		map[r as usize][c as usize] = WALL;
		// map.as_flattened_mut()[index_of((r, c), BOUND)] = WALL;
	}
	
	let mut queue = VecDeque::with_capacity(8);
	unsafe {
		initial_fill(map.as_flattened_mut(), BOUND, &mut queue);
	}
	for line in input.lines().rev() {
		let point = unsafe { parse_coord_from_ascii(line.as_bytes()).filter(|&(r, c)| r <= BOUND && c <= BOUND).unwrap_unchecked() };
		if unsafe { remove_wall(map.as_flattened_mut(), BOUND, point, &mut queue) } {
			return line;
		}
	}
	
	unreachable!();
}

fn filter_map<I: IntoIterator, O, F: FnMut(I::Item) -> Option<O>>(i: I, f: F) -> iter::FilterMap<I::IntoIter, F> {
	i.into_iter().filter_map(f)
}

//	fn reverse<I: IntoIterator>(i: I) -> iter::Rev<I::IntoIter> where I::IntoIter: DoubleEndedIterator {
//		i.into_iter().rev()
//	}
