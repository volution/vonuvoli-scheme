

/*
	See:
		http://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/
		https://github.com/preshing/RandomSequence/blob/master/randomsequence.h
*/




use std::num::Wrapping;




#[ derive (Clone, Debug, Default) ]
pub struct PermutationCounter {
	pub index : u32,
	pub offset : u32,
	pub initialized : bool,
}


impl PermutationCounter {
	
	
	pub fn new () -> (Self) {
		PermutationCounter::with_seed (0, 0)
	}
	
	pub fn with_seed (index : u32, offset : u32) -> (Self) {
		PermutationCounter {
				index : index,
				offset : offset,
				initialized : false,
			}
	}
	
	
	pub fn initialize (&mut self) -> () {
		if !self.initialized {
			self.index = self.permute (self.permute (Wrapping (self.index)) + FUZZ_2) .0;
			self.offset = self.permute (self.permute (Wrapping (self.offset)) + FUZZ_3) .0;
			self.initialized = true;
		}
	}
	
	pub fn next (&mut self) -> (u32) {
		self.initialize ();
		self.index = (Wrapping (self.index) + Wrapping (1u32)) .0;
		let output = self.permute (Wrapping (self.index));
		let output = output + Wrapping (self.offset);
		let output = self.permute (output);
		let output = output ^ FUZZ_1;
		return output.0;
	}
	
	fn permute (&self, index : Wrapping<u32>) -> (Wrapping<u32>) {
		if index >= PRIME {
			return index;
		}
		let residue = Wrapping ((((index.0 as u64) * (index.0 as u64)) % (PRIME.0 as u64)) as u32);
		if index <= (PRIME >> 1) {
			return residue;
		} else {
			return PRIME - residue;
		}
	}
}


static PRIME : Wrapping<u32> = Wrapping (4294967291);
static FUZZ_1 : Wrapping<u32> = Wrapping (0x5bf03635);
static FUZZ_2 : Wrapping<u32> = Wrapping (0x682f0161);
static FUZZ_3 : Wrapping<u32> = Wrapping (0x46790905);




/*
fn main () -> () {
	
	use std::io::Write;
	
	let stdout = std::io::stdout ();
	let mut stdout = stdout.lock ();
	let mut counter = counters::PermutationCounter::new ();
	
	loop {
		let value = counter.next ();
		let bytes : [u8; 4] = unsafe {
			std::mem::transmute (value)
		};
		stdout.write (bytes.as_ref ());
	}
*/

