use std::{collections::BinaryHeap, iter};

fn can_fill_towel(target: &str, components_by_length: &[&str], buffer: &mut BinaryHeap<usize>, visited_buffer: &mut Vec<bool>) -> bool {
	let Some(new_size) = target.len().checked_sub(1) else {
		return false;
	};
	visited_buffer.clear();
	visited_buffer.resize(new_size, false);
	
	buffer.clear();
	buffer.push(0);
	
	while let Some(index) = buffer.pop() { // the one time where this being a max-heap is (semi-)useful?
		let target = unsafe { target.get_unchecked(index..) };
		
		let mut candidates = reverse(components_by_length);
		let mut prev_length = target.len() + 1;
		while let Some(next) = candidates.find(|&s| s.len() < prev_length) {
			if next.len() == target.len() || visited_buffer.get(index + next.len() - 1).is_some_and(|&visited| !visited) {
				if target.starts_with(next) {
					if next.len() == target.len() {
						return true;
					} else {
						visited_buffer[index + next.len() - 1] = true;
						buffer.push(index + next.len());
						prev_length = next.len();
					}
				}
			} else {
				prev_length = next.len();
			}
		}
	}
	
	false
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
	let mut lines = input.lines();
	
	let mut patterns = Vec::with_capacity(500);
	patterns.extend(lines.next().unwrap_or_default().split(", ").filter(|&s| !s.is_empty()));
	patterns.sort_by_key(|&s| s.len());
	
	lines.next();
	
	let mut queue = BinaryHeap::with_capacity(64);
	let mut visited = Vec::with_capacity(128);

//	eprintln!("[i]: Capacities — Queue: {} / Visited: {} / Patterns: {} (length {})", queue.capacity(), visited.capacity(), patterns.capacity(), patterns.len());
	lines.filter(|target| can_fill_towel(target, &patterns, &mut queue, &mut visited)).count()
}

fn ways_to_fill(target: &str, components_by_length: &[&str], buffer: &mut Vec<u64>) -> u64 {
	debug_assert!(!components_by_length.iter().copied().any(str::is_empty));
	let Some(new_size) = target.len().checked_sub(1) else {
		return 1;
	};
	buffer.clear();
	buffer.resize(new_size, 0);
	
	let mut total = 0;
	
	for index in 0..target.len() {
		let ways = if let Some(i) = index.checked_sub(1) {
			buffer[i]
		} else {
			1
		};
		
		if ways > 0 {
			let target = unsafe { target.get_unchecked(index..) };
			
			let mut candidates = reverse(components_by_length);
			let mut length_under = target.len() + 1;
			while let Some(next) = candidates.find(|&s| s.len() < length_under) {
				let add_to = buffer.get_mut(index + next.len() - 1).unwrap_or(&mut total);
				
				if target.starts_with(next) {
					*add_to += ways;
					length_under = next.len();
				}
			}
		}
	}
	
	total
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
	let mut lines = input.lines();
	
	let mut patterns = Vec::with_capacity(500);
	patterns.extend(lines.next().unwrap_or_default().split(", ").filter(|&s| !s.is_empty()));
	patterns.sort_by_key(|&s| s.len());
	
	lines.next();
	
	let mut storage = Vec::with_capacity(128);

	lines.map(|target| ways_to_fill(&target, &patterns, &mut storage)).sum()
}

fn reverse<I: IntoIterator>(i: I) -> iter::Rev<I::IntoIter> where I::IntoIter: DoubleEndedIterator {
	i.into_iter().rev()
}
