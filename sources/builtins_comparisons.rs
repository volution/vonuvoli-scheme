

use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::extended_syntaxes::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::native_syntaxes::exports::*;
use super::ports::exports::*;
use super::primitives::exports::*;
use super::processes::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{Equivalence, Ordering, Comparison};
	
	pub use super::{
			equivalent_by_identity_2,
			equivalent_by_value_strict_2,
			equivalent_by_value_strict_recursive_2,
			equivalent_by_value_coerced_2,
			equivalent_by_value_coerced_recursive_2,
	};
	
	pub use super::{
			
			compare_1,
			value_singleton_compare_1, value_singleton_compare_1a,
			boolean_compare_1, boolean_compare_1a,
			number_compare_1,
			number_integer_compare_1, number_integer_compare_1a,
			number_real_compare_1, number_real_compare_1a,
			character_compare_1, character_compare_1a,
			symbol_compare_1, symbol_compare_1a,
			string_compare_1,
			string_immutable_compare_1, string_immutable_compare_1a,
			string_mutable_compare_1, string_mutable_compare_1a,
			bytes_compare_1,
			bytes_immutable_compare_1, bytes_immutable_compare_1a,
			bytes_mutable_compare_1, bytes_mutable_compare_1a,
			pair_compare_1,
			pair_immutable_compare_1, pair_immutable_compare_1a,
			pair_mutable_compare_1, pair_mutable_compare_1a,
			array_compare_1,
			array_immutable_compare_1, array_immutable_compare_1a,
			array_mutable_compare_1, array_mutable_compare_1a,
			values_compare_1, values_compare_1a,
			error_compare_1, error_compare_1a,
			procedure_primitive_compare_1, procedure_primitive_compare_1a,
			procedure_extended_compare_1, procedure_extended_compare_1a,
			procedure_native_compare_1, procedure_native_compare_1a,
			procedure_lambda_compare_1, procedure_lambda_compare_1a,
			syntax_primitive_compare_1, syntax_primitive_compare_1a,
			syntax_extended_compare_1, syntax_extended_compare_1a,
			syntax_native_compare_1, syntax_native_compare_1a,
			syntax_lambda_compare_1, syntax_lambda_compare_1a,
			port_compare_1, port_compare_1a,
			process_compare_1, process_compare_1a,
			context_compare_1, context_compare_1a,
			binding_compare_1, binding_compare_1a,
			
			compare_2,
			value_singleton_compare_2, value_singleton_compare_2a,
			boolean_compare_2, boolean_compare_2a,
			number_compare_2,
			number_integer_compare_2, number_integer_compare_2a,
			number_real_compare_2, number_real_compare_2a,
			character_compare_2, character_compare_2a,
			symbol_compare_2, symbol_compare_2a,
			string_compare_2,
			string_immutable_compare_2, string_immutable_compare_2a,
			string_mutable_compare_2, string_mutable_compare_2a,
			bytes_compare_2,
			bytes_immutable_compare_2, bytes_immutable_compare_2a,
			bytes_mutable_compare_2, bytes_mutable_compare_2a,
			pair_compare_2,
			pair_immutable_compare_2, pair_immutable_compare_2a,
			pair_mutable_compare_2, pair_mutable_compare_2a,
			array_compare_2,
			array_immutable_compare_2, array_immutable_compare_2a,
			array_mutable_compare_2, array_mutable_compare_2a,
			values_compare_2, values_compare_2a,
			error_compare_2, error_compare_2a,
			procedure_primitive_compare_2, procedure_primitive_compare_2a,
			procedure_extended_compare_2, procedure_extended_compare_2a,
			procedure_native_compare_2, procedure_native_compare_2a,
			procedure_lambda_compare_2, procedure_lambda_compare_2a,
			syntax_primitive_compare_2, syntax_primitive_compare_2a,
			syntax_extended_compare_2, syntax_extended_compare_2a,
			syntax_native_compare_2, syntax_native_compare_2a,
			syntax_lambda_compare_2, syntax_lambda_compare_2a,
			port_compare_2, port_compare_2a,
			process_compare_2, process_compare_2a,
			context_compare_2, context_compare_2a,
			binding_compare_2, binding_compare_2a,
			
			compare_3,
			value_singleton_compare_3, value_singleton_compare_3a,
			boolean_compare_3, boolean_compare_3a,
			number_compare_3,
			number_integer_compare_3, number_integer_compare_3a,
			number_real_compare_3, number_real_compare_3a,
			character_compare_3, character_compare_3a,
			symbol_compare_3, symbol_compare_3a,
			string_compare_3,
			string_immutable_compare_3, string_immutable_compare_3a,
			string_mutable_compare_3, string_mutable_compare_3a,
			bytes_compare_3,
			bytes_immutable_compare_3, bytes_immutable_compare_3a,
			bytes_mutable_compare_3, bytes_mutable_compare_3a,
			pair_compare_3,
			pair_immutable_compare_3, pair_immutable_compare_3a,
			pair_mutable_compare_3, pair_mutable_compare_3a,
			array_compare_3,
			array_immutable_compare_3, array_immutable_compare_3a,
			array_mutable_compare_3, array_mutable_compare_3a,
			values_compare_3, values_compare_3a,
			error_compare_3, error_compare_3a,
			procedure_primitive_compare_3, procedure_primitive_compare_3a,
			procedure_extended_compare_3, procedure_extended_compare_3a,
			procedure_native_compare_3, procedure_native_compare_3a,
			procedure_lambda_compare_3, procedure_lambda_compare_3a,
			syntax_primitive_compare_3, syntax_primitive_compare_3a,
			syntax_extended_compare_3, syntax_extended_compare_3a,
			syntax_native_compare_3, syntax_native_compare_3a,
			syntax_lambda_compare_3, syntax_lambda_compare_3a,
			port_compare_3, port_compare_3a,
			process_compare_3, process_compare_3a,
			context_compare_3, context_compare_3a,
			binding_compare_3, binding_compare_3a,
			
			compare_4,
			value_singleton_compare_4, value_singleton_compare_4a,
			boolean_compare_4, boolean_compare_4a,
			number_compare_4,
			number_integer_compare_4, number_integer_compare_4a,
			number_real_compare_4, number_real_compare_4a,
			character_compare_4, character_compare_4a,
			symbol_compare_4, symbol_compare_4a,
			string_compare_4,
			string_immutable_compare_4, string_immutable_compare_4a,
			string_mutable_compare_4, string_mutable_compare_4a,
			bytes_compare_4,
			bytes_immutable_compare_4, bytes_immutable_compare_4a,
			bytes_mutable_compare_4, bytes_mutable_compare_4a,
			pair_compare_4,
			pair_immutable_compare_4, pair_immutable_compare_4a,
			pair_mutable_compare_4, pair_mutable_compare_4a,
			array_compare_4,
			array_immutable_compare_4, array_immutable_compare_4a,
			array_mutable_compare_4, array_mutable_compare_4a,
			values_compare_4, values_compare_4a,
			error_compare_4, error_compare_4a,
			procedure_primitive_compare_4, procedure_primitive_compare_4a,
			procedure_extended_compare_4, procedure_extended_compare_4a,
			procedure_native_compare_4, procedure_native_compare_4a,
			procedure_lambda_compare_4, procedure_lambda_compare_4a,
			syntax_primitive_compare_4, syntax_primitive_compare_4a,
			syntax_extended_compare_4, syntax_extended_compare_4a,
			syntax_native_compare_4, syntax_native_compare_4a,
			syntax_lambda_compare_4, syntax_lambda_compare_4a,
			port_compare_4, port_compare_4a,
			process_compare_4, process_compare_4a,
			context_compare_4, context_compare_4a,
			binding_compare_4, binding_compare_4a,
			
			compare_n,
			value_singleton_compare_n, value_singleton_compare_na,
			boolean_compare_n, boolean_compare_na,
			number_compare_n,
			number_integer_compare_n, number_integer_compare_na,
			number_real_compare_n, number_real_compare_na,
			character_compare_n, character_compare_na,
			symbol_compare_n, symbol_compare_na,
			string_compare_n,
			string_immutable_compare_n, string_immutable_compare_na,
			string_mutable_compare_n, string_mutable_compare_na,
			bytes_compare_n,
			bytes_immutable_compare_n, bytes_immutable_compare_na,
			bytes_mutable_compare_n, bytes_mutable_compare_na,
			pair_compare_n,
			pair_immutable_compare_n, pair_immutable_compare_na,
			pair_mutable_compare_n, pair_mutable_compare_na,
			array_compare_n,
			array_immutable_compare_n, array_immutable_compare_na,
			array_mutable_compare_n, array_mutable_compare_na,
			values_compare_n, values_compare_na,
			error_compare_n, error_compare_na,
			procedure_primitive_compare_n, procedure_primitive_compare_na,
			procedure_extended_compare_n, procedure_extended_compare_na,
			procedure_native_compare_n, procedure_native_compare_na,
			procedure_lambda_compare_n, procedure_lambda_compare_na,
			syntax_primitive_compare_n, syntax_primitive_compare_na,
			syntax_extended_compare_n, syntax_extended_compare_na,
			syntax_native_compare_n, syntax_native_compare_na,
			syntax_lambda_compare_n, syntax_lambda_compare_na,
			port_compare_n, port_compare_na,
			process_compare_n, process_compare_na,
			context_compare_n, context_compare_na,
			binding_compare_n, binding_compare_na,
			
	};
	
	pub use super::{
			
			vec_compare_2,
			
	};
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Equivalence {
	
	ByIdentity,
	ByValue,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Ordering {
	
	Lesser,
	LesserOrEqual,
	Equal,
	GreaterOrEqual,
	Greater,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Comparison {
	Equivalence ( Equivalence, Option<bool>, Option<bool> ),
	Ordering ( Ordering, Option<bool>, Option<bool> ),
}


impl Comparison {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_aggregated (&self, last : bool) -> (Comparison) {
		match *self {
			
			Comparison::Equivalence (equivalence, coercion, recursive) =>
				match equivalence {
					Equivalence::ByIdentity =>
						Comparison::Equivalence (Equivalence::ByIdentity, coercion, Some (false)),
					Equivalence::ByValue =>
						match recursive {
							None | Some (false) =>
								Comparison::Equivalence (Equivalence::ByIdentity, coercion, Some (false)),
							Some (true) =>
								Comparison::Equivalence (Equivalence::ByValue, coercion, Some (true)),
						},
				},
			
			Comparison::Ordering (ordering, case_sensitivity, recursive) => {
				let ordering = if ! last {
					Ordering::Equal
				} else {
					ordering
				};
				match recursive {
					None | Some (false) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (false)),
					Some (true) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (true)),
				}
			},
			
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_identity_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByIdentity, None, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_strict_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_strict_recursive_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_coerced_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_coerced_recursive_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)));
}




