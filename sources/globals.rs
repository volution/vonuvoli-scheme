

use super::counters::*;
use super::runtime::*;




pub(crate) mod exports {
	pub(crate) use super::context_handles_next;
	pub(crate) use super::registers_handles_next;
	pub(crate) use super::binding_handles_next;
	pub(crate) use super::parameters_handles_next;
	pub(crate) use super::parameter_handles_next;
	pub(crate) use super::record_handles_next;
	pub(crate) use super::lambda_handles_next;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub(crate) use super::port_handles_next;
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	pub(crate) use super::process_handles_next;
}




static mut CONTEXT_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x514765cd,
		offset : 0x4d564bb6,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn context_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut CONTEXT_HANDLES, 0xbe57f14a66e40068)
	}
}




static mut REGISTERS_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x2b6abc57,
		offset : 0x88b3547d,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn registers_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut REGISTERS_HANDLES, 0x09d44b48186625d8)
	}
}




static mut BINDING_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0xc8b5516d,
		offset : 0x767a8d5c,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn binding_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut BINDING_HANDLES, 0xb6f4ec1eb3746759)
	}
}




static mut PARAMETERS_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x7f678d4c,
		offset : 0x182256dd,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn parameters_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut PARAMETERS_HANDLES, 0xae0a37c517dac5db)
	}
}




static mut PARAMETER_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x5ec9e479,
		offset : 0x92d12a5f,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn parameter_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut PARAMETER_HANDLES, 0x6c6430ddb9c3d635)
	}
}




static mut RECORD_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x8251b601,
		offset : 0xac4eff38,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn record_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut RECORD_HANDLES, 0xc94118e6eb43a5ad)
	}
}




static mut LAMBDA_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x33faad68,
		offset : 0xe28c918f,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn lambda_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut LAMBDA_HANDLES, 0xb80bdab7a2daeefa)
	}
}




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
static mut PORT_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0xa7e6ecf7,
		offset : 0x1e5c47b7,
		initialized : false,
	};

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn port_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut PORT_HANDLES, 0x671b9e29055f5095)
	}
}




#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
static mut PROCESS_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x30df6208,
		offset : 0xa58f88be,
		initialized : false,
	};

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn process_handles_next () -> (Handle) {
	unsafe {
		handles_next (&mut PROCESS_HANDLES, 0x8a37eaa32ffc5892)
	}
}




static mut EPOCH_HANDLES : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0,
		offset : 0,
		initialized : false,
	};

static mut EPOCH_HANDLES_RESET : PermutationCounter = PermutationCounter {
		count : 0,
		index : 0x7e740a0c,
		offset : 0x80ea7309,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
pub(crate) fn handles_next (counter : &mut PermutationCounter, fuzz : u64) -> (Handle) {
	loop {
		let epoch = unsafe {
			if (EPOCH_HANDLES.count & 0xffff) == 0 {
				EPOCH_HANDLES.index = EPOCH_HANDLES_RESET.index;
				EPOCH_HANDLES.offset = EPOCH_HANDLES_RESET.offset;
				EPOCH_HANDLES.initialized = EPOCH_HANDLES_RESET.initialized;
				EPOCH_HANDLES_RESET.next ();
			}
			EPOCH_HANDLES.next () as u64
		};
		if epoch == 0 {
			continue;
		}
		let value = counter.next () as u64;
		let handle = ((epoch << 32) | (value << 0)) ^ fuzz;
		return Handle::new (handle)
	}
}

