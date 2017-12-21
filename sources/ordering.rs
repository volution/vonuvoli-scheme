

use super::contexts::exports::*;
use super::lambdas::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




impl cmp::Eq for Value {}

impl cmp::PartialEq for Value {
	
	#[ inline (always) ]
	fn eq (&self, other : &Value) -> (bool) {
		match (self, other) {
			
			(&Value::Singleton (_, ref self_0, _), &Value::Singleton (_, ref other_0, _)) => ValueSingleton::eq (self_0, other_0),
			
			(&Value::Boolean (_, ref self_0, _), &Value::Boolean (_, ref other_0, _)) => Boolean::eq (self_0, other_0),
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => NumberInteger::eq (self_0, other_0),
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => NumberReal::eq (self_0, other_0),
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => Character::eq (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => Symbol::eq (self_0, other_0),
			(&Value::String (_, ref self_0, _), &Value::String (_, ref other_0, _)) => String::eq (self_0, other_0),
			(&Value::Bytes (_, ref self_0, _), &Value::Bytes (_, ref other_0, _)) => Bytes::eq (self_0, other_0),
			
			(&Value::Pair (_, ref self_0, _), &Value::Pair (_, ref other_0, _)) => Pair::eq (self_0, other_0),
			(&Value::Array (_, ref self_0, _), &Value::Array (_, ref other_0, _)) => Array::eq (self_0, other_0),
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => Values::eq (self_0, other_0),
			
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => Error::eq (self_0, other_0),
			
			(&Value::ProcedurePrimitive (_, ref self_0, _), &Value::ProcedurePrimitive (_, ref other_0, _)) => ProcedurePrimitive::eq (self_0, other_0),
			(&Value::ProcedureExtended (_, ref self_0, _), &Value::ProcedureExtended (_, ref other_0, _)) => ProcedureExtended::eq (self_0, other_0),
			(&Value::ProcedureNative (_, ref self_0, _), &Value::ProcedureNative (_, ref other_0, _)) => ProcedureNative::eq (self_0, other_0),
			(&Value::ProcedureLambda (_, ref self_0, _), &Value::ProcedureLambda (_, ref other_0, _)) => ProcedureLambda::eq (self_0, other_0),
			
			(&Value::SyntaxPrimitive (_, ref self_0, _), &Value::SyntaxPrimitive (_, ref other_0, _)) => SyntaxPrimitive::eq (self_0, other_0),
			(&Value::SyntaxExtended (_, ref self_0, _), &Value::SyntaxExtended (_, ref other_0, _)) => SyntaxExtended::eq (self_0, other_0),
			(&Value::SyntaxNative (_, ref self_0, _), &Value::SyntaxNative (_, ref other_0, _)) => SyntaxNative::eq (self_0, other_0),
			(&Value::SyntaxLambda (_, ref self_0, _), &Value::SyntaxLambda (_, ref other_0, _)) => SyntaxLambda::eq (self_0, other_0),
			
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => Port::eq (self_0, other_0),
			
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => Context::eq (self_0, other_0),
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => Binding::eq (self_0, other_0),
			
			(_, _) => false,
			
		}
	}
}

impl cmp::Ord for Value {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Value) -> (cmp::Ordering) {
		let self_class = self.class ();
		let other_class = other.class ();
		match ValueClass::cmp (&self_class, &other_class) {
			ordering @ cmp::Ordering::Less | ordering @ cmp::Ordering::Greater =>
				return ordering,
			cmp::Ordering::Equal =>
				(),
		}
		match (self, other) {
			
			(&Value::Singleton (_, ref self_0, _), &Value::Singleton (_, ref other_0, _)) => ValueSingleton::cmp (self_0, other_0),
			
			(&Value::Boolean (_, ref self_0, _), &Value::Boolean (_, ref other_0, _)) => Boolean::cmp (self_0, other_0),
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => NumberInteger::cmp (self_0, other_0),
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => NumberReal::cmp (self_0, other_0),
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => Character::cmp (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => Symbol::cmp (self_0, other_0),
			(&Value::String (_, ref self_0, _), &Value::String (_, ref other_0, _)) => String::cmp (self_0, other_0),
			(&Value::Bytes (_, ref self_0, _), &Value::Bytes (_, ref other_0, _)) => Bytes::cmp (self_0, other_0),
			
			(&Value::Pair (_, ref self_0, _), &Value::Pair (_, ref other_0, _)) => Pair::cmp (self_0, other_0),
			(&Value::Array (_, ref self_0, _), &Value::Array (_, ref other_0, _)) => Array::cmp (self_0, other_0),
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => Values::cmp (self_0, other_0),
			
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => Error::cmp (self_0, other_0),
			
			(&Value::ProcedurePrimitive (_, ref self_0, _), &Value::ProcedurePrimitive (_, ref other_0, _)) => ProcedurePrimitive::cmp (self_0, other_0),
			(&Value::ProcedureExtended (_, ref self_0, _), &Value::ProcedureExtended (_, ref other_0, _)) => ProcedureExtended::cmp (self_0, other_0),
			(&Value::ProcedureNative (_, ref self_0, _), &Value::ProcedureNative (_, ref other_0, _)) => ProcedureNative::cmp (self_0, other_0),
			(&Value::ProcedureLambda (_, ref self_0, _), &Value::ProcedureLambda (_, ref other_0, _)) => ProcedureLambda::cmp (self_0, other_0),
			
			(&Value::SyntaxPrimitive (_, ref self_0, _), &Value::SyntaxPrimitive (_, ref other_0, _)) => SyntaxPrimitive::cmp (self_0, other_0),
			(&Value::SyntaxExtended (_, ref self_0, _), &Value::SyntaxExtended (_, ref other_0, _)) => SyntaxExtended::cmp (self_0, other_0),
			(&Value::SyntaxNative (_, ref self_0, _), &Value::SyntaxNative (_, ref other_0, _)) => SyntaxNative::cmp (self_0, other_0),
			(&Value::SyntaxLambda (_, ref self_0, _), &Value::SyntaxLambda (_, ref other_0, _)) => SyntaxLambda::cmp (self_0, other_0),
			
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => Port::cmp (self_0, other_0),
			
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => Context::cmp (self_0, other_0),
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => Binding::cmp (self_0, other_0),
			
			(_, _) => panic! ("def6bd79"),
			
		}
	}
}

impl cmp::PartialOrd for Value {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Value) -> (Option<cmp::Ordering>) {
		Some (Value::cmp (self, other))
	}
}




