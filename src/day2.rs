#[derive(Copy, Eq, Clone, PartialEq)]
enum Dir {
	Increasing,
	Decreasing
}

impl Dir {
	fn compare<T: Ord>(a: &T, b: &T) -> Option<Self> {
		use std::cmp::Ordering;
		match Ord::cmp(a, b) {
			Ordering::Less => Some(Self::Increasing),
			Ordering::Greater => Some(Self::Decreasing),
			Ordering::Equal => None
		}
	}
}

#[derive(Copy, Clone)]
enum ProblemState {
	Start, // No values encountered.
	First{ previous: i32 }, // Only one value has been computed thus far.
	Normal{ previous: [i32; 2] }, // No problems have been encountered yet.
	JustEncountered{ previous: [i32; 2] }, // Both previous values are okay individually, but one must be removed.
	Dampened{ previous: i32 }, // The Problem Dampener™ has already been used.
	Invalid
}

#[derive(Copy, Clone)]
struct StateTracker {
	intended_direction: Dir,
	state: ProblemState
}

fn check_validity(a: i32, b: i32, intended_direction: Dir) -> Result<(), bool> { // Err - bool: true if values were equal.
	const RANGE: std::ops::RangeInclusive<u32> = 1..=3;
	
	match Dir::compare(&a, &b) {
		Some(dir) if dir == intended_direction && RANGE.contains(&a.abs_diff(b)) => Ok(()),
		None => Err(true),
		_ => Err(false)
	}
}

impl StateTracker {
	fn for_direction(intended_direction: Dir) -> Self {
		Self {
			intended_direction,
			state: ProblemState::Start
		}
	}
	
	fn apply_value(&mut self, next: i32) {
		use ProblemState::*;
		
		self.state = match self.state {
			Start => First{ previous: next },
			First{ previous } => {
			//	match Dir::compare(&previous, &next) {
			//		Some(dir) if dir == self.intended_direction && RANGE.contains(previous.abs_diff(next)) => Normal { previous: [previous, next] },
			//		None => Dampened { previous }, // Values are equal (so either can be used).
			//		_ => JustEncountered { previous: [previous, next] } // Either value can be use as the first.
			//	}
				match check_validity(previous, next, self.intended_direction) {
					Ok(()) => Normal { previous: [previous, next] },
					Err(true) => Dampened { previous }, // Values are equal (so either can be used).
					Err(false) => JustEncountered { previous: [previous, next] } // Either value can be used as the first.
				}
			},
			Normal{ previous: [first, second] } => {
				match check_validity(second, next, self.intended_direction) {
					Ok(()) => Normal { previous: [second, next] },
					Err(true) => Dampened { previous: second },
					Err(false) => {
						if check_validity(first, next, self.intended_direction).is_ok() {
							JustEncountered { previous: [second, next] }
						} else {
							Dampened { previous: second } // Current ('next') value must be removed.
						}
					}
				}
			},
			JustEncountered{previous: [a, b]} => {
				if check_validity(a, next, self.intended_direction).or_else(|_| check_validity(b, next, self.intended_direction)).is_ok() {
					Dampened { previous: next }
				} else {
					Invalid
				}
			},
			Dampened{ previous } => {
				if check_validity(previous, next, self.intended_direction).is_ok() {
					Dampened{ previous: next }
				} else {
					Invalid
				}
			}
			Invalid => Invalid
		}
	}
	
	fn is_invalid(&self) -> bool {
		matches!(self.state, ProblemState::Invalid)
	}
	
	#[allow(unused)]
	fn is_valid(&self) -> bool {
		use ProblemState::*;
		
		!matches!(self.state, Start | Invalid)
	}
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
	input.lines().filter(|line| {
		let mut ascending_state = StateTracker::for_direction(Dir::Increasing);
		let mut descending_state = StateTracker::for_direction(Dir::Decreasing);
		
		for num in line.split(' ') {
			let num = unsafe { num.parse().unwrap_unchecked() };
			ascending_state.apply_value(num);
			descending_state.apply_value(num);
			
			if ascending_state.is_invalid() && descending_state.is_invalid() {
				return false;
			}
		}
		
		true // ascending_state.is_valid() || descending_state.is_valid()
	}).count()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
	input.lines().filter(|line| {
		let mut direction = None;
		let mut previous = None;
		for num in line.split_whitespace() {
			let num = unsafe { num.parse().unwrap_unchecked() };
			if let Some(prev) = previous {
				if (1..=3).contains(&i32::abs_diff(num, prev)) {
					let dir = i32::cmp(&num, &prev);
					if let Some(direction) = direction {
						if direction != dir {
							return false;
						}
					} else {
						direction = Some(dir);
					}
				} else {
					return false;
				}
			}
			
			previous = Some(num);
		}
		
		true // previous.is_some() // lines should probably have at least one reading to count
	}).count()
}