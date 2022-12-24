

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
			
			TranscriptMessage,
			TranscriptError,
			
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
			transcript_code_new,
			
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
	
	fn trace_format (&self, level : TranscriptLevel, code : Option<TranscriptCode>, arguments : fmt::Arguments, stylize : bool, error : Option<&dyn TranscriptError>, backend : Option<&dyn TranscriptBackend>) -> (bool);
	fn trace_message (&self, level : TranscriptLevel, code : Option<TranscriptCode>, message : &str, stylize : bool, error : Option<&dyn TranscriptError>, backend : Option<&dyn TranscriptBackend>) -> (bool);
	fn trace_values (&self, level : TranscriptLevel, code : Option<TranscriptCode>, format : &str, values : &[impl StdAsRef<Value>], backend : Option<&dyn TranscriptBackend>) -> (Outcome<(bool)>);
	
	fn trace_buffer (&self, level : TranscriptLevel, code : Option<TranscriptCode>, buffer : TranscriptBuffer<Self>, stylize : bool, backend : Option<&dyn TranscriptBackend>) -> (bool) {
		return self.trace_message (level, code, &buffer.buffer, stylize, None, backend);
	}
	
	fn is_active (&self, level : TranscriptLevel) -> (bool);
	
	fn output_supports_ansi_sequences (&self, backend : Option<&dyn TranscriptBackend>) -> (bool);
	
	fn tracer <'a> (&'a self, level : TranscriptLevel, code : Option<TranscriptCode>, backend : Option<&'a dyn TranscriptBackend>) -> (TranscriptTracer<'a, Self>) {
		TranscriptTracer {
				transcript : self,
				level : level,
				code : code,
				backend : backend,
			}
	}
	
	fn buffer <'a> (&'a self, backend : Option<&'a dyn TranscriptBackend>) -> (TranscriptBuffer<'a, Self>) {
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
	backend : Option<&'a dyn TranscriptBackend>,
}


impl <'a, T : Transcript + ?Sized + 'a> TranscriptTracer<'a, T> {
	
	pub fn trace_format (&self, arguments : fmt::Arguments, stylize : bool, error : Option<&dyn TranscriptError>) -> (bool) {
		return self.transcript.trace_format (self.level, self.code, arguments, stylize, error, self.backend);
	}
	
	pub fn trace_message (&self, message : &str, stylize : bool, error : Option<&dyn TranscriptError>) -> (bool) {
		return self.transcript.trace_message (self.level, self.code, message, stylize, error, self.backend);
	}
	
	pub fn trace_values (&self, format : &str, values : &[impl StdAsRef<Value>]) -> (Outcome<(bool)>) {
		return self.transcript.trace_values (self.level, self.code, format, values, self.backend);
	}
	
	pub fn trace_buffer (&self, buffer : TranscriptBuffer<T>, stylize : bool) -> (bool) {
		return self.transcript.trace_buffer (self.level, self.code, buffer, stylize, self.backend);
	}
	
	fn is_active (&self, level : TranscriptLevel) -> (bool) {
		return self.transcript.is_active (level);
	}
	
	pub fn output_supports_ansi_sequences (&self) -> (bool) {
		return self.transcript.output_supports_ansi_sequences (self.backend);
	}
	
	pub fn buffer (&self) -> (TranscriptBuffer<'a, T>) {
		return self.transcript.buffer (self.backend);
	}
}




pub struct TranscriptBuffer <'a, T : Transcript + ?Sized + 'a> {
	transcript : &'a T,
	buffer : StdString,
	backend : Option<&'a dyn TranscriptBackend>,
}


impl <'a, T : Transcript + ?Sized + 'a> TranscriptBuffer<'a, T> {
	
	pub fn push_fmt (&mut self, arguments : fmt::Arguments) -> () {
		match self.buffer.write_fmt (arguments) {
			Ok (()) =>
				(),
			Err (_error) =>
				panic_0! (0x232a35a0, (github_issue, 48)),
		}
	}
	
	pub fn push_str (&mut self, message : &str) -> () {
		match self.buffer.write_str (message) {
			Ok (()) =>
				(),
			Err (_error) =>
				panic_0! (0x4308fce7, (github_issue, 48)),
		}
	}
	
	pub fn output_supports_ansi_sequences (&self) -> (bool) {
		return self.transcript.output_supports_ansi_sequences (self.backend);
	}
}




