use std::iter;

#[derive(Copy, Eq, Clone, PartialEq)]
enum ThreeBit {
	Zero = 0,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven
}

impl std::fmt::Display for ThreeBit {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Write::write_char(f, match self {
			Self::Zero => '0',
			Self::One => '1',
			Self::Two => '2',
			Self::Three => '3',
			Self::Four => '4',
			Self::Five => '5',
			Self::Six => '6',
			Self::Seven => '7'
		})
	}
}

enum Continuation {
	Continue,
	JumpTo(ThreeBit),
	Output(ThreeBit),
}

enum InstrError {
	Reserved
}

impl ThreeBit {
	pub fn from_value(v: u8) -> Option<Self> {
		Some(match v {
			0 => Self::Zero,
			1 => Self::One,
			2 => Self::Two,
			3 => Self::Three,
			4 => Self::Four,
			5 => Self::Five,
			6 => Self::Six,
			7 => Self::Seven,
			_ => return None
		})
	}
	
	pub fn from_ascii(c: u8) -> Option<Self> {
		Self::from_value(c.wrapping_sub(b'0'))
	}
	
	pub fn from_char(c: char) -> Option<Self> {
		u8::try_from(c).ok().and_then(Self::from_ascii)
	}
	
	pub fn evaluate_as_combo(self, registers: &[u64; 3]) -> Option<u64> {
		Some(match self {
			Self::Zero => 0,
			Self::One => 1,
			Self::Two => 2,
			Self::Three => 3,
			Self::Four => registers[0], // A
			Self::Five => registers[1], // B
			Self::Six => registers[2],  // C
			Self::Seven => return None // 'reserved'
		})
	}
	
	fn evaluate_div(operand: Self, registers: &[u64; 3]) -> Result<u64, InstrError> {
		let numerator = registers[0];
		let shift = operand.evaluate_as_combo(registers).ok_or(InstrError::Reserved)?;
		Ok(if shift < 32 {
			numerator >> shift
		} else {
			0
		})
	}
	
	pub fn evaluate_as_instruction_with(self, operand: Self, registers: &mut [u64; 3]) -> Result<Continuation, InstrError> {
		match self {
			Self::Zero => { // adv
				registers[0] = Self::evaluate_div(operand, registers)?;
				Ok(Continuation::Continue)
			},
			Self::One => { // bxl
				registers[1] ^= operand as u64;
				Ok(Continuation::Continue)
			},
			Self::Two => { // bst
				registers[1] = 7 & operand.evaluate_as_combo(registers).ok_or(InstrError::Reserved)?;
				Ok(Continuation::Continue)
			},
			Self::Three => { // jnz
				Ok(if registers[0] != 0 {
					Continuation::JumpTo(operand)
				} else {
					Continuation::Continue
				})
			},
			Self::Four => { // bxc
				registers[1] ^= registers[2];
				Ok(Continuation::Continue)
			},
			Self::Five => { // out
				operand.evaluate_as_combo(registers)
						.ok_or(InstrError::Reserved)
						.map(|v| Self::from_value((7 & v) as u8).unwrap())
						.map(Continuation::Output)
			},
			Self::Six => { // bdv
				registers[1] = Self::evaluate_div(operand, registers)?;
				Ok(Continuation::Continue)
			},
			Self::Seven => { // cdv
				registers[2] = Self::evaluate_div(operand, registers)?;
				Ok(Continuation::Continue)
			}
		}
	}
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
	const VALUE_AT: usize = "Register A: ".len();
	let (a, remainder) = input[VALUE_AT..].split_once('\n').unwrap();
	
	let mut registers = [a.parse().unwrap(), 0, 0];
	
	let mut program = Vec::with_capacity(16);
	program.extend(
		remainder.split_once('P').unwrap().1["rogram: ".len()..]
			.split(',').map(str::trim).map(str::parse).map(Result::unwrap)
			.map(ThreeBit::from_char).map(Option::unwrap)
	);
	
	let mut output = String::with_capacity(20);
	let mut index = 0;
	loop {
		let Some([opcode, operand]) = program.get(index..index + 2).map(|slice| <[ThreeBit; 2]>::try_from(slice).unwrap()) else {
			break output;
		};
		
		match opcode.evaluate_as_instruction_with(operand, &mut registers) {
			Ok(Continuation::Continue) => index += 2,
			Ok(Continuation::JumpTo(i)) => index = i as usize,
			Ok(Continuation::Output(val)) => {
				if !output.is_empty() {
					output.reserve(2);
					output.push(',');
				}
				output.push((b'0' + val as u8) as char);
				
				index += 2;
			},
			Err(_) => unreachable!()
		}
	}
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
	let mut program = Vec::with_capacity(16);
	program.extend(
		input.split_once('P').unwrap().1["rogram: ".len()..]
			.split(',').map(str::trim).map(str::parse).map(Result::unwrap)
			.map(ThreeBit::from_char).map(Option::unwrap)
	);
	
	let mut running_total = 0;
	for (match_index, &match_value) in enumerate(&program).rev() {
		let shift = match_index as u32 * 3;
		let filler = (1 << shift) - 1;
		
		running_total |= 'success: loop {
			for test in 0..=7 {
				let extra = test << shift;
				let new_total = running_total | extra | filler;
				
				let mut registers = [new_total, 0, 0];
				
				let mut output_i = 0;
				let mut index = 0;
				let success = loop {
					let Some([opcode, operand]) = program.get(index..index + 2).map(|slice| <[ThreeBit; 2]>::try_from(slice).unwrap()) else {
						break false;
					};
					
					match opcode.evaluate_as_instruction_with(operand, &mut registers) {
						Ok(Continuation::Continue) => index += 2,
						Ok(Continuation::JumpTo(i)) => index = i as usize,
						Ok(Continuation::Output(val)) => {
							if output_i == match_index {
								break val == match_value;
							} else {
								output_i += 1;
							}
							
							index += 2;
						},
						Err(_) => break false
					}
				};
				
				if success {
					break 'success extra;
				}
			}
			
			unreachable!();
		};
	}
	running_total
}

fn enumerate<I: IntoIterator>(i: I) -> iter::Enumerate<I::IntoIter> {
	i.into_iter().enumerate()
}
