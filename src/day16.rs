use std::{cmp::{self, Reverse}, collections::{BinaryHeap, /* HashMap, HashSet */}, iter, num::NonZero};

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet, FxBuildHasher};

#[derive(Copy, Eq, Hash, Clone, PartialEq)]
enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {
	fn turned_clockwise(self) -> Self {
		match self {
			Self::North => Self::East,
			Self::East => Self::South,
			Self::South => Self::West,
			Self::West => Self::North
		}
	}
	
	fn turned_counterclockwise(self) -> Self {
		match self {
			Self::North => Self::West,
			Self::East => Self::North,
			Self::South => Self::East,
			Self::West => Self::South
		}
	}
}

#[derive(Copy, Eq, Hash, Clone, PartialEq)]
struct Reindeer {
	pub position: (usize, usize),
	pub facing: Direction
}

impl Reindeer {
	pub fn from_position(position: (usize, usize)) -> Self {
		Self::from_pos_and_dir(position, Direction::East)
	}
	
	pub fn from_pos_and_dir(position: (usize, usize), facing: Direction) -> Self {
		Self { position, facing }
	}
}

#[derive(Copy, Clone)]
struct ReindeerState {
	pub score: u32,
	pub at: Reindeer
}

impl PartialEq for ReindeerState {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score
	}
}

impl Eq for ReindeerState {}

impl Ord for ReindeerState {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		Ord::cmp(&self.score, &other.score)
	}
}

impl PartialOrd for ReindeerState {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl ReindeerState {
	pub fn start(at: Reindeer) -> Self {
		Self { at, score: 0 }
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

fn go(mut passable: impl FnMut((usize, usize)) -> bool, start: Reindeer, end: (usize, usize),
		buffer: &mut BinaryHeap<Reverse<ReindeerState>>, visited_buffer: &mut HashSet<Reindeer>) -> Option<u32> {
	visited_buffer.clear();
	buffer.clear();
	buffer.push(Reverse(ReindeerState::start(start)));
	while let Some(Reverse(ReindeerState { score, at })) = buffer.pop() {
		let Reindeer { position, facing } = at;
		if position == end {
			// eprintln!("[i] Queue capacity: {} / Visited set entries: {}", buffer.capacity(), visited_buffer.len());
			return Some(score);
		} else if visited_buffer.insert(at) {
			for (facing, increase) in [(facing.turned_counterclockwise(), 1001), (facing, 1), (facing.turned_clockwise(), 1001)] {
				let position = wrapping_offset(position, facing);
				if passable(position) /*map.get(position.0, position.1).is_some_and(|&space| matches!(space, Space::Empty))*/ {
					let score = score + increase;
					buffer.push(Reverse(ReindeerState { score, at: Reindeer { position, facing }}));
				}
			}
		}
	}
	
	None
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
	let row_length = NonZero::new(input.find('\n').unwrap_or(input.len()) + 1).unwrap();
	let start = input.find('S').map(|i| (i / row_length, i % row_length)).unwrap();
	let end = input.find('E').map(|i| (i / row_length, i % row_length)).unwrap();
	
	let input = input.as_bytes();
	go(|(row, col)| input.get(row * row_length.get() + col).is_some_and(|&b| b != b'#'), Reindeer::from_position(start), end,
			&mut BinaryHeap::with_capacity(256), &mut HashSet::with_capacity_and_hasher(20000, FxBuildHasher)).unwrap()
}

struct ReindeerState2 {
	pub score: u32,
	pub at: Reindeer,
	pub visited: HashSet<(usize, usize)>
}

impl PartialEq for ReindeerState2 {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score
	}
}

impl Eq for ReindeerState2 {}

impl Ord for ReindeerState2 {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		Ord::cmp(&self.score, &other.score)
	}
}

impl PartialOrd for ReindeerState2 {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl ReindeerState2 {
	pub fn start(at: Reindeer) -> Self {
		Self { at, score: 0 , visited: HashSet::default() }
	}
}

fn go2(mut passable: impl FnMut((usize, usize)) -> bool, start: Reindeer, end: (usize, usize),
		buffer: &mut BinaryHeap<Reverse<ReindeerState2>>, visited_buffer: &mut HashMap<Reindeer, u32>) -> Option<usize> {
	visited_buffer.clear();
	buffer.clear();
	buffer.push(Reverse(ReindeerState2::start(start)));
	
	let mut best_visiteds: Option<HashSet<_>> = None;
	let mut finished_score = None;
	
	let mut nexts = [(0, Reindeer::from_position((0, 0))), (0, Reindeer::from_position((0, 0))), (0, Reindeer::from_position((0, 0)))];
	while let Some(Reverse(ReindeerState2 { score, at, mut visited })) = buffer.pop() {
		if finished_score.is_some_and(|max| score > max) {
			break;
		}
		visited.insert(at.position);
		
		let Reindeer { position, facing } = at;
		if position == end {
			finished_score = Some(score);
			if let Some(best) = best_visiteds.as_mut() {
				best.extend(visited);
			} else {
				best_visiteds = Some(visited);
			}
		} else if finished_score.is_none() && *visited_buffer.entry(at).or_insert(score) == score {
			// It would almost certainly be better to create some kind of merging system for if two states with the same score
			// but (possibly) different visited sets entered the same location (and direction), rather than just continuing to
			// keep track of both separately as is done here (line above) via semi-cursed hash map usage.
			let mut next_i = 0;
			for (facing, increase) in [(facing.turned_counterclockwise(), 1001), (facing.turned_clockwise(), 1001), (facing, 1)] {
				let position = wrapping_offset(position, facing);
				if passable(position) {
					nexts[next_i] = (score + increase, Reindeer { position, facing });
					next_i += 1;
				}
			}
			
			for (i, &(score, at)) in enumerate(&nexts[..next_i]) {
				if i == next_i - 1 {
					buffer.push(Reverse(ReindeerState2 { score, at, visited }));
					break;
				} else {
					buffer.push(Reverse(ReindeerState2 { score, at, visited: visited.clone() }));
				}
			}
		}
	}
	
	// eprintln!("[i] Queue capacity: {} / Visited set entries: {}", buffer.capacity(), visited_buffer.len());
	best_visiteds.as_ref().map(HashSet::len)
} 

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
	let row_length = NonZero::new(input.find('\n').unwrap_or(input.len()) + 1).unwrap();
	let start = input.find('S').map(|i| (i / row_length, i % row_length)).unwrap();
	let end = input.find('E').map(|i| (i / row_length, i % row_length)).unwrap();
	
	let input = input.as_bytes(); // Could probably at least prune dead ends first to make this a fair bit faster. Unfortunately, I am lazy.
	go2(|(row, col)| input.get(row * row_length.get() + col).is_some_and(|&b| b != b'#'), Reindeer::from_position(start), end,
			&mut BinaryHeap::with_capacity(1024), &mut HashMap::with_capacity_and_hasher(20000, FxBuildHasher)).unwrap()
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}