pub trait TranscriptContext {
	
	fn identifier (&self) -> (&str);
	fn activation_level (&self) -> (Option<TranscriptLevel>);
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
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
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct TranscriptCode ( u64 );




pub trait TranscriptMessage : fmt::Display + fmt::Debug {}

impl <Message : fmt::Display + fmt::Debug + ?Sized> TranscriptMessage for Message {}




pub trait TranscriptError : fmt::Display + fmt::Debug {
	
	fn transcript_message (&self) -> (Option<borrow::Cow<str>>);
	
	fn transcript_code_2 (&self) -> (Option<(u32, u32)>) {
		None
	}
	
	fn transcript_set_reported (&self) -> () {}
	
	fn transcript_was_reported (&self) -> (bool) {
		false
	}
}

impl TranscriptError for Error {
	
	fn transcript_message (&self) -> (Option<borrow::Cow<str>>) {
		option_map! (self.message (), message, borrow::Cow::Borrowed (message))
	}
	
	fn transcript_code_2 (&self) -> (Option<(u32, u32)>) {
		Some (self.code_2 ())
	}
	
	fn transcript_set_reported (&self) -> () {
		self.set_reported (true)
	}
	
	fn transcript_was_reported (&self) -> (bool) {
		self.was_reported ()
	}
}

impl TranscriptError for io::Error {
	
	fn transcript_message (&self) -> (Option<borrow::Cow<str>>) {
		Some (borrow::Cow::Borrowed (::std::error::Error::description (self)))
	}
}

impl TranscriptError for StdString {
	
	fn transcript_message (&self) -> (Option<borrow::Cow<str>>) {
		Some (borrow::Cow::Borrowed (self.as_str ()))
	}
}




pub trait TranscriptFrontend {
	
	type Context : TranscriptContext;
	type Backend : TranscriptBackend;
	
	fn context (&self) -> (&Self::Context);
	fn backend (&self) -> (&Self::Backend);
	
}


impl <Frontent : TranscriptFrontend + ?Sized> Transcript for Frontent {
	
	fn trace_format (&self, level : TranscriptLevel, code : Option<TranscriptCode>, arguments : fmt::Arguments, stylize : bool, error : Option<&dyn TranscriptError>, backend : Option<&dyn TranscriptBackend>) -> (bool) {
		if ! self.is_active (level) {
			return false;
		}
		let context = self.context ();
		let backend = backend.unwrap_or_else (|| self.backend ());
		return backend.trace_push (context, level, code, &arguments, stylize, error);
	}
	
	fn trace_message (&self, level : TranscriptLevel, code : Option<TranscriptCode>, message : &str, stylize : bool, error : Option<&dyn TranscriptError>, backend : Option<&dyn TranscriptBackend>) -> (bool) {
		if ! self.is_active (level) {
			return false;
		}
		let context = self.context ();
		let code = code.or_else (|| transcript_code_for_message_value (message, None, None));
		let backend = backend.unwrap_or_else (|| self.backend ());
		return backend.trace_push (context, level, code, &message, stylize, error);
	}
	
