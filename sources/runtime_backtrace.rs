

use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Backtrace;
	pub use super::BacktraceSymbol;
}




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
pub struct Backtrace (
	ext::backtrace::Backtrace
);

#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
pub struct Backtrace ();




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
impl Backtrace {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new () -> (Backtrace) {
		Backtrace (ext::backtrace::Backtrace::new_unresolved ())
	}
	
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report <T : Transcript + ?Sized> (&self, transcript : &TranscriptTracer<T>) -> () {
		let mut backtrace = self.0.clone ();
		backtrace.resolve ();
		let mut transcript_buffer = transcript.buffer ();
		'done : for frame in backtrace.frames () {
			for symbol in frame.symbols () {
				if ! self.report_symbol (symbol, &mut transcript_buffer) {
					break 'done;
				}
			}
		}
		transcript.trace_buffer (transcript_buffer, false);
	}
	
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn report_symbol <T : Transcript + ?Sized> (&self, symbol : &ext::backtrace::BacktraceSymbol, transcript_buffer : &mut TranscriptBuffer<T>) -> (bool) {
		let transcript_color = transcript_buffer.output_supports_ansi_sequences ();
		let name = option_and_then! (symbol.name (), name, name.as_str ());
		match name {
			Some (name) => {
				let mut name_buffer = StdString::new ();
				let demangled = match ext::rustc_demangle::try_demangle (name) {
					Ok (demangled) =>
						match write! (name_buffer, "{:#}", demangled) {
							Ok (()) =>
								true,
							Err (_) => {
								name_buffer.clear ();
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
							"vonuvoli_scheme::runtime::execute_main::{{closure}}" |
							"vonuvoli_scheme::runtime::panic_with_error" |
							"vonuvoli_scheme::errors::Error::new" |
							"vonuvoli_scheme::errors::error_generic" |
							"vonuvoli_scheme::errors::error_panic" |
							"vonuvoli_scheme::errors::error_unimplemented" =>
								return true,
							"vonuvoli_scheme::runtime::execute_main" =>
								// NOTE:  After this there doesn't seem to be anything interesting for us!
								return false,
							_ =>
								(),
						}
						let file = symbol.filename ();
						let file = option_and_then! (file, file.file_name ());
						let file = option_and_then! (file, file.to_str ());
						let line = symbol.lineno ();
						transcript_buffer.push_fmt (format_args! ("-- {}\n", transcript_style (name_buffer, STYLE_SYMBOL_NAME, transcript_color)));
						if file.is_some () && line.is_some () {
							let file = file.unwrap_or ("<invalid>");
							let line = line.unwrap_or (0);
							transcript_buffer.push_fmt (format_args! ("     @ {} : {}\n", file, line));
							TODO! ("optimize this");
							let line = line as i32;
							for &((source_file, source_line), source_data) in SOURCES {
								let source_line = source_line as i32;
								if (source_file == file) && (source_line >= (line - 5)) && (source_line <= (line + 5)) {
									if source_line == line {
										transcript_buffer.push_fmt (format_args! ("     >> {}\n", transcript_style (source_data, STYLE_SYMBOL_LINE_EXACT, transcript_color)));
									} else {
										if source_line == (line + 5) {
											transcript_buffer.push_fmt (format_args! ("     :  {}", source_data));
											break;
										} else {
											transcript_buffer.push_fmt (format_args! ("     :  {}\n", source_data));
										}
									}
								}
							}
						}
						transcript_buffer.push_str ("\u{1e}");
						return true;
					} else if name_buffer.starts_with ("backtrace::") {
						// NOTE:  These frames were captured while creating the backtrace!
						return true;
					} else {
						match name_buffer.as_str () {
							// NOTE:  These frames are not interesting!
							"core::ops::function::Fn::call" |
							"core::ops::function::FnMut::call_mut" |
							"core::ops::function::FnOnce::call_once" |
							// NOTE:  These frames are part of the error handling mechanism!
							"std::panic::catch_unwind" |
							"std::panicking::begin_panic" |
							"std::panicking::try" |
							"std::panicking::try::do_call" |
							"std::panicking::rust_panic_with_hook" =>
								(),
							_ => {
								transcript_buffer.push_fmt (format_args! (".. {}\n", transcript_style (name_buffer, STYLE_SYMBOL_NAME, transcript_color)));
								transcript_buffer.push_str ("\u{1e}");
							},
						}
						return true;
					}
				} else {
					match name_buffer.as_str () {
						// NOTE:  These frames are part of the error handling mechanism!
						"__rust_maybe_catch_panic" =>
							(),
						_ => {
							transcript_buffer.push_fmt (format_args! (".. {}\n", transcript_style (name_buffer, STYLE_SYMBOL_NAME, transcript_color)));
							transcript_buffer.push_str ("\u{1e}");
						},
					}
					return true;
				}
			},
			_ => {
				transcript_buffer.push_fmt (format_args! (".. {}\n", transcript_style ("???", STYLE_SYMBOL_NAME, transcript_color)));
				transcript_buffer.push_str ("\u{1e}");
				return true;
			},
		}
	}
}


