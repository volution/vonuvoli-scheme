

use super::contexts::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

use super::prelude::*;




impl cmp::Eq for Value {}

impl cmp::PartialEq for Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Value) -> (bool) {
		let match_as_ref = Value::kind_match_as_ref_2 (self, other);
		if let Some (equals) = match_as_ref.eq () {
			return equals;
		} else {
			return false;
		}
	}
}

impl cmp::Ord for Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Value) -> (cmp::Ordering) {
		let match_as_ref = Value::kind_match_as_ref_2 (self, other);
		if let Some (ordering) = match_as_ref.cmp () {
			return ordering;
		} else {
			let self_kind = self.kind ();
			let other_kind = other.kind ();
			match ValueKind::cmp (&self_kind, &other_kind) {
				ordering @ cmp::Ordering::Less | ordering @ cmp::Ordering::Greater =>
					return ordering,
				cmp::Ordering::Equal =>
					panic_0! (0x610d52ed),
			}
		}
	}
}

impl cmp::PartialOrd for Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Value) -> (Option<cmp::Ordering>) {
		Some (Value::cmp (self, other))
	}
}




impl <'a> ValueKindMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn eq (&self) -> (Option<bool>) {
		match *self {
			
			ValueKindMatchAsRef2::Null => Some (true),
			ValueKindMatchAsRef2::Void => Some (true),
			ValueKindMatchAsRef2::Undefined => Some (true),
			ValueKindMatchAsRef2::Singleton (_) => Some (true),
			
			ValueKindMatchAsRef2::Boolean (self_0, other_0) => Some (Boolean::eq (self_0, other_0)),
			ValueKindMatchAsRef2::NumberInteger (self_0, other_0) => Some (NumberInteger::eq (self_0, other_0)),
			ValueKindMatchAsRef2::NumberReal (self_0, other_0) => Some (NumberReal::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::Character (self_0, other_0) => Some (Character::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Some (Symbol::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef2::Keyword (self_0, other_0) => Some (Keyword::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef2::Unique (self_0, other_0) => Some (Unique::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => Some (StringImmutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => Some (StringMutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => Some (BytesImmutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => Some (BytesMutable::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef2::StringRegex (self_0, other_0) => Some (StringRegex::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef2::BytesRegex (self_0, other_0) => Some (BytesRegex::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => Some (PairImmutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => Some (PairMutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => Some (ArrayImmutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => Some (ArrayMutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef2::Values (self_0, other_0) => Some (Values::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordKind (self_0, other_0) => Some (RecordKind::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordImmutable (self_0, other_0) => Some (RecordImmutable::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordMutable (self_0, other_0) => Some (RecordMutable::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef2::Error (self_0, other_0) => Some (Error::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => Some (ProcedurePrimitive::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => Some (ProcedureExtended::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => Some (ProcedureNative::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => Some (ProcedureLambda::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => Some (SyntaxPrimitive::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => Some (SyntaxExtended::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => Some (SyntaxNative::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => Some (SyntaxLambda::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef2::Path (self_0, other_0) => Some (Path::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef2::Port (self_0, other_0) => Some (Port::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef2::Process (self_0, other_0) => Some (Process::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Context (self_0, other_0) => Some (Context::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Some (Binding::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameters (self_0, other_0) => Some (Parameters::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameter (self_0, other_0) => Some (Parameter::eq (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef2::Promise (self_0, other_0) => Some (Promise::eq (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef2::Opaque (self_0, other_0) => Some (Opaque::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Missmatched => None,
			
		}
	}
}

impl <'a> ValueKindMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self) -> (Option<cmp::Ordering>) {
		match *self {
			
			ValueKindMatchAsRef2::Null => Some (cmp::Ordering::Equal),
			ValueKindMatchAsRef2::Void => Some (cmp::Ordering::Equal),
			ValueKindMatchAsRef2::Undefined => Some (cmp::Ordering::Equal),
			ValueKindMatchAsRef2::Singleton (_) => Some (cmp::Ordering::Equal),
			
			ValueKindMatchAsRef2::Boolean (self_0, other_0) => Some (Boolean::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::NumberInteger (self_0, other_0) => Some (NumberInteger::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::NumberReal (self_0, other_0) => Some (NumberReal::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::Character (self_0, other_0) => Some (Character::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Some (Symbol::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef2::Keyword (self_0, other_0) => Some (Keyword::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef2::Unique (self_0, other_0) => Some (Unique::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => Some (StringImmutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => Some (StringMutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => Some (BytesImmutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => Some (BytesMutable::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef2::StringRegex (self_0, other_0) => Some (StringRegex::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef2::BytesRegex (self_0, other_0) => Some (BytesRegex::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => Some (PairImmutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => Some (PairMutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => Some (ArrayImmutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => Some (ArrayMutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef2::Values (self_0, other_0) => Some (Values::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordKind (self_0, other_0) => Some (RecordKind::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordImmutable (self_0, other_0) => Some (RecordImmutable::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordMutable (self_0, other_0) => Some (RecordMutable::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef2::Error (self_0, other_0) => Some (Error::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => Some (ProcedurePrimitive::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => Some (ProcedureExtended::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => Some (ProcedureNative::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => Some (ProcedureLambda::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => Some (SyntaxPrimitive::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => Some (SyntaxExtended::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => Some (SyntaxNative::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => Some (SyntaxLambda::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef2::Path (self_0, other_0) => Some (Path::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef2::Port (self_0, other_0) => Some (Port::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef2::Process (self_0, other_0) => Some (Process::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Context (self_0, other_0) => Some (Context::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Some (Binding::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameters (self_0, other_0) => Some (Parameters::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameter (self_0, other_0) => Some (Parameter::cmp (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef2::Promise (self_0, other_0) => Some (Promise::cmp (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef2::Opaque (self_0, other_0) => Some (Opaque::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Missmatched => None,
			
		}
	}
}




impl cmp::Eq for NumberReal {}

impl cmp::PartialEq for NumberReal {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &NumberReal) -> (bool) {
		let self_0 = self.value ();
		let other_0 = other.value ();
		if self_0.is_nan () && other_0.is_nan () {
			true
		} else {
			f64::eq (&self_0, &other_0)
		}
	}
}

impl cmp::Ord for NumberReal {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &NumberReal) -> (cmp::Ordering) {
		let self_0 = self.value ();
		let other_0 = other.value ();
		if let Some (cmp) = f64::partial_cmp (&self_0, &other_0) {
			cmp
		} else if self_0.is_nan () && other_0.is_nan () {
			cmp::Ordering::Equal
		} else {
			u64::cmp (&self_0.to_bits (), &other_0.to_bits ())
		}
	}
}

impl cmp::PartialOrd for NumberReal {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &NumberReal) -> (Option<cmp::Ordering>) {
		Some (NumberReal::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::Eq for StringImmutable {}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::PartialEq for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringImmutable) -> (bool) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::Ord for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringImmutable) -> (cmp::Ordering) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::PartialOrd for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringImmutable) -> (Option<cmp::Ordering>) {
		Some (StringImmutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::Eq for StringMutable {}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::PartialEq for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringMutable) -> (bool) {
		let self_0 = try_or_return! (self.string_ref (), false);
		let other_0 = try_or_return! (other.string_ref (), false);
		StringRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::Ord for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringMutable) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.string_ref (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.string_ref (), cmp::Ordering::Equal);
		StringRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl cmp::PartialOrd for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringMutable) -> (Option<cmp::Ordering>) {
		Some (StringMutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> cmp::Eq for StringRef<'a> {}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> cmp::PartialEq for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringRef) -> (bool) {
		let self_0 = self.string_as_str ();
		let other_0 = other.string_as_str ();
		str::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> cmp::Ord for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringRef) -> (cmp::Ordering) {
		let self_0 = self.string_as_str ();
		let other_0 = other.string_as_str ();
		str::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> cmp::PartialOrd for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringRef) -> (Option<cmp::Ordering>) {
		Some (StringRef::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::Eq for BytesImmutable {}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::PartialEq for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesImmutable) -> (bool) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::Ord for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesImmutable) -> (cmp::Ordering) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::PartialOrd for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesImmutable) -> (Option<cmp::Ordering>) {
		Some (BytesImmutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::Eq for BytesMutable {}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::PartialEq for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesMutable) -> (bool) {
		let self_0 = try_or_return! (self.bytes_ref (), false);
		let other_0 = try_or_return! (other.bytes_ref (), false);
		BytesRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::Ord for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesMutable) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.bytes_ref (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.bytes_ref (), cmp::Ordering::Equal);
		BytesRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl cmp::PartialOrd for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesMutable) -> (Option<cmp::Ordering>) {
		Some (BytesMutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> cmp::Eq for BytesRef<'a> {}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> cmp::PartialEq for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesRef) -> (bool) {
		let self_0 = self.bytes_as_slice ();
		let other_0 = other.bytes_as_slice ();
		<[u8]>::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> cmp::Ord for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesRef) -> (cmp::Ordering) {
		let self_0 = self.bytes_as_slice ();
		let other_0 = other.bytes_as_slice ();
		<[u8]>::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> cmp::PartialOrd for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesRef) -> (Option<cmp::Ordering>) {
		Some (BytesRef::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::Eq for StringRegex {}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::PartialEq for StringRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringRegex) -> (bool) {
		let self_0 = self.regex_ref () .as_str ();
		let other_0 = other.regex_ref () .as_str ();
		str::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::Ord for StringRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringRegex) -> (cmp::Ordering) {
		let self_0 = self.regex_ref () .as_str ();
		let other_0 = other.regex_ref () .as_str ();
		str::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::PartialOrd for StringRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringRegex) -> (Option<cmp::Ordering>) {
		Some (StringRegex::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::Eq for BytesRegex {}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::PartialEq for BytesRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesRegex) -> (bool) {
		let self_0 = self.regex_ref () .as_str ();
		let other_0 = other.regex_ref () .as_str ();
		str::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::Ord for BytesRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesRegex) -> (cmp::Ordering) {
		let self_0 = self.regex_ref () .as_str ();
		let other_0 = other.regex_ref () .as_str ();
		str::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl cmp::PartialOrd for BytesRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesRegex) -> (Option<cmp::Ordering>) {
		Some (BytesRegex::cmp (self, other))
	}
}




impl cmp::Eq for PairImmutable {}

impl cmp::PartialEq for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &PairImmutable) -> (bool) {
		let self_0 = self.pair_ref ();
		let other_0 = other.pair_ref ();
		PairRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &PairImmutable) -> (cmp::Ordering) {
		let self_0 = self.pair_ref ();
		let other_0 = other.pair_ref ();
		PairRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &PairImmutable) -> (Option<cmp::Ordering>) {
		Some (PairImmutable::cmp (self, other))
	}
}


impl cmp::Eq for PairMutable {}

impl cmp::PartialEq for PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &PairMutable) -> (bool) {
		let self_0 = try_or_return! (self.pair_ref (), false);
		let other_0 = try_or_return! (other.pair_ref (), false);
		PairRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &PairMutable) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.pair_ref (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.pair_ref (), cmp::Ordering::Equal);
		PairRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &PairMutable) -> (Option<cmp::Ordering>) {
		Some (PairMutable::cmp (self, other))
	}
}


impl <'a> cmp::Eq for PairRef<'a> {}

impl <'a> cmp::PartialEq for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &PairRef) -> (bool) {
		let (self_left, self_right) = self.left_and_right ();
		let (other_left, other_right) = other.left_and_right ();
		if Value::eq (self_left, other_left) {
			Value::eq (self_right, other_right)
		} else {
			false
		}
	}
}

impl <'a> cmp::Ord for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &PairRef) -> (cmp::Ordering) {
		let (self_left, self_right) = self.left_and_right ();
		let (other_left, other_right) = other.left_and_right ();
		match Value::cmp (self_left, other_left) {
			cmp::Ordering::Equal =>
				Value::cmp (self_right, other_right),
			ordering =>
				ordering,
		}
	}
}

impl <'a> cmp::PartialOrd for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &PairRef) -> (Option<cmp::Ordering>) {
		Some (PairRef::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::Eq for ArrayImmutable {}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::PartialEq for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayImmutable) -> (bool) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::Ord for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayImmutable) -> (cmp::Ordering) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::PartialOrd for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayImmutable) -> (Option<cmp::Ordering>) {
		Some (ArrayImmutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::Eq for ArrayMutable {}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::PartialEq for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayMutable) -> (bool) {
		let self_0 = try_or_return! (self.array_ref (), false);
		let other_0 = try_or_return! (other.array_ref (), false);
		ArrayRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::Ord for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayMutable) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.array_ref (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.array_ref (), cmp::Ordering::Equal);
		ArrayRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl cmp::PartialOrd for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayMutable) -> (Option<cmp::Ordering>) {
		Some (ArrayMutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl <'a> cmp::Eq for ArrayRef<'a> {}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl <'a> cmp::PartialEq for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayRef) -> (bool) {
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		<[Value]>::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl <'a> cmp::Ord for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayRef) -> (cmp::Ordering) {
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		<[Value]>::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl <'a> cmp::PartialOrd for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayRef) -> (Option<cmp::Ordering>) {
		Some (ArrayRef::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Eq for RecordKind {}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialEq for RecordKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &RecordKind) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Ord for RecordKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &RecordKind) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialOrd for RecordKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &RecordKind) -> (Option<cmp::Ordering>) {
		Some (RecordKind::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Eq for RecordImmutable {}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialEq for RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &RecordImmutable) -> (bool) {
		let self_0 = self.record_ref ();
		let other_0 = other.record_ref ();
		RecordRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Ord for RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &RecordImmutable) -> (cmp::Ordering) {
		let self_0 = self.record_ref ();
		let other_0 = other.record_ref ();
		RecordRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialOrd for RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &RecordImmutable) -> (Option<cmp::Ordering>) {
		Some (RecordImmutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Eq for RecordMutable {}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialEq for RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &RecordMutable) -> (bool) {
		let self_0 = try_or_return! (self.record_ref (), false);
		let other_0 = try_or_return! (other.record_ref (), false);
		RecordRef::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::Ord for RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &RecordMutable) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.record_ref (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.record_ref (), cmp::Ordering::Equal);
		RecordRef::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl cmp::PartialOrd for RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &RecordMutable) -> (Option<cmp::Ordering>) {
		Some (RecordMutable::cmp (self, other))
	}
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl <'a> cmp::Eq for RecordRef<'a> {}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl <'a> cmp::PartialEq for RecordRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &RecordRef) -> (bool) {
		let self_kind = self.kind ();
		let other_kind = other.kind ();
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		RecordKind::eq (self_kind, other_kind) && <[Value]>::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl <'a> cmp::Ord for RecordRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &RecordRef) -> (cmp::Ordering) {
		let self_kind = self.kind ();
		let other_kind = other.kind ();
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		match RecordKind::cmp (self_kind, other_kind) {
			cmp::Ordering::Equal =>
				<[Value]>::cmp (self_0, other_0),
			ordering =>
				ordering,
		}
	}
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl <'a> cmp::PartialOrd for RecordRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &RecordRef) -> (Option<cmp::Ordering>) {
		Some (RecordRef::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Eq for LambdaInternals {}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialEq for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &LambdaInternals) -> (bool) {
		Handle::eq (&self.handle_2, &other.handle_2)
	}
}


#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Ord for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &LambdaInternals) -> (cmp::Ordering) {
		Handle::cmp (&self.handle_2, &other.handle_2)
	}
}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialOrd for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &LambdaInternals) -> (Option<cmp::Ordering>) {
		Some (LambdaInternals::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Eq for Lambda {}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialEq for Lambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Lambda) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::eq (self_0, other_0)
	}
}


#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Ord for Lambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Lambda) -> (cmp::Ordering) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialOrd for Lambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Lambda) -> (Option<cmp::Ordering>) {
		Some (Lambda::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Eq for ProcedureLambda {}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialEq for ProcedureLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ProcedureLambda) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::eq (self_0, other_0)
	}
}


#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Ord for ProcedureLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ProcedureLambda) -> (cmp::Ordering) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialOrd for ProcedureLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ProcedureLambda) -> (Option<cmp::Ordering>) {
		Some (ProcedureLambda::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Eq for SyntaxLambda {}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialEq for SyntaxLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &SyntaxLambda) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::eq (self_0, other_0)
	}
}


#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::Ord for SyntaxLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &SyntaxLambda) -> (cmp::Ordering) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		LambdaInternals::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl cmp::PartialOrd for SyntaxLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &SyntaxLambda) -> (Option<cmp::Ordering>) {
		Some (SyntaxLambda::cmp (self, other))
	}
}




impl cmp::Eq for Error {}

impl cmp::PartialEq for Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Error) -> (bool) {
		u64::eq (&self.code (), &other.code ())
	}
}

impl cmp::Ord for Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Error) -> (cmp::Ordering) {
		u64::cmp (&self.code (), &other.code ())
	}
}

impl cmp::PartialOrd for Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Error) -> (Option<cmp::Ordering>) {
		Some (Error::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::Eq for ProcedureNative {}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::PartialEq for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ProcedureNative) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::Ord for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ProcedureNative) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::PartialOrd for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ProcedureNative) -> (Option<cmp::Ordering>) {
		Some (ProcedureNative::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::Eq for SyntaxNative {}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::PartialEq for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &SyntaxNative) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::Ord for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &SyntaxNative) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl cmp::PartialOrd for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &SyntaxNative) -> (Option<cmp::Ordering>) {
		Some (SyntaxNative::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl cmp::Eq for Path {}

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl cmp::PartialEq for Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Path) -> (bool) {
		let self_0 = self.path_ref ();
		let other_0 = other.path_ref ();
		fs_path::Path::eq (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl cmp::Ord for Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Path) -> (cmp::Ordering) {
		let self_0 = self.path_ref ();
		let other_0 = other.path_ref ();
		fs_path::Path::cmp (self_0, other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl cmp::PartialOrd for Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Path) -> (Option<cmp::Ordering>) {
		Some (Path::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl cmp::Eq for Port {}

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl cmp::PartialEq for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Port) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl cmp::Ord for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Port) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl cmp::PartialOrd for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Port) -> (Option<cmp::Ordering>) {
		Some (Port::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl cmp::Eq for Process {}

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl cmp::PartialEq for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Process) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl cmp::Ord for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Process) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl cmp::PartialOrd for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Process) -> (Option<cmp::Ordering>) {
		Some (Process::cmp (self, other))
	}
}




impl cmp::Eq for Context {}

impl cmp::PartialEq for Context {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for Context {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for Context {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Context::cmp (self, other))
	}
}




impl cmp::Eq for Registers {}

impl cmp::PartialEq for Registers {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for Registers {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for Registers {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Registers::cmp (self, other))
	}
}




impl cmp::Eq for Binding {}

impl cmp::PartialEq for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Binding::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::Eq for Parameters {}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::PartialEq for Parameters {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::Ord for Parameters {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::PartialOrd for Parameters {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Parameters::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::Eq for Parameter {}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::PartialEq for Parameter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = try_or_return! (self.handle (), false);
		let other_0 = try_or_return! (other.handle (), false);
		Handle::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::Ord for Parameter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = try_or_return! (self.handle (), cmp::Ordering::Equal);
		let other_0 = try_or_return! (other.handle (), cmp::Ordering::Equal);
		Handle::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl cmp::PartialOrd for Parameter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Parameter::cmp (self, other))
	}
}




#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl cmp::Eq for Opaque {}

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl cmp::PartialEq for Opaque {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.handle ();
		let other_0 = other.handle ();
		Handle::eq (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl cmp::Ord for Opaque {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = self.handle ();
		let other_0 = other.handle ();
		Handle::cmp (&self_0, &other_0)
	}
}

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl cmp::PartialOrd for Opaque {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Opaque::cmp (self, other))
	}
}

