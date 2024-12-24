use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};

fn next(mut secret: u32) -> u32 {
	secret = (secret ^ (secret << 6)) & 0xFFFFFF;
	secret = (secret ^ (secret >> 5)) & 0xFFFFFF;
	(secret ^ (secret << 11)) & 0xFFFFFF
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
	input.lines().map(|line| {
		let mut secret = unsafe { line.parse().unwrap_unchecked() };
		for _ in 0..2000 {
			secret = next(secret);
		}
		secret as u64
	}).sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
	let mut counts = HashMap::with_capacity_and_hasher(41000, FxBuildHasher);
	let mut max_bananas = 0;
	let mut seen_this_monkey = HashSet::with_capacity_and_hasher(3000, FxBuildHasher);
	
	input.lines().map(|line| unsafe { line.parse().unwrap_unchecked() }).for_each(|init| {
		seen_this_monkey.clear();
		let mut sequence = [i8::MIN, i8::MIN, i8::MIN, i8::MIN];
		let mut previous = (init % 10) as i8;
		let mut value = init;
		for _ in 0..=2000 {
			value = next(value);
			let bananas = (value % 10) as i8;
			sequence.rotate_left(1);
			sequence[3] = bananas - previous;
			previous = bananas;
			if sequence[0] != i8::MIN {
				if seen_this_monkey.insert(sequence) {
					let count = counts.entry(sequence).or_insert(0u32);
					*count += bananas as u32;
					max_bananas = max_bananas.max(*count);
				}
			}
		}
	});
	
	max_bananas
}


#[allow(unused)]
pub fn part2_30_minute_bruteforce(input: &str) -> u32 {
	let initial_secrets: Vec<u32> = input.lines().filter_map(|line| {
		match line.parse() {
			Ok(secret) => Some(secret),
			Err(error) => {
				eprintln!("[v] Invalid number: '{line}' ({error})");
				None
			} 
		}
	}).collect();
	
	let mut max_bananas = 0u32;
	for a in -9..=9 {
		for b in -9..=9 {
			for c in -9..=9 {
				for d in -9..=9 {
					if is_possible_sequence(a, b, c, d) {
						let target = [a, b, c, d];
						max_bananas = max_bananas.max(
							map(&initial_secrets, |&init| {
								let mut sequence = [i8::MIN, i8::MIN, i8::MIN, i8::MIN];
								let mut previous = (init % 10) as i8;
								let mut value = init;
								for _ in 0..=2000 {
									value = next(value);
									let bananas = (value % 10) as i8;
									sequence.rotate_left(1);
									sequence[3] = bananas - previous;
									previous = bananas;
									if sequence == target {
										//	if target == [-2, 1, -1, 3] {
										//		eprintln!("[i] Got {bananas} bananas from monkey {init}.");
										//	}
										
										return bananas as u32;
									}
								}
								0
							}).sum()
						);
					}
				}
			}
		}
	}

	max_bananas
}

fn is_possible_sequence(a: i8, b: i8, c: i8, d: i8) -> bool {
	for mut value in 0..=9 {
		value += a;
		if !(0..=9).contains(&value) { continue; }
		value += b;
		if !(0..=9).contains(&value) { continue; }
		value += c;
		if !(0..=9).contains(&value) { continue; }
		value += d;
		if !(0..=9).contains(&value) { continue; }
		return true;
	}
	
	false
}

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(i: I, f: F) -> std::iter::Map<I::IntoIter, F> {
	i.into_iter().map(f)
}
