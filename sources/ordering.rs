

use super::contexts::exports::*;
use super::lambdas::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

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
					panic! ("610d52ed"),
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
			ValueKindMatchAsRef2::Character (self_0, other_0) => Some (Character::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Some (Symbol::eq (self_0, other_0)),
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => Some (StringImmutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => Some (StringMutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => Some (BytesImmutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => Some (BytesMutable::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => Some (PairImmutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => Some (PairMutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => Some (ArrayImmutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => Some (ArrayMutable::eq (self_0, other_0)),
			ValueKindMatchAsRef2::Values (self_0, other_0) => Some (Values::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Error (self_0, other_0) => Some (Error::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => Some (ProcedurePrimitive::eq (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => Some (ProcedureExtended::eq (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => Some (ProcedureNative::eq (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => Some (ProcedureLambda::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => Some (SyntaxPrimitive::eq (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => Some (SyntaxExtended::eq (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => Some (SyntaxNative::eq (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => Some (SyntaxLambda::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Port (self_0, other_0) => Some (Port::eq (self_0, other_0)),
			ValueKindMatchAsRef2::Process (self_0, other_0) => Some (Process::eq (self_0, other_0)),
			
			ValueKindMatchAsRef2::Context (self_0, other_0) => Some (Context::eq (self_0, other_0)),
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Some (Binding::eq (self_0, other_0)),
			
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
			ValueKindMatchAsRef2::Character (self_0, other_0) => Some (Character::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Some (Symbol::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => Some (StringImmutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => Some (StringMutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => Some (BytesImmutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => Some (BytesMutable::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => Some (PairImmutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => Some (PairMutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => Some (ArrayImmutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => Some (ArrayMutable::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::Values (self_0, other_0) => Some (Values::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Error (self_0, other_0) => Some (Error::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => Some (ProcedurePrimitive::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => Some (ProcedureExtended::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => Some (ProcedureNative::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => Some (ProcedureLambda::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => Some (SyntaxPrimitive::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => Some (SyntaxExtended::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => Some (SyntaxNative::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => Some (SyntaxLambda::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Port (self_0, other_0) => Some (Port::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::Process (self_0, other_0) => Some (Process::cmp (self_0, other_0)),
			
			ValueKindMatchAsRef2::Context (self_0, other_0) => Some (Context::cmp (self_0, other_0)),
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Some (Binding::cmp (self_0, other_0)),
			
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




impl cmp::Eq for StringImmutable {}

impl cmp::PartialEq for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringImmutable) -> (bool) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringImmutable) -> (cmp::Ordering) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringImmutable) -> (Option<cmp::Ordering>) {
		Some (StringImmutable::cmp (self, other))
	}
}


impl cmp::Eq for StringMutable {}

impl cmp::PartialEq for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringMutable) -> (bool) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringMutable) -> (cmp::Ordering) {
		let self_0 = self.string_ref ();
		let other_0 = other.string_ref ();
		StringRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringMutable) -> (Option<cmp::Ordering>) {
		Some (StringMutable::cmp (self, other))
	}
}


impl <'a> cmp::Eq for StringRef<'a> {}

impl <'a> cmp::PartialEq for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &StringRef) -> (bool) {
		let self_0 = self.string_as_str ();
		let other_0 = other.string_as_str ();
		str::eq (self_0, other_0)
	}
}

impl <'a> cmp::Ord for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &StringRef) -> (cmp::Ordering) {
		let self_0 = self.string_as_str ();
		let other_0 = other.string_as_str ();
		str::cmp (self_0, other_0)
	}
}

impl <'a> cmp::PartialOrd for StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &StringRef) -> (Option<cmp::Ordering>) {
		Some (StringRef::cmp (self, other))
	}
}




impl cmp::Eq for BytesImmutable {}

impl cmp::PartialEq for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesImmutable) -> (bool) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesImmutable) -> (cmp::Ordering) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesImmutable) -> (Option<cmp::Ordering>) {
		Some (BytesImmutable::cmp (self, other))
	}
}


impl cmp::Eq for BytesMutable {}

impl cmp::PartialEq for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesMutable) -> (bool) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesMutable) -> (cmp::Ordering) {
		let self_0 = self.bytes_ref ();
		let other_0 = other.bytes_ref ();
		BytesRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesMutable) -> (Option<cmp::Ordering>) {
		Some (BytesMutable::cmp (self, other))
	}
}


impl <'a> cmp::Eq for BytesRef<'a> {}

impl <'a> cmp::PartialEq for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &BytesRef) -> (bool) {
		let self_0 = self.bytes_as_slice ();
		let other_0 = other.bytes_as_slice ();
		<[u8]>::eq (self_0, other_0)
	}
}

impl <'a> cmp::Ord for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &BytesRef) -> (cmp::Ordering) {
		let self_0 = self.bytes_as_slice ();
		let other_0 = other.bytes_as_slice ();
		<[u8]>::cmp (self_0, other_0)
	}
}

impl <'a> cmp::PartialOrd for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &BytesRef) -> (Option<cmp::Ordering>) {
		Some (BytesRef::cmp (self, other))
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
		let self_0 = self.pair_ref ();
		let other_0 = other.pair_ref ();
		PairRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &PairMutable) -> (cmp::Ordering) {
		let self_0 = self.pair_ref ();
		let other_0 = other.pair_ref ();
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




impl cmp::Eq for ArrayImmutable {}

impl cmp::PartialEq for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayImmutable) -> (bool) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayImmutable) -> (cmp::Ordering) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayImmutable) -> (Option<cmp::Ordering>) {
		Some (ArrayImmutable::cmp (self, other))
	}
}


impl cmp::Eq for ArrayMutable {}

impl cmp::PartialEq for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayMutable) -> (bool) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::eq (&self_0, &other_0)
	}
}

impl cmp::Ord for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayMutable) -> (cmp::Ordering) {
		let self_0 = self.array_ref ();
		let other_0 = other.array_ref ();
		ArrayRef::cmp (&self_0, &other_0)
	}
}

impl cmp::PartialOrd for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayMutable) -> (Option<cmp::Ordering>) {
		Some (ArrayMutable::cmp (self, other))
	}
}


impl <'a> cmp::Eq for ArrayRef<'a> {}

impl <'a> cmp::PartialEq for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ArrayRef) -> (bool) {
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		<[Value]>::eq (self_0, other_0)
	}
}

impl <'a> cmp::Ord for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ArrayRef) -> (cmp::Ordering) {
		let self_0 = self.values_as_slice ();
		let other_0 = other.values_as_slice ();
		<[Value]>::cmp (self_0, other_0)
	}
}

impl <'a> cmp::PartialOrd for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ArrayRef) -> (Option<cmp::Ordering>) {
		Some (ArrayRef::cmp (self, other))
	}
}




impl cmp::Eq for LambdaInternals {}

impl cmp::PartialEq for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &LambdaInternals) -> (bool) {
		Handle::eq (&self.handle_2, &other.handle_2)
	}
}


impl cmp::Ord for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &LambdaInternals) -> (cmp::Ordering) {
		Handle::cmp (&self.handle_2, &other.handle_2)
	}
}

impl cmp::PartialOrd for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &LambdaInternals) -> (Option<cmp::Ordering>) {
		Some (LambdaInternals::cmp (self, other))
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




impl cmp::Eq for ProcedureNative {}

impl cmp::PartialEq for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &ProcedureNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::Ord for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &ProcedureNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &ProcedureNative) -> (Option<cmp::Ordering>) {
		Some (ProcedureNative::cmp (self, other))
	}
}




impl cmp::Eq for SyntaxNative {}

impl cmp::PartialEq for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &SyntaxNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::Ord for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &SyntaxNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &SyntaxNative) -> (Option<cmp::Ordering>) {
		Some (SyntaxNative::cmp (self, other))
	}
}




impl cmp::Eq for Port {}

impl cmp::PartialEq for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Port) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Port) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Port) -> (Option<cmp::Ordering>) {
		Some (Port::cmp (self, other))
	}
}




impl cmp::Eq for Process {}

impl cmp::PartialEq for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn eq (&self, other : &Process) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Process) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

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
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Context {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
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
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Registers {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
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
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Some (Binding::cmp (self, other))
	}
}

