use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};

use std::{hash::Hash, iter};

fn with_default_capacity<T: Hash>() -> HashSet<T> {
	HashSet::with_capacity_and_hasher(13, FxBuildHasher)
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
	let mut connections = HashMap::with_capacity_and_hasher(520, FxBuildHasher);
	let mut triplets = 0;
	
	for line in input.lines() {
		let &[a1, a2, b'-', b1, b2] = line.as_bytes() else { unsafe { std::hint::unreachable_unchecked() } };
		let (a, b) = ([a1, a2], [b1, b2]);
		connections.entry(a).or_insert_with(with_default_capacity).insert(b);
		connections.entry(b).or_insert_with(with_default_capacity).insert(a);
		
		let outer_t = a[0] == b't' || b[0] == b't';
		triplets += connections[&a].intersection(&connections[&b]).filter(|&c| outer_t || c[0] == b't').count();
	}
	
	triplets
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
	let mut connections = HashMap::with_capacity_and_hasher(520, FxBuildHasher);
	for line in input.lines() {
		let &[a1, a2, b'-', b1, b2] = line.as_bytes() else { unsafe { std::hint::unreachable_unchecked() } };
		let (a, b) = ([a1, a2], [b1, b2]);
		connections.entry(a).or_insert_with(with_default_capacity).insert(b);
		connections.entry(b).or_insert_with(with_default_capacity).insert(a);
	}
	
	let mut largest = Vec::with_capacity(13);
	let mut stack = Vec::with_capacity(12);
	let mut seen = HashSet::with_capacity_and_hasher(connections.len(), FxBuildHasher);
	for (&node, connected) in &connections {
		inner_thing(&connections, node, &mut stack, filter(connected, |&pc| !seen.contains(pc)).copied(), &mut largest);
		seen.insert(node); // We've now checked every possible network containing 'node', so no need to include it in any future potential networks.
	}
	
	largest.sort();
	unsafe { String::from_utf8_unchecked(largest.join(&b',')) }
}

fn inner_thing<I: Iterator<Item = [u8; 2]> + Clone>(connections: &HashMap<[u8; 2], HashSet<[u8; 2]>>,
		start: [u8; 2], stack: &mut Vec<[u8; 2]>, mut iter: I, largest: &mut Vec<[u8; 2]>) {
	while let Some(next) = iter.size_hint().1.is_none_or(|upper| stack.len() + upper >= largest.len()).then(|| iter.next()).flatten() {
		let connected = &connections[&next];
		if all(&*stack, |prev| connected.contains(prev)) {
			stack.push(next);
			
			inner_thing(connections, start, stack, iter.clone(), largest);
			
			if stack.len() >= largest.len() {
				largest.clear();
				largest.extend(iter::once(&start).chain(&*stack).copied());
			}
			stack.pop();
		}
	}
}

fn all<I: IntoIterator, P: FnMut(I::Item) -> bool>(i: I, f: P) -> bool {
	i.into_iter().all(f)
}

fn filter<I: IntoIterator, P: FnMut(&I::Item) -> bool>(i: I, f: P) -> iter::Filter<I::IntoIter, P> {
	i.into_iter().filter(f)
}
