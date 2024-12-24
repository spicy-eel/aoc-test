use std::{cmp::{Ordering, Reverse}, collections::BinaryHeap, fmt, iter, ops};

#[derive(Copy, Eq, Clone, PartialEq)]
enum Numpad {
	B7 = 7, B8 = 8, B9 = 9,
	B4 = 4, B5 = 5, B6 = 6,
	B1 = 1, B2 = 2, B3 = 3,
	        B0 = 0,  A = 10
}

impl Numpad {
	fn from_value(value: u8) -> Option<Self> {
		match value {
			0 => Some(Self::B0),
			1 => Some(Self::B1),
			2 => Some(Self::B2),
			3 => Some(Self::B3),
			4 => Some(Self::B4),
			5 => Some(Self::B5),
			6 => Some(Self::B6),
			7 => Some(Self::B7),
			8 => Some(Self::B8),
			9 => Some(Self::B9),
			_ => None
		}
	}
}

#[derive(Copy, Default, Eq, Clone, PartialEq)]
struct NumpadMap<T>([T; 11]);

impl<T> ops::Index<Numpad> for NumpadMap<T> {
	type Output = T;
	
	fn index(&self, i: Numpad) -> &T {
		unsafe { self.0.get_unchecked(i as usize) }
	}
}

impl<T> ops::IndexMut<Numpad> for NumpadMap<T> {
	fn index_mut(&mut self, i: Numpad) -> &mut T {
		unsafe { self.0.get_unchecked_mut(i as usize) }
	}
}

fn code_from_ascii(bytes: &[u8]) -> Option<([Numpad; 4], u32)> {
	let &[a, b, c, b'A'] = bytes else {
		return None;
	};
	
	let (a, b, c) = (a.wrapping_sub(b'0'), b.wrapping_sub(b'0'), c.wrapping_sub(b'0'));
	Some(([Numpad::from_value(a)?, Numpad::from_value(b)?, Numpad::from_value(c)?, Numpad::A], a as u32 * 100 + b as u32 * 10 + c as u32))
}

const NUMPAD_NAVIGATION_COSTS_2: NumpadMap<NumpadMap<u32>> = NumpadMap([
	     // To: 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , A 
	NumpadMap([  1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 10 ]), // From: 0
	NumpadMap([ 21,  1, 10, 11, 12, 19, 20, 13, 20, 21, 22 ]), // From: 1
	NumpadMap([ 16, 18,  1, 10, 21, 12, 19, 22, 13, 20, 17 ]), // From: 2
	NumpadMap([ 21, 19, 18,  1, 22, 21, 12, 23, 22, 13, 16 ]), // From: 3
	NumpadMap([ 22, 16, 17, 18,  1, 10, 11, 12, 19, 20, 23 ]), // From: 4
	NumpadMap([ 17, 21, 16, 17, 18,  1, 10, 21, 12, 19, 18 ]), // From: 5
	NumpadMap([ 22, 22, 21, 16, 19, 18,  1, 22, 21, 12, 17 ]), // From: 6
	NumpadMap([ 23, 17, 18, 19, 16, 17, 18,  1, 10, 11, 24 ]), // From: 7
	NumpadMap([ 18, 22, 17, 18, 21, 16, 17, 18,  1, 10, 19 ]), // From: 8
	NumpadMap([ 23, 23, 22, 17, 22, 21, 16, 19, 18,  1, 18 ]), // From: 9
	NumpadMap([ 18, 26, 21, 12, 27, 22, 13, 28, 23, 14,  1 ])  // From: A
]);

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u32 {
	// print_mappings();
	input.lines().map(|line| {
		let (code, numeric) = unsafe { code_from_ascii(line.as_bytes()).unwrap_unchecked() };
		
		fold(code, (0, Numpad::A), |(moves, from), to|
			(moves + NUMPAD_NAVIGATION_COSTS_2[from][to], to)
		).0 * numeric
	}).sum()
}

