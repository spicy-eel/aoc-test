use std::{convert, iter, mem};

#[derive(Copy, Clone)]
struct Robot {
	x: u32,
	y: u32,
	vx: i32,
	vy: i32
}


fn try_parse_robot(line: &str) -> Option<Robot> {
	let (p, v) = line.strip_prefix("p=")?.split_once(" v=")?;
	let ((px, py), (vx, vy)) = (p.split_once(',')?, v.split_once(',')?);
	
	Some(Robot { x: px.parse().ok()?, y: py.parse().ok()?, vx: vx.parse().ok()?, vy: vy.parse().ok()? })
}

const WIDTH: u32 = 101;
const HEIGHT: u32 = 103;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
	let [mut tl, mut tr, mut bl, mut br] = [0u32; 4];
	
	for line in input.lines() {
		let robot = unsafe { try_parse_robot(&line).unwrap_unchecked() };
		
		let end_x = (robot.x as i64 + (robot.vx as i64 * 100)).rem_euclid(WIDTH as i64) as u32;
		let end_y = (robot.y as i64 + (robot.vy as i64 * 100)).rem_euclid(HEIGHT as i64) as u32;
		
		if end_x < WIDTH / 2 {
			if end_y < HEIGHT / 2 {
				tl += 1;
			} else if end_y > HEIGHT / 2 {
				bl += 1;
			}
		} else if end_x > WIDTH / 2 {
			if end_y < HEIGHT / 2 {
				tr += 1;
			} else if end_y > HEIGHT / 2 {
				br += 1;
			}
		}
	}
	
	tl * tr * bl * br
}


#[aoc(day14, part2)]
pub fn part2(input: &str) -> u32 {
	let mut robots = Vec::with_capacity(500);
	
	for line in input.lines() {
		let robot = unsafe { try_parse_robot(&line).unwrap_unchecked() };
		
		robots.push(robot);
	}
	
	let mut picture = [[false; WIDTH as usize]; HEIGHT as usize];
	
	for iteration in 3000.. {
		let mut had_overlap = false;
		for &robot in &robots {
			let x = (robot.x as i64 + robot.vx as i64 * iteration as i64).rem_euclid(WIDTH as i64) as usize;
			let y = (robot.y as i64 + robot.vy as i64 * iteration as i64).rem_euclid(HEIGHT as i64) as usize;
			
			
			if mem::replace(&mut picture[y][x], true) {
				had_overlap = true;
				break;
			};
		}
		
		if !had_overlap {
			let mut adjacencies = 0;
			for (row, contents) in enumerate(&picture) {
				let (adjacent_rows, max_index) = if row == 0 {
					([&picture[1], &picture[1]], 0)
				} else if row == HEIGHT as usize - 1 {
					([&picture[HEIGHT as usize - 2], &picture[HEIGHT as usize - 2]], 0)
				} else {
					([&picture[row - 1], &picture[row + 1]], 1)
				};
				
				let adjacent_rows = &adjacent_rows[..=max_index];
				
				for col in enumerate(contents).filter_map(|(i, &b)| b.then(|| i)) {
					if map(adjacent_rows, |r| r[col]).chain(filter_map([col.wrapping_sub(1), col + 1], |i| contents.get(i).copied())).any(convert::identity) {
						adjacencies += 1;
						if adjacencies >= 353 {
							return iteration;
						}
					}
				}
			}
		}
		
		picture.as_flattened_mut().fill(false);
	}
	
	u32::MAX
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}

fn filter_map<I: IntoIterator, O, F: FnMut(I::Item) -> Option<O>>(i: I, f: F) -> iter::FilterMap<I::IntoIter, F> {
	i.into_iter().filter_map(f)
}

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::Map<I::IntoIter, F> {
	i.into_iter().map(f)
}

