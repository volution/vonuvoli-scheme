

use super::values::exports::Value;




pub mod exports {
	pub use super::{TestCase, TestAction, TestVerbosity};
}




#[ derive ( Clone ) ] // OK ??
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub struct TestCase {
	pub expression : Value,
	pub action : TestAction,
	pub verbosity : TestVerbosity,
}

#[ derive ( Clone ) ] // OK ??
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub enum TestAction {
	Expect ( Value ),
	Debug,
	Ignore,
	Skip,
}

#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum TestVerbosity {
	Quiet,
	Verbose,
	Debug,
	Default,
}