const NUMPAD_NAVIGATION_COSTS_25: NumpadMap<NumpadMap<u64>> = NumpadMap([
	     // To:      0     ,      1     ,      2     ,      3     ,      4     ,      5     ,      6     ,      7     ,      8     ,      9     ,      A     
	NumpadMap([           1, 31420065369, 14752615084, 24095973437, 31420065370, 14752615085, 24095973438, 31420065371, 14752615086, 24095973439, 14287938116 ]), // From: 0
	NumpadMap([ 27052881363,           1, 14287938116, 14287938117, 14752615084, 24095973437, 24095973438, 14752615085, 24095973438, 24095973439, 27052881364 ]), // From: 1
	NumpadMap([ 20790420654, 22411052532,           1, 14287938116, 28154654777, 14752615084, 24095973437, 28154654778, 14752615085, 24095973438, 22778092491 ]), // From: 2
	NumpadMap([ 27622800565, 22411052533, 22411052532,           1, 28154654778, 28154654777, 14752615084, 28154654779, 28154654778, 14752615085, 20790420654 ]), // From: 3
	NumpadMap([ 27052881364, 20790420654, 22778092491, 22778092492,           1, 14287938116, 14287938117, 14752615084, 24095973437, 24095973438, 27052881365 ]), // From: 4
	NumpadMap([ 20790420655, 27622800565, 20790420654, 22778092491, 22411052532,           1, 14287938116, 28154654777, 14752615084, 24095973437, 22778092492 ]), // From: 5
	NumpadMap([ 27622800566, 27622800566, 27622800565, 20790420654, 22411052533, 22411052532,           1, 28154654778, 28154654777, 14752615084, 20790420655 ]), // From: 6
	NumpadMap([ 27052881365, 20790420655, 22778092492, 22778092493, 20790420654, 22778092491, 22778092492,           1, 14287938116, 14287938117, 27052881366 ]), // From: 7
	NumpadMap([ 20790420656, 27622800566, 20790420655, 22778092492, 27622800565, 20790420654, 22778092491, 22411052532,           1, 14287938116, 22778092493 ]), // From: 8
	NumpadMap([ 27622800567, 27622800567, 27622800566, 20790420655, 27622800566, 27622800565, 20790420654, 22411052533, 22411052532,           1, 20790420656 ]), // From: 9
	NumpadMap([ 22411052532, 31420065370, 28154654777, 14752615084, 31420065371, 28154654778, 14752615085, 31420065372, 28154654779, 14752615086,           1 ])  // From: A
]);

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
	input.lines().map(|line| {
		let (code, numeric) = unsafe { code_from_ascii(line.as_bytes()).unwrap_unchecked() };
		
		fold(code, (0, Numpad::A), |(moves, from), to|
			(moves + NUMPAD_NAVIGATION_COSTS_25[from][to], to)
		).0 * numeric as u64
	}).sum()
}

fn fold<I: IntoIterator, T, F: FnMut(T, I::Item) -> T>(i: I, init: T, f: F) -> T {
	i.into_iter().fold(init, f)
}

// - Code for deriving above mappings:

#[derive(Copy, Eq, Clone, PartialEq)]
enum Dirpad {
	           Up  = 2,   A   = 4,
	Left = 0, Down = 1, Right = 3
}

enum MoveResult<T> {
	Pressed(T),
	MovedTo(T),
	Failed
}

impl<T> From<Option<T>> for MoveResult<T> {
	fn from(r: Option<T>) -> Self {
		r.map(Self::MovedTo).unwrap_or(Self::Failed)
	}
}

trait Gridlike: Copy + Eq {
	fn up(self) -> Option<Self>;
	fn down(self) -> Option<Self>;
	fn left(self) -> Option<Self>;
	fn right(self) -> Option<Self>;
	
	fn after_move_input(self, button: Dirpad) -> MoveResult<Self> {
		match button {
			Dirpad::A => MoveResult::Pressed(self),
			Dirpad::Left => self.left().into(),
			Dirpad::Down => self.down().into(),
			Dirpad::Up => self.up().into(),
			Dirpad::Right => self.right().into()
		}
	}
	
	fn row(self) -> u8;
	fn col(self) -> u8;
	
	fn is_above(self, other: Self) -> bool    { self.row() < other.row() }
	fn is_below(self, other: Self) -> bool    { self.row() > other.row() }
	fn is_left_of(self, other: Self) -> bool  { self.col() < other.col() }
	fn is_right_of(self, other: Self) -> bool { self.col() > other.col() }
	