macro_rules! def_fn_compare {
	(
			$type : ident,
			$compare_1 : ident, $compare_2 : ident, $compare_3 : ident, $compare_4 : ident, $compare_n : ident
	) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_3 <ValueRef : StdAsRef<$type>> (input_1 : ValueRef, input_2 : ValueRef, input_3 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = input_1.as_ref ();
			let input_2 = input_2.as_ref ();
			let input_3 = input_3.as_ref ();
			if ! try! ($compare_2 (input_1, input_2, comparison)) {
				succeed! (false);
			}
			return $compare_2 (input_2, input_3, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_4 <ValueRef : StdAsRef<$type>> (input_1 : ValueRef, input_2 : ValueRef, input_3 : ValueRef, input_4 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = input_1.as_ref ();
			let input_2 = input_2.as_ref ();
			let input_3 = input_3.as_ref ();
			let input_4 = input_4.as_ref ();
			if ! try! ($compare_2 (input_1, input_2, comparison)) {
				succeed! (false);
			}
			if ! try! ($compare_2 (input_2, input_3, comparison)) {
				succeed! (false);
			}
			return $compare_2 (input_3, input_4, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_n <ValueRef : StdAsRef<$type>> (inputs : &[ValueRef], comparison : Comparison) -> (Outcome<bool>) {
			let inputs_count = inputs.len ();
			match inputs_count {
				0 =>
					succeed! (true),
				1 =>
					return $compare_1 (inputs[0].as_ref (), comparison),
				_ =>
					(),
			}
			let mut inputs_iterator = inputs.iter ();
			let mut input_previous = inputs_iterator.next () .unwrap ();
			for input_current in inputs_iterator {
				if ! try! ($compare_2 (input_previous, input_current, comparison)) {
					succeed! (false);
				}
				input_previous = input_current;
			}
			succeed! (true);
		}
	);
	(
			$type : ident,
			$compare_1 : ident, $compare_2 : ident, $compare_3 : ident, $compare_4 : ident, $compare_n : ident,
			$compare_1a : ident, $compare_2a : ident, $compare_3a : ident, $compare_4a : ident, $compare_na : ident
	) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_1 <ValueRef : StdAsRef<Value>> (input_1 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_1.as_ref ()));
			return $compare_1a (input_1, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_2 <ValueRef : StdAsRef<Value>> (input_1 : ValueRef, input_2 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_1.as_ref ()));
			let input_2 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_2.as_ref ()));
			return $compare_2a (input_1, input_2, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_3 <ValueRef : StdAsRef<Value>> (input_1 : ValueRef, input_2 : ValueRef, input_3 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_1.as_ref ()));
			let input_2 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_2.as_ref ()));
			let input_3 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_3.as_ref ()));
			return $compare_3a (input_1, input_2, input_3, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_4 <ValueRef : StdAsRef<Value>> (input_1 : ValueRef, input_2 : ValueRef, input_3 : ValueRef, input_4 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_1.as_ref ()));
			let input_2 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_2.as_ref ()));
			let input_3 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_3.as_ref ()));
			let input_4 = try! (StdTryAsRef0::<$type>::try_as_ref_0 (input_4.as_ref ()));
			return $compare_4a (input_1, input_2, input_3, input_4, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_n <ValueRef : StdAsRef<Value>> (inputs : &[ValueRef], comparison : Comparison) -> (Outcome<bool>) {
			let inputs_count = inputs.len ();
			match inputs_count {
				0 =>
					succeed! (true),
				1 =>
					return $compare_1 (inputs[0].as_ref (), comparison),
				_ =>
					(),
			}
			// FIXME:  Try to eliminate extra vector creation!
			let inputs = try! (inputs.iter () .map (|input| StdTryAsRef0::<$type>::try_as_ref_0 (input.as_ref ())) .collect::<Outcome<StdVec<_>>> ());
			let mut inputs_iterator = inputs.iter ();
			let mut input_previous = inputs_iterator.next () .unwrap ();
			for input_current in inputs_iterator {
				if ! try! ($compare_2a (input_previous, input_current, comparison)) {
					succeed! (false);
				}
				input_previous = input_current;
			}
			succeed! (true);
		}
		def_fn_compare! ($type, $compare_1a, $compare_2a, $compare_3a, $compare_4a, $compare_na);
	);
}