	fn trace_values (&self, level : TranscriptLevel, code : Option<TranscriptCode>, format : &str, values : &[impl StdAsRef<Value>], backend : Option<&dyn TranscriptBackend>) -> (Outcome<bool>) {
		if ! self.is_active (level) {
			succeed! (false);
		}
		let context = self.context ();
		let code = code.or_else (|| transcript_code_for_message_value (format, None, None));
		let backend = backend.unwrap_or_else (|| self.backend ());
		
		FIXME! ("add support for actual formatting");
		
		let format_parts = format.split ("{}") .collect::<StdVec<_>> ();
		let format_parts = format_parts.as_slice ();
		let parts_count = format_parts.len () - 1;
		if parts_count != values.len () {
			fail! (0x95af0ca0);
		}
		
		macro_rules! trace_push {
			( $( $argument : tt )* ) => (
				succeed! (backend.trace_push (context, level, code, & format_args! ( $( $argument )* ), true, None))
			);
		}
		
		match parts_count {
			0 =>
				trace_push! ("{}", format_parts[0]),
			1 =>
				trace_push! ("{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1]),
			2 =>
				trace_push! ("{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2]),
			3 =>
				trace_push! ("{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3]),
			4 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4]),
			5 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5]),
			6 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5], values[5].as_ref (), format_parts[6]),
			7 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5], values[5].as_ref (), format_parts[6], values[6].as_ref (), format_parts[7]),
			8 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5], values[5].as_ref (), format_parts[6], values[6].as_ref (), format_parts[7], values[7].as_ref (), format_parts[8]),
			9 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5], values[5].as_ref (), format_parts[6], values[6].as_ref (), format_parts[7], values[7].as_ref (), format_parts[8], values[8].as_ref (), format_parts[9]),
			10 =>
				trace_push! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format_parts[0], values[0].as_ref (), format_parts[1], values[1].as_ref (), format_parts[2], values[2].as_ref (), format_parts[3], values[3].as_ref (), format_parts[4], values[4].as_ref (), format_parts[5], values[5].as_ref (), format_parts[6], values[6].as_ref (), format_parts[7], values[7].as_ref (), format_parts[8], values[8].as_ref (), format_parts[9], values[9].as_ref (), format_parts[10]),
			_ =>
				trace_push! ("{} >> {}", format, super::display::ValueSliceDisplay (values)),
		}
	}
	
	fn is_active (&self, level : TranscriptLevel) -> (bool) {
		let context = self.context ();
		return level.is_active (context.activation_level ());
	}
	
	fn output_supports_ansi_sequences (&self, backend : Option<&dyn TranscriptBackend>) -> (bool) {
		let backend = backend.unwrap_or_else (|| self.backend ());
		return backend.output_supports_ansi_sequences ();
	}
}




pub trait TranscriptBackend {
	
	fn stream (&self) -> (&dyn TranscriptStream);
	
	fn trace_push (&self, context : &dyn TranscriptContext, level : TranscriptLevel, code : Option<TranscriptCode>, message : &dyn TranscriptMessage, stylize : bool, error : Option<&dyn TranscriptError>) -> (bool) {
		const IDENTIFIER_LENGTH : usize = 20;
		let transcript_color = self.output_supports_ansi_sequences ();
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
			let code_2 = (code & 0x00000000ffffffff) as u32;
			let code_2 = transcript_style (format! ("{:08x}", code_2), header_style, transcript_color);
			if let Some (error) = error {
				let (error_code_1, error_code_2) = error.transcript_code_2 () .unwrap_or ((0, 0));
				let error_message = error.transcript_message ();
				let error_message = option_ref_map! (error_message, error_message.deref ());
				let error_message = error_message.unwrap_or ("<no message>");
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}][{}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								code_2,
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}\u{1d}[{:08x}:{:08x}]\n{}", message, error_code_1, error_code_2, error_message),
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
				let (error_code_1, error_code_2) = error.transcript_code_2 () .unwrap_or ((0, 0));
				let error_message = error.transcript_message ();
				let error_message = option_ref_map! (error_message, error_message.deref ());
				let error_message = error_message.unwrap_or ("<no message>");
				stream.output_push_fmt (
						Some (format_args! (
								"{} [{}{}{}{:identifier_padding$}]",
								level_slug,
								identifier_prefix, identifier_trimmed, identifier_suffix, "",
								identifier_padding = identifier_padding,
							)),
						header_length,
						format_args! ("{}\u{1d}[{:08x}:{:08x}]\n{}", message, error_code_1, error_code_2, error_message),
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
		return true;
	}
	
