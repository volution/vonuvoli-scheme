

use super::errors::exports::*;
use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::main;
	
	pub use super::ToolInputs;
	
}




def_transcript_root! (transcript);




#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct ToolInputs {
	pub tool_binary : ffi::OsString,
	pub tool_commands : StdVec<StdString>,
	pub tool_arguments : StdVec<ffi::OsString>,
	pub tool_environment : StdVec<(ffi::OsString, ffi::OsString)>,
	pub rest_arguments : StdVec<ffi::OsString>,
	pub rest_environment : StdVec<(ffi::OsString, ffi::OsString)>,
}


type ToolMain = fn (ToolInputs) -> (Outcome<u32>);




pub fn main () -> () {
	execute_main (main_0, &transcript);
}


fn main_0 () -> (Outcome<u32>) {
	
	let mut tool_inputs = r#try! (main_inputs ());
	
	let (tool_main, tool_commands_drop) : (ToolMain, usize)
	= match vec_map! (tool_inputs.tool_commands.iter (), command, command.as_str ()) .as_slice () {
		[] =>
			fail! (0x00c99a91),
		#[ cfg ( feature = "vonuvoli_tools_interpreter" ) ]
		["interpreter"] =>
			(super::tools_interpreter::main, 1),
		#[ cfg ( feature = "vonuvoli_tools_compiler" ) ]
		["compiler"] =>
			(super::tools_compiler::main, 1),
		#[ cfg ( feature = "vonuvoli_tools_tester" ) ]
		["tester"] =>
			(super::tools_tester::main, 1),
		#[ cfg ( feature = "vonuvoli_tools_bencher" ) ]
		["bencher"] =>
			(super::tools_bencher::main, 1),
		#[ cfg ( feature = "vonuvoli_tools_reports" ) ]
		["reports", ..] =>
			(super::tools_reports::main, 1),
		#[ cfg ( feature = "vonuvoli_tools_documentation" ) ]
		["documentation", ..] =>
			(super::tools_documentation::main, 1),
		_ =>
			fail! (0xb2051df0),
	};
	
	for _ in 0..tool_commands_drop {
		tool_inputs.tool_commands.remove (0);
	}
	
	return tool_main (tool_inputs);
}




fn main_inputs () -> (Outcome<ToolInputs>) {
	
	let os_arguments = vec_map! (env::args_os (), argument, argument);
	let os_environment = vec_map! (env::vars_os (), variable, variable);
	
	let (tool_arguments, rest_arguments) = r#try! (parse_os_arguments (os_arguments));
	let (tool_environment, rest_environment) = r#try! (parse_os_environment (os_environment));
	
	let (tool_binary, tool_commands, tool_arguments) = {
		
		let mut tool_arguments = tool_arguments;
		let mut tool_commands = StdVec::new ();
		
		tool_arguments.reverse ();
		
		let tool_binary = if let Some (first) = tool_arguments.pop () {
			first
		} else {
			fail! (0xe54f7088);
		};
		
		while let Some (first) = tool_arguments.pop () {
			let first = try_or_fail! (first.into_string (), 0x93d796f7);
			if let Some (first_char) = first.chars () .next () {
				match first_char {
					'-' =>
						if first == "-" {
							break;
						} else {
							let first = ffi::OsString::from (first);
							tool_arguments.push (first);
							break;
						},
					'0' ..= '9' |
					'a' ..= 'z' |
					'A' ..= 'Z' =>
						tool_commands.push (first),
					_ =>
						fail! (0x81c077b6),
				}
			} else {
				fail! (0xd5121e1f);
			}
		}
		
		tool_arguments.reverse ();
		
		(tool_binary, tool_commands, tool_arguments)
	};
	
	let tool_inputs = ToolInputs {
			tool_binary,
			tool_commands,
			tool_arguments, tool_environment,
			rest_arguments, rest_environment,
		};
	
	succeed! (tool_inputs);
}