#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
impl Backtrace {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new () -> (Backtrace) {
		Backtrace ()
	}
	
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report <T : Transcript + ?Sized> (&self, _transcript : &TranscriptTracer<T>) -> () {}
}


impl fmt::Debug for Backtrace {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("Backtrace") .finish ()
	}
}




pub struct BacktraceSymbol ( ptr::NonNull<os::raw::c_void> );




impl BacktraceSymbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (address : fn () -> ()) -> (BacktraceSymbol) {
		let address = unsafe { mem::transmute (address) };
		if let Some (address) = ptr::NonNull::new (address) {
			return BacktraceSymbol (address);
		} else {
			panic_0! (0xbe8eae73, github_issue_new);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let value = unsafe { mem::transmute_copy (&self.0.as_ptr ()) };
		return Handle::new (value);
	}
}




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
impl BacktraceSymbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_name (&self) -> (StdString) {
		let mut name_buffer = StdString::new ();
		ext::backtrace::resolve (self.0.as_ptr (), |symbol| {
				let name = option_and_then! (symbol.name (), name, name.as_str ());
				if let Some (name) = name {
					match ext::rustc_demangle::try_demangle (name) {
						Ok (demangled) =>
							match write! (name_buffer, "{:#}", demangled) {
								Ok (()) =>
									(),
								Err (_) => {
									name_buffer.clear ();
									name_buffer.push_str (name);
									()
								},
							},
						Err (_) => {
							name_buffer.push_str (name);
							()
						}
					}
				}
			});
		if name_buffer.is_empty () {
			match write! (name_buffer, "{:p}", self.0.as_ptr ()) {
				Ok (()) =>
					(),
				Err (_) =>
					panic_0! (0xaa3b117f, github_issue_new),
			}
		}
		return name_buffer;
	}
}


#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
impl BacktraceSymbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_name (&self) -> (StdString) {
		format! ("{:p}", self.0.as_ptr ())
	}
}




#[ cfg ( all ( feature = "vonuvoli_backtrace", feature = "vonuvoli_backtrace_sources" ) ) ]
include! ("../target/lib_sources.in");

#[ cfg ( not ( all ( feature = "vonuvoli_backtrace", feature = "vonuvoli_backtrace_sources" ) ) ) ]
static SOURCES : &'static [((&'static str, usize), &'static str)] = &[];




#[ cfg ( feature = "vonuvoli_transcript" ) ]
static STYLE_SYMBOL_NAME : TranscriptStyle = TRANSCRIPT_STYLE_YELLOW_BOLD;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
static STYLE_SYMBOL_LINE_EXACT : TranscriptStyle = TRANSCRIPT_STYLE_RED_BOLD;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
static STYLE_NONE : TranscriptStyle = TRANSCRIPT_STYLE_NONE;

