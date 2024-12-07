use std::num::NonZero;

fn parse_line<'b>(line: &str, buffer: &'b mut Vec<NonZero<u64>>) -> Option<(NonZero<u64>, NonZero<u64>, &'b [NonZero<u64>])> {
	let (target, values) = line.split_once(':')?;
		
	let target = target.parse().ok()?;
	buffer.clear();
	for value in values.split_whitespace().map(str::parse).map(Result::ok) {
		buffer.push(value?);
	}
	
	let (&first, remaining) = buffer.split_first()?;
	Some((target, first, remaining))
}

fn check_for_match_from_end_1(target: NonZero<u64>, remaining: &[NonZero<u64>], current: NonZero<u64>) -> bool {
	match remaining.split_last() {
		None => current == target,
		Some((&next, remaining)) => current.get().checked_sub(next.get()).and_then(NonZero::new).is_some_and(|diff| check_for_match_from_end_1(target, remaining, diff)) ||
				(current.get() % next == 0 && check_for_match_from_end_1(target, remaining, unsafe { NonZero::new_unchecked(current.get() / next) } ))
	}
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
	let mut buffer = Vec::with_capacity(10);
	input.lines().filter_map(|line| {
	//	let (target, values) = line.split_once(':').unwrap();
	//	let start = match target.parse().unwrap();
	//	let values: Vec<NonZero<u64>> = Result::unwrap(values.split_whitespace().map(str::parse).collect());
	//	let (&target, remaining) = values.split_first().unwrap();
		let (start, target, remaining) = unsafe { parse_line(line, &mut buffer).unwrap_unchecked() };
		check_for_match_from_end_1(target, remaining, start).then(|| start.get())
	}).sum()
}

fn deconcatenate(from: u64, remove: NonZero<u64>) -> Option<u64> {
	const TEN: NonZero<u64> = NonZero::new(10).unwrap();
	
	let multiplier = TEN.checked_pow(unsafe { NonZero::new_unchecked(remove.ilog10() + 1) }.into())?;
	(from % multiplier == remove.get()).then(|| from / multiplier)
}

fn check_for_match_from_end_2(target: NonZero<u64>, remaining: &[NonZero<u64>], current: NonZero<u64>) -> bool {
	match remaining.split_last() {
		None => current == target,
		Some((&next, remaining)) => current.get().checked_sub(next.get()).and_then(NonZero::new).is_some_and(|diff| check_for_match_from_end_2(target, remaining, diff)) ||
				(current.get() % next == 0 && check_for_match_from_end_2(target, remaining, unsafe { NonZero::new_unchecked(current.get() / next) } )) ||
				deconcatenate(current.get(), next).and_then(NonZero::new).is_some_and(|prefix| check_for_match_from_end_2(target, remaining, prefix))
	}
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
	let mut buffer = Vec::with_capacity(10);
	input.lines().filter_map(|line| {
		let (start, target, remaining) = unsafe { parse_line(line, &mut buffer).unwrap_unchecked() };
		check_for_match_from_end_2(target, remaining, start).then(|| start.get())
	}).sum()
}

fn concatenate(left: NonZero<u64>, right: NonZero<u64>) -> Option<NonZero<u64>> {
	const TEN: NonZero<u64> = NonZero::new(10).unwrap();
	
	let multiplier = TEN.checked_pow(unsafe { NonZero::new_unchecked(right.ilog10() + 1) }.into())?;
	left.checked_mul(multiplier)?.checked_add(right.get())
}

#[allow(unused)]
fn check_for_match(target: NonZero<u64>, current: NonZero<u64>, remaining: &[NonZero<u64>]) -> bool {
	match remaining.split_first() {
		None => current == target,
		Some(_) if current > target => false,
		Some((&next, remaining)) => current.checked_add(next.get()).is_some_and(|sum| check_for_match(target, sum, remaining)) ||
				current.checked_mul(next).is_some_and(|product| check_for_match(target, product, remaining)) ||
				concatenate(current, next).is_some_and(|result| check_for_match(target, result, remaining))
	}
}