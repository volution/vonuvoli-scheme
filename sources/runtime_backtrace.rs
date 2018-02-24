

use super::prelude::*;




pub mod exports {
	pub use super::Backtrace;
}




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
pub struct Backtrace (
	backtrace::Backtrace
);

#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
pub struct Backtrace ();




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
impl Backtrace {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new () -> (Backtrace) {
		Backtrace (backtrace::Backtrace::new_unresolved ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report (&self, transcript : &mut io::Write) -> (io::Result<()>) {
		let mut backtrace = self.0.clone ();
		backtrace.resolve ();
		try! (write! (transcript, "[ee]      ---------------------------------------\n"));
		'done : for frame in backtrace.frames () {
			for symbol in frame.symbols () {
				match self.report_symbol (symbol, transcript) {
					Ok (true) =>
						(),
					Ok (false) =>
						break 'done,
					Err (error) =>
						return Err (error),
				}
			}
		}
		try! (write! (transcript, "[ee]      ---------------------------------------\n"));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn report_symbol (&self, symbol : &backtrace::BacktraceSymbol, transcript : &mut io::Write) -> (io::Result<bool>) {
		let name = option_and_then! (symbol.name (), name, name.as_str ());
		match name {
			Some (name) => {
				let mut name_buffer = StdString::new ();
				let demangled = match rustc_demangle::try_demangle (name) {
					Ok (demangled) =>
						match write! (name_buffer, "{:#}", demangled) {
							Ok (()) =>
								true,
							Err (_) => {
								name_buffer.push_str (name);
								false
							},
						},
					Err (_) => {
						name_buffer.push_str (name);
						false
					}
				};
				if demangled {
					if name_buffer.starts_with ("vonuvoli_scheme") {
						match name_buffer.as_str () {
							"vonuvoli_scheme::runtime_backtrace::Backtrace::new" |
							"vonuvoli_scheme::errors::Error::new" |
							"vonuvoli_scheme::errors::error_generic" |
							"vonuvoli_scheme::errors::error_panic" |
							"vonuvoli_scheme::errors::error_unimplemented" |
							"__unreachable__" =>
								succeed! (true),
							_ =>
								(),
						}
						let file = symbol.filename ();
						let file = option_and_then! (file, file.file_name ());
						let file = option_and_then! (file, file.to_str ());
						let line = symbol.lineno ();
						try! (write! (transcript, "[ee]      -- {}\n", name_buffer));
						if file.is_some () && line.is_some () {
							let file = file.unwrap_or ("<invalid>");
							let line = line.unwrap_or (0);
							try! (write! (transcript, "[ee]           @ {} : {}\n", file, line));
							// TODO:  Optimize this!
							let line = line as i32;
							for &((source_file, source_line), source_data) in SOURCES {
								let source_line = source_line as i32;
								if (source_file == file) && (source_line >= (line - 5)) && (source_line <= (line + 5)) {
									if source_line == line {
										try! (write! (transcript, "[ee]           >> {}    <<<<<<<<\n", source_data));
									} else {
										try! (write! (transcript, "[ee]           :  {}\n", source_data));
									}
								}
							}
						}
						succeed! (true);
					} else if name_buffer.starts_with ("backtrace::") {
						// NOTE:  These frames were captured while creating the backtrace!
						succeed! (true);
					} else if name_buffer.starts_with ("std::rt::lang_start::") {
						// NOTE:  After this there doesn't seem to be anything interesting for us!
						succeed! (false);
					} else {
						try! (write! (transcript, "[ee]         {}\n", name_buffer));
						succeed! (true);
					}
				} else {
					try! (write! (transcript, "[ee]         {}\n", name_buffer));
					succeed! (true);
				}
			},
			_ => {
				try! (write! (transcript, "[ee]         ???\n"));
				succeed! (true);
			}
		}
	}
}


#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
impl Backtrace {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new () -> (Backtrace) {
		Backtrace ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report (&self, _transcript : &mut io::Write) -> (io::Result<()>) {
		succeed! (());
	}
}




#[ cfg ( all ( feature = "vonuvoli_backtrace", feature = "vonuvoli_backtrace_sources" ) ) ]
include! ("../target/lib_sources.in");

#[ cfg ( not ( all ( feature = "vonuvoli_backtrace", feature = "vonuvoli_backtrace_sources" ) ) ) ]
static SOURCES : &'static [((&'static str, usize), &'static str)] = &[];

