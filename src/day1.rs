// #![feature(binary_heap_into_iter_sorted, iter_next_chunk)]
use std::{collections::{BinaryHeap, HashMap}, iter};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
	let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());
	
	for line in input.lines() {
		match line.split_whitespace().map(str::parse::<u64>).next_chunk() {
			Ok([Ok(l), Ok(r)]) => {
				left.push(l);
				right.push(r);
			},
			Ok([Err(e), _] | [_, Err(e)]) => eprintln!("[v] Error parsing number in '{line}': {e}."),
			Err(_) => eprintln!("[v] Did not find two numbers in '{line}'.")
		}
	}
	
	iter::zip(left.into_iter_sorted(), right.into_iter_sorted()).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
	let (mut left, mut right) = (HashMap::new(), HashMap::new());
	
	for line in input.lines() {
		match line.split_whitespace().map(str::parse::<u64>).next_chunk() {
			Ok([Ok(l), Ok(r)]) => {
				*left.entry(l).or_insert(0) += 1;
				*right.entry(r).or_insert(0) += 1;
			},
			Ok([Err(e), _] | [_, Err(e)]) => eprintln!("[v] Error parsing number in '{line}': {e}."),
			Err(_) => eprintln!("[v] Did not find two numbers in '{line}'.")
		}
	}
	
	map(left, |(value, occurances)| value * occurances * right.get(&value).copied().unwrap_or(0)).sum()
}

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(iterable: I, f: F) -> iter::Map<I::IntoIter, F> {
	iterable.into_iter().map(f)
}
