

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
	pub(crate) use super::port_handles_next;
	pub(crate) use super::process_handles_next;
}




static mut CONTEXT_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x514765cd,
		offset : 0x4d564bb6,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn context_handles_next () -> (Handle) {
	unsafe {
		Handle::new (CONTEXT_HANDLES.next ())
	}
}




static mut REGISTERS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x2b6abc57,
		offset : 0x88b3547d,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn registers_handles_next () -> (Handle) {
	unsafe {
		Handle::new (REGISTERS_HANDLES.next ())
	}
}




static mut BINDING_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xc8b5516d,
		offset : 0x767a8d5c,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn binding_handles_next () -> (Handle) {
	unsafe {
		Handle::new (BINDING_HANDLES.next ())
	}
}




static mut PARAMETERS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x7f678d4c,
		offset : 0x182256dd,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn parameters_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PARAMETERS_HANDLES.next ())
	}
}




static mut PARAMETER_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x5ec9e479,
		offset : 0x92d12a5f,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn parameter_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PARAMETER_HANDLES.next ())
	}
}




static mut RECORD_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x8251b601,
		offset : 0xac4eff38,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn record_handles_next () -> (Handle) {
	unsafe {
		Handle::new (RECORD_HANDLES.next ())
	}
}




static mut LAMBDA_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x33faad68,
		offset : 0xe28c918f,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn lambda_handles_next () -> (Handle) {
	unsafe {
		Handle::new (LAMBDA_HANDLES.next ())
	}
}




static mut PORT_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xa7e6ecf7,
		offset : 0x1e5c47b7,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn port_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PORT_HANDLES.next ())
	}
}




static mut PROCESS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x30df6208,
		offset : 0xa58f88be,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn process_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PROCESS_HANDLES.next ())
	}
}

