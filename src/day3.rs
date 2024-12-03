#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total = 0;
	for candidate in input.split("mul(").skip(1) {
		if let Some((first, rest)) = candidate.split_once(',') {
			if let Some((second, _)) = rest.split_once(')') {
				if let Ok((first, second)) = first.parse().and_then(|f| Ok((f, second.parse()?))) {
					let (_, _): (u32, u32) = (first, second);
					total += first as u64 * second as u64;
				}
			}
		}
	}
	
	total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
	let mut total = 0;
	let mut remaining = input;
	
	while !remaining.is_empty() {
		let (yes, no) = remaining.split_once("don't()").unwrap_or((remaining, ""));
		
		total += part1(yes);
		
		let (_, yes) = no.split_once("do()").unwrap_or((no, ""));
		remaining = yes;
	}
	
	total
}