def_fn_compare! (Value,
		compare_1, compare_2, compare_3, compare_4, compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let value = value.as_ref ();
	match value.kind_match_as_ref () {
		
		ValueKindMatchAsRef::Null =>
			return value_singleton_compare_1a (&NULL, comparison),
		ValueKindMatchAsRef::Void =>
			return value_singleton_compare_1a (&VOID, comparison),
		ValueKindMatchAsRef::Undefined =>
			return value_singleton_compare_1a (&UNDEFINED, comparison),
		ValueKindMatchAsRef::Singleton (value) =>
			return value_singleton_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Boolean (value) =>
			return boolean_compare_1a (value,  comparison),
		
		ValueKindMatchAsRef::NumberInteger (value) =>
			return number_integer_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::NumberReal (value) =>
			return number_real_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Character (value) =>
			return character_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Symbol (value) =>
			return symbol_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::StringImmutable (value) =>
			return string_immutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::StringMutable (value) =>
			return string_mutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::BytesImmutable (value) =>
			return bytes_immutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::BytesMutable (value) =>
			return bytes_mutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::PairImmutable (value) =>
			return pair_immutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::PairMutable (value) =>
			return pair_mutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ArrayImmutable (value) =>
			return array_immutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ArrayMutable (value) =>
			return array_mutable_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Values (value) =>
			return values_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Error (value) =>
			return error_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ProcedurePrimitive (value) =>
			return procedure_primitive_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ProcedureExtended (value) =>
			return procedure_extended_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ProcedureNative (value) =>
			return procedure_native_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ProcedureLambda (value) =>
			return procedure_lambda_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::SyntaxPrimitive (value) =>
			return syntax_primitive_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::SyntaxExtended (value) =>
			return syntax_extended_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::SyntaxNative (value) =>
			return syntax_native_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::SyntaxLambda (value) =>
			return syntax_lambda_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Port (value) =>
			return port_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Process (value) =>
			return process_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Context (value) =>
			return context_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Binding (value) =>
			return binding_compare_1a (value, comparison),
		
	}
	
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match Value::kind_match_as_ref_2 (left, right) {
		
		ValueKindMatchAsRef2::Null =>
			return value_singleton_compare_2a (&NULL, &NULL, comparison),
		ValueKindMatchAsRef2::Void =>
			return value_singleton_compare_2a (&VOID, &VOID, comparison),
		ValueKindMatchAsRef2::Undefined =>
			return value_singleton_compare_2a (&UNDEFINED, &UNDEFINED, comparison),
		ValueKindMatchAsRef2::Singleton (value) =>
			return value_singleton_compare_2a (value, value, comparison),
		
		ValueKindMatchAsRef2::Boolean (left, right) =>
			return boolean_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::NumberInteger (left, right) =>
			return number_integer_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::NumberReal (left, right) =>
			return number_real_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Character (left, right) =>
			return character_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Symbol (left, right) =>
			return symbol_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::StringImmutable (left, right) =>
			return string_immutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::StringMutable (left, right) =>
			return string_mutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::BytesImmutable (left, right) =>
			return bytes_immutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::BytesMutable (left, right) =>
			return bytes_mutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::PairImmutable (left, right) =>
			return pair_immutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::PairMutable (left, right) =>
			return pair_mutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ArrayImmutable (left, right) =>
			return array_immutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ArrayMutable (left, right) =>
			return array_mutable_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Values (left, right) =>
			return values_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Error (left, right) =>
			return error_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ProcedurePrimitive (left, right) =>
			return procedure_primitive_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ProcedureExtended (left, right) =>
			return procedure_extended_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ProcedureNative (left, right) =>
			return procedure_native_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ProcedureLambda (left, right) =>
			return procedure_lambda_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::SyntaxPrimitive (left, right) =>
			return syntax_primitive_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::SyntaxExtended (left, right) =>
			return syntax_extended_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::SyntaxNative (left, right) =>
			return syntax_native_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::SyntaxLambda (left, right) =>
			return syntax_lambda_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Port (left, right) =>
			return port_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Process (left, right) =>
			return process_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Context (left, right) =>
			return context_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Binding (left, right) =>
			return binding_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Missmatched =>
			match Value::class_match_as_ref_2 (left, right) {
				
				ValueClassMatchAsRef2::Number (ref class) =>
					return number_match_as_ref_compare_2a (class, comparison),
				
				ValueClassMatchAsRef2::String (ref class) => {
					let (left, right) = class.string_ref ();
					return string_ref_compare_2a (&left, &right, comparison);
				},
				
				ValueClassMatchAsRef2::Bytes (ref class) => {
					let (left, right) = class.bytes_ref ();
					return bytes_ref_compare_2a (&left, &right, comparison);
				},
				
				ValueClassMatchAsRef2::Array (ref class) => {
					let (left, right) = class.array_ref ();
					return array_ref_compare_2a (&left, &right, comparison);
				}
				
				ValueClassMatchAsRef2::Pair (ref class) => {
					let (left, right) = class.pair_ref ();
					return pair_ref_compare_2a (&left, &right, comparison);
				}
				
				_ =>
					match comparison {
						Comparison::Equivalence (_, _, _) =>
							succeed! (false),
						Comparison::Ordering (ordering, _, _) => {
							let left_kind = left.kind ();
							let right_kind = right.kind ();
							return value_kind_compare_2a_ordering (left_kind, right_kind, ordering);
						},
					},
				
			},
		
	}
}




def_fn_compare! (ValueSingleton,
		value_singleton_compare_1, value_singleton_compare_2, value_singleton_compare_3, value_singleton_compare_4, value_singleton_compare_n,
		value_singleton_compare_1a, value_singleton_compare_2a, value_singleton_compare_3a, value_singleton_compare_4a, value_singleton_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn value_singleton_compare_1a <ValueRef : StdAsRef<ValueSingleton>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let value = value.as_ref ();
	match *value {
		ValueSingleton::Undefined =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (true),
				Comparison::Ordering (_, _, _) =>
					fail! (0x534ee60c),
			},
		_ =>
			succeed! (true),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn value_singleton_compare_2a <ValueRef : StdAsRef<ValueSingleton>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match (*left, *right) {
		(ValueSingleton::Undefined, ValueSingleton::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (true),
				Comparison::Ordering (_, _, _) =>
					fail! (0xec7931c0),
			},
		(ValueSingleton::Undefined, _) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (false),
				Comparison::Ordering (_, _, _) =>
					fail! (0xa7c9f145),
			},
		(_, ValueSingleton::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (false),
				Comparison::Ordering (_, _, _) =>
					fail! (0xb57c53eb),
			},
		_ =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Boolean,
		boolean_compare_1, boolean_compare_2, boolean_compare_3, boolean_compare_4, boolean_compare_n,
		boolean_compare_1a, boolean_compare_2a, boolean_compare_3a, boolean_compare_4a, boolean_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn boolean_compare_1a <ValueRef : StdAsRef<Boolean>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn boolean_compare_2a <ValueRef : StdAsRef<Boolean>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}




def_fn_compare! (NumberInteger,
		number_integer_compare_1, number_integer_compare_2, number_integer_compare_3, number_integer_compare_4, number_integer_compare_n,
		number_integer_compare_1a, number_integer_compare_2a, number_integer_compare_3a, number_integer_compare_4a, number_integer_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_integer_compare_1a <ValueRef : StdAsRef<NumberInteger>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_integer_compare_2a <ValueRef : StdAsRef<NumberInteger>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}