	fn worth_pressing_to_get_to(self, target: Self, button: Dirpad) -> bool {
		match button {
			Dirpad::Up => self.is_below(target),
			Dirpad::Down => self.is_above(target),
			
			Dirpad::Left => self.is_right_of(target),
			Dirpad::Right => self.is_left_of(target),
			
			Dirpad::A => self == target
		}
	}
}

impl Gridlike for Numpad {
	fn up(self) -> Option<Self> {
		match self {
			Self::B7 | Self::B8 | Self::B9 => None,
			
			Self::B4 => Some(Self::B7),
			Self::B5 => Some(Self::B8),
			Self::B6 => Some(Self::B9),
			
			Self::B1 => Some(Self::B4),
			Self::B2 => Some(Self::B5),
			Self::B3 => Some(Self::B6),
			
			Self::B0 => Some(Self::B2),
			Self::A  => Some(Self::B3)
		}
	}
	
	fn down(self) -> Option<Self> {
		match self {
			Self::B7 => Some(Self::B4),
			Self::B8 => Some(Self::B5),
			Self::B9 => Some(Self::B6),
			
			Self::B4 => Some(Self::B1),
			Self::B5 => Some(Self::B2),
			Self::B6 => Some(Self::B3),
			
			Self::B2 => Some(Self::B0),
			Self::B3 => Some(Self::A ),
			
			Self::B1 | Self::B0 | Self::A => None
		}
	}
	
	fn left(self) -> Option<Self> {
		match self {
			Self::B0 | Self::B1 | Self::B4 | Self::B7 => None,
			
			Self::B8 => Some(Self::B7),
			Self::B5 => Some(Self::B4),
			Self::B2 => Some(Self::B1),
			
			Self::B9 => Some(Self::B8),
			Self::B6 => Some(Self::B5),
			Self::B3 => Some(Self::B2),
			Self::A  => Some(Self::B0)
		}
	}
	
	fn right(self) -> Option<Self> {
		match self {
			Self::B7 => Some(Self::B8),
			Self::B4 => Some(Self::B5),
			Self::B1 => Some(Self::B2),
			
			Self::B8 => Some(Self::B9),
			Self::B5 => Some(Self::B6),
			Self::B2 => Some(Self::B3),
			Self::B0 => Some(Self::A ),
			
			Self::A | Self::B3 | Self::B6 | Self::B9 => None
		}
	}
	
	fn row(self) -> u8 {
		match self {
			Self::B7 | Self::B8 | Self::B9 => 0,
			Self::B4 | Self::B5 | Self::B6 => 1,
			Self::B1 | Self::B2 | Self::B3 => 2,
			           Self::B0 |  Self::A => 3,
		}
	}
	
	fn col(self) -> u8 {
		match self {
			           Self::B1 | Self::B4 | Self::B7 => 0,
			Self::B0 | Self::B2 | Self::B5 | Self::B8 => 1,
			Self::A  | Self::B3 | Self::B6 | Self::B9 => 2
		}
	}
}

impl Gridlike for Dirpad {
	fn up(self) -> Option<Self> {
		match self {
			Self::Left | Self::Up | Self::A => None,
			
			Self::Down  => Some(Self::Up),
			Self::Right => Some(Self::A )
		}
	}
	
	fn down(self) -> Option<Self> {
		match self {
			Self::Up => Some(Self::Down),
			Self::A  => Some(Self::Right),
			
			Self::Left | Self::Down | Self::Right => None
		}
	}
	
	fn left(self) -> Option<Self> {
		match self {
			Self::Left | Self::Up => None,
			
			Self::Down => Some(Self::Left),
			
			Self::A     => Some(Self::Up),
			Self::Right => Some(Self::Down)
		}
	}
	
	fn right(self) -> Option<Self> {
		match self {
			Self::Left => Some(Self::Down),
			
			Self::Up   => Some(Self::A),
			Self::Down => Some(Self::Right),
			
			Self::A | Self::Right => None
		}
	}
	
	fn row(self) -> u8 {
		match self {
			              Self::Up  |   Self::A   => 0,
			Self::Left | Self::Down | Self::Right => 1,
		}
	}
	
