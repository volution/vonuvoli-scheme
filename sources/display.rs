

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::lambdas::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




impl fmt::Display for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			
			Value::Singleton (_, ref value, _) => value.fmt (formatter),
			
			Value::Boolean (_, ref value, _) => value.fmt (formatter),
			Value::NumberInteger (_, ref value, _) => value.fmt (formatter),
			Value::NumberReal (_, ref value, _) => value.fmt (formatter),
			Value::Character (_, ref value, _) => value.fmt (formatter),
			
			Value::Symbol (_, ref value, _) => value.fmt (formatter),
			Value::String (_, ref value, _) => value.fmt (formatter),
			Value::Bytes (_, ref value, _) => value.fmt (formatter),
			
			Value::Pair (_, ref value, _) => value.fmt (formatter),
			Value::Array (_, ref value, _) => value.fmt (formatter),
			Value::Values (_, ref value, _) => value.fmt (formatter),
			
			Value::Error (_, ref value, _) => value.fmt (formatter),
			
			Value::ProcedurePrimitive (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureExtended (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureNative (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureLambda (_, ref value, _) => value.fmt (formatter),
			
			Value::SyntaxPrimitive (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxExtended (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxNative (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxLambda (_, ref value, _) => value.fmt (formatter),
			
			Value::Port (_, ref value, _) => value.fmt (formatter),
			
			Value::Context (_, ref value, _) => value.fmt (formatter),
			Value::Binding (_, ref value, _) => value.fmt (formatter),
			
		}
	}
}

impl fmt::Debug for Value {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			
			Value::Singleton (_, ref value, _) => value.fmt (formatter),
			
			Value::Boolean (_, ref value, _) => value.fmt (formatter),
			Value::NumberInteger (_, ref value, _) => value.fmt (formatter),
			Value::NumberReal (_, ref value, _) => value.fmt (formatter),
			Value::Character (_, ref value, _) => value.fmt (formatter),
			
			Value::Symbol (_, ref value, _) => value.fmt (formatter),
			Value::String (_, ref value, _) => value.fmt (formatter),
			Value::Bytes (_, ref value, _) => value.fmt (formatter),
			
			Value::Pair (_, ref value, _) => value.fmt (formatter),
			Value::Array (_, ref value, _) => value.fmt (formatter),
			Value::Values (_, ref value, _) => value.fmt (formatter),
			
			Value::Error (_, ref value, _) => value.fmt (formatter),
			
			Value::ProcedurePrimitive (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureExtended (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureNative (_, ref value, _) => value.fmt (formatter),
			Value::ProcedureLambda (_, ref value, _) => value.fmt (formatter),
			
			Value::SyntaxPrimitive (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxExtended (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxNative (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxLambda (_, ref value, _) => value.fmt (formatter),
			
			Value::Port (_, ref value, _) => value.fmt (formatter),
			
			Value::Context (_, ref value, _) => value.fmt (formatter),
			Value::Binding (_, ref value, _) => value.fmt (formatter),
			
		}
	}
}




impl fmt::Display for ValueSingleton {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			ValueSingleton::Null => formatter.write_str ("#null"),
			ValueSingleton::Void => formatter.write_str ("#void"),
			ValueSingleton::Undefined => formatter.write_str ("#undefined"),
			ValueSingleton::PortEof => formatter.write_str ("#enf-of-file"),
		}
	}
}

impl fmt::Debug for ValueSingleton {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			ValueSingleton::Null => formatter.debug_struct ("Null") .finish (),
			ValueSingleton::Void => formatter.debug_struct ("Void") .finish (),
			ValueSingleton::Undefined => formatter.debug_struct ("Undefined") .finish (),
			ValueSingleton::PortEof => formatter.debug_struct ("PortEof") .finish (),
		}
	}
}




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




impl fmt::Display for NumberInteger {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{:+}", self.value ())
	}
}




impl fmt::Display for NumberReal {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{:+e}", self.value ())
	}
}




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
						try! (write! (formatter, "#\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
		}
		succeed! (());
	}
}




impl fmt::Display for String {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let string = self.string_as_str ();
		try! (formatter.write_char ('"'));
		for character in string.chars () {
			match character {
				'"' | '\\' => {
					try! (formatter.write_char ('\\'));
					try! (formatter.write_char (character));
				},
				' ' ... '~' =>
					try! (formatter.write_char (character)),
				_ =>
					try! (write! (formatter, "#\\x{:02x};", character as u32)),
			}
		}
		try! (formatter.write_char ('"'));
		succeed! (());
	}
}