	fn output_supports_ansi_sequences (&self) -> (bool) {
		let stream = self.stream ();
		return stream.output_supports_ansi_sequences ();
	}
}




pub trait TranscriptStream {
	
	fn output_push_fmt (&self, header : Option<fmt::Arguments>, header_length : usize, message : fmt::Arguments, message_style : Option<TranscriptStyle>, sanitize : bool) -> ();
	fn output_push_str (&self, header : Option<fmt::Arguments>, header_length : usize, message : &str, message_style : Option<TranscriptStyle>, sanitize : bool) -> ();
	fn output_flush (&self) -> ();
	
	fn output_supports_ansi_sequences (&self) -> (bool) {
		false
	}
}




pub struct TranscriptBackendForStderr ();


impl TranscriptBackendForStderr {
	
	pub const fn new () -> (TranscriptBackendForStderr) {
		TranscriptBackendForStderr ()
	}
}


impl TranscriptBackend for TranscriptBackendForStderr {
	
	fn stream (&self) -> (&dyn TranscriptStream) {
		return self;
	}
}


impl TranscriptStream for TranscriptBackendForStderr {
	
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
		let mut stream = io::stderr () .lock ();
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
								_ => {
									TODO! ("replace with equivalent control picture character");
									char::REPLACEMENT_CHARACTER
								},
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
							style_initialized = true;
						}
						buffer.push (message_character);
					}
					// NOTE:  This is needed in case the line contains only the unit separator!
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
					}
					if style_initialized {
						transcript_style_push_finalize (&mut buffer, message_style, transcript_color);
						#[ allow (unused_assignments) ]  // NOTE:  For some reason the compiler emits a warning...
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
				self.output_outcome (stream.write_all (b"???\n"));
			}
		} else {
			let message_buffer = message.to_string ();
			self.output_outcome (stream.write_all (header.as_bytes ()));
			self.output_outcome (stream.write_all (SEPARATOR_HEADER.as_bytes ()));
			self.output_outcome (stream.write_fmt (format_args! ("{}", transcript_style (message_buffer, message_style, transcript_color))));
		}
	}
	
	fn output_push_str (&self, header : Option<fmt::Arguments>, header_length : usize, message : &str, message_style : Option<TranscriptStyle>, sanitize : bool)  -> () {
		self.output_push_fmt (header, header_length, format_args! ("{}", message), message_style, sanitize);
	}
	
	fn output_flush (&self)  -> () {
		let mut stream = io::stderr () .lock ();
		self.output_outcome (stream.flush ());
	}
	
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
	
	fn is_active (self, level : Option<TranscriptLevel>) -> (bool) {
		if let Some (level) = level {
			return level >= self;
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
	
	pub const fn new (module : &'static str, parent : &'static TranscriptForModule) -> (TranscriptForModule) {
		TranscriptForModule {
				parent : Some (parent),
				module : module,
				activation_level : None,
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
	pub const fn new_with_level (module : &'static str, activation_level : TranscriptLevel, parent : &'static TranscriptForModule) -> (TranscriptForModule) {
		TranscriptForModule {
				parent : Some (parent),
				module : module,
				activation_level : Some (activation_level),
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
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
	
	fn context (&self) -> (&Self::Context) {
		self
	}
	
	fn backend (&self) -> (&Self::Backend) {
		&self.backend
	}
}


impl TranscriptContext for TranscriptForModule {
	
	fn identifier (&self) -> (&str) {
		self.module
	}
	
	fn activation_level (&self) -> (Option<TranscriptLevel>) {
		TODO! ("cache the activation level computation");
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
	
	pub const fn new (module : &'static str) -> (TranscriptForScript) {
		TranscriptForScript {
				module : module,
				activation_level : None,
				backend : TranscriptBackendForStderr::new (),
			}
	}
	
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
	
	fn context (&self) -> (&Self::Context) {
		self
	}
	
	fn backend (&self) -> (&Self::Backend) {
		&self.backend
	}
}


impl TranscriptContext for TranscriptForScript {
	
	fn identifier (&self) -> (&str) {
		self.module
	}
	
	fn activation_level (&self) -> (Option<TranscriptLevel>) {
		self.activation_level
	}
}


impl fmt::Debug for TranscriptForScript {
	
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		TODO! ("imlement this");
		formatter.debug_tuple ("TranscriptForScript") .finish ()
	}
}


pub fn transcript_for_script () -> (Outcome<StdRc<TranscriptForScript>>) {
	FIXME! ("refactor out this activation level determination");
	let activation_level = if let Some (level) = env::var_os ("VONUVOLI_SCHEME_SCRIPT_TRANSCRIPT_LEVEL") {
		let level = try_some! (level.to_str (), 0x4558b241);
		match level {
			"critical" => TranscriptLevel::Critical,
			"error" => TranscriptLevel::Error,
			"warning" => TranscriptLevel::Warning,
			"notice" => TranscriptLevel::Notice,
			"information" => TranscriptLevel::Information,
			"internal" => TranscriptLevel::Internal,
			"debugging" => TranscriptLevel::Debugging,
			_ => fail! (0xbc95f5a7),
		}
	} else {
		if cfg! (debug_assertions) {
			TranscriptLevel::Debugging
		} else {
			TranscriptLevel::Notice
		}
	};
	let transcript = TranscriptForScript::new_with_level ("<script>", activation_level);
	succeed! (StdRc::new (transcript));
}




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




pub fn transcript_code_for_source (code : u32, _file : Option<&'static str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let code = u64::from (code);
	Some (TranscriptCode (code))
}

pub fn transcript_code_for_message_static (message : &'static str, _file : Option<&'static str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let code = unsafe { mem::transmute (message.as_ptr ()) };
	Some (TranscriptCode (code))
}

pub fn transcript_code_new (code : u32) -> (Option<TranscriptCode>) {
	let code = u64::from (code);
	Some (TranscriptCode (code))
}


#[ cfg ( feature = "vonuvoli_transcript_code_hashes" ) ]
pub fn transcript_code_for_message_value (message : &str, _file : Option<&str>, _line : Option<usize>) -> (Option<TranscriptCode>) {
	let hash = ext::blake3::hash (message.as_bytes ());
	let hash = <[u8; ext::blake3::OUT_LEN]>::from (hash);
	let hash = [hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]];
	let code = unsafe { mem::transmute (hash) };
	Some (TranscriptCode (code))
}

#[ cfg ( not ( feature = "vonuvoli_transcript_code_hashes" ) ) ]
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
pub fn transcript_style <I> (input : I, _style : TranscriptStyle, _color : bool) -> (I) {
	return input;
}


#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
pub fn transcript_style_push_initialize <W : fmt::Write> (writer : &mut W, style : &ext::ansi_term::Style, color : bool) -> () {
	if color {
		try_or_panic_0! (writer.write_fmt (format_args! ("{}", style.prefix ())), 0x99f5cdb5);
	}
}

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
pub fn transcript_style_push_initialize <W : fmt::Write> (_writer : &mut W, _style : TranscriptStyle, _color : bool) -> () {}


#[ cfg ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ]
pub fn transcript_style_push_finalize <W : fmt::Write> (writer : &mut W, style : &ext::ansi_term::Style, color : bool) -> () {
	if color {
		try_or_panic_0! (writer.write_fmt (format_args! ("{}", style.suffix ())), 0x8c44fc95);
	}
}

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
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
pub const TRANSCRIPT_STYLE_NONE : &ext::ansi_term::Style = & ext::ansi_term::Style {
		foreground : None,
		background : None,
		is_bold : false, is_italic : false, is_underline : false, is_strikethrough : false,
		is_dimmed : false, is_blink : false, is_reverse : false, is_hidden : false,
	};

#[ cfg ( not ( all ( feature = "vonuvoli_terminal", feature = "vonuvoli_transcript_ansi_enabled" ) ) ) ]
pub const TRANSCRIPT_STYLE_NONE : () = ();

