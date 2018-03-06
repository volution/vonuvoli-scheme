

use super::runtime::exports::*;

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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report (&self, transcript : &mut io::Write, color : bool) -> (io::Result<()>) {
		let mut backtrace = self.0.clone ();
		backtrace.resolve ();
		try! (write! (transcript, "[ee]      ---------------------------------------\n"));
		'done : for frame in backtrace.frames () {
			for symbol in frame.symbols () {
				match self.report_symbol (symbol, transcript, color) {
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
	fn report_symbol (&self, symbol : &ext::backtrace::BacktraceSymbol, transcript : &mut io::Write, color : bool) -> (io::Result<bool>) {
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
						try! (write! (transcript, "[ee]      -- {}\n", style_paint (name_buffer, STYLE_SYMBOL_NAME, color)));
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
										try! (write! (transcript, "[ee]           >> {}\n", style_paint (source_data, STYLE_SYMBOL_LINE_EXACT, color)));
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
						try! (write! (transcript, "[ee]         {}\n", style_paint (name_buffer, STYLE_SYMBOL_NAME, color)));
						succeed! (true);
					}
				} else {
					try! (write! (transcript, "[ee]         {}\n", style_paint (name_buffer, STYLE_SYMBOL_NAME, color)));
					succeed! (true);
				}
			},
			_ => {
				try! (write! (transcript, "[ee]         {}\n", style_paint ("???", STYLE_SYMBOL_NAME, color)));
				succeed! (true);
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn report (&self, _transcript : &mut io::Write, _color : bool) -> (io::Result<()>) {
		succeed! (());
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
			panic! ("be8eae73");
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
					panic! ("aa3b117f"),
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




#[ cfg ( feature = "vonuvoli_terminal" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn style_paint <'a, I, S> (input : I, style : ext::ansi_term::Style, color : bool) -> (ext::ansi_term::ANSIGenericString<'a, S>)
		where
			I : StdInto<borrow::Cow<'a, S>>,
			S : 'a + borrow::ToOwned + ?Sized,
			<S as borrow::ToOwned>::Owned : fmt::Debug
{
	if color {
		style.paint (input)
	} else {
		STYLE_NONE.paint (input)
	}
}

#[ cfg ( not ( feature = "vonuvoli_terminal" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn style_paint <I> (input : I, _style : (), _color : bool) -> (I)
{
	input
}




#[ cfg ( feature = "vonuvoli_terminal" ) ]
static STYLE_SYMBOL_NAME : ext::ansi_term::Style = ext::ansi_term::Style {
		foreground : Some (ext::ansi_term::Colour::Yellow),
		background : None,
		is_bold : true, is_italic : false, is_underline : false, is_strikethrough : false,
		is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
	};

#[ cfg ( feature = "vonuvoli_terminal" ) ]
static STYLE_SYMBOL_LINE_EXACT : ext::ansi_term::Style = ext::ansi_term::Style {
		foreground : Some (ext::ansi_term::Colour::Red),
		background : None,
		is_bold : true, is_italic : false, is_underline : false, is_strikethrough : false,
		is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
	};

#[ cfg ( feature = "vonuvoli_terminal" ) ]
static STYLE_NONE : ext::ansi_term::Style = ext::ansi_term::Style {
		foreground : None,
		background : None,
		is_bold : false, is_italic : false, is_underline : false, is_strikethrough : false,
		is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
	};


#[ cfg ( not ( feature = "vonuvoli_terminal" ) ) ]
static STYLE_SYMBOL_NAME : () = ();

#[ cfg ( not ( feature = "vonuvoli_terminal" ) ) ]
static STYLE_SYMBOL_LINE_EXACT : () = ();

