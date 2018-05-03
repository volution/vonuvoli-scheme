

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			Transcript,
			TranscriptLevel,
			TranscriptCode,
			
			TranscriptContext,
			
			TranscriptTracer,
			TranscriptBuffer,
			TranscriptOutputable,
			
		};
	
	pub use super::{
			
			TranscriptForModule,
			TranscriptForScript,
			
			transcript_for_script,
			
		};
	
	pub use super::{
			
			TranscriptFrontend,
			TranscriptBackend,
			TranscriptStream,
			
		};
	
	pub use super::{
			
			transcript_code_for_source,
			transcript_code_for_message_static,
			transcript_code_for_message_value,
			
		};
	
	pub use super::{
			
			TranscriptStyle,
			transcript_style,
			transcript_style_push_initialize,
			transcript_style_push_finalize,
			
			TRANSCRIPT_STYLE_BLACK, TRANSCRIPT_STYLE_BLACK_BOLD,
			TRANSCRIPT_STYLE_RED, TRANSCRIPT_STYLE_RED_BOLD,
			TRANSCRIPT_STYLE_GREEN, TRANSCRIPT_STYLE_GREEN_BOLD,
			TRANSCRIPT_STYLE_YELLOW, TRANSCRIPT_STYLE_YELLOW_BOLD,
			TRANSCRIPT_STYLE_BLUE, TRANSCRIPT_STYLE_BLUE_BOLD,
			TRANSCRIPT_STYLE_PURPLE, TRANSCRIPT_STYLE_PURPLE_BOLD,
			TRANSCRIPT_STYLE_CYAN, TRANSCRIPT_STYLE_CYAN_BOLD,
			TRANSCRIPT_STYLE_WHITE, TRANSCRIPT_STYLE_WHITE_BOLD,
			
			TRANSCRIPT_STYLE_GRAY, TRANSCRIPT_STYLE_GRAY_BOLD,
			TRANSCRIPT_STYLE_RED_2, TRANSCRIPT_STYLE_RED_2_BOLD,
			TRANSCRIPT_STYLE_GREEN_2, TRANSCRIPT_STYLE_GREEN_2_BOLD,
			TRANSCRIPT_STYLE_YELLOW_2, TRANSCRIPT_STYLE_YELLOW_2_BOLD,
			TRANSCRIPT_STYLE_BLUE_2, TRANSCRIPT_STYLE_BLUE_2_BOLD,
			TRANSCRIPT_STYLE_PURPLE_2, TRANSCRIPT_STYLE_PURPLE_2_BOLD,
			TRANSCRIPT_STYLE_CYAN_2, TRANSCRIPT_STYLE_CYAN_2_BOLD,
			TRANSCRIPT_STYLE_WHITE_2, TRANSCRIPT_STYLE_WHITE_2_BOLD,
			
			TRANSCRIPT_STYLE_NONE,
			
		};
	
}




pub trait Transcript {
	
	fn trace_format (&self, level : TranscriptLevel, code : Option<TranscriptCode>, arguments : fmt::Arguments, stylize : bool, error : Option<&TranscriptOutputable>, backend : Option<&TranscriptBackend>) -> ();
	fn trace_message (&self, level : TranscriptLevel, code : Option<TranscriptCode>, message : &str, stylize : bool, error : Option<&TranscriptOutputable>, backend : Option<&TranscriptBackend>) -> ();
	fn trace_values (&self, level : TranscriptLevel, code : Option<TranscriptCode>, format : &str, values : &[&Value], backend : Option<&TranscriptBackend>) -> (Outcome<()>);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn trace_buffer (&self, level : TranscriptLevel, code : Option<TranscriptCode>, buffer : TranscriptBuffer<Self>, stylize : bool, backend : Option<&TranscriptBackend>) -> () {
		self.trace_message (level, code, &buffer.buffer, stylize, None, backend);
	}
	
	fn is_active (&self, level : TranscriptLevel) -> (bool);
	
	fn output_supports_ansi_sequences (&self, backend : Option<&TranscriptBackend>) -> (bool);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn tracer <'a> (&'a self, level : TranscriptLevel, code : Option<TranscriptCode>, backend : Option<&'a TranscriptBackend>) -> (TranscriptTracer<'a, Self>) {
		TranscriptTracer {
				transcript : self,
				level : level,
				code : code,
				backend : backend,
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer <'a> (&'a self, backend : Option<&'a TranscriptBackend>) -> (TranscriptBuffer<'a, Self>) {
		TranscriptBuffer {
				transcript : self,
				buffer : StdString::with_capacity (TRANSCRIPT_BUFFER_SIZE),
				backend : backend,
			}
	}
}




