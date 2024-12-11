use std::mem;

use rustc_hash::{FxBuildHasher, FxHashMap as HashMap};

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u32 {
	let mut current = HashMap::with_capacity_and_hasher(800, FxBuildHasher);
	
	for num in input.split_whitespace() {
		let num: u64 = unsafe { num.parse().unwrap_unchecked() };
		*current.entry(num).or_insert(0u32) += 1;
	}
	
	let mut target = HashMap::with_capacity_and_hasher(800, FxBuildHasher);
	
	for _ in 0..25 {
		// target.clear();
		for (num, times) in current.drain() {
			if let Some(digits_minus_one) = num.checked_ilog10() {
				if digits_minus_one % 2 == 0 { // Odd digit count.
					*target.entry(num * 2024).or_insert(0) += times;
				} else { // Even digit count.
					let pow = 10u64.pow((digits_minus_one + 1) / 2);
					for half in [num / pow, num % pow] {
						*target.entry(half).or_insert(0) += times;
					}
				}
			} else {
				*target.entry(1).or_insert(0) += times;
			}
		}
		mem::swap(&mut target, &mut current);
	}
	
	current.into_values().sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
	let mut current = HashMap::with_capacity_and_hasher(4000, FxBuildHasher);
	
	for num in input.split_whitespace() {
		let num: u64 = unsafe { num.parse().unwrap_unchecked() };
		*current.entry(num).or_insert(0u64) += 1;
	}
	
	let mut target = HashMap::with_capacity_and_hasher(4000, FxBuildHasher);
	
	for _ in 0..75 {
		for (num, times) in current.drain() {
			if let Some(digits_minus_one) = num.checked_ilog10() {
				if digits_minus_one % 2 == 0 { // Odd digit count.
					*target.entry(num * 2024).or_insert(0) += times;
				} else { // Even digit count.
					let pow = 10u64.pow((digits_minus_one + 1) / 2);
					for half in [num / pow, num % pow] {
						*target.entry(half).or_insert(0) += times;
					}
				}
			} else {
				*target.entry(1).or_insert(0) += times;
			}
		}
		mem::swap(&mut target, &mut current);
	}
	
	current.into_values().sum()
}
