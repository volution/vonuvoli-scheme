

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
		
		if library.has_categories () {
			try_writeln! (stream, "  * categories:");
			for category in library.categories () {
				try_writeln! (stream, "    * category `{}`", category.identifier ());
				if category.has_definitions () {
					try_writeln! (stream, "      * definitions:");
					for definition in category.definitions () {
						try_writeln! (stream, "        * definition `{}`", definition.identifier ());
					}
				}
				if category.has_value_kinds () {
					try_writeln! (stream, "      * types:");
					for value_kind in category.value_kinds () {
						try_writeln! (stream, "        * type `{}`", value_kind.identifier ());
					}
				}
			}
		}
		
		if library.has_definitions () {
			try_writeln! (stream, "  * definitions:");
			for definition in library.definitions () {
				try_writeln! (stream, "    * definition `{}`", definition.identifier ());
				{
					try_write! (stream, "      * types: `{}`", definition.kind () .identifier ());
					for definition_kind in definition.kind () .parents () {
						try_write! (stream, " `{}`", definition_kind.identifier ());
					}
					try_writeln! (stream);
				}
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
		
		if library.has_value_kinds () {
			try_writeln! (stream, "  * types:");
			for value_kind in library.value_kinds () {
				try_writeln! (stream, "    * type `{}`", value_kind.identifier ());
				if value_kind.has_categories () {
					try_write! (stream, "      * categories:");
					for category in value_kind.categories () {
						try_write! (stream, " `{}`", category.identifier ());
					}
					try_writeln! (stream);
				}
				if value_kind.has_definitions () {
					try_writeln! (stream, "      * definitions:");
					for definition in value_kind.definitions () {
						try_writeln! (stream, "        * definition `{}`", definition.identifier ());
					}
				}
				if value_kind.has_aliases () {
					try_write! (stream, "      * aliases:");
					for alias in value_kind.aliases () {
						try_write! (stream, " `{}`", alias);
					}
					try_writeln! (stream);
				}
			}
		}
	}
	
	succeed! (0);
}