def_fn_compare! (NumberReal,
		number_real_compare_1, number_real_compare_2, number_real_compare_3, number_real_compare_4, number_real_compare_n,
		number_real_compare_1a, number_real_compare_2a, number_real_compare_3a, number_real_compare_4a, number_real_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_real_compare_1a <ValueRef : StdAsRef<NumberReal>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let value = value.as_ref () .value ();
	if value.is_nan () {
		match comparison {
			Comparison::Equivalence (_, _, _) =>
				succeed! (true),
			Comparison::Ordering (_, _, _) =>
				succeed! (false),
		}
	} else {
		succeed! (true);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_real_compare_2a <ValueRef : StdAsRef<NumberReal>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .value ();
	let right = right.as_ref () .value ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			if left.is_nan () && right.is_nan () {
				succeed! (true);
			} else {
				succeed! (left == right);
			},
		Comparison::Ordering (ordering, _, _) =>
			if left.is_nan () || right.is_nan () {
				succeed! (false);
			} else {
				return std_ord_compare_2_ordering_val (left, right, ordering);
			},
	}
}




def_fn_compare! (Character,
		character_compare_1, character_compare_2, character_compare_3, character_compare_4, character_compare_n,
		character_compare_1a, character_compare_2a, character_compare_3a, character_compare_4a, character_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_compare_1a <ValueRef : StdAsRef<Character>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_compare_2a <ValueRef : StdAsRef<Character>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .value ();
	let right = right.as_ref () .value ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_val (left, right, ordering),
				_ =>
					fail_unimplemented! (0xea3c51f1), // deferred
			},
	}
}




def_fn_compare! (Symbol,
		symbol_compare_1, symbol_compare_2, symbol_compare_3, symbol_compare_4, symbol_compare_n,
		symbol_compare_1a, symbol_compare_2a, symbol_compare_3a, symbol_compare_4a, symbol_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_compare_1a <ValueRef : StdAsRef<Symbol>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_compare_2a <ValueRef : StdAsRef<Symbol>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .string_as_str ();
	let right = right.as_ref () .string_as_str ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_val (left, right, ordering),
				_ =>
					fail_unimplemented! (0xc4ef7065), // deferred
			},
	}
}




def_fn_compare! (StringImmutable,
		string_immutable_compare_1, string_immutable_compare_2, string_immutable_compare_3, string_immutable_compare_4, string_immutable_compare_n,
		string_immutable_compare_1a, string_immutable_compare_2a, string_immutable_compare_3a, string_immutable_compare_4a, string_immutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_compare_1a <ValueRef : StdAsRef<StringImmutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_compare_2a <ValueRef : StdAsRef<StringImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .string_ref ();
	let right = right.as_ref () .string_ref ();
	return string_ref_compare_2a (&left, &right, comparison);
}


def_fn_compare! (StringMutable,
		string_mutable_compare_1, string_mutable_compare_2, string_mutable_compare_3, string_mutable_compare_4, string_mutable_compare_n,
		string_mutable_compare_1a, string_mutable_compare_2a, string_mutable_compare_3a, string_mutable_compare_4a, string_mutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_compare_1a <ValueRef : StdAsRef<StringMutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_compare_2a <ValueRef : StdAsRef<StringMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .string_ref ();
	let right = right.as_ref () .string_ref ();
	return string_ref_compare_2a (&left, &right, comparison);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_ref_compare_2a <'a, ValueRef : StdAsRef<StringRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (StringRef::is_self (left, right)),
				Equivalence::ByValue =>
					succeed! (left == right),
			},
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering),
				_ =>
					fail_unimplemented! (0x2736b1f6), // deferred
			},
	}
}




def_fn_compare! (BytesImmutable,
		bytes_immutable_compare_1, bytes_immutable_compare_2, bytes_immutable_compare_3, bytes_immutable_compare_4, bytes_immutable_compare_n,
		bytes_immutable_compare_1a, bytes_immutable_compare_2a, bytes_immutable_compare_3a, bytes_immutable_compare_4a, bytes_immutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_compare_1a <ValueRef : StdAsRef<BytesImmutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_compare_2a <ValueRef : StdAsRef<BytesImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .bytes_ref ();
	let right = right.as_ref () .bytes_ref ();
	return bytes_ref_compare_2a (&left, &right, comparison);
}


def_fn_compare! (BytesMutable,
		bytes_mutable_compare_1, bytes_mutable_compare_2, bytes_mutable_compare_3, bytes_mutable_compare_4, bytes_mutable_compare_n,
		bytes_mutable_compare_1a, bytes_mutable_compare_2a, bytes_mutable_compare_3a, bytes_mutable_compare_4a, bytes_mutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_compare_1a <ValueRef : StdAsRef<BytesMutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_compare_2a <ValueRef : StdAsRef<BytesMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .bytes_ref ();
	let right = right.as_ref () .bytes_ref ();
	return bytes_ref_compare_2a (&left, &right, comparison);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_ref_compare_2a <'a, ValueRef : StdAsRef<BytesRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (BytesRef::is_self (left, right)),
				Equivalence::ByValue =>
					succeed! (left == right),
			},
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering),
				_ =>
					fail_unimplemented! (0x5be9ea91), // deferred
			},
	}
}




def_fn_compare! (PairImmutable,
		pair_immutable_compare_1, pair_immutable_compare_2, pair_immutable_compare_3, pair_immutable_compare_4, pair_immutable_compare_n,
		pair_immutable_compare_1a, pair_immutable_compare_2a, pair_immutable_compare_3a, pair_immutable_compare_4a, pair_immutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_compare_1a <ValueRef : StdAsRef<PairImmutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_compare_2a <ValueRef : StdAsRef<PairImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .pair_ref ();
	let right = right.as_ref () .pair_ref ();
	return pair_ref_compare_2a (&left, &right, comparison);
}


