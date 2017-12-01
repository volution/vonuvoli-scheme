

use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::procedures::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use std::cmp;




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
			
			compare_2,
			
			boolean_compare_2,
			number_integer_compare_2,
			number_real_compare_2,
			character_compare_2,
			symbol_compare_2,
			string_compare_2,
			bytes_compare_2,
			pair_compare_2,
			array_compare_2,
			values_compare_2,
			error_compare_2,
			lambda_compare_2,
			procedure_primitive_compare_2,
			syntax_primitive_compare_2,
			context_compare_2,
			binding_compare_2,
			
			number_compare_2,
			
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
	
	pub fn for_aggregated (&self) -> (Comparison) {
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
			
			Comparison::Ordering (ordering, case_sensitivity, recursive) =>
				match recursive {
					None | Some (false) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (false)),
					Some (true) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (true)),
				},
			
		}
	}
}




pub fn equivalent_by_identity_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByIdentity, None, None));
}

pub fn equivalent_by_value_strict_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)));
}

pub fn equivalent_by_value_strict_recursive_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)));
}

pub fn equivalent_by_value_coerced_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)));
}

pub fn equivalent_by_value_coerced_recursive_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)));
}




pub fn compare_2 (left : &Value, right : &Value, comparison : Comparison) -> (Outcome<bool>) {
	
	match (left.class (), right.class ()) {
		
		(ValueClass::Null, ValueClass::Null) |
		(ValueClass::Void, ValueClass::Void) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (true),
				Comparison::Ordering (ordering, _, _) =>
					match ordering {
						Ordering::Equal | Ordering::LesserOrEqual | Ordering::GreaterOrEqual =>
							succeed! (true),
						Ordering::Lesser | Ordering::Greater =>
							succeed! (false),
					},
			},
		
		(ValueClass::Boolean, ValueClass::Boolean) =>
			return boolean_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberInteger, ValueClass::NumberInteger) =>
			return number_integer_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberReal, ValueClass::NumberReal) =>
			return number_real_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Character, ValueClass::Character) =>
			return character_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Symbol, ValueClass::Symbol) =>
			return symbol_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::String, ValueClass::String) =>
			return string_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Bytes, ValueClass::Bytes) =>
			return bytes_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Pair, ValueClass::Pair) =>
			return pair_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Array, ValueClass::Array) =>
			return array_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Values, ValueClass::Values) =>
			return values_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Error, ValueClass::Error) =>
			return error_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Lambda, ValueClass::Lambda) =>
			return lambda_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::ProcedurePrimitive, ValueClass::ProcedurePrimitive) =>
			return procedure_primitive_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::SyntaxPrimitive, ValueClass::SyntaxPrimitive) =>
			return syntax_primitive_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Context, ValueClass::Context) =>
			return context_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Binding, ValueClass::Binding) =>
			return binding_compare_2 (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberInteger, ValueClass::NumberReal) =>
			return number_compare_2 (left, right, comparison),
		
		(ValueClass::NumberReal, ValueClass::NumberInteger) =>
			return number_compare_2 (left, right, comparison),
		
		(ValueClass::Undefined, ValueClass::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (true),
				Comparison::Ordering (_, _, _) =>
					fail! (0xec7931c0),
			},
		
		(ValueClass::Undefined, _) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (false),
				Comparison::Ordering (_, _, _) =>
					fail! (0xa7c9f145),
			},
		
		(_, ValueClass::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (false),
				Comparison::Ordering (_, _, _) =>
					fail! (0xb57c53eb),
			},
		
		(left_class, right_class) =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (false),
				Comparison::Ordering (ordering, _, _) =>
					return value_class_compare_2_ordering (left_class, right_class, ordering),
			},
		
	}
}




pub fn boolean_compare_2 (left : &Boolean, right : &Boolean, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}

pub fn number_integer_compare_2 (left : &NumberInteger, right : &NumberInteger, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}

pub fn number_real_compare_2 (left : &NumberReal, right : &NumberReal, comparison : Comparison) -> (Outcome<bool>) {
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
				return std_ord_compare_2_ordering (left, right, ordering);
			},
	}
}


pub fn character_compare_2 (left : &Character, right : &Character, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering (left, right, ordering),
				_ =>
					fail_unimplemented! (0xea3c51f1),
			},
	}
}

pub fn symbol_compare_2 (left : &Symbol, right : &Symbol, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering (left, right, ordering),
				_ =>
					fail_unimplemented! (0xc4ef7065),
			},
	}
}

pub fn string_compare_2 (left : &String, right : &String, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (String::is_self (left, right)),
				Equivalence::ByValue =>
					succeed! (left == right),
			},
		Comparison::Ordering (ordering, case_sensitivity, _) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering (left, right, ordering),
				_ =>
					fail_unimplemented! (0x2736b1f6),
			},
	}
}

pub fn bytes_compare_2 (left : &Bytes, right : &Bytes, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Bytes::is_self (left, right)),
				Equivalence::ByValue =>
					succeed! (left == right),
			},
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering (left, right, ordering),
	}
}


pub fn pair_compare_2 (left : &Pair, right : &Pair, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Pair::is_self (left, right)),
				Equivalence::ByValue => {
					let comparison = comparison.for_aggregated ();
					succeed! (
							try! (compare_2 (left.left (), right.left (), comparison)) &&
							try! (compare_2 (left.right (), right.right (), comparison)));
				},
			},
		Comparison::Ordering (_, _, _) => {
			let comparison = comparison.for_aggregated ();
			succeed! (
					try! (compare_2 (left.left (), right.left (), comparison)) &&
					try! (compare_2 (left.right (), right.right (), comparison)));
		},
	}
}

