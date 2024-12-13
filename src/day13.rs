// #![feature(iter_next_chunk)]
use std::num;

#[derive(Copy, Eq, Clone, PartialEq)]
struct Position {
	pub x: u64,
	pub y: u64
}

fn try_extract_pos(line: &str) -> Result<Position, Option<num::ParseIntError>> {
	let (x, y) = line.split_once(',').ok_or(None)?;
	let (x, y) = (x.trim_matches(|c: char| !c.is_ascii_digit()), y.trim_matches(|c: char| !c.is_ascii_digit()));
	
	Ok(Position { x: x.parse()?, y: y.parse()? })
}

// 94a + 22b = 8400
// 34a + 67b = 5400
// 
// b = (8400 - 94a)/22
// 
// 34a + 67(8400 - 94a)/22 = 5400
// 
// 34a + (562800 - 6298a)/22 = 5400
// 748a + 562800 - 6298a = 118800
// 
// 444000 = 5550a
// 
// a = 80
// 
// b = (8400 - 94(80))/22 = 40
// 
// 
// [ax]a + [by]b = [px]
// [ay]a + [by]b = [py]
// 
// b = ([px] - [ax]a)/[bx]
// 
// [ay]a + [by]([px] - [ax]a)/[bx] = [py]
// [bx][ay]a + [by][px] - [by][ax]a = [py][bx]
// ([bx][ay] - [by][ax])a = [py][bx] - [by][px]
// a = ([py][bx] - [by][px])/([bx][ay] - [by][ax])

fn div_exact(dividend: u64, divisor: u64) -> Option<u64> {
	(divisor != 0 && dividend % divisor == 0).then(|| dividend / divisor)
}

// Does not (currently) account for infinite/multiple solutions or negative solutions (or overflow/intermediate values ≥ 2⁶⁴).
// Does technically account for parallel (non-collinear) lines/no solution.
fn tokens_to_win(button_a: Position, button_b: Position, prize: Position) -> Option<u64> {
	let (a, b) = (button_a, button_b);
	let a_times = div_exact((prize.y * b.x).abs_diff(prize.x * b.y), (b.x * a.y).abs_diff(a.x * b.y))?;
	let b_times = div_exact(prize.x.abs_diff(a.x * a_times), button_b.x)?;
	
	Some(a_times * 3 + b_times)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u64 {
	let mut lines = input.lines();
	
	let mut tokens = 0;
	loop {
		match lines.next_chunk() {
			Ok([button_a, button_b, prize]) => {
				match (try_extract_pos(&button_a), try_extract_pos(&button_b), try_extract_pos(&prize)) {
					(Ok(button_a), Ok(button_b), Ok(prize)) => {
						tokens += tokens_to_win(button_a, button_b, prize).unwrap_or(0);
					},
					_ => unsafe { std::hint::unreachable_unchecked() }
				}
			},
			Err(_) => break
		}
		
		lines.next();
	}
	
	tokens
}


#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
	let mut lines = input.lines();
	
	let mut tokens = 0;
	loop {
		match lines.next_chunk() {
			Ok([button_a, button_b, prize]) => {
				match (try_extract_pos(&button_a), try_extract_pos(&button_b), try_extract_pos(&prize)) {
					(Ok(button_a), Ok(button_b), Ok(prize)) => {
						let prize = Position { x: prize.x + 10000000000000, y: prize.y + 10000000000000 };
						
						tokens += tokens_to_win(button_a, button_b, prize).unwrap_or(0);
					},
					_ => unsafe { std::hint::unreachable_unchecked() }
				}
			},
			Err(_) => break
		}
		
		lines.next();
	}
	
	tokens
}
