

use super::counters::PermutationCounter;




static mut CONTEXT_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x514765cd,
		offset : 0x4d564bb6,
		initialized : false,
	};

pub fn context_handles_next () -> (u32) {
	unsafe {
		CONTEXT_HANDLES.next ()
	}
}


static mut BINDINGS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xc8b5516d,
		offset : 0x767a8d5c,
		initialized : false,
	};

pub fn bindings_handles_next () -> (u32) {
	unsafe {
		BINDINGS_HANDLES.next ()
	}
}

