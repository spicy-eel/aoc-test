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
			let mut enclosed = 0;
			
			// There are 7 characters between the edge of the tree's frame and the outermost "enclosed" pixel horizontally.
			// There are actually only 6 vertically, but this'll still skip at most two of the 161 in the picture, which still leaves much more than 100.
			// (Looking for 100 is probably overly conservative, too, seeing as the only time my input produces "enclosed" pixels at all is when the tree appears.)
			for (row, contents) in enumerate(&picture[..(HEIGHT - 7) as usize]).skip(7) {
				let adj_rows = [&picture[row - 1], &picture[row + 1]];
				for col in enumerate(&contents[..(WIDTH - 7) as usize]).skip(7).filter_map(|(i, &b)| b.then(|| i)) {
					if map([col - 1, col + 1], |i| contents[i]).chain(map(adj_rows, |r| r[col])).all(convert::identity) {
						enclosed += 1;
						if enclosed >= 100 { // Should be exactly 161 in tree.
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

fn map<I: IntoIterator, O, F: FnMut(I::Item) -> O>(i: I, f: F) -> iter::Map<I::IntoIter, F> {
	i.into_iter().map(f)
}