pub fn array_compare_2 (left : &Array, right : &Array, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Array::is_self (left, right)),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison.for_aggregated ()),
			},
		Comparison::Ordering (_, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison.for_aggregated ()),
	}
}

pub fn values_compare_2 (left : &Values, right : &Values, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Values::is_self (left, right)),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison.for_aggregated ()),
			},
		Comparison::Ordering (_, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison.for_aggregated ()),
	}
}


pub fn error_compare_2 (left : &Error, right : &Error, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Error::is_self (left, right)),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering (&left.code, &right.code, ordering),
	}
}

pub fn lambda_compare_2 (left : &Lambda, right : &Lambda, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Lambda::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x53fd2c24),
	}
}

pub fn procedure_primitive_compare_2 (left : &ProcedurePrimitive, right : &ProcedurePrimitive, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}

pub fn syntax_primitive_compare_2 (left : &SyntaxPrimitive, right : &SyntaxPrimitive, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}

pub fn context_compare_2 (left : &Context, right : &Context, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Context::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x9296c028),
	}
}

pub fn binding_compare_2 (left : &Binding, right : &Binding, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Binding::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x4466d4a7),
	}
}




pub fn number_compare_2 (left : &Value, right : &Value, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (_, coercion, _) =>
			match coercion {
				
				None | Some (false) =>
					match (left.class (), right.class ()) {
						(ValueClass::NumberInteger, ValueClass::NumberInteger) =>
							succeed! (NumberInteger::as_ref (left) == NumberInteger::as_ref (right)),
						(ValueClass::NumberReal, ValueClass::NumberReal) => {
							let left = NumberReal::as_ref (left);
							let right = NumberReal::as_ref (right);
							if left.is_nan () && right.is_nan () {
								succeed! (true);
							} else {
								succeed! (left == right);
							}
						},
						(ValueClass::NumberInteger, ValueClass::NumberReal) =>
							succeed! (false),
						(ValueClass::NumberReal, ValueClass::NumberInteger) =>
							succeed! (false),
						(ValueClass::NumberInteger, _) =>
							fail! (0x4878e871),
						(ValueClass::NumberReal, _) =>
							fail! (0xf6401a54),
						(_, ValueClass::NumberInteger) =>
							fail! (0xb73515b9),
						(_, ValueClass::NumberReal) =>
							fail! (0xbbdcc17a),
						(_, _) =>
							fail! (0xfc1f9e8b),
					},
				
				Some (true) =>
					match try! (number_coerce_2a (left, right)) {
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
			match try! (number_coerce_2a (left, right)) {
				NumberCoercion2::Integer (left, right) =>
					return std_ord_compare_2_ordering (&left, &right, ordering),
				NumberCoercion2::Real (left, right) =>
					if left.is_nan () || right.is_nan () {
						succeed! (false);
					} else {
						return std_ord_compare_2_ordering (&left, &right, ordering);
					},
			},
		
	}
}




pub fn value_class_compare_2_ordering (left : ValueClass, right : ValueClass, ordering : Ordering) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering (&left, &right, ordering);
}




pub fn vec_compare_2 (left : &[Value], right : &[Value], comparison : Comparison) -> (Outcome<bool>) {
	
	let left_length = left.len ();
	let right_length = right.len ();
	
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			if left_length != right_length {
				succeed! (false);
			},
		Comparison::Ordering (_, _, _) =>
			(),
	}
	
	let mut left_iterator = left.iter ();
	let mut right_iterator = right.iter ();
	loop {
		let left_next = left_iterator.next ();
		let right_next = right_iterator.next ();
		match (left_next, right_next) {
			
			(Some (left_next), Some (right_next)) =>
				if ! try! (compare_2 (left_next, right_next, comparison)) {
					succeed! (false);
				},
			
			(None, None) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						succeed! (true),
					Comparison::Ordering (ordering, _, _) =>
						match ordering {
							Ordering::Lesser | Ordering::Greater =>
								succeed! (false),
							Ordering::Equal | Ordering::LesserOrEqual | Ordering::GreaterOrEqual =>
								succeed! (true),
						}
				},
			
			(None, Some (_)) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						fail_panic! (0xee6ad119),
					Comparison::Ordering (ordering, _, _) =>
						match ordering {
							Ordering::Lesser | Ordering::LesserOrEqual =>
								succeed! (true),
							Ordering::Equal | Ordering::GreaterOrEqual | Ordering::Greater =>
								succeed! (false),
						}
				},
			
			(Some (_), None) =>
				match comparison {
					Comparison::Equivalence (_, _, _) =>
						fail_panic! (0xee6ad119),
					Comparison::Ordering (ordering, _, _) =>
						match ordering {
							Ordering::Greater | Ordering::GreaterOrEqual =>
								succeed! (true),
							Ordering::Equal | Ordering::LesserOrEqual | Ordering::Lesser =>
								succeed! (false),
						}
				},
			
		}
	}
}




pub fn std_ord_compare_2 <Value> (left : &Value, right : &Value, comparison : Comparison) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (left == right),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering (left, right, ordering),
	}
}

pub fn std_ord_compare_2_ordering <Value> (left : &Value, right : &Value, ordering : Ordering) -> (Outcome<bool>)
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