impl fmt::Display for Bytes {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let bytes = self.values_as_iter ();
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
}




impl fmt::Display for Pair {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		try! (formatter.write_char ('('));
		let mut cursor = self;
		loop {
			let (left, right) = cursor.left_and_right ();
			try! (left.fmt (formatter));
			match *right {
				Value::Singleton (_, ValueSingleton::Null, _) =>
					break,
				Value::Pair (_, ref right, _) => {
					try! (formatter.write_char (' '));
					cursor = right;
				},
				_ => {
					try! (formatter.write_char (' '));
					try! (formatter.write_char ('.'));
					try! (formatter.write_char (' '));
					try! (right.fmt (formatter));
					break;
				},
			}
			if self.is_self (cursor) {
				try! (formatter.write_char ('.'));
				try! (formatter.write_char (' '));
				try! (formatter.write_str ("#cyclic"));
				break;
			}
		}
		try! (formatter.write_char (')'));
		succeed! (());
	}
}




impl fmt::Display for Array {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let values = self.values_as_iter ();
		try! (formatter.write_str ("#("));
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




impl fmt::Display for Values {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let values = self.values_as_iter ();
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




impl fmt::Display for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

impl fmt::Debug for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.internals_ref () .fmt (formatter)
	}
}




impl fmt::Display for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

impl fmt::Debug for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.internals_ref () .fmt (formatter)
	}
}




impl fmt::Display for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ())
	}
}

impl fmt::Debug for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.internals_ref () .fmt (formatter)
	}
}




impl fmt::Display for ProcedurePrimitive {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<procedure-primitive>")
	}
}




impl fmt::Display for SyntaxPrimitive {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<syntax-primitive>")
	}
}




impl fmt::Display for ProcedureExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<procedure-extended>")
	}
}

impl fmt::Debug for ProcedureExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.internals_ref () .fmt (formatter)
	}
}




impl fmt::Display for SyntaxExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<syntax-extended>")
	}
}

impl fmt::Debug for SyntaxExtended {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.internals_ref () .fmt (formatter)
	}
}




impl fmt::Display for ProcedureNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<procedure-native:{:016x}>", self.handle_value ())
	}
}

impl fmt::Debug for ProcedureNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative") .field (&self.handle_value ()) .finish ()
	}
}




impl fmt::Display for SyntaxNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<syntax-native:{:016x}>", self.handle_value ())
	}
}

impl fmt::Debug for SyntaxNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("SyntaxNative") .field (&self.handle_value ()) .finish ()
	}
}




impl fmt::Display for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<error:{:08x}>", self.code)
	}
}

impl fmt::Debug for Error {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<error:{:08x}>", self.code)
	}
}




impl fmt::Display for Port {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<port:{:08x}>", self.handle () .value ())
	}
}

impl fmt::Debug for Port {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<port>")
	}
}




impl fmt::Display for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<context:{:08x}>", self.handle () .value ())
	}
}

impl fmt::Debug for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		formatter
				.debug_struct ("Context")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("bindings", &self_0.bindings)
				.field ("parent", &self_0.parent)
				.finish ()
	}
}




impl fmt::Display for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<context:{:08x}>", self.handle () .value ())
	}
}

impl fmt::Debug for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		formatter
				.debug_struct ("Registers")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("registers", &self_0.registers)
				.finish ()
	}
}




impl fmt::Display for Register {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<register>")
	}
}

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




impl fmt::Display for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		if let Some (ref identifier) = self_0.identifier {
			write! (formatter, "#<binding:{:08x} {} {}>", self_0.handle.value (), identifier, self_0.value)
		} else {
			write! (formatter, "#<binding:{:08x} {}>", self_0.handle.value (), self_0.value)
		}
	}
}

impl fmt::Debug for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		formatter
				.debug_struct ("Binding")
				.field ("identifier", &self_0.identifier)
				.field ("initialized", &self_0.initialized)
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.finish ()
	}
}




impl fmt::Debug for ExpressionForProcedureNativeCall {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("ProcedureNativeCall")
	}
}




impl fmt::Display for Handle {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<handle:{:08x}>", self.value ())
	}
}

impl fmt::Debug for Handle {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "Handle({:08x})", self.value ())
	}
}