def_fn_compare! (PairMutable,
		pair_mutable_compare_1, pair_mutable_compare_2, pair_mutable_compare_3, pair_mutable_compare_4, pair_mutable_compare_n,
		pair_mutable_compare_1a, pair_mutable_compare_2a, pair_mutable_compare_3a, pair_mutable_compare_4a, pair_mutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_compare_1a <ValueRef : StdAsRef<PairMutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_compare_2a <ValueRef : StdAsRef<PairMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .pair_ref ();
	let right = right.as_ref () .pair_ref ();
	return pair_ref_compare_2a (&left, &right, comparison);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_ref_compare_2a <'a, ValueRef : StdAsRef<PairRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (PairRef::value_is_self (left, right)),
				Equivalence::ByValue => {
					let comparison = comparison.for_aggregated (false);
					succeed! (
							try! (compare_2 (left.left (), right.left (), comparison)) &&
							try! (compare_2 (left.right (), right.right (), comparison)));
				},
			},
		
		Comparison::Ordering (_, _, _) => {
			let comparison_for_last = comparison.for_aggregated (true);
			let comparison_for_non_last = comparison.for_aggregated (false);
			
			if ! try! (compare_2 (left.left (), right.left (), comparison_for_non_last)) {
				if comparison_for_non_last == comparison_for_last {
					succeed! (false);
				} else {
					if try! (compare_2 (left.left (), right.left (), comparison_for_last)) {
						succeed! (true);
					} else {
						succeed! (false);
					}
				}
			}
			
			if try! (compare_2 (left.right (), right.right (), comparison_for_last)) {
				succeed! (true);
			} else {
				succeed! (false);
			}
			
		},
		
	}
}




def_fn_compare! (ArrayImmutable,
		array_immutable_compare_1, array_immutable_compare_2, array_immutable_compare_3, array_immutable_compare_4, array_immutable_compare_n,
		array_immutable_compare_1a, array_immutable_compare_2a, array_immutable_compare_3a, array_immutable_compare_4a, array_immutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_compare_1a <ValueRef : StdAsRef<ArrayImmutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_compare_2a <ValueRef : StdAsRef<ArrayImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .array_ref ();
	let right = right.as_ref () .array_ref ();
	return array_ref_compare_2a (&left, &right, comparison);
}


def_fn_compare! (ArrayMutable,
		array_mutable_compare_1, array_mutable_compare_2, array_mutable_compare_3, array_mutable_compare_4, array_mutable_compare_n,
		array_mutable_compare_1a, array_mutable_compare_2a, array_mutable_compare_3a, array_mutable_compare_4a, array_mutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_compare_1a <ValueRef : StdAsRef<ArrayMutable>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_compare_2a <ValueRef : StdAsRef<ArrayMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .array_ref ();
	let right = right.as_ref () .array_ref ();
	return array_ref_compare_2a (&left, &right, comparison);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_ref_compare_2a <'a, ValueRef : StdAsRef<ArrayRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (ArrayRef::is_self (left, right)),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
			},
		Comparison::Ordering (_, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
	}
}




def_fn_compare! (Values,
		values_compare_1, values_compare_2, values_compare_3, values_compare_4, values_compare_n,
		values_compare_1a, values_compare_2a, values_compare_3a, values_compare_4a, values_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_compare_1a <ValueRef : StdAsRef<Values>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_compare_2a <ValueRef : StdAsRef<Values>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Values::is_self (left, right)),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
			},
		Comparison::Ordering (_, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
	}
}




def_fn_compare! (Error,
		error_compare_1, error_compare_2, error_compare_3, error_compare_4, error_compare_n,
		error_compare_1a, error_compare_2a, error_compare_3a, error_compare_4a, error_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_compare_1a <ValueRef : StdAsRef<Error>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_compare_2a <ValueRef : StdAsRef<Error>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Error::is_self (left, right)),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering_val (left.code, right.code, ordering),
	}
}




def_fn_compare! (ProcedurePrimitive,
		procedure_primitive_compare_1, procedure_primitive_compare_2, procedure_primitive_compare_3, procedure_primitive_compare_4, procedure_primitive_compare_n,
		procedure_primitive_compare_1a, procedure_primitive_compare_2a, procedure_primitive_compare_3a, procedure_primitive_compare_4a, procedure_primitive_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_primitive_compare_1a <ValueRef : StdAsRef<ProcedurePrimitive>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_primitive_compare_2a <ValueRef : StdAsRef<ProcedurePrimitive>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}


