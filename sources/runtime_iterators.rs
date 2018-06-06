

use super::errors::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::RangeIterator;
	pub use super::RangeIteratorForOutcome;
}




pub struct RangeIterator <Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Value>
{
		iterator : IteratorDelegate,
		index : usize,
		range_start : usize,
		range_end : Option<usize>,
		completed : bool,
}


impl <Value, IteratorDelegate> RangeIterator<Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Value>
{
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (iterator : IteratorDelegate, range_start : usize, range_end : Option<usize>) -> (Outcome<Self>) {
		succeed! (RangeIterator {
				iterator : iterator,
				index : 0,
				range_start : range_start,
				range_end : range_end,
				completed : false,
			});
	}
}


impl <Value, IteratorDelegate> iter::Iterator for RangeIterator<Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Value>
{
	type Item = Outcome<Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		
		if self.completed {
			return None;
		}
		
		while self.index < self.range_start {
			if let Some (_) = self.iterator.next () {
				self.index += 1;
			} else {
				return Some (failed! (0xb0d17971));
			}
		}
		if let Some (range_end) = self.range_end {
			if self.index >= range_end {
				self.completed = true;
				return None;
			}
		}
		
		if let Some (value) = self.iterator.next () {
			self.index += 1;
			return Some (succeeded! (value));
		} else {
			if let Some (range_end) = self.range_end {
				if self.index == range_end {
					self.completed = true;
					return None;
				} else {
					return Some (failed! (0x75a86cb5));
				}
			} else {
				self.completed = true;
				return None;
			}
		}
	}
}




pub struct RangeIteratorForOutcome <Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Outcome<Value>>
{
		iterator : IteratorDelegate,
		index : usize,
		range_start : usize,
		range_end : Option<usize>,
		completed : bool,
}


impl <Value, IteratorDelegate> RangeIteratorForOutcome<Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Outcome<Value>>
{
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (iterator : IteratorDelegate, range_start : usize, range_end : Option<usize>) -> (Outcome<Self>) {
		succeed! (RangeIteratorForOutcome {
				iterator : iterator,
				index : 0,
				range_start : range_start,
				range_end : range_end,
				completed : false,
			});
	}
}


impl <Value, IteratorDelegate> iter::Iterator for RangeIteratorForOutcome<Value, IteratorDelegate>
		where IteratorDelegate : iter::Iterator<Item = Outcome<Value>>
{
	type Item = Outcome<Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		
		if self.completed {
			return None;
		}
		
		while self.index < self.range_start {
			if let Some (outcome) = self.iterator.next () {
				match outcome {
					Ok (_) =>
						self.index += 1,
					error =>
						return Some (error),
				}
			} else {
				return Some (failed! (0xe26d72ae));
			}
		}
		if let Some (range_end) = self.range_end {
			if self.index >= range_end {
				self.completed = true;
				return None;
			}
		}
		
		if let Some (outcome) = self.iterator.next () {
			match outcome {
				Ok (value) => {
					self.index += 1;
					return Some (succeeded! (value));
				},
				error =>
					return Some (error),
			}
		} else {
			if let Some (range_end) = self.range_end {
				if self.index == range_end {
					self.completed = true;
					return None;
				} else {
					return Some (failed! (0x9a76c55c));
				}
			} else {
				self.completed = true;
				return None;
			}
		}
	}
}