	fn col(self) -> u8 {
		match self {
			           Self::Left  => 0,
			Self::Up | Self::Down  => 1,
			Self::A  | Self::Right => 2
		}
	}
}

#[derive(Copy, Default, Eq, Clone, PartialEq)]
struct DirpadMap<T>([T; 5]);

impl<T> ops::Index<Dirpad> for DirpadMap<T> {
	type Output = T;
	
	fn index(&self, i: Dirpad) -> &T {
		unsafe { self.0.get_unchecked(i as usize) }
	}
}

impl<T> ops::IndexMut<Dirpad> for DirpadMap<T> {
	fn index_mut(&mut self, i: Dirpad) -> &mut T {
		unsafe { self.0.get_unchecked_mut(i as usize) }
	}
}

#[derive(Copy, Clone)]
enum State<T> {
	Ongoing{ cost: u64, at: T, last_dir: Dirpad },
	Finished(u64)
}

impl<T> State<T> {
	fn cost(&self) -> u64 {
		match self {
			&Self::Ongoing{ cost, .. } => cost,
			&Self::Finished(cost) => cost
		}
	}
}

impl<T> Ord for State<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost().cmp(&other.cost())
	}
}

impl<T> PartialOrd for State<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T> PartialEq for State<T> {
	fn eq(&self, other: &Self) -> bool {
		self.cost() == other.cost()
	}
}

impl<T> Eq for State<T> {}

// Asserts that all values of the type can be reached from any other value via
// at least one possible sequence of 'worth pressing' up/down/left/right/A inputs.
unsafe trait AssertAllReachable : Gridlike {}

unsafe impl AssertAllReachable for Numpad {}
unsafe impl AssertAllReachable for Dirpad {}

fn navigation_cost<T: AssertAllReachable>(from: T, to: T, button_costs: &DirpadMap<DirpadMap<u64>>, buffer: &mut BinaryHeap<Reverse<State<T>>>) -> u64 {
	buffer.clear();
	
	buffer.push(Reverse(State::Ongoing{ cost: 0, at: from, last_dir: Dirpad::A }));
	while let Some(Reverse(state)) = buffer.pop() {
		match state {
			State::Finished(cost) => return cost,
			State::Ongoing{ cost, at, last_dir } => {
				buffer.extend(
					filter_map(Dirpad::ALL, |next_dir| {
						at.worth_pressing_to_get_to(to, next_dir).then(|| {
							let new_cost = cost + button_costs[last_dir][next_dir];
							match at.after_move_input(next_dir) {
								MoveResult::Pressed(button) if button == to => Some(State::Finished(new_cost)),
								MoveResult::MovedTo(button) => Some(State::Ongoing{ cost: new_cost, at: button, last_dir: next_dir }),
								MoveResult::Failed | MoveResult::Pressed(_) => None
							}
						}).flatten()
					}).map(Reverse)
				)
			}
		}
	}
	
	unsafe { std::hint::unreachable_unchecked() }
}

trait Enumerated {
	type All: IntoIterator<Item = Self>;
	const ALL: Self::All;
}

impl Enumerated for Dirpad {
	type All = [Self; 5];
	const ALL: [Self; 5] = [Self::Left, Self::Down, Self::Up, Self::Right, Self::A];
}

impl Enumerated for Numpad {
	type All = [Self; 11];
	const ALL: [Self; 11] = [Self::B0, Self::B1, Self::B2, Self::B3, Self::B4, Self::B5, Self::B6, Self::B7, Self::B8, Self::B9, Self::A];
}

trait DualArrayMappable<T> {
	type Inner: ops::Index<Self, Output = T> + ops::IndexMut<Self>;
	type Outer: ops::Index<Self, Output = Self::Inner> + ops::IndexMut<Self>;
}

impl<T> DualArrayMappable<T> for Dirpad {
	type Inner = DirpadMap<T>;
	type Outer = DirpadMap<DirpadMap<T>>;
}

impl<T> DualArrayMappable<T> for Numpad {
	type Inner = NumpadMap<T>;
	type Outer = NumpadMap<NumpadMap<T>>;
}