pub struct TranscriptTracer <'a, T : Transcript + ?Sized + 'a> {
	transcript : &'a T,
	level : TranscriptLevel,
	code : Option<TranscriptCode>,
	backend : Option<&'a TranscriptBackend>,
}


impl <'a, T : Transcript + ?Sized + 'a> TranscriptTracer<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn trace_format (&self, arguments : fmt::Arguments, stylize : bool, error : Option<&TranscriptOutputable>) -> () {
		self.transcript.trace_format (self.level, self.code, arguments, stylize, error, self.backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn trace_message (&self, message : &str, stylize : bool, error : Option<&TranscriptOutputable>) -> () {
		self.transcript.trace_message (self.level, self.code, message, stylize, error, self.backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn trace_values (&self, format : &str, values : &[&Value]) -> (Outcome<()>) {
		return self.transcript.trace_values (self.level, self.code, format, values, self.backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn trace_buffer (&self, buffer : TranscriptBuffer<T>, stylize : bool) -> () {
		self.transcript.trace_buffer (self.level, self.code, buffer, stylize, self.backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_active (&self, level : TranscriptLevel) -> (bool) {
		return self.transcript.is_active (level);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn output_supports_ansi_sequences (&self) -> (bool) {
		return self.transcript.output_supports_ansi_sequences (self.backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn buffer (&self) -> (TranscriptBuffer<'a, T>) {
		return self.transcript.buffer (self.backend);
	}
}




pub struct TranscriptBuffer <'a, T : Transcript + ?Sized + 'a> {
	transcript : &'a T,
	buffer : StdString,
	backend : Option<&'a TranscriptBackend>,
}


impl <'a, T : Transcript + ?Sized + 'a> TranscriptBuffer<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn push_fmt (&mut self, arguments : fmt::Arguments) -> () {
		match self.buffer.write_fmt (arguments) {
			Ok (()) =>
				(),
			Err (_error) =>
				panic_0! (0x232a35a0, (github_issue, 48)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn push_str (&mut self, message : &str) -> () {
		match self.buffer.write_str (message) {
			Ok (()) =>
				(),
			Err (_error) =>
				panic_0! (0x4308fce7, (github_issue, 48)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn output_supports_ansi_sequences (&self) -> (bool) {
		return self.transcript.output_supports_ansi_sequences (self.backend);
	}
}




pub trait TranscriptContext {
	
	fn identifier (&self) -> (&str);
	fn activation_level (&self) -> (Option<TranscriptLevel>);
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum TranscriptLevel {
	Critical,
	Error,
	Warning,
	Notice,
	Information,
	Internal,
	Debugging,
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub struct TranscriptCode ( u64 );




pub trait TranscriptOutputable : fmt::Display + fmt::Debug {}

impl <Outputable : fmt::Display + fmt::Debug + ?Sized> TranscriptOutputable for Outputable {}




pub trait TranscriptFrontend {
	
	type Context : TranscriptContext;
	type Backend : TranscriptBackend;
	
	fn context (&self) -> (&Self::Context);
	fn backend (&self) -> (&Self::Backend);
	
}


impl <Frontent : TranscriptFrontend + ?Sized> Transcript for Frontent {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn trace_format (&self, level : TranscriptLevel, code : Option<TranscriptCode>, arguments : fmt::Arguments, stylize : bool, error : Option<&TranscriptOutputable>, backend : Option<&TranscriptBackend>) -> () {
		if ! self.is_active (level) {
			return;
		}
		let context = self.context ();
		let backend = backend.unwrap_or_else (|| self.backend ());
		backend.trace_push (context, level, code, &arguments, stylize, error);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn trace_message (&self, level : TranscriptLevel, code : Option<TranscriptCode>, message : &str, stylize : bool, error : Option<&TranscriptOutputable>, backend : Option<&TranscriptBackend>) -> () {
		if ! self.is_active (level) {
			return;
		}
		let context = self.context ();
		let code = code.or_else (|| transcript_code_for_message_value (message, None, None));
		let backend = backend.unwrap_or_else (|| self.backend ());
		backend.trace_push (context, level, code, &message, stylize, error);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn trace_values (&self, level : TranscriptLevel, code : Option<TranscriptCode>, format : &str, values : &[&Value], backend : Option<&TranscriptBackend>) -> (Outcome<()>) {
		if ! self.is_active (level) {
			succeed! (());
		}
		let context = self.context ();
		let code = code.or_else (|| transcript_code_for_message_value (format, None, None));
		let backend = backend.unwrap_or_else (|| self.backend ());
		// FIXME:  Add support for actual formatting!
		
		let format_parts = format.split ("{}") .collect::<StdVec<_>> ();
		let format_parts = format_parts.as_slice ();
		let parts_count = format_parts.len () - 1;
		if parts_count != values.len () {
			fail! (0x95af0ca0);
		}
		
		macro_rules! trace_push {
			( $( $argument : tt )* ) => (
				backend.trace_push (context, level, code, & format_args! ( $( $argument )* ), true, None)
			);
		}
		
		match parts_count {
			0 =>
				trace_push! ("{}", format_parts[0]),
			1 =>
				trace_push! ("{}{}{}", format_parts[0], values[0], format_parts[1]),
			2 =>
				trace_push! ("{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2]),
			3 =>
				trace_push! ("{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3]),
			4 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4]),
			5 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5]),
			6 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5], values[5], format_parts[6]),
			7 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5], values[5], format_parts[6], values[6], format_parts[7]),
			8 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5], values[5], format_parts[6], values[6], format_parts[7], values[7], format_parts[8]),
			9 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5], values[5], format_parts[6], values[6], format_parts[7], values[7], format_parts[8], values[8], format_parts[9]),
			10 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0], format_parts[1], values[1], format_parts[2], values[2], format_parts[3], values[3], format_parts[4], values[4], format_parts[5], values[5], format_parts[6], values[6], format_parts[7], values[7], format_parts[8], values[8], format_parts[9], values[9], format_parts[10]),
			_ =>
				trace_push! ("{} >> {}", format, super::display::ValueSliceDisplay (values)),
		}
		
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_active (&self, level : TranscriptLevel) -> (bool) {
		let context = self.context ();
		return level.is_active (context.activation_level ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_supports_ansi_sequences (&self, backend : Option<&TranscriptBackend>) -> (bool) {
		let backend = backend.unwrap_or_else (|| self.backend ());
		return backend.output_supports_ansi_sequences ();
	}
}




pub trait TranscriptBackend {
	
	fn stream (&self) -> (&TranscriptStream);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn trace_push (&self, context : &TranscriptContext, level : TranscriptLevel, code : Option<TranscriptCode>, message : &TranscriptOutputable, stylize : bool, error : Option<&TranscriptOutputable>) -> () {
		let transcript_color = self.output_supports_ansi_sequences ();
		const IDENTIFIER_LENGTH : usize = 20;
		let identifier = context.identifier ();
		let (identifier_prefix, identifier_trimmed) = if identifier.starts_with ("vonuvoli_scheme::") {
			("VS~", &identifier[17 ..])
		} else if identifier.starts_with ("vonuvoli_scheme_") {
			("VS~", &identifier[16 ..])
		} else {
			("", identifier)
		};
		let (identifier_suffix, identifier_trimmed, identifier_padding) = if (identifier_prefix.len () + identifier_trimmed.len ()) > IDENTIFIER_LENGTH {
			("~", &identifier_trimmed[.. (IDENTIFIER_LENGTH - 1 - identifier_prefix.len ())], 0)
		} else {
			("", identifier_trimmed, IDENTIFIER_LENGTH - identifier_prefix.len () - identifier_trimmed.len ())
		};
		let (level_slug, header_style, message_style) = transcript_level_styles (level);
		let level_slug = transcript_style (level_slug, header_style, transcript_color);
		let identifier_trimmed = transcript_style (identifier_trimmed, header_style, transcript_color);
		let identifier_prefix = transcript_style (identifier_prefix, header_style, transcript_color);
		let identifier_suffix = transcript_style (identifier_suffix, header_style, transcript_color);
		let message_style = if stylize { Some (message_style) } else { None };
		let header_length = 4 + 1 + 1 + IDENTIFIER_LENGTH + 1 + 1 + 8 + 1;
		let stream = self.stream ();
		if let Some (code) = code {
			let code = code.0;
			// let code_1 = ((code & 0xffffffff00000000) >> 32) as u32;
			let code_2 = ((code & 0x00000000ffffffff) >> 0) as u32;
			let code_2 = transcript_style (format! ("{:08x}", code_2), header_style, transcript_color);
			if let Some (error) = error {
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}][{}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								code_2,
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}\u{1d}{}\u{1e}{:#?}", message, error, error),
						message_style,
						true,
					);
			} else {
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}][{}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								code_2,
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}", message),
						message_style,
						true,
					);
			}
		} else {
			if let Some (error) = error {
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}\u{1d}{}\u{1e}{:#?}", message, error, error),
						message_style,
						true,
					);
			} else {
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}", message),
						message_style,
						true,
					);
			}
		}
		stream.output_flush ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_supports_ansi_sequences (&self) -> (bool) {
		let stream = self.stream ();
		return stream.output_supports_ansi_sequences ();
	}
}




pub trait TranscriptStream {
	
	fn output_push_fmt (&self, header : Option<fmt::Arguments>, header_length : usize, message : fmt::Arguments, message_style : Option<TranscriptStyle>, sanitize : bool) -> ();
	fn output_push_str (&self, header : Option<fmt::Arguments>, header_length : usize, message : &str, message_style : Option<TranscriptStyle>, sanitize : bool) -> ();
	fn output_flush (&self) -> ();
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_supports_ansi_sequences (&self) -> (bool) {
		false
	}
}




pub struct TranscriptBackendForStderr ();


impl TranscriptBackendForStderr {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new () -> (TranscriptBackendForStderr) {
		TranscriptBackendForStderr ()
	}
}


impl TranscriptBackend for TranscriptBackendForStderr {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn stream (&self) -> (&TranscriptStream) {
		return self;
	}
}


impl TranscriptStream for TranscriptBackendForStderr {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_push_fmt (&self, header : Option<fmt::Arguments>, header_length : usize, message : fmt::Arguments, message_style : Option<TranscriptStyle>, sanitize : bool) -> () {
		const SEPARATOR_HEADER : &str = " |  ";
		const SEPARATOR_MULTILINE : &str = " :  ";
		const SEPARATOR_UNIT : &str = " -  ";
		const SEPARATOR_UNIT_BREAK : bool = TRANSCRIPT_OUTPUT_SEPARATOR_UNIT_BREAK;
		const MULTILINE_ALIGN_RIGHT : bool = TRANSCRIPT_OUTPUT_MULTILINE_ALIGN_RIGHT;
		const MULTILINE_BREAK_BEFORE : bool = TRANSCRIPT_OUTPUT_MULTILINE_BREAK_BEFORE;
		const MULTILINE_BREAK_HEADER : bool = TRANSCRIPT_OUTPUT_MULTILINE_BREAK_HEADER;
		const MULTILINE_BREAK_AFTER : bool = TRANSCRIPT_OUTPUT_MULTILINE_BREAK_AFTER;
		let transcript_color = TranscriptStream::output_supports_ansi_sequences (self) && message_style.is_some ();
		let message_style = message_style.unwrap_or (TRANSCRIPT_STYLE_NONE);
		let stream = io::stderr ();
		let mut stream = stream.lock ();
		let header = if let Some (header) = header {
			let mut header_buffer = header.to_string ();
			header_buffer
		} else {
			StdString::new ()
		};
		if sanitize {
			let padding = if MULTILINE_ALIGN_RIGHT {
				" " .repeat (header_length)
			} else {
				" " .repeat (4)
			};
			let message_buffer = message.to_string ();
			let message_buffer = message_buffer.trim_matches (&['\n', '\r'][..]);
			let message_lines = message_buffer.lines () .collect::<StdVec<_>> ();
			let message_lines_count = message_lines.len ();
			if message_lines_count > 0 {
				let mut buffer = StdString::with_capacity (TRANSCRIPT_BUFFER_SIZE);
				for (index, message_line) in message_lines.into_iter () .enumerate () {
					buffer.clear ();
					if index == 0 {
						if MULTILINE_BREAK_BEFORE && (message_lines_count > 1) {
							buffer.push ('\n');
						}
						buffer.push_str (&header);
						if ! MULTILINE_ALIGN_RIGHT && message_lines_count > 1 {
							buffer.push_str ("\n");
							if MULTILINE_BREAK_HEADER {
								buffer.push_str ("\n");
							}
							buffer.push_str (&padding);
							buffer.push_str (SEPARATOR_MULTILINE);
						} else {
							buffer.push_str (SEPARATOR_HEADER);
						}
					} else {
						buffer.push_str (&padding);
						buffer.push_str (SEPARATOR_MULTILINE);
					}
					let mut push_unit = false;
					let mut style_initialized = false;
					for message_character in message_line.chars () {
						let message_character = if message_character.is_control () {
							match message_character {
								'\t' =>
									message_character,
								'\u{1b}' =>
									message_character,
								'\u{1c}' | '\u{1d}' | '\u{1e}' | '\u{1f}' => {
									push_unit = true;
									continue;
								},
								_ =>
									// TODO:  Replace with equivalent "control picture character"!
									char::REPLACEMENT_CHARACTER,
							}
						} else {
							message_character
						};
						if push_unit {
							if style_initialized {
								transcript_style_push_finalize (&mut buffer, message_style, transcript_color);
								style_initialized = false;
							}
							buffer.push_str ("\n");
							buffer.push_str (&padding);
							buffer.push_str (SEPARATOR_UNIT);
							if SEPARATOR_UNIT_BREAK {
								buffer.push_str ("\n");
								buffer.push_str (&padding);
								buffer.push_str (SEPARATOR_MULTILINE);
							}
							push_unit = false;
						}
						if ! style_initialized {
							transcript_style_push_initialize (&mut buffer, message_style, transcript_color);
							// TODO:  Why does the compiler thinks we don't use this?
							#[ allow (unused_assignments) ]
							style_initialized = true;
						}
						buffer.push (message_character);
					}
					if style_initialized {
						transcript_style_push_finalize (&mut buffer, message_style, transcript_color);
						// TODO:  Why does the compiler thinks we don't use this?
						#[ allow (unused_assignments) ]
						style_initialized = false;
					}
					buffer.push ('\n');
					if MULTILINE_BREAK_AFTER && (message_lines_count > 1) && (index == (message_lines_count - 1)) {
						buffer.push ('\n');
					}
					self.output_outcome (stream.write_all (buffer.as_bytes ()));
				}
			} else {
				self.output_outcome (stream.write_all (header.as_bytes ()));
				self.output_outcome (stream.write_all (SEPARATOR_HEADER.as_bytes ()));
				self.output_outcome (stream.write_all ("???\n".as_bytes ()));
			}
		} else {
			let message_buffer = message.to_string ();
			self.output_outcome (stream.write_all (header.as_bytes ()));
			self.output_outcome (stream.write_all (SEPARATOR_HEADER.as_bytes ()));
			self.output_outcome (stream.write_fmt (format_args! ("{}", transcript_style (message_buffer, message_style, transcript_color))));
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_push_str (&self, header : Option<fmt::Arguments>, header_length : usize, message : &str, message_style : Option<TranscriptStyle>, sanitize : bool)  -> () {
		self.output_push_fmt (header, header_length, format_args! ("{}", message), message_style, sanitize);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_flush (&self)  -> () {
		let stream = io::stderr ();
		let mut stream = stream.lock ();
		self.output_outcome (stream.flush ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_supports_ansi_sequences (&self) -> (bool) {
		if ! TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ENABLED {
			return false;
		}
		if TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ALWAYS {
			return true;
		}
		#[ cfg ( feature = "vonuvoli_terminal" ) ]
		return ext::atty::is (ext::atty::Stream::Stderr);
		#[ cfg ( not ( feature = "vonuvoli_terminal" ) ) ]
		return false;
	}
}


impl TranscriptBackendForStderr {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_outcome (&self, outcome : io::Result<()>) -> () {
		match outcome {
			Ok (()) =>
				(),
			Err (_error) =>
				panic_0! (0x89a3e336, (github_issue, 48)),
		}
	}
}




impl TranscriptLevel {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_active (&self, level : Option<TranscriptLevel>) -> (bool) {
		if let Some (level) = level {
			if level >= *self {
				return true;
			} else {
				return false;
			}
		} else {
			return true;
		}
	}
}




pub struct TranscriptForModule {
	parent : Option<&'static TranscriptForModule>,
	module : &'static str,
	activation_level : Option<TranscriptLevel>,
	backend : TranscriptBackendForStderr,
}


impl TranscriptForModule {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new (module : &'static str, parent : &'static TranscriptForModule) -> (TranscriptForModule) {
		TranscriptForModule {
				parent : Some (parent),
				module : module,
				activation_level : None,
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new_with_level (module : &'static str, activation_level : TranscriptLevel, parent : &'static TranscriptForModule) -> (TranscriptForModule) {
		TranscriptForModule {
				parent : Some (parent),
				module : module,
				activation_level : Some (activation_level),
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new_root (module : &'static str, activation_level : Option<TranscriptLevel>) -> (TranscriptForModule) {
		TranscriptForModule {
				parent : None,
				module : module,
				activation_level : activation_level,
				backend : TranscriptBackendForStderr::new (),
			}
	}
}


impl TranscriptFrontend for TranscriptForModule {
	
	type Context = TranscriptForModule;
	type Backend = TranscriptBackendForStderr;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn context (&self) -> (&Self::Context) {
		self
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn backend (&self) -> (&Self::Backend) {
		&self.backend
	}
}


impl TranscriptContext for TranscriptForModule {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier (&self) -> (&str) {
		self.module
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn activation_level (&self) -> (Option<TranscriptLevel>) {
		// TODO:  Cache the activation level computation!
		if self.activation_level.is_some () {
			self.activation_level
		} else if let Some (parent) = self.parent {
			parent.activation_level ()
		} else {
			None
		}
	}
}




pub struct TranscriptForScript {
	module : &'static str,
	activation_level : Option<TranscriptLevel>,
	backend : TranscriptBackendForStderr,
}


impl TranscriptForScript {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new (module : &'static str) -> (TranscriptForScript) {
		TranscriptForScript {
				module : module,
				activation_level : None,
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn new_with_level (module : &'static str, activation_level : TranscriptLevel) -> (TranscriptForScript) {
		TranscriptForScript {
				module : module,
				activation_level : Some (activation_level),
				backend : TranscriptBackendForStderr::new (),
			}
	}
}


impl TranscriptFrontend for TranscriptForScript {
	
	type Context = TranscriptForScript;
	type Backend = TranscriptBackendForStderr;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn context (&self) -> (&Self::Context) {
		self
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn backend (&self) -> (&Self::Backend) {
		&self.backend
	}
}


impl TranscriptContext for TranscriptForScript {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier (&self) -> (&str) {
		self.module
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn activation_level (&self) -> (Option<TranscriptLevel>) {
		self.activation_level
	}
}


impl fmt::Debug for TranscriptForScript {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		// TODO:  Imlement this!
		formatter.debug_tuple ("TranscriptForScript") .finish ()
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_for_script () -> (StdRc<TranscriptForScript>) {
	let transcript = TranscriptForScript::new ("<script>");
	return StdRc::new (transcript);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn transcript_level_styles (level : TranscriptLevel) -> (&'static str, TranscriptStyle, TranscriptStyle) {
	match level {
		TranscriptLevel::Critical =>
			("[!!]", TRANSCRIPT_HEADER_STYLE_FOR_CRITICAL, TRANSCRIPT_MESSAGE_STYLE_FOR_CRITICAL),
		TranscriptLevel::Error =>
			("[ee]", TRANSCRIPT_HEADER_STYLE_FOR_ERROR, TRANSCRIPT_MESSAGE_STYLE_FOR_CRITICAL),
		TranscriptLevel::Warning =>
			("[ww]", TRANSCRIPT_HEADER_STYLE_FOR_WARNING, TRANSCRIPT_MESSAGE_STYLE_FOR_WARNING),
		TranscriptLevel::Notice =>
			("[ii]", TRANSCRIPT_HEADER_STYLE_FOR_NOTICE, TRANSCRIPT_MESSAGE_STYLE_FOR_NOTICE),
		TranscriptLevel::Information =>
			("[ii]", TRANSCRIPT_HEADER_STYLE_FOR_INFORMATION, TRANSCRIPT_MESSAGE_STYLE_FOR_INFORMATION),
		TranscriptLevel::Internal =>
			("[dd]", TRANSCRIPT_HEADER_STYLE_FOR_INTERNAL, TRANSCRIPT_MESSAGE_STYLE_FOR_INTERNAL),
		TranscriptLevel::Debugging =>
			("[dd]", TRANSCRIPT_HEADER_STYLE_FOR_DEBUGGING, TRANSCRIPT_MESSAGE_STYLE_FOR_DEBUGGING),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_code_for_source (code : u32, _file : Option<&'static str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let code = code as u64;
	Some (TranscriptCode (code))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_code_for_message_static (message : &'static str, _file : Option<&'static str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let code = unsafe { mem::transmute (message.as_ptr ()) };
	Some (TranscriptCode (code))
}


#[ cfg ( feature = "vonuvoli_transcript_code_hashes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_code_for_message_value (message : &str, _file : Option<&str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let hash = ext::blake2_rfc::blake2s::blake2s (64 / 8, &[], message.as_bytes ());
	let hash = hash.as_bytes ();
	let hash = [hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]];
	let code = unsafe { mem::transmute (hash) };
	Some (TranscriptCode (code))
}

#[ cfg ( not ( feature = "vonuvoli_transcript_code_hashes" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_code_for_message_value (_message : &str, _file : Option<&str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	None
}




static TRANSCRIPT_HEADER_STYLE_FOR_CRITICAL : TranscriptStyle = TRANSCRIPT_STYLE_RED;
static TRANSCRIPT_HEADER_STYLE_FOR_ERROR : TranscriptStyle = TRANSCRIPT_STYLE_RED;
static TRANSCRIPT_HEADER_STYLE_FOR_WARNING : TranscriptStyle = TRANSCRIPT_STYLE_YELLOW;
static TRANSCRIPT_HEADER_STYLE_FOR_NOTICE : TranscriptStyle = TRANSCRIPT_STYLE_GREEN;
static TRANSCRIPT_HEADER_STYLE_FOR_INFORMATION : TranscriptStyle = TRANSCRIPT_STYLE_WHITE;
static TRANSCRIPT_HEADER_STYLE_FOR_INTERNAL : TranscriptStyle = TRANSCRIPT_STYLE_WHITE;
static TRANSCRIPT_HEADER_STYLE_FOR_DEBUGGING : TranscriptStyle = TRANSCRIPT_STYLE_WHITE;

static TRANSCRIPT_MESSAGE_STYLE_FOR_CRITICAL : TranscriptStyle = TRANSCRIPT_STYLE_RED_BOLD;
static TRANSCRIPT_MESSAGE_STYLE_FOR_ERROR : TranscriptStyle = TRANSCRIPT_STYLE_RED_BOLD;
static TRANSCRIPT_MESSAGE_STYLE_FOR_WARNING : TranscriptStyle = TRANSCRIPT_STYLE_YELLOW_BOLD;
static TRANSCRIPT_MESSAGE_STYLE_FOR_NOTICE : TranscriptStyle = TRANSCRIPT_STYLE_GREEN_BOLD;
static TRANSCRIPT_MESSAGE_STYLE_FOR_INFORMATION : TranscriptStyle = TRANSCRIPT_STYLE_WHITE_BOLD;
static TRANSCRIPT_MESSAGE_STYLE_FOR_INTERNAL : TranscriptStyle = TRANSCRIPT_STYLE_WHITE;
static TRANSCRIPT_MESSAGE_STYLE_FOR_DEBUGGING : TranscriptStyle = TRANSCRIPT_STYLE_WHITE;




#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style <'a, I, S> (input : I, style : &ext::ansi_term::Style, color : bool) -> (ext::ansi_term::ANSIGenericString<'a, S>)
		where
			I : StdInto<borrow::Cow<'a, S>>,
			S : 'a + borrow::ToOwned + ?Sized,
			<S as borrow::ToOwned>::Owned : fmt::Debug,
{
	if color {
		return style.paint (input);
	} else {
		return TRANSCRIPT_STYLE_NONE.paint (input);
	}
}

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style <I> (input : I, _style : TranscriptStyle, _color : bool) -> (I) {
	return input;
}


#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style_push_initialize <W : fmt::Write> (writer : &mut W, style : &ext::ansi_term::Style, color : bool) -> () {
	#![ allow (unused_must_use) ]
	if color {
		// TODO:  Handle this error!
		writer.write_fmt (format_args! ("{}", style.prefix ()));
	}
}

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style_push_initialize <W : fmt::Write> (_writer : &mut W, _style : TranscriptStyle, _color : bool) -> () {}


#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style_push_finalize <W : fmt::Write> (writer : &mut W, style : &ext::ansi_term::Style, color : bool) -> () {
	#![ allow (unused_must_use) ]
	if color {
		// TODO:  Handle this error!
		writer.write_fmt (format_args! ("{}", style.suffix ()));
	}
}

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn transcript_style_push_finalize <W : fmt::Write> (_writer : &mut W, _style : TranscriptStyle, _color : bool) -> () {}




#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
pub type TranscriptStyle = &'static ext::ansi_term::Style;

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
pub type TranscriptStyle = ();




macro_rules! def_transcript_style {
	( $identifier_normal : ident, $identifier_bold : ident, $color : expr ) => (
		
		#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
		pub const $identifier_normal : &'static ext::ansi_term::Style = & ext::ansi_term::Style {
				foreground : Some (ext::ansi_term::Colour::Fixed ($color)),
				background : None,
				is_bold : false, is_italic : false, is_underline : false, is_strikethrough : false,
				is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
			};
		
		#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
		pub const $identifier_bold : &'static ext::ansi_term::Style = & ext::ansi_term::Style {
				foreground : Some (ext::ansi_term::Colour::Fixed ($color)),
				background : None,
				is_bold : true, is_italic : false, is_underline : false, is_strikethrough : false,
				is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
			};
		
		#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
		pub const $identifier_normal : () = ();
		
		#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
		pub const $identifier_bold : () = ();
		
	);
}


def_transcript_style! (TRANSCRIPT_STYLE_BLACK, TRANSCRIPT_STYLE_BLACK_BOLD, 0);
def_transcript_style! (TRANSCRIPT_STYLE_RED, TRANSCRIPT_STYLE_RED_BOLD, 1);
def_transcript_style! (TRANSCRIPT_STYLE_GREEN, TRANSCRIPT_STYLE_GREEN_BOLD, 2);
def_transcript_style! (TRANSCRIPT_STYLE_YELLOW, TRANSCRIPT_STYLE_YELLOW_BOLD, 3);
def_transcript_style! (TRANSCRIPT_STYLE_BLUE, TRANSCRIPT_STYLE_BLUE_BOLD, 4);
def_transcript_style! (TRANSCRIPT_STYLE_PURPLE, TRANSCRIPT_STYLE_PURPLE_BOLD, 5);
def_transcript_style! (TRANSCRIPT_STYLE_CYAN, TRANSCRIPT_STYLE_CYAN_BOLD, 6);
def_transcript_style! (TRANSCRIPT_STYLE_WHITE, TRANSCRIPT_STYLE_WHITE_BOLD, 7);

def_transcript_style! (TRANSCRIPT_STYLE_GRAY, TRANSCRIPT_STYLE_GRAY_BOLD, 8);
def_transcript_style! (TRANSCRIPT_STYLE_RED_2, TRANSCRIPT_STYLE_RED_2_BOLD, 9);
def_transcript_style! (TRANSCRIPT_STYLE_GREEN_2, TRANSCRIPT_STYLE_GREEN_2_BOLD, 10);
def_transcript_style! (TRANSCRIPT_STYLE_YELLOW_2, TRANSCRIPT_STYLE_YELLOW_2_BOLD, 11);
def_transcript_style! (TRANSCRIPT_STYLE_BLUE_2, TRANSCRIPT_STYLE_BLUE_2_BOLD, 12);
def_transcript_style! (TRANSCRIPT_STYLE_PURPLE_2, TRANSCRIPT_STYLE_PURPLE_2_BOLD, 13);
def_transcript_style! (TRANSCRIPT_STYLE_CYAN_2, TRANSCRIPT_STYLE_CYAN_2_BOLD, 14);
def_transcript_style! (TRANSCRIPT_STYLE_WHITE_2, TRANSCRIPT_STYLE_WHITE_2_BOLD, 15);




#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
pub const TRANSCRIPT_STYLE_NONE : &'static ext::ansi_term::Style = & ext::ansi_term::Style {
		foreground : None,
		background : None,
		is_bold : false, is_italic : false, is_underline : false, is_strikethrough : false,
		is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
	};

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
pub const TRANSCRIPT_STYLE_NONE : () = ();