impl cmp::Eq for NumberReal {}

impl cmp::PartialEq for NumberReal {
	
	#[ inline (always) ]
	fn eq (&self, other : &NumberReal) -> (bool) {
		f64::eq (&self.value (), &other.value ())
	}
}

impl cmp::Ord for NumberReal {
	
	#[ inline (always) ]
	fn cmp (&self, other : &NumberReal) -> (cmp::Ordering) {
		if let Some (cmp) = f64::partial_cmp (&self.value (), &other.value ()) {
			cmp
		} else {
			u64::cmp (&self.value () .to_bits (), &other.value () .to_bits ())
		}
	}
}

impl cmp::PartialOrd for NumberReal {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &NumberReal) -> (Option<cmp::Ordering>) {
		f64::partial_cmp (&self.value (), &other.value ())
	}
}




impl cmp::Eq for LambdaInternals {}

impl cmp::PartialEq for LambdaInternals {
	
	#[ inline (always) ]
	fn eq (&self, other : &LambdaInternals) -> (bool) {
		Handle::eq (&self.handle_2, &other.handle_2)
	}
}


impl cmp::Ord for LambdaInternals {
	
	#[ inline (always) ]
	fn cmp (&self, other : &LambdaInternals) -> (cmp::Ordering) {
		Handle::cmp (&self.handle_2, &other.handle_2)
	}
}

impl cmp::PartialOrd for LambdaInternals {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &LambdaInternals) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle_2, &other.handle_2)
	}
}




impl cmp::Eq for ProcedureNative {}

impl cmp::PartialEq for ProcedureNative {
	
	#[ inline (always) ]
	fn eq (&self, other : &ProcedureNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::Ord for ProcedureNative {
	
	#[ inline (always) ]
	fn cmp (&self, other : &ProcedureNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for ProcedureNative {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &ProcedureNative) -> (Option<cmp::Ordering>) {
		u64::partial_cmp (&self.handle_value (), &other.handle_value ())
	}
}




impl cmp::Eq for SyntaxNative {}

impl cmp::PartialEq for SyntaxNative {
	
	#[ inline (always) ]
	fn eq (&self, other : &SyntaxNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::Ord for SyntaxNative {
	
	#[ inline (always) ]
	fn cmp (&self, other : &SyntaxNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for SyntaxNative {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &SyntaxNative) -> (Option<cmp::Ordering>) {
		u64::partial_cmp (&self.handle_value (), &other.handle_value ())
	}
}




impl cmp::Eq for Port {}

impl cmp::PartialEq for Port {
	
	#[ inline (always) ]
	fn eq (&self, other : &Port) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Port {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Port) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Port {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Port) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle (), &other.handle ())
	}
}




impl cmp::Eq for Context {}

impl cmp::PartialEq for Context {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Context {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Context {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle (), &other.handle ())
	}
}




impl cmp::Eq for Registers {}

impl cmp::PartialEq for Registers {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Registers {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Registers {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle (), &other.handle ())
	}
}




impl cmp::Eq for Binding {}

impl cmp::PartialEq for Binding {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		Handle::eq (&self.handle (), &other.handle ())
	}
}

impl cmp::Ord for Binding {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		Handle::cmp (&self.handle (), &other.handle ())
	}
}

impl cmp::PartialOrd for Binding {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle (), &other.handle ())
	}
}

