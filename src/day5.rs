use std::{collections::{HashMap, HashSet}, hint, iter, mem, num};

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
	let mut lines = input.lines();
	
	let mut ordering_rules: HashMap<u8, HashSet<u8>> = HashMap::new();
	while let Some(rule) = lines.next().filter(|l| !l.is_empty()) {
		match rule.split_once('|').map(|(a, b)| Ok::<_, num::ParseIntError>((a.parse()?, b.parse()?))) {
			Some(Ok((before, after))) => {
				ordering_rules.entry(before).or_insert_with(HashSet::new).insert(after);
			},
			// Some(Err(e)) => eprintln!("[v] Error parsing '|'-separated number in '{rule}': {e}."),
			// None => eprintln!("[v] No bar separator ('|') found in '{rule}'.")
			_ => unsafe { hint::unreachable_unchecked(); }
		}
	}
	
	let ordering_rules = ordering_rules;
	// eprintln!("{ordering_rules:?}");
	
	let mut page_order = Vec::with_capacity(23);
	let mut pages_encountered = HashSet::with_capacity(23);
	lines.filter_map(|line| {
		page_order.clear();
		pages_encountered.clear();
		
		for num in line.split(',') {
			match num.parse() {
				Ok(num) => {
					if let Some(after) = ordering_rules.get(&num) {
						if /* let Some(before) = */ pages_encountered.intersection(after).next().is_some() {
							// eprintln!("[i] {before} came before {num} in line '{line}'.");
							return None;
						}
					}
					
					page_order.push(num);
					pages_encountered.insert(num);
				},
				Err(_) => unsafe {
					// eprintln!("[v] Error parsing page number '{num}' ({e}), skipping line.");
					// return None;
					hint::unreachable_unchecked();
				}
			}
		}
		
		// (!page_order.is_empty()).then(|| page_order[page_order.len() / 2] as u32)
		Some(unsafe { *page_order.get_unchecked(page_order.len() / 2) } as u32)
	}).sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
	let mut lines = input.lines();
	
	let mut ordering_rules: HashMap<u8, HashSet<u8>> = HashMap::new();
	while let Some(rule) = lines.next().filter(|l| !l.is_empty()) {
		match rule.split_once('|').map(|(a, b)| Ok::<_, num::ParseIntError>((a.parse()?, b.parse()?))) {
			Some(Ok((before, after))) => {
				ordering_rules.entry(before).or_insert_with(HashSet::new).insert(after);
			},
			// Some(Err(e)) => eprintln!("[v] Error parsing '|'-separated number in '{rule}': {e}."),
			// None => eprintln!("[v] No bar separator ('|') found in '{rule}'.")
			_ => unsafe { hint::unreachable_unchecked(); }
		}
	}
	
	let ordering_rules = ordering_rules;
	// println!("{ordering_rules:?}");
	
	let mut page_order = Vec::with_capacity(23);
	// let mut pages_encountered = HashMap::with_capacity(23);
	lines.filter_map(|line| {
		page_order.clear();
		// pages_encountered.clear();
		
		let mut swapped = false;
		for num in line.split(',') {
			match num.parse() {
				Ok(mut num) => {
					if let Some(after) = ordering_rules.get(&num) {
						// this feels very overengineered and fragile.
						// if let Some((before, i)) = after.intersection(pages_encountered.keys()).next().map(|n| (n, pages_encountered[&n])) {
						if let Some(prev) = filter(&mut page_order, |&&mut num| after.contains(&num)).next() {
							// eprintln!(" - Swapped '{prev}' and '{num}'.");
							mem::swap(&mut num, prev);
							swapped = true;
						}
					}
					
					// pages_encountered.insert(num, page_order.len());
					page_order.push(num);
				},
				Err(_) => unsafe {
					// eprintln!("[v] Error parsing page number '{num}' ({e}), skipping line.");
					// return None;
					hint::unreachable_unchecked();
				}
			}
		}
		// eprintln!("{page_order:?}");
		
		let mut still_swapped = swapped;
		// let mut iterations = 0;
		while still_swapped {
		//	if { iterations += 1; iterations } > 100 {
		//		eprintln!("[v] Couldn't sort {page_order:?} after 100 iterations (from: '{line}').");
		//		return None;
		//	}
			still_swapped = false;
			for i in 1..page_order.len() {
				if let Some(after) = ordering_rules.get(&page_order[i]) {
					if let Some(j) = enumerate(&page_order).take(i).find_map(|(i, prev)| after.contains(prev).then(|| i)) {
						page_order.swap(i, j);
						still_swapped = true;
					}
				}
			}
		}
		
		swapped.then(|| page_order[page_order.len() / 2] as u32)
	}).sum()
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}

fn filter<I: IntoIterator, P: FnMut(&I::Item) -> bool>(i: I, f: P) -> iter::Filter<I::IntoIter, P> {
	i.into_iter().filter(f)
}
