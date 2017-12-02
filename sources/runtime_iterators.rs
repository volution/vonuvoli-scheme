

use super::errors::exports::*;




pub mod exports {
	pub use super::RangeIterator;
}




pub struct RangeIterator <Value, IteratorDelegate>
		where IteratorDelegate : Iterator<Item = Value>
{
		iterator : IteratorDelegate,
		index : usize,
		range_start : usize,
		range_end : Option<usize>,
}


impl <Value, IteratorDelegate> RangeIterator<Value, IteratorDelegate>
		where IteratorDelegate : Iterator<Item = Value>
{
	
	pub fn new (iterator : IteratorDelegate, range_start : usize, range_end : Option<usize>) -> (Outcome<Self>) {
		succeed! (RangeIterator {
				iterator : iterator,
				index : 0,
				range_start : range_start,
				range_end : range_end
			});
	}
}


impl <Value, IteratorDelegate> Iterator for RangeIterator<Value, IteratorDelegate>
		where IteratorDelegate : Iterator<Item = Value>
{
	type Item = Outcome<Value>;
	
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		
		while self.index < self.range_start {
			if let Some (_) = self.iterator.next () {
				self.index += 1;
			} else {
				return Some (failed! (0xb0d17971));
			}
		}
		if let Some (range_end) = self.range_end {
			if self.index >= range_end {
				return None;
			}
		}
		
		if let Some (value) = self.iterator.next () {
			self.index += 1;
			return Some (succeeded! (value));
		} else {
			if let Some (range_end) = self.range_end {
				if self.index == range_end {
					return None;
				} else {
					return Some (failed! (0x98c83cbe));
				}
			} else {
				return None;
			}
		}
	}
}

