

/*
	See:
		http://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/
		https://github.com/preshing/RandomSequence/blob/master/randomsequence.h
*/




use std::num;




#[ derive (Clone, Debug, Default) ]
pub struct PermutationCounter {
	pub index : u32,
	pub offset : u32,
	pub initialized : bool,
}


impl PermutationCounter {
	
	
	#[ inline (always) ]
	pub fn new () -> (Self) {
		PermutationCounter::with_seed (0, 0)
	}
	
	#[ inline (always) ]
	pub fn with_seed (index : u32, offset : u32) -> (Self) {
		PermutationCounter {
				index : index,
				offset : offset,
				initialized : false,
			}
	}
	
	
	#[ inline (always) ]
	pub fn initialize (&mut self) -> () {
		if !self.initialized {
			self.index = self.permute (self.permute (num::Wrapping (self.index)) + FUZZ_2) .0;
			self.offset = self.permute (self.permute (num::Wrapping (self.offset)) + FUZZ_3) .0;
			self.initialized = true;
		}
	}
	
	#[ inline (always) ]
	pub fn next (&mut self) -> (u32) {
		self.initialize ();
		self.index = (num::Wrapping (self.index) + num::Wrapping (1u32)) .0;
		let output = self.permute (num::Wrapping (self.index));
		let output = output + num::Wrapping (self.offset);
		let output = self.permute (output);
		let output = output ^ FUZZ_1;
		return output.0;
	}
	
	#[ inline (always) ]
	fn permute (&self, index : num::Wrapping<u32>) -> (num::Wrapping<u32>) {
		if index >= PRIME {
			return index;
		}
		let residue = num::Wrapping ((((index.0 as u64) * (index.0 as u64)) % (PRIME.0 as u64)) as u32);
		if index <= (PRIME >> 1) {
			return residue;
		} else {
			return PRIME - residue;
		}
	}
}


static PRIME : num::Wrapping<u32> = num::Wrapping (4294967291);
static FUZZ_1 : num::Wrapping<u32> = num::Wrapping (0x5bf03635);
static FUZZ_2 : num::Wrapping<u32> = num::Wrapping (0x682f0161);
static FUZZ_3 : num::Wrapping<u32> = num::Wrapping (0x46790905);