fn calculate_movement_costs<T>(button_costs: &DirpadMap<DirpadMap<u64>>, buffer: &mut BinaryHeap<Reverse<State<T>>>) -> T::Outer
	where T: AssertAllReachable + Enumerated + DualArrayMappable<u64>,
	      T::Outer: Default
{
	let mut cost_map = T::Outer::default();
	for from in T::ALL {
		for to in T::ALL {
			cost_map[from][to] = navigation_cost(from, to, button_costs, buffer);
		}
	}
	cost_map
}

impl DirpadMap<u64> {
	const fn ones() -> Self {
		Self([1; 5])
	}
}

impl DirpadMap<DirpadMap<u64>> {
	const fn ones() -> Self {
		Self([DirpadMap::<u64>::ones(); 5])
	}
}

fn generate_numpad_costs(dirpads: u8, dir_buffer: &mut BinaryHeap<Reverse<State<Dirpad>>>,
			num_buffer: &mut BinaryHeap<Reverse<State<Numpad>>>) -> NumpadMap<NumpadMap<u64>> {
	// Overflows (and panics in debug) on dirpads ≥ 47.
	let mut button_costs = <DirpadMap<DirpadMap<u64>>>::ones();
	for _ in 0..dirpads {
		button_costs = calculate_movement_costs::<Dirpad>(&button_costs, dir_buffer);
	}
	
	calculate_movement_costs::<Numpad>(&button_costs, num_buffer)
}

fn filter_map<I: IntoIterator, O, F: FnMut(I::Item) -> Option<O>>(i: I, f: F) -> iter::FilterMap<I::IntoIter, F> {
	i.into_iter().filter_map(f)
}

fn flat_map<I: IntoIterator, O: IntoIterator, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::FlatMap<I::IntoIter, O, F> {
	i.into_iter().flat_map(f)
}

struct ExportNumpadMap<'a, T: fmt::Display + Length>(&'a NumpadMap<NumpadMap<T>>);

trait Length {
	fn length(&self) -> usize;
}

impl Length for u64 {
	fn length(&self) -> usize {
		(self.checked_ilog10().unwrap_or(0) + 1) as usize
	}
}

impl Length for u32 {
	fn length(&self) -> usize {
		(self.checked_ilog10().unwrap_or(0) + 1) as usize
	}
}

impl<'a, T: fmt::Display + Length> fmt::Display for ExportNumpadMap<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fn to_char(button: Numpad) -> char {
			match button {
				Numpad::B0 => '0',
				Numpad::B1 => '1',
				Numpad::B2 => '2',
				Numpad::B3 => '3',
				Numpad::B4 => '4',
				Numpad::B5 => '5',
				Numpad::B6 => '6',
				Numpad::B7 => '7',
				Numpad::B8 => '8',
				Numpad::B9 => '9',
				Numpad::A  => 'A'
			}
		}
		
		let width = flat_map(&self.0.0, |row| &row.0).map(Length::length).max().unwrap();
		
		writeln!(f, "NumpadMap([")?;
		writeln!(f, "\t     // To: {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}, {:^w$}",
				'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', w = width)?;
		const LAST: Numpad = Numpad::A;
		for (from, layer) in iter::zip(Numpad::ALL, &self.0.0) {
			write!(f, "\tNumpadMap([ ")?;
			for (to, pushes) in iter::zip(Numpad::ALL, &layer.0) {
				write!(f, "{pushes:width$}")?;
				if to != LAST {
					write!(f, ", ")?;
				}
			}
			let from_c = to_char(from);
			if from != LAST {
				writeln!(f, " ]), // From: {from_c}")?;
			} else {
				writeln!(f, " ])  // From: {from_c}")?;
			}
		}
		write!(f, "])")
	}
}

#[allow(unused)]
pub fn print_mappings() {
	let (mut dir, mut num) = (BinaryHeap::new(), BinaryHeap::new());
	println!("// - Part 1:");
	println!("const NUMPAD_NAVIGATION_COSTS_2: NumpadMap<NumpadMap<u32>> = {};", ExportNumpadMap(&generate_numpad_costs(2, &mut dir, &mut num)));
	println!("\n// - Part 2:");
	println!("const NUMPAD_NAVIGATION_COSTS_25: NumpadMap<NumpadMap<u64>> = {};", ExportNumpadMap(&generate_numpad_costs(25, &mut dir, &mut num)));
}
