use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Copy, Eq, Clone, PartialEq, Debug)]
enum Operator { And, Or, Xor }

#[derive(Copy, Eq, Clone, PartialEq, Debug)]
struct Gate {
	pub a: [u8; 3],
	op: Operator,
	pub b: [u8; 3]
}

#[allow(unused)]
#[derive(Copy, Eq, Clone, PartialEq)]
enum Wire {
	Fixed(bool),
	Gate(Gate)
}

enum GetError {
	Undefined([u8; 3]),
	LoopAt([u8; 3])
}

fn get(id: [u8; 3], gates: &HashMap<[u8; 3], Gate>, values: &mut HashMap<[u8; 3], bool>, checking: &mut HashSet<[u8; 3]>) -> Result<bool, GetError> {
	if let Some(&value) = values.get(&id) {
		return Ok(value);
	}
	
	if !checking.insert(id) {
		return Err(GetError::LoopAt(id));
	}
	
	let Gate{ a, op, b } = *gates.get(&id).ok_or_else(|| GetError::Undefined(id))?;
	let value = match op {
		Operator::And => get(a, gates, values, checking)? && get(b, gates, values, checking)?,
		Operator::Or => get(a, gates, values, checking)? || get(b, gates, values, checking)?,
		Operator::Xor => get(a, gates, values, checking)? ^ get(b, gates, values, checking)?,
	};
	checking.remove(&id);
	values.insert(id, value);
	Ok(value)
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u128 {
	let mut gates = HashMap::default();
	let mut values = HashMap::default();
	
	// let mut undefined = HashSet::new();
	
	for line in input.lines() {
		let (id, wire) = match line.as_bytes() {
			&[a1, a2, a3, b' ', b'A', b'N', b'D', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
				([o1, o2, o3], Wire::Gate(Gate{ a: [a1, a2, a3], op: Operator::And, b: [b1, b2, b3] })),
			&[a1, a2, a3, b' ', b'O', b'R', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
					([o1, o2, o3], Wire::Gate(Gate{ a: [a1, a2, a3], op: Operator::Or, b: [b1, b2, b3] })),
			&[a1, a2, a3, b' ', b'X', b'O', b'R', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
					([o1, o2, o3], Wire::Gate(Gate{ a: [a1, a2, a3], op: Operator::Xor, b: [b1, b2, b3] })),
			&[o1, o2, o3, b':', b' ', b'0'] => ([o1, o2, o3], Wire::Fixed(false)),
			&[o1, o2, o3, b':', b' ', b'1'] => ([o1, o2, o3], Wire::Fixed(true)),
			_ => { continue },
		//	_ => {
		//		eprintln!("[v] Line did not match expected pattern: '{line}'");
		//		continue
		//	}
		};
		
		// undefined.remove(&id);
		
		match wire {
			Wire::Fixed(v) => {
				values.insert(id, v);
			},
			Wire::Gate(gate) => {
				gates.insert(id, gate);
				//	for id in [gate.a, gate.b] {
				//		if gates.get(&id).is_none() && values.get(&id).is_none() {
				//			undefined.insert(id);
				//		}
				//	}
			}
		}
	}
	
	//	if !undefined.is_empty() {
	//		eprintln!("[!] Wire(s) referenced but not defined (shown here as byte values for reasons™): {undefined:?}");
	//		return;
	//	}
	
	let mut output = 0u128;
	let mut seen = HashSet::default();
	for z in 0..100 {
		let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
		seen.clear();
		match get(zid, &gates, &mut values, &mut seen) {
			Ok(v) => { //output = (output << 1) | u128::from(v),
				output |= u128::from(v) << z as u32;
			},
			Err(GetError::Undefined(id)) => {
				// eprintln!("[i] no value for {}{}{}", id[0] as char, id[1] as char, id[2] as char);
				debug_assert_eq!(zid, id);
				break;
			},
			Err(GetError::LoopAt(_id)) => unreachable!() // {
			//		let z = unsafe { std::str::from_utf8_unchecked(&zid) };
			//		let id = unsafe { std::str::from_utf8_unchecked(&id) };
			//		
			//		eprintln!("[!] Encountered loop while getting value for '{z}': value of gate '{id}' depended on itself.");
			//		break;
			//	}
		}
	}

	output
}

#[derive(Copy, Eq, Clone, PartialEq)]
enum GetOr0Error {
	LoopAt([u8; 3])
}

fn get_or_0(id: [u8; 3], gates: &HashMap<[u8; 3], Gate>, overrides: &HashMap<[u8; 3], Gate>,
		values: &mut HashMap<[u8; 3], bool>, seen: &mut HashSet<[u8; 3]>) -> Result<bool, GetOr0Error> {
	if let Some(&value) = values.get(&id) {
		return Ok(value);
	}
	
	let Some(&Gate{ a, op, b }) = overrides.get(&id).or_else(|| gates.get(&id)) else { return Ok(false) };
	if !seen.insert(id) {
		return Err(GetOr0Error::LoopAt(id));
	}
	
	let value = match op {
		Operator::And => get_or_0(a, gates, overrides, values, seen)? && get_or_0(b, gates, overrides, values, seen)?,
		Operator::Or => get_or_0(a, gates, overrides, values, seen)? || get_or_0(b, gates, overrides, values, seen)?,
		Operator::Xor => get_or_0(a, gates, overrides, values, seen)? ^ get_or_0(b, gates, overrides, values, seen)?,
	};
	seen.remove(&id);
	values.insert(id, value);
	Ok(value)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
	let mut gates = HashMap::default();
	let mut values = HashMap::default();
	
	for line in input.lines() {
		let (id, gate) = match line.as_bytes() {
			&[a1, a2, a3, b' ', b'A', b'N', b'D', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
				([o1, o2, o3], Gate{ a: [a1, a2, a3], op: Operator::And, b: [b1, b2, b3] }),
			&[a1, a2, a3, b' ', b'O', b'R', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
					([o1, o2, o3], Gate{ a: [a1, a2, a3], op: Operator::Or, b: [b1, b2, b3] }),
			&[a1, a2, a3, b' ', b'X', b'O', b'R', b' ', b1, b2, b3, b' ', b'-', b'>', b' ', o1, o2, o3] =>
					([o1, o2, o3], Gate{ a: [a1, a2, a3], op: Operator::Xor, b: [b1, b2, b3] }),
			_ => continue
		};
		
		gates.insert(id, gate);
	}
	
	// for kind in ['x', 'y'] { println!("{kind}:");
	let mut potential_swaps = Vec::new();
	let mut seen = HashSet::default();
	for i in 0..=44 {
		let xid = [b'x', i / 10 + b'0', i % 10 + b'0'];
		values.clear();
		values.insert(xid, true);
		
		let zid = [b'z', i / 10 + b'0', i % 10 + b'0'];
		seen.clear();
		match get_or_0(zid, &gates, &HashMap::default(), &mut values, &mut seen) {
			Ok(v) => { //output = (output << 1) | u128::from(v),
				if !v {
					let mut entries = gates.iter();
					let mut test = HashMap::with_capacity_and_hasher(2, FxBuildHasher);
					while let Some((&first, &gate1)) = entries.next() {
						'outer: for (&second, &gate2) in entries.clone() {
							test.clear();
							test.extend([(second, gate1), (first, gate2)]);
							
							for (x_test, y_test) in [(false, true), (true, false), (true, true)] {
								let yid = [b'y', i / 10 + b'0', i % 10 + b'0'];
								seen.clear();
								values.clear();
								values.insert(xid, x_test);
								values.insert(yid, y_test);
								
								let z_expected = x_test ^ y_test;
								if get_or_0(zid, &gates, &test, &mut values, &mut seen) != Ok(z_expected) {
									// println!(" - test? ({test:?})");
									// println!(" - test? (swapping {} and {} / x, y = {x_test}, {y_test})", )
									continue 'outer;
								}
								
								let i2 = i + 1;
								let z2_id = [b'z', i2 / 10 + b'0', i2 % 10 + b'0'];
								let z2_expected = x_test && y_test;
								if get_or_0(z2_id, &gates, &test, &mut values, &mut seen) != Ok(z2_expected) {
									continue 'outer;
								}
								
								let expected = (u64::from(x_test) << i) + (u64::from(y_test) << i);
								let mut output = 0u64;
								seen.clear();
								values.clear();
								values.insert(xid, x_test);
								values.insert(yid, y_test);
								for z in 0..=45 {
									let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
									let Ok(v) = get_or_0(zid, &gates, &test, &mut values, &mut seen) else {
										continue 'outer
									};
									output |= u64::from(v) << z;
								}
								if output != expected {
									continue 'outer;
								}
							}
							
							potential_swaps.push([(first, gate2), (second, gate1)]);
						//	println!("Swapping {} and {} seems to fix i = {i}?",
						//			unsafe { std::str::from_utf8_unchecked(&first) }, unsafe { std::str::from_utf8_unchecked(&second) });
						}
					}
					// return;
				}
			},
			Err(GetOr0Error::LoopAt(_id)) => unreachable!()
		}
		// println!("{} {}: {expected} -> {output}", if expected == output { ' ' } else { '!' }, unsafe { std::str::from_utf8_unchecked(&aid) });
	}
	
	let mut swapping: Vec<[u8; 3]> = Vec::with_capacity(8);
	let mut test = HashMap::with_capacity_and_hasher(8, FxBuildHasher);
	for (i, &[(s1, s1to), (s2, s2to)]) in potential_swaps.iter().enumerate() {
		let after = &potential_swaps[(i + 1)..];
		for (i, &[(s3, s3to), (s4, s4to)]) in after.iter().enumerate() {
			let after = &after[(i + 1)..];
			for (i, &[(s5, s5to), (s6, s6to)]) in after.iter().enumerate() {
				let after = &after[(i + 1)..];
				for (_, &[(s7, s7to), (s8, s8to)]) in after.iter().enumerate() {
					swapping.clear();
					swapping.extend([s1, s2, s3, s4, s5, s6, s7, s8]);
					swapping.sort();
					swapping.dedup();
					if swapping.len() == 8 {
						test.clear();
						test.extend([(s1, s1to), (s2, s2to), (s3, s3to), (s4, s4to), (s5, s5to), (s6, s6to), (s7, s7to), (s8, s8to)]);
						
						if test_thing(&gates, &test, &mut values, &mut seen) {
							// let thing: Vec<u8> = swapping.iter().copied().flatten().collect();
							return unsafe { String::from_utf8_unchecked(swapping.join(&b',')) };
						}
					}
				}
			}
		}
	}
	
	unsafe { String::from_utf8_unchecked(swapping.join(&b',')) }
}

fn test_thing(gates: &HashMap<[u8; 3], Gate>, overrides: &HashMap<[u8; 3], Gate>, values: &mut HashMap<[u8; 3], bool>, seen: &mut HashSet<[u8; 3]>) -> bool {
	for kind in ['x', 'y'] {
		// println!("{kind}:");
		for a in 0..=44 {
			seen.clear();
			values.clear();
			let aid = [kind as u8, a / 10 + b'0', a % 10 + b'0'];
			values.insert(aid, true);
			
			let mut output = 0u64;
			for z in 0..=45 {
				let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
				match get_or_0(zid, gates, overrides, values, seen) {
					Ok(v) => { //output = (output << 1) | u128::from(v),
						output |= u64::from(v) << z as u32;
					},
					Err(GetOr0Error::LoopAt(_)) => return false
				}
			}
			
			let expected = 1u64 << a; 
			if output != expected { return false; }
		}
	}
	
//	for x in 0..=44 {
//		for y in 0..=44 {
//			seen.clear();
//			values.clear();
//			let xid = [b'x', x / 10 + b'0', x % 10 + b'0'];
//			let yid = [b'y', y / 10 + b'0', y % 10 + b'0'];
//			values.insert(xid, true);
//			values.insert(yid, true);
//			
//			let mut output = 0u64;
//			for z in 0..=45 {
//				let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
//				match get_or_0(zid, gates, overrides, values, seen) {
//					Ok(v) => { //output = (output << 1) | u128::from(v),
//						output |= u64::from(v) << z as u32;
//					},
//					Err(GetOr0Error::LoopAt(_)) => return false
//				}
//			}
//			let expected = (1u64 << x) + (1u64 << y); 
//			if output != expected { return false; }	
//		}
//	}
	for a in 0..=44 {
		seen.clear();
		values.clear();
		let xid = [b'x', a / 10 + b'0', a % 10 + b'0'];
		let yid = [b'y', a / 10 + b'0', a % 10 + b'0'];
		values.insert(xid, true);
		values.insert(yid, true);
		
		let mut output = 0u64;
		for z in 0..=45 {
			let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
			match get_or_0(zid, gates, overrides, values, seen) {
				Ok(v) => { //output = (output << 1) | u128::from(v),
					output |= u64::from(v) << z as u32;
				},
				Err(GetOr0Error::LoopAt(_)) => return false
			}
		}
		
		let expected = 1u64 << (a + 1); 
		if output != expected { return false; }
	}
	
	seen.clear();
	values.clear();
	for a in 0..=44 {
		let xid = [b'x', a / 10 + b'0', a % 10 + b'0'];
		let yid = [b'y', a / 10 + b'0', a % 10 + b'0'];
		values.insert(xid, true);
		values.insert(yid, true);
	}
	{
		let mut output = 0u64;
		for z in 0..=45 {
			let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
			match get_or_0(zid, gates, overrides, values, seen) {
				Ok(v) => { //output = (output << 1) | u128::from(v),
					output |= u64::from(v) << z as u32;
				},
				Err(GetOr0Error::LoopAt(_)) => return false
			}
		}
	
		let expected = ((1u64 << 45) - 1) * 2;
		if output != expected { return false; }
	}
	
	for x in 0..=44 {
		seen.clear();
		values.clear();
		let xid = [b'x', x / 10 + b'0', x % 10 + b'0'];
		values.insert(xid, true);
		for y in 0..=44 {
			let yid = [b'y', y / 10 + b'0', y % 10 + b'0'];
			values.insert(yid, true);
		}
		
		
		let mut output = 0u64;
		for z in 0..=45 {
			let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
			match get_or_0(zid, gates, overrides, values, seen) {
				Ok(v) => { //output = (output << 1) | u128::from(v),
					output |= u64::from(v) << z as u32;
				},
				Err(GetOr0Error::LoopAt(_)) => return false
			}
		}
		
		let expected = ((1u64 << 45) - 1) + (1u64 << x); 
		if output != expected { return false; }
	}
	
	for y in 0..=44 {
		seen.clear();
		values.clear();
		let yid = [b'y', y / 10 + b'0', y % 10 + b'0'];
		values.insert(yid, true);
		for x in 0..=44 {
			let xid = [b'x', x / 10 + b'0', x % 10 + b'0'];
			values.insert(xid, true);
		}
		
		
		let mut output = 0u64;
		for z in 0..=45 {
			let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
			match get_or_0(zid, gates, overrides, values, seen) {
				Ok(v) => { //output = (output << 1) | u128::from(v),
					output |= u64::from(v) << z as u32;
				},
				Err(GetOr0Error::LoopAt(_)) => return false
			}
		}
		
		let expected = ((1u64 << 45) - 1) + (1u64 << y); 
		if output != expected { return false; }
	}
	
	//	for x in 0..=44 {
	//		seen.clear();
	//		values.clear();
	//		let xid = [b'x', a / 10 + b'0', a % 10 + b'0'];
	//		values.insert(xid, true);
	//		let yid = [b'y', a / 10 + b'0', a % 10 + b'0'];
	//		values.insert(yid, true);
	//		
	//		let mut output = 0u64;
	//		for z in 0..=45 {
	//			let zid = [b'z', z / 10 + b'0', z % 10 + b'0'];
	//			match get_or_0(zid, gates, overrides, values, seen) {
	//				Ok(v) => { //output = (output << 1) | u128::from(v),
	//					output |= u64::from(v) << z as u32;
	//				},
	//				Err(GetOr0Error::LoopAt(_)) => return false
	//			}
	//		}
	
	true
}
