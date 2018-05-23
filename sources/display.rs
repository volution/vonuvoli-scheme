

use super::values::exports::*;

#[ allow (unused_imports) ] // OK
use super::constants::exports::*;

#[ allow (unused_imports) ] // OK
use super::contexts::exports::*;

#[ allow (unused_imports) ] // OK
use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ allow (unused_imports) ] // OK
use super::expressions::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ allow (unused_imports) ] // OK
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ allow (unused_imports) ] // OK
use super::native_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ allow (unused_imports) ] // OK
use super::native_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ allow (unused_imports) ] // OK
use super::parameters::exports::*;

use super::prelude::*;




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.kind_match_as_ref () {
			
			ValueKindMatchAsRef::Null => NULL.fmt (formatter),
			ValueKindMatchAsRef::Void => VOID.fmt (formatter),
			ValueKindMatchAsRef::Undefined => UNDEFINED.fmt (formatter),
			ValueKindMatchAsRef::Singleton (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::Boolean (self_0) => self_0.fmt (formatter),
			ValueKindMatchAsRef::NumberInteger (self_0) => self_0.fmt (formatter),
			ValueKindMatchAsRef::NumberReal (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::Character (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::Symbol (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef::Keyword (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef::Unique (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::StringMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::BytesMutable (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringRegex (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesRegex (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::PairImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::PairMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef::ArrayImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::ArrayMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef::Values (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordKind (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::RecordMutable (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef::Error (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::ProcedurePrimitive (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::SyntaxPrimitive (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::SyntaxExtended (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::SyntaxNative (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::SyntaxLambda (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef::Path (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef::Port (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef::Process (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Context (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Binding (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameters (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef::Promise (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef::Opaque (self_0) => self_0.fmt (formatter),
			
		}
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.kind_match_as_ref () {
			
			ValueKindMatchAsRef::Null => NULL.fmt (formatter),
			ValueKindMatchAsRef::Void => VOID.fmt (formatter),
			ValueKindMatchAsRef::Undefined => UNDEFINED.fmt (formatter),
			ValueKindMatchAsRef::Singleton (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::Boolean (self_0) => self_0.fmt (formatter),
			ValueKindMatchAsRef::NumberInteger (self_0) => self_0.fmt (formatter),
			ValueKindMatchAsRef::NumberReal (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::Character (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::Symbol (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef::Keyword (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef::Unique (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::StringMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::BytesMutable (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringRegex (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesRegex (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::PairImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::PairMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef::ArrayImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::ArrayMutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef::Values (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordKind (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordImmutable (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::RecordMutable (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef::Error (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::ProcedurePrimitive (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (self_0) => self_0.fmt (formatter),
			
			ValueKindMatchAsRef::SyntaxPrimitive (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::SyntaxExtended (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::SyntaxNative (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::SyntaxLambda (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef::Path (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef::Port (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef::Process (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Context (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Binding (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameters (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (self_0) => self_0.fmt (formatter),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef::Promise (self_0) => self_0.fmt (formatter),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef::Opaque (self_0) => self_0.fmt (formatter),
			
		}
	}
}




pub(crate) struct ValueSliceDisplay <'a, ValueAsRef : StdAsRef<Value> + 'a> ( pub(crate) &'a [ValueAsRef] );

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl <'a, ValueAsRef : StdAsRef<Value> + 'a> fmt::Display for ValueSliceDisplay<'a, ValueAsRef> {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		try! (formatter.write_str ("[ "));
		for value in self.0 {
			let value = value.as_ref ();
			try! (value.fmt (formatter));
			try! (formatter.write_str (" "));
		}
		try! (formatter.write_str ("]"));
		succeed! (());
	}
}

#[ cfg ( not ( feature = "vonuvoli_fmt_display" ) ) ]
impl <'a, ValueAsRef : StdAsRef<Value> + 'a> fmt::Display for ValueSliceDisplay<'a, ValueAsRef> {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<values:display-not-supported>")
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for ValueSingleton {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			ValueSingleton::Null => formatter.write_str ("#null"),
			ValueSingleton::Void => formatter.write_str ("#void"),
			ValueSingleton::Undefined => formatter.write_str ("#undefined"),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueSingleton::PortEof => formatter.write_str ("#enf-of-file"),
		}
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for ValueSingleton {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			ValueSingleton::Null => formatter.debug_struct ("Null") .finish (),
			ValueSingleton::Void => formatter.debug_struct ("Void") .finish (),
			ValueSingleton::Undefined => formatter.debug_struct ("Undefined") .finish (),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueSingleton::PortEof => formatter.debug_struct ("PortEof") .finish (),
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Boolean {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.value () {
			true =>
				formatter.write_str ("#true"),
			false =>
				formatter.write_str ("#false"),
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for NumberInteger {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let value = self.value ();
		if value == 0 {
			write! (formatter, "0")
		} else {
			write! (formatter, "{:+}", value)
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for NumberReal {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let value = self.value ();
		if value == 0.0 {
			write! (formatter, "0.0")
		} else if value.is_nan () {
			write! (formatter, "nan.0")
		} else if value.is_infinite () {
			if value.is_sign_positive () {
				write! (formatter, "+inf.0")
			} else {
				write! (formatter, "-inf.0")
			}
		} else {
			write! (formatter, "{:+e}", value)
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl fmt::Display for Character {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let character = self.value ();
		match character {
			'!' ... '~' => {
				try! (formatter.write_str ("#\\"));
				try! (formatter.write_char (character));
			},
			_ =>
				try! (write! (formatter, "#\\x{:02x}", character as u32)),
		}
		succeed! (());
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Symbol {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let string = self.string_as_str ();
		if string.is_empty () {
			try! (formatter.write_str ("||"));
		} else {
			try! (formatter.write_char ('|'));
			for character in string.chars () {
				match character {
					'|' | '\\' => {
						try! (formatter.write_char ('\\'));
						try! (formatter.write_char (character));
					},
					' ' ... '~' =>
						try! (formatter.write_char (character)),
					_ =>
						try! (write! (formatter, "\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
		}
		succeed! (());
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
impl fmt::Display for Keyword {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let string = self.string_as_str ();
		if string.is_empty () {
			try! (formatter.write_str ("#:||"));
		} else {
			try! (formatter.write_str ("#:|"));
			for character in string.chars () {
				match character {
					'|' | '\\' => {
						try! (formatter.write_char ('\\'));
						try! (formatter.write_char (character));
					},
					' ' ... '~' =>
						try! (formatter.write_char (character)),
					_ =>
						try! (write! (formatter, "\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
		}
		succeed! (());
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( any ( feature = "vonuvoli_values_unique", feature = "vonuvoli_builtins_parameters" ) ) ]
impl fmt::Display for Unique {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.data_ref ();
		write! (formatter, "#<unique:{:032x}>", self_0.fingerprint.value ())
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl fmt::Display for StringImmutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let string = self.string_ref ();
		return string_fmt (string.string_as_str (), "\"", "\"", formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl fmt::Display for StringMutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let string = try_or_return! (self.string_ref (), Err (fmt::Error::default ()));
		return string_fmt (string.string_as_str (), "\"", "\"", formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_fmt (string : &str, prefix : &str, suffix : &str, formatter : &mut fmt::Formatter) -> (fmt::Result) {
	try! (formatter.write_str (prefix));
	for character in string.chars () {
		match character {
			'"' | '\\' => {
				try! (formatter.write_char ('\\'));
				try! (formatter.write_char (character));
			},
			' ' ... '~' =>
				try! (formatter.write_char (character)),
			_ =>
				try! (write! (formatter, "\\x{:02x};", character as u32)),
		}
	}
	try! (formatter.write_str (suffix));
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl fmt::Display for BytesImmutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let bytes = self.bytes_ref ();
		return bytes_fmt (bytes.bytes_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl fmt::Display for BytesMutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let bytes = try_or_return! (self.bytes_ref (), Err (fmt::Error::default ()));
		return bytes_fmt (bytes.bytes_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_fmt (bytes : &[u8], formatter : &mut fmt::Formatter) -> (fmt::Result) {
	try! (formatter.write_str ("#u8("));
	let mut is_first = true;
	for byte in bytes {
		if !is_first {
			try! (formatter.write_char (' '));
		} else {
			is_first = false;
		}
		try! (write! (formatter, "{}", byte));
	}
	try! (formatter.write_char (')'));
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl fmt::Display for StringRegex {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		write! (formatter, "#<string-regex>")
	}
}


#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl fmt::Display for BytesRegex {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		write! (formatter, "#<bytes-regex>")
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for PairImmutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let pair = self.pair_ref ();
		return pair_fmt (pair, formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl fmt::Display for PairMutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let pair = try_or_return! (self.pair_ref (), Err (fmt::Error::default ()));
		return pair_fmt (pair, formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn pair_fmt (pair : PairRef, formatter : &mut fmt::Formatter) -> (fmt::Result) {
	try! (formatter.write_char ('('));
	let pair = pair.left_and_right ();
	try! (pair_fmt_0 (pair, pair, formatter));
	try! (formatter.write_char (')'));
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
fn pair_fmt_0 (head : (&Value, &Value), cursor : (&Value, &Value), formatter : &mut fmt::Formatter) -> (fmt::Result) {
	let mut cursor = cursor;
	loop {
		let left = cursor.0;
		let right = cursor.1;
		
		FIXME! ("make sure `left` is not recursive");
		try! (fmt::Display::fmt (left, formatter));
		
		match right.list_match_as_ref () {
			
			ListMatchAsRef::Null =>
				succeed! (()),
			
			ListMatchAsRef::PairImmutable (pair) => {
				try! (formatter.write_char (' '));
				cursor = pair.left_and_right ();
			},
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ListMatchAsRef::PairMutable (pair) => {
				try! (formatter.write_char (' '));
				let pair = try_or_return! (pair.pair_ref (), Err (fmt::Error::default ()));
				return pair_fmt_0 (head, pair.left_and_right (), formatter);
			},
			
			ListMatchAsRef::Value (value) => {
				try! (formatter.write_char (' '));
				try! (formatter.write_char ('.'));
				try! (formatter.write_char (' '));
				try! (fmt::Display::fmt (value, formatter));
				succeed! (());
			},
			
		}
		
		/*
		FIXME! ("find a better way to detect recursive lists");
		if ptr::eq (head, cursor) {
			try! (formatter.write_char ('.'));
			try! (formatter.write_char (' '));
			try! (formatter.write_str ("#cyclic"));
			succeed! (());
		}
		*/
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl fmt::Display for ArrayImmutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let array = self.array_ref ();
		return array_fmt (array.values_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl fmt::Display for ArrayMutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let array = try_or_return! (self.array_ref (), Err (fmt::Error::default ()));
		return array_fmt (array.values_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn array_fmt (values : &[Value], formatter : &mut fmt::Formatter) -> (fmt::Result) {
	try! (formatter.write_str ("#("));
	let mut is_first = true;
	for value in values {
		if !is_first {
			try! (formatter.write_char (' '));
		} else {
			is_first = false;
		}
		try! (fmt::Display::fmt (value, formatter));
	}
	try! (formatter.write_char (')'));
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl fmt::Display for Values {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let values = self.values_iter ();
		try! (formatter.write_str ("#values("));
		let mut is_first = true;
		for value in values {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (value.fmt (formatter));
		}
		try! (formatter.write_char (')'));
		succeed! (());
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl fmt::Display for RecordKind {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		if let Some (ref identifier) = self_0.identifier {
			write! (formatter, "#<record-type:{:016x}:{}>", self_0.handle.value (), identifier)
		} else {
			write! (formatter, "#<record-type:{:016x}>", self_0.handle.value ())
		}
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl fmt::Debug for RecordKind {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl fmt::Display for RecordImmutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let record = self.record_ref ();
		return record_fmt (record.kind (), record.values_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl fmt::Display for RecordMutable {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let record = try_or_return! (self.record_ref (), Err (fmt::Error::default ()));
		return record_fmt (record.kind (), record.values_as_slice (), formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn record_fmt (kind : &RecordKind, values : &[Value], formatter : &mut fmt::Formatter) -> (fmt::Result) {
	let kind_0 = kind.internals_ref ();
	if let Some (ref identifier) = kind_0.identifier {
		try! (write! (formatter, "#<record:{:016x}:{}>(", kind_0.handle.value (), identifier));
	} else {
		try! (write! (formatter, "#<record:{:016x}>(", kind_0.handle.value ()));
	}
	let mut is_first = true;
	for value in values {
		if !is_first {
			try! (formatter.write_char (' '));
		} else {
			is_first = false;
		}
		try! (fmt::Display::fmt (value, formatter));
	}
	try! (formatter.write_char (')'));
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Display for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:016x}:{:016x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Debug for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Display for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:016x}:{:016x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Debug for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Display for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:016x}:{:016x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl fmt::Debug for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for ProcedurePrimitive {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		formatter.write_str ("#<procedure-primitive>")
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for SyntaxPrimitive {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		formatter.write_str ("#<syntax-primitive>")
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl fmt::Display for ProcedureExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		formatter.write_str ("#<procedure-extended>")
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl fmt::Debug for ProcedureExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl fmt::Display for SyntaxExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		FIXME! ("implement this");
		formatter.write_str ("#<syntax-extended>")
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl fmt::Debug for SyntaxExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let internals = self.internals_ref ();
		return internals.fmt (formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNativeInternals {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			ProcedureNativeInternals::Native0 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native1 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native2 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native3 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native4 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native5 (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native0E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native1E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native2E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native3E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native4E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::Native5E (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.fmt (formatter),
			ProcedureNativeInternals::NativeV (ref native) =>
				return native.fmt (formatter),
		}
	}
}


#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative0 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-0:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative1 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-1:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative2 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-2:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative3 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-3:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative4 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-4:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative5 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-5:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNativeN {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-n:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}


#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative0E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-0:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative1E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-1:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative2E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-2:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative3E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-3:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative4E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-4:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNative5E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-5:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNativeNE {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-n:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}


#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for ProcedureNativeV {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native-v:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}




#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative0 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative0") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative1 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative1") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative2 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative2") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative3 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative3") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative4 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative4") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative5 {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative5") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNativeN {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNativeN") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}


#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative0E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative0E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative1E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative1E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative2E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative2E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative3E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative3E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative4E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative4E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNative5E {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative5E") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNativeNE {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNativeNE") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}


#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for ProcedureNativeV {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNativeV") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for SyntaxNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let internals = self.internals_ref ();
		return internals.fmt (formatter);
	}
}

#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for SyntaxNativeInternals {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			SyntaxNativeInternals::NativeG (ref native) =>
				return native.fmt (formatter),
		}
	}
}


#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Display for SyntaxNativeG {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<syntax-native-g:{:016x}:({})>", self.handle () .value (), self.symbol () .resolve_name ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl fmt::Debug for SyntaxNativeG {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("SyntaxNativeG") .field (&self.symbol () .resolve_name ()) .finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let (code_1, code_2) = self.code_2 ();
		write! (formatter, "#<error:{:08x}:{:08x}>", code_1, code_2)
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let (code_1, code_2) = self.code_2 ();
		let self_0 = self.internals_ref ();
		formatter.debug_struct ("Error")
				.field ("code", &format! ("{:08x}:{:08x}", code_1, code_2))
				.field ("internals", &self_0)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl fmt::Display for Path {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let path = self.path_ref ();
		let path = path.to_string_lossy ();
		return string_fmt (&path, "#<path:\"", "\">", formatter);
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl fmt::Display for Port {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		write! (formatter, "#<port:{:016x}>", self_0.handle.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl fmt::Debug for Port {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl fmt::Display for Process {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		write! (formatter, "#<process:{:016x}:{:09}>", self_0.handle.value (), self_0.process_id)
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl fmt::Debug for Process {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		self_0.fmt (formatter)
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		write! (formatter, "#<context:{:016x}>", self_0.handle.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		formatter
				.debug_struct ("Context")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("bindings", &self_0.bindings)
				.field ("parent", &self_0.parent)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		write! (formatter, "#<context:{:016x}>", self_0.handle.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		formatter
				.debug_struct ("Registers")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("registers", &self_0.registers)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Register {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<register>")
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Register {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			Register::Binding (ref binding) =>
				formatter
						.debug_tuple ("Binding")
						.field (binding)
						.finish (),
			Register::Value (_, immutable) =>
				formatter
						.debug_tuple ("Value")
						.field (&immutable)
						.finish (),
			Register::Uninitialized (immutable) =>
				formatter
						.debug_tuple ("Uninitialized")
						.field (&immutable)
						.finish (),
			Register::Undefined =>
				formatter
						.debug_tuple ("Undefined")
						.finish (),
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		if let Some (ref identifier) = self_0.identifier {
			write! (formatter, "#<binding:{:016x}:{}>({})", self_0.handle.value (), identifier, self_0.value)
		} else {
			write! (formatter, "#<binding:{:016x}>({})", self_0.handle.value (), self_0.value)
		}
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		formatter
				.debug_struct ("Binding")
				.field ("identifier", &self_0.identifier)
				.field ("initialized", &self_0.initialized)
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl fmt::Display for Parameters {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		write! (formatter, "#<parameters:{:016x}>", self_0.handle.value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl fmt::Debug for Parameters {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		formatter
				.debug_struct ("Parameters")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("bindings", &self_0.bindings)
				.field ("parent", &self_0.parent)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl fmt::Display for Parameter {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		if let Some (ref identifier) = self_0.identifier {
			write! (formatter, "#<parameter:{:016x}:{}>", self_0.handle.value (), identifier)
		} else {
			write! (formatter, "#<parameter:{:016x}>", self_0.handle.value ())
		}
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl fmt::Debug for Parameter {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = try_or_return! (self.internals_ref (), Err (fmt::Error::default ()));
		formatter
				.debug_struct ("Parameter")
				.field ("identifier", &self_0.identifier)
				.field ("global", &self_0.global)
				.field ("conversion", &self_0.conversion)
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl fmt::Display for Opaque {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<opaque:{:016x}>", self.handle () .value ())
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl fmt::Debug for Opaque {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter
				.debug_struct ("Opaque")
				.field ("handle", &self.handle ())
				.finish ()
	}
}




#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
impl fmt::Display for Handle {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let value = self.value ();
		write! (formatter, "#<handle:{:016x}>", value)
	}
}

#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
impl fmt::Debug for Handle {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("Handle") .field (&format! ("{:016x}", self.value ())) .finish ()
	}
}




#[ cfg ( not ( feature = "vonuvoli_fmt_display" ) ) ]
impl fmt::Display for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<value:display-not-supported>")
	}
}

#[ cfg ( not ( feature = "vonuvoli_fmt_display" ) ) ]
impl fmt::Display for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<error:display-not-supported>")
	}
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( not ( feature = "vonuvoli_fmt_display" ) ) ]
impl fmt::Display for Expression {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<expression:display-not-supported>")
	}
}




#[ cfg ( not ( feature = "vonuvoli_fmt_debug" ) ) ]
impl fmt::Debug for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("Value") .field (&format! ("debug-not-supported")) .finish ()
	}
}

#[ cfg ( not ( feature = "vonuvoli_fmt_debug" ) ) ]
impl fmt::Debug for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("Error") .field (&format! ("debug-not-supported")) .finish ()
	}
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( not ( feature = "vonuvoli_fmt_debug" ) ) ]
impl fmt::Debug for Expression {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("Expression") .field (&format! ("debug-not-supported")) .finish ()
	}
}

