use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};

fn try_offset_within(start: (u16, u16), offset: (i32, i32), max: (u16, u16)) -> Option<(u16, u16)> {
	Some((
		(start.0 as u32).checked_add_signed(offset.0).and_then(|r| r.try_into().ok()).filter(|&row| row <= max.0)?,
		(start.1 as u32).checked_add_signed(offset.1).and_then(|c| c.try_into().ok()).filter(|&col| col <= max.0)?
	))
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
	let mut antennae = HashMap::with_capacity_and_hasher(36, FxBuildHasher);
	let (mut max_row, max_col) = (0u16, input.lines().next().unwrap_or_default().len() as u16);
	
	for (row, line) in input.lines().enumerate().map(|(r, l)| (r as u16, l)) {
		max_row = row;
		for (col, c) in line.bytes().enumerate().filter_map(|(c, ch)| (ch != b'.').then(|| (c as u16, ch))) {
			antennae.entry(c).or_insert_with(Vec::new).push((row, col));
		}
	}
	
	let bounds = (max_row, max_col);
	
	let mut antinodes = HashSet::with_capacity_and_hasher(400, FxBuildHasher);

	for (_, positions) in antennae {
		let mut positions = positions.iter();
		while let Some(&a) = positions.next() {
			for &b in positions.clone() {
				let row_delta = b.0 as i32 - a.0 as i32;
				let col_delta = b.1 as i32 - a.1 as i32;
				
				antinodes.extend(try_offset_within(a, (-row_delta, -col_delta), bounds));
				antinodes.extend(try_offset_within(b, (row_delta, col_delta), bounds));
			}
		}
	}
	
	antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
	let mut antennae = HashMap::with_capacity_and_hasher(36, FxBuildHasher);
	let (mut max_row, max_col) = (0u16, input.lines().next().unwrap_or_default().len() as u16);
	
	for (row, line) in input.lines().enumerate().map(|(r, l)| (r as u16, l)) {
		max_row = row;
		for (col, c) in line.bytes().enumerate().filter_map(|(c, ch)| (ch != b'.').then(|| (c as u16, ch))) {
			antennae.entry(c).or_insert_with(Vec::new).push((row, col));
		}
	}
	
	let bounds = (max_row, max_col);
	
	let mut antinodes = HashSet::with_capacity_and_hasher(1500, FxBuildHasher);

	for (_, positions) in antennae {
		if positions.len() <= 1 {
			continue;
		}
		let mut positions = positions.iter();
		while let Some(&a) = positions.next() {
			antinodes.insert(a);
			for &b in positions.clone() {
				let row_delta = b.0 as i32 - a.0 as i32;
				let col_delta = b.1 as i32 - a.1 as i32;
				
				for (mut start, offset) in [(a, (-row_delta, -col_delta)), (b, (row_delta, col_delta))] {
					while let Some(next) = try_offset_within(start, offset, bounds) {
						antinodes.insert(next);
						start = next;
					}
				};
			}
		}
	}
	
	antinodes.len()
}
