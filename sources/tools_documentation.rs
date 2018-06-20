

use super::documentation::exports::*;
use super::errors::exports::*;
use super::tools::exports::*;

use super::prelude::*;

use super::documentation::Entity;




pub mod exports {
	pub use super::main;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	if ! inputs.tool_arguments.is_empty () {
		fail! (0x2f6cb42b);
	}
	if ! inputs.rest_arguments.is_empty () {
		fail! (0x6b36228d);
	}
	
	let libraries = try! (parse_library_specifications_for_builtins ());
	
	for library in libraries.libraries () {
		
		try_writeln! (stream, "* library `{}`", library.identifier ());
		
		try_writeln! (stream, "  * categories:");
		for category in library.categories () {
			try_writeln! (stream, "    * category `{}`", category.identifier ());
			if category.has_definitions () {
				try_writeln! (stream, "      * definitions:");
				for definition in category.definitions () {
					try_writeln! (stream, "        * definition `{}`", definition.identifier ());
				}
			}
		}
		
		try_writeln! (stream, "  * definitions:");
		for definition in library.definitions () {
			try_writeln! (stream, "    * definition `{}`", definition.identifier ());
			try_writeln! (stream, "      * type `{}`", definition.kind () .identifier ());
			if definition.has_categories () {
				try_write! (stream, "      * categories:");
				for category in definition.categories () {
					try_write! (stream, " `{}`", category.identifier ());
				}
				try_writeln! (stream);
			}
			if definition.has_aliases () {
				try_write! (stream, "      * aliases:");
				for alias in definition.aliases () {
					try_write! (stream, " `{}`", alias);
				}
				try_writeln! (stream);
			}
		}
	}
	
	succeed! (0);
}