def_fn_compare! (ProcedureExtended,
		procedure_extended_compare_1, procedure_extended_compare_2, procedure_extended_compare_3, procedure_extended_compare_4, procedure_extended_compare_n,
		procedure_extended_compare_1a, procedure_extended_compare_2a, procedure_extended_compare_3a, procedure_extended_compare_4a, procedure_extended_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_compare_1a <ValueRef : StdAsRef<ProcedureExtended>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_compare_2a <ValueRef : StdAsRef<ProcedureExtended>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (ProcedureExtended::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


def_fn_compare! (ProcedureNative,
		procedure_native_compare_1, procedure_native_compare_2, procedure_native_compare_3, procedure_native_compare_4, procedure_native_compare_n,
		procedure_native_compare_1a, procedure_native_compare_2a, procedure_native_compare_3a, procedure_native_compare_4a, procedure_native_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_native_compare_1a <ValueRef : StdAsRef<ProcedureNative>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_native_compare_2a <ValueRef : StdAsRef<ProcedureNative>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (ProcedureNative::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


def_fn_compare! (ProcedureLambda,
		procedure_lambda_compare_1, procedure_lambda_compare_2, procedure_lambda_compare_3, procedure_lambda_compare_4, procedure_lambda_compare_n,
		procedure_lambda_compare_1a, procedure_lambda_compare_2a, procedure_lambda_compare_3a, procedure_lambda_compare_4a, procedure_lambda_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_lambda_compare_1a <ValueRef : StdAsRef<ProcedureLambda>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_lambda_compare_2a <ValueRef : StdAsRef<ProcedureLambda>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (ProcedureLambda::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (SyntaxPrimitive,
		syntax_primitive_compare_1, syntax_primitive_compare_2, syntax_primitive_compare_3, syntax_primitive_compare_4, syntax_primitive_compare_n,
		syntax_primitive_compare_1a, syntax_primitive_compare_2a, syntax_primitive_compare_3a, syntax_primitive_compare_4a, syntax_primitive_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_primitive_compare_1a <ValueRef : StdAsRef<SyntaxPrimitive>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_primitive_compare_2a <ValueRef : StdAsRef<SyntaxPrimitive>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}


def_fn_compare! (SyntaxExtended,
		syntax_extended_compare_1, syntax_extended_compare_2, syntax_extended_compare_3, syntax_extended_compare_4, syntax_extended_compare_n,
		syntax_extended_compare_1a, syntax_extended_compare_2a, syntax_extended_compare_3a, syntax_extended_compare_4a, syntax_extended_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_extended_compare_1a <ValueRef : StdAsRef<SyntaxExtended>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_extended_compare_2a <ValueRef : StdAsRef<SyntaxExtended>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (SyntaxExtended::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


def_fn_compare! (SyntaxNative,
		syntax_native_compare_1, syntax_native_compare_2, syntax_native_compare_3, syntax_native_compare_4, syntax_native_compare_n,
		syntax_native_compare_1a, syntax_native_compare_2a, syntax_native_compare_3a, syntax_native_compare_4a, syntax_native_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_native_compare_1a <ValueRef : StdAsRef<SyntaxNative>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_native_compare_2a <ValueRef : StdAsRef<SyntaxNative>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (SyntaxNative::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


def_fn_compare! (SyntaxLambda,
		syntax_lambda_compare_1, syntax_lambda_compare_2, syntax_lambda_compare_3, syntax_lambda_compare_4, syntax_lambda_compare_n,
		syntax_lambda_compare_1a, syntax_lambda_compare_2a, syntax_lambda_compare_3a, syntax_lambda_compare_4a, syntax_lambda_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_lambda_compare_1a <ValueRef : StdAsRef<SyntaxLambda>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_lambda_compare_2a <ValueRef : StdAsRef<SyntaxLambda>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (SyntaxLambda::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Port,
		port_compare_1, port_compare_2, port_compare_3, port_compare_4, port_compare_n,
		port_compare_1a, port_compare_2a, port_compare_3a, port_compare_4a, port_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_compare_1a <ValueRef : StdAsRef<Port>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_compare_2a <ValueRef : StdAsRef<Port>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Port::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Process,
		process_compare_1, process_compare_2, process_compare_3, process_compare_4, process_compare_n,
		process_compare_1a, process_compare_2a, process_compare_3a, process_compare_4a, process_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_compare_1a <ValueRef : StdAsRef<Process>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_compare_2a <ValueRef : StdAsRef<Process>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Process::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Context,
		context_compare_1, context_compare_2, context_compare_3, context_compare_4, context_compare_n,
		context_compare_1a, context_compare_2a, context_compare_3a, context_compare_4a, context_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn context_compare_1a <ValueRef : StdAsRef<Context>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn context_compare_2a <ValueRef : StdAsRef<Context>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Context::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Binding,
		binding_compare_1, binding_compare_2, binding_compare_3, binding_compare_4, binding_compare_n,
		binding_compare_1a, binding_compare_2a, binding_compare_3a, binding_compare_4a, binding_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn binding_compare_1a <ValueRef : StdAsRef<Binding>> (_value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn binding_compare_2a <ValueRef : StdAsRef<Binding>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Binding::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (Value,
		number_compare_1, number_compare_2, number_compare_3, number_compare_4, number_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let value = value.as_ref ();
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (ref class) =>
			return number_match_as_ref_compare_1a (class, comparison),
		_ =>
			fail! (0x733461d4),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match Value::class_match_as_ref_2 (left, right) {
		ValueClassMatchAsRef2::Number (ref class) =>
			return number_match_as_ref_compare_2a (class, comparison),
		_ =>
			fail! (0x482308ce),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_match_as_ref_compare_1a (class : &NumberMatchAsRef, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (_, _, _) =>
			match *class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true),
				NumberMatchAsRef::Real (_) =>
					succeed! (true),
			},
		
		Comparison::Ordering (_, _, _) =>
			match *class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true),
				NumberMatchAsRef::Real (value) => {
					let value = value.value ();
					if value.is_nan () {
						succeed! (false);
					} else {
						succeed! (true);
					}
				},
			},
		
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_match_as_ref_compare_2a (class : &NumberMatchAsRef2, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (_, coercion, _) =>
			match coercion {
				
				None | Some (false) =>
					match *class {
						NumberMatchAsRef2::IntegerBoth (left, right) => {
							let left = left.value ();
							let right = right.value ();
							succeed! (left == right);
						},
						NumberMatchAsRef2::RealBoth (left, right) => {
							let left = left.value ();
							let right = right.value ();
							if left.is_nan () && right.is_nan () {
								succeed! (true);
							} else {
								succeed! (left == right);
							}
						},
						NumberMatchAsRef2::IntegerAndReal (_, _) =>
							succeed! (false),
						NumberMatchAsRef2::RealAndInteger (_, _) =>
							succeed! (false),
					},
				
				Some (true) =>
					match number_coerce_2e (class) {
						NumberCoercion2::Integer (left, right) =>
							succeed! (left == right),
						NumberCoercion2::Real (left, right) =>
							if left.is_nan () && right.is_nan () {
								succeed! (true);
							} else {
								succeed! (left == right);
							},
					},
				
			},
		
		Comparison::Ordering (ordering, _, _) =>
			match number_coerce_2e (class) {
				NumberCoercion2::Integer (left, right) =>
					return std_ord_compare_2_ordering_val (left, right, ordering),
				NumberCoercion2::Real (left, right) =>
					if left.is_nan () || right.is_nan () {
						succeed! (false);
					} else {
						return std_ord_compare_2_ordering_val (left, right, ordering);
					},
			},
		
	}
}




def_fn_compare! (Value,
		string_compare_1, string_compare_2, string_compare_3, string_compare_4, string_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (StringRef::try (value.as_ref ()));
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (StringRef::try (left.as_ref ()));
	let right = try! (StringRef::try (right.as_ref ()));
	return string_ref_compare_2a (&left, &right, comparison);
}




def_fn_compare! (Value,
		bytes_compare_1, bytes_compare_2, bytes_compare_3, bytes_compare_4, bytes_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (BytesRef::try (value.as_ref ()));
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (BytesRef::try (left.as_ref ()));
	let right = try! (BytesRef::try (right.as_ref ()));
	return bytes_ref_compare_2a (&left, &right, comparison);
}




def_fn_compare! (Value,
		pair_compare_1, pair_compare_2, pair_compare_3, pair_compare_4, pair_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (PairRef::try_ref (value.as_ref ()));
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (PairRef::try_ref (left.as_ref ()));
	let right = try! (PairRef::try_ref (right.as_ref ()));
	return pair_ref_compare_2a (&left, &right, comparison);
}




def_fn_compare! (Value,
		array_compare_1, array_compare_2, array_compare_3, array_compare_4, array_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, _comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (ArrayRef::try (value.as_ref ()));
	succeed! (true);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (ArrayRef::try (left.as_ref ()));
	let right = try! (ArrayRef::try (right.as_ref ()));
	return array_ref_compare_2a (&left, &right, comparison);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn value_kind_compare_2a_ordering (left : ValueKind, right : ValueKind, ordering : Ordering) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering_val (left, right, ordering);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn value_class_compare_2a_ordering (left : ValueClass, right : ValueClass, ordering : Ordering) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering_val (left, right, ordering);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_compare_2 (left : &[Value], right : &[Value], comparison : Comparison) -> (Outcome<bool>) {
	
	let left_length = left.len ();
	let right_length = right.len ();
	
	match comparison {
		
		Comparison::Equivalence (_, _, _) =>
			if left_length != right_length {
				succeed! (false);
			} else if (left_length == 0) && (right_length == 0) {
				succeed! (true);
			},
		
		Comparison::Ordering (ordering, _, _) =>
			if (left_length == 0) && (right_length == 0) {
				match ordering {
					Ordering::LesserOrEqual | Ordering::Equal | Ordering::GreaterOrEqual =>
						succeed! (true),
					Ordering::Lesser | Ordering::Greater =>
						succeed! (false),
				}
			},
		
	}
	
	let mut left_iterator = left.iter ();
	let mut right_iterator = right.iter ();
	let comparison_for_last = comparison.for_aggregated (true);
	let comparison_for_non_last = comparison.for_aggregated (false);
	let index_last = usize::max (left_length, right_length) - 1;
	let mut index_next = 0;
	loop {
		let left_next = left_iterator.next ();
		let right_next = right_iterator.next ();
		match (left_next, right_next) {
			
			(Some (left_next), Some (right_next)) =>
				if index_next == index_last {
					if ! try! (compare_2 (left_next, right_next, comparison_for_last)) {
						succeed! (false);
					}
				} else {
					if ! try! (compare_2 (left_next, right_next, comparison_for_non_last)) {
						if comparison_for_non_last == comparison_for_last {
							succeed! (false);
						}
						if try! (compare_2 (left_next, right_next, comparison_for_last)) {
							succeed! (true);
						} else {
							succeed! (false);
						}
					}
				},
			
			(None, None) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						succeed! (true),
					Comparison::Ordering (_, _, _) =>
						succeed! (true),
				},
			
			(None, Some (_)) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						fail_unreachable! (0x97e8a0c7),
					Comparison::Ordering (ordering, _, _) =>
						match ordering {
							Ordering::Lesser | Ordering::LesserOrEqual =>
								succeed! (true),
							Ordering::Equal | Ordering::GreaterOrEqual | Ordering::Greater =>
								succeed! (false),
						},
				},
			
			(Some (_), None) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						fail_unreachable! (0x2558c7ee),
					Comparison::Ordering (ordering, _, _) =>
						match ordering {
							Ordering::Greater | Ordering::GreaterOrEqual =>
								succeed! (true),
							Ordering::Equal | Ordering::LesserOrEqual | Ordering::Lesser =>
								succeed! (false),
						},
				},
			
		}
		
		index_next += 1;
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn std_ord_compare_2_ref <Value, ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left.as_ref () == right.as_ref ()),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering_ref (left, right, ordering),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn std_ord_compare_2_ordering_ref <Value, ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, ordering : Ordering) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	let left = left.as_ref ();
	let right = right.as_ref ();
	let output = match ordering {
		Ordering::Lesser =>
			left < right,
		Ordering::LesserOrEqual =>
			left <= right,
		Ordering::Equal =>
			left == right,
		Ordering::GreaterOrEqual =>
			left >= right,
		Ordering::Greater =>
			left > right,
	};
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn std_ord_compare_2_val <Value> (left : Value, right : Value, comparison : Comparison) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering_val (left, right, ordering),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn std_ord_compare_2_ordering_val <Value> (left : Value, right : Value, ordering : Ordering) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	let output = match ordering {
		Ordering::Lesser =>
			left < right,
		Ordering::LesserOrEqual =>
			left <= right,
		Ordering::Equal =>
			left == right,
		Ordering::GreaterOrEqual =>
			left >= right,
		Ordering::Greater =>
			left > right,
	};
	succeed! (output);
}

