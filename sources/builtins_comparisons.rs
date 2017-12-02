

use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::procedures::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
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
			
			compare_1,
			boolean_compare_1, boolean_compare_1a,
			number_integer_compare_1, number_integer_compare_1a,
			number_real_compare_1, number_real_compare_1a,
			character_compare_1, character_compare_1a,
			symbol_compare_1, symbol_compare_1a,
			string_compare_1, string_compare_1a,
			bytes_compare_1, bytes_compare_1a,
			pair_compare_1, pair_compare_1a,
			array_compare_1, array_compare_1a,
			values_compare_1, values_compare_1a,
			error_compare_1, error_compare_1a,
			lambda_compare_1, lambda_compare_1a,
			procedure_primitive_compare_1, procedure_primitive_compare_1a,
			syntax_primitive_compare_1, syntax_primitive_compare_1a,
			context_compare_1, context_compare_1a,
			binding_compare_1, binding_compare_1a,
			number_compare_1, number_compare_1a,
			
			compare_2,
			boolean_compare_2, boolean_compare_2a,
			number_integer_compare_2, number_integer_compare_2a,
			number_real_compare_2, number_real_compare_2a,
			character_compare_2, character_compare_2a,
			symbol_compare_2, symbol_compare_2a,
			string_compare_2, string_compare_2a,
			bytes_compare_2, bytes_compare_2a,
			pair_compare_2, pair_compare_2a,
			array_compare_2, array_compare_2a,
			values_compare_2, values_compare_2a,
			error_compare_2, error_compare_2a,
			lambda_compare_2, lambda_compare_2a,
			procedure_primitive_compare_2, procedure_primitive_compare_2a,
			syntax_primitive_compare_2, syntax_primitive_compare_2a,
			context_compare_2, context_compare_2a,
			binding_compare_2, binding_compare_2a,
			number_compare_2, number_compare_2a,
			
			compare_3,
			boolean_compare_3, boolean_compare_3a,
			number_integer_compare_3, number_integer_compare_3a,
			number_real_compare_3, number_real_compare_3a,
			character_compare_3, character_compare_3a,
			symbol_compare_3, symbol_compare_3a,
			string_compare_3, string_compare_3a,
			bytes_compare_3, bytes_compare_3a,
			pair_compare_3, pair_compare_3a,
			array_compare_3, array_compare_3a,
			values_compare_3, values_compare_3a,
			error_compare_3, error_compare_3a,
			lambda_compare_3, lambda_compare_3a,
			procedure_primitive_compare_3, procedure_primitive_compare_3a,
			syntax_primitive_compare_3, syntax_primitive_compare_3a,
			context_compare_3, context_compare_3a,
			binding_compare_3, binding_compare_3a,
			number_compare_3, number_compare_3a,
			
			compare_4,
			boolean_compare_4, boolean_compare_4a,
			number_integer_compare_4, number_integer_compare_4a,
			number_real_compare_4, number_real_compare_4a,
			character_compare_4, character_compare_4a,
			symbol_compare_4, symbol_compare_4a,
			string_compare_4, string_compare_4a,
			bytes_compare_4, bytes_compare_4a,
			pair_compare_4, pair_compare_4a,
			array_compare_4, array_compare_4a,
			values_compare_4, values_compare_4a,
			error_compare_4, error_compare_4a,
			lambda_compare_4, lambda_compare_4a,
			procedure_primitive_compare_4, procedure_primitive_compare_4a,
			syntax_primitive_compare_4, syntax_primitive_compare_4a,
			context_compare_4, context_compare_4a,
			binding_compare_4, binding_compare_4a,
			number_compare_4, number_compare_4a,
			
			compare_n,
			boolean_compare_n, boolean_compare_na,
			number_integer_compare_n, number_integer_compare_na,
			number_real_compare_n, number_real_compare_na,
			character_compare_n, character_compare_na,
			symbol_compare_n, symbol_compare_na,
			string_compare_n, string_compare_na,
			bytes_compare_n, bytes_compare_na,
			pair_compare_n, pair_compare_na,
			array_compare_n, array_compare_na,
			values_compare_n, values_compare_na,
			error_compare_n, error_compare_na,
			lambda_compare_n, lambda_compare_na,
			procedure_primitive_compare_n, procedure_primitive_compare_na,
			syntax_primitive_compare_n, syntax_primitive_compare_na,
			context_compare_n, context_compare_na,
			binding_compare_n, binding_compare_na,
			number_compare_n, number_compare_na,
			
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




pub fn equivalent_by_identity_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2a (left, right, Comparison::Equivalence (Equivalence::ByIdentity, None, None));
}

pub fn equivalent_by_value_strict_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2a (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)));
}

pub fn equivalent_by_value_strict_recursive_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2a (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)));
}

pub fn equivalent_by_value_coerced_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2a (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)));
}

pub fn equivalent_by_value_coerced_recursive_2 (left : &Value, right : &Value) -> (Outcome<bool>) {
	return compare_2a (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)));
}




macro_rules! def_fn_compare {
	(
			$type : ident,
			$compare_1 : ident, $compare_2 : ident, $compare_3 : ident, $compare_4 : ident, $compare_n : ident,
			$compare_1a : ident, $compare_2a : ident, $compare_3a : ident, $compare_4a : ident, $compare_na : ident
	) => (
		pub fn $compare_1 (input_1 : &Value, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! ($type::try_as_ref (input_1));
			return $compare_1a (input_1, comparison);
		}
		pub fn $compare_2 (input_1 : &Value, input_2 : &Value, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! ($type::try_as_ref (input_1));
			let input_2 = try! ($type::try_as_ref (input_2));
			return $compare_2a (input_1, input_2, comparison);
		}
		pub fn $compare_3 (input_1 : &Value, input_2 : &Value, input_3 : &Value, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! ($type::try_as_ref (input_1));
			let input_2 = try! ($type::try_as_ref (input_2));
			let input_3 = try! ($type::try_as_ref (input_3));
			return $compare_3a (input_1, input_2, input_3, comparison);
		}
		pub fn $compare_4 (input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = try! ($type::try_as_ref (input_1));
			let input_2 = try! ($type::try_as_ref (input_2));
			let input_3 = try! ($type::try_as_ref (input_3));
			let input_4 = try! ($type::try_as_ref (input_4));
			return $compare_4a (input_1, input_2, input_3, input_4, comparison);
		}
		pub fn $compare_3a (input_1 : &$type, input_2 : &$type, input_3 : &$type, comparison : Comparison) -> (Outcome<bool>) {
			if ! try! ($compare_2a (input_1, input_2, comparison)) {
				succeed! (false);
			}
			return $compare_2a (input_2, input_3, comparison);
		}
		pub fn $compare_4a (input_1 : &$type, input_2 : &$type, input_3 : &$type, input_4 : &$type, comparison : Comparison) -> (Outcome<bool>) {
			if ! try! ($compare_2a (input_1, input_2, comparison)) {
				succeed! (false);
			}
			if ! try! ($compare_2a (input_2, input_3, comparison)) {
				succeed! (false);
			}
			return $compare_2a (input_3, input_4, comparison);
		}
		pub fn $compare_n (inputs : &[Value], comparison : Comparison) -> (Outcome<bool>) {
			let inputs_count = inputs.len ();
			match inputs_count {
				0 =>
					succeed! (true),
				1 =>
					return $compare_1 (&inputs[0], comparison),
				2 =>
					return $compare_2 (&inputs[0], &inputs[1], comparison),
				3 =>
					return $compare_3 (&inputs[0], &inputs[1], &inputs[2], comparison),
				4 =>
					return $compare_4 (&inputs[0], &inputs[1], &inputs[2], &inputs[3], comparison),
				_ =>
					(),
			}
			let inputs = try! (inputs.iter () .map (|input| $type::try_as_ref (input)) .collect::<Outcome<StdVec<_>>> ());
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
		pub fn $compare_na (inputs : &[$type], comparison : Comparison) -> (Outcome<bool>) {
			let inputs_count = inputs.len ();
			match inputs_count {
				0 =>
					succeed! (true),
				1 =>
					return $compare_1a (&inputs[0], comparison),
				2 =>
					return $compare_2a (&inputs[0], &inputs[1], comparison),
				3 =>
					return $compare_3a (&inputs[0], &inputs[1], &inputs[2], comparison),
				4 =>
					return $compare_4a (&inputs[0], &inputs[1], &inputs[2], &inputs[3], comparison),
				_ =>
					(),
			}
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
	);
}




def_fn_compare! (Value,
		compare_1, compare_2, compare_3, compare_4, compare_n,
		compare_1a, compare_2a, compare_3a, compare_4a, compare_na);

pub fn compare_1a (value : &Value, comparison : Comparison) -> (Outcome<bool>) {
	match value.class () {
		
		ValueClass::Null |
		ValueClass::Void =>
			succeed! (true),
		
		ValueClass::Boolean =>
			return boolean_compare_1a (value.as_ref (),  comparison),
		
		ValueClass::NumberInteger =>
			return number_integer_compare_1a (value.as_ref (), comparison),
		
		ValueClass::NumberReal =>
			return number_real_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Character =>
			return character_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Symbol =>
			return symbol_compare_1a (value.as_ref (), comparison),
		
		ValueClass::String =>
			return string_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Bytes =>
			return bytes_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Pair =>
			return pair_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Array =>
			return array_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Values =>
			return values_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Error =>
			return error_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Lambda =>
			return lambda_compare_1a (value.as_ref (), comparison),
		
		ValueClass::ProcedurePrimitive =>
			return procedure_primitive_compare_1a (value.as_ref (), comparison),
		
		ValueClass::SyntaxPrimitive =>
			return syntax_primitive_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Context =>
			return context_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Binding =>
			return binding_compare_1a (value.as_ref (), comparison),
		
		ValueClass::Undefined =>
			match comparison {
				Comparison::Equivalence (_, _, _) =>
					succeed! (true),
				Comparison::Ordering (_, _, _) =>
					fail! (0x534ee60c),
			},
		
	}
	
}

pub fn compare_2a (left : &Value, right : &Value, comparison : Comparison) -> (Outcome<bool>) {
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
			return boolean_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberInteger, ValueClass::NumberInteger) =>
			return number_integer_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberReal, ValueClass::NumberReal) =>
			return number_real_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Character, ValueClass::Character) =>
			return character_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Symbol, ValueClass::Symbol) =>
			return symbol_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::String, ValueClass::String) =>
			return string_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Bytes, ValueClass::Bytes) =>
			return bytes_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Pair, ValueClass::Pair) =>
			return pair_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Array, ValueClass::Array) =>
			return array_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Values, ValueClass::Values) =>
			return values_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Error, ValueClass::Error) =>
			return error_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Lambda, ValueClass::Lambda) =>
			return lambda_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::ProcedurePrimitive, ValueClass::ProcedurePrimitive) =>
			return procedure_primitive_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::SyntaxPrimitive, ValueClass::SyntaxPrimitive) =>
			return syntax_primitive_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Context, ValueClass::Context) =>
			return context_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::Binding, ValueClass::Binding) =>
			return binding_compare_2a (left.as_ref (), right.as_ref (), comparison),
		
		(ValueClass::NumberInteger, ValueClass::NumberReal) =>
			return number_compare_2a (left, right, comparison),
		
		(ValueClass::NumberReal, ValueClass::NumberInteger) =>
			return number_compare_2a (left, right, comparison),
		
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
					return value_class_compare_2a_ordering (left_class, right_class, ordering),
			},
		
	}
}




def_fn_compare! (Boolean,
		boolean_compare_1, boolean_compare_2, boolean_compare_3, boolean_compare_4, boolean_compare_n,
		boolean_compare_1a, boolean_compare_2a, boolean_compare_3a, boolean_compare_4a, boolean_compare_na);

pub fn boolean_compare_1a (_value : &Boolean, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn boolean_compare_2a (left : &Boolean, right : &Boolean, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}


def_fn_compare! (NumberInteger,
		number_integer_compare_1, number_integer_compare_2, number_integer_compare_3, number_integer_compare_4, number_integer_compare_n,
		number_integer_compare_1a, number_integer_compare_2a, number_integer_compare_3a, number_integer_compare_4a, number_integer_compare_na);

pub fn number_integer_compare_1a (_value : &NumberInteger, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn number_integer_compare_2a (left : &NumberInteger, right : &NumberInteger, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}


def_fn_compare! (NumberReal,
		number_real_compare_1, number_real_compare_2, number_real_compare_3, number_real_compare_4, number_real_compare_n,
		number_real_compare_1a, number_real_compare_2a, number_real_compare_3a, number_real_compare_4a, number_real_compare_na);

pub fn number_real_compare_1a (value : &NumberReal, _comparison : Comparison) -> (Outcome<bool>) {
	if value.is_nan () {
		succeed! (false);
	} else {
		succeed! (true);
	}
}

pub fn number_real_compare_2a (left : &NumberReal, right : &NumberReal, comparison : Comparison) -> (Outcome<bool>) {
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


def_fn_compare! (Character,
		character_compare_1, character_compare_2, character_compare_3, character_compare_4, character_compare_n,
		character_compare_1a, character_compare_2a, character_compare_3a, character_compare_4a, character_compare_na);

pub fn character_compare_1a (_value : &Character, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn character_compare_2a (left : &Character, right : &Character, comparison : Comparison) -> (Outcome<bool>) {
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


def_fn_compare! (Symbol,
		symbol_compare_1, symbol_compare_2, symbol_compare_3, symbol_compare_4, symbol_compare_n,
		symbol_compare_1a, symbol_compare_2a, symbol_compare_3a, symbol_compare_4a, symbol_compare_na);

pub fn symbol_compare_1a (_value : &Symbol, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn symbol_compare_2a (left : &Symbol, right : &Symbol, comparison : Comparison) -> (Outcome<bool>) {
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


def_fn_compare! (String,
		string_compare_1, string_compare_2, string_compare_3, string_compare_4, string_compare_n,
		string_compare_1a, string_compare_2a, string_compare_3a, string_compare_4a, string_compare_na);

pub fn string_compare_1a (_value : &String, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn string_compare_2a (left : &String, right : &String, comparison : Comparison) -> (Outcome<bool>) {
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


def_fn_compare! (Bytes,
		bytes_compare_1, bytes_compare_2, bytes_compare_3, bytes_compare_4, bytes_compare_n,
		bytes_compare_1a, bytes_compare_2a, bytes_compare_3a, bytes_compare_4a, bytes_compare_na);

pub fn bytes_compare_1a (_value : &Bytes, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn bytes_compare_2a (left : &Bytes, right : &Bytes, comparison : Comparison) -> (Outcome<bool>) {
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


def_fn_compare! (Pair,
		pair_compare_1, pair_compare_2, pair_compare_3, pair_compare_4, pair_compare_n,
		pair_compare_1a, pair_compare_2a, pair_compare_3a, pair_compare_4a, pair_compare_na);

pub fn pair_compare_1a (_value : &Pair, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn pair_compare_2a (left : &Pair, right : &Pair, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Pair::is_self (left, right)),
				Equivalence::ByValue => {
					let comparison = comparison.for_aggregated (false);
					succeed! (
							try! (compare_2a (left.left (), right.left (), comparison)) &&
							try! (compare_2a (left.right (), right.right (), comparison)));
				},
			},
		
		Comparison::Ordering (_, _, _) => {
			let comparison_for_last = comparison.for_aggregated (true);
			let comparison_for_non_last = comparison.for_aggregated (false);
			
			if ! try! (compare_2a (left.left (), right.left (), comparison_for_non_last)) {
				if comparison_for_non_last == comparison_for_last {
					succeed! (false);
				} else {
					if try! (compare_2a (left.left (), right.left (), comparison_for_last)) {
						succeed! (true);
					} else {
						succeed! (false);
					}
				}
			}
			
			if try! (compare_2a (left.right (), right.right (), comparison_for_last)) {
				succeed! (true);
			} else {
				succeed! (false);
			}
			
		},
		
	}
}


def_fn_compare! (Array,
		array_compare_1, array_compare_2, array_compare_3, array_compare_4, array_compare_n,
		array_compare_1a, array_compare_2a, array_compare_3a, array_compare_4a, array_compare_na);

pub fn array_compare_1a (_value : &Array, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn array_compare_2a (left : &Array, right : &Array, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (equivalence, _, _) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Array::is_self (left, right)),
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

pub fn values_compare_1a (_value : &Values, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn values_compare_2a (left : &Values, right : &Values, comparison : Comparison) -> (Outcome<bool>) {
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

pub fn error_compare_1a (_value : &Error, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn error_compare_2a (left : &Error, right : &Error, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Error::is_self (left, right)),
		Comparison::Ordering (ordering, _, _) =>
			return std_ord_compare_2_ordering (&left.code, &right.code, ordering),
	}
}


def_fn_compare! (Lambda,
		lambda_compare_1, lambda_compare_2, lambda_compare_3, lambda_compare_4, lambda_compare_n,
		lambda_compare_1a, lambda_compare_2a, lambda_compare_3a, lambda_compare_4a, lambda_compare_na);

pub fn lambda_compare_1a (_value : &Lambda, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn lambda_compare_2a (left : &Lambda, right : &Lambda, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Lambda::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x53fd2c24),
	}
}


def_fn_compare! (ProcedurePrimitive,
		procedure_primitive_compare_1, procedure_primitive_compare_2, procedure_primitive_compare_3, procedure_primitive_compare_4, procedure_primitive_compare_n,
		procedure_primitive_compare_1a, procedure_primitive_compare_2a, procedure_primitive_compare_3a, procedure_primitive_compare_4a, procedure_primitive_compare_na);

pub fn procedure_primitive_compare_1a (_value : &ProcedurePrimitive, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn procedure_primitive_compare_2a (left : &ProcedurePrimitive, right : &ProcedurePrimitive, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}


def_fn_compare! (SyntaxPrimitive,
		syntax_primitive_compare_1, syntax_primitive_compare_2, syntax_primitive_compare_3, syntax_primitive_compare_4, syntax_primitive_compare_n,
		syntax_primitive_compare_1a, syntax_primitive_compare_2a, syntax_primitive_compare_3a, syntax_primitive_compare_4a, syntax_primitive_compare_na);

pub fn syntax_primitive_compare_1a (_value : &SyntaxPrimitive, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn syntax_primitive_compare_2a (left : &SyntaxPrimitive, right : &SyntaxPrimitive, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2 (left, right, comparison);
}


def_fn_compare! (Context,
		context_compare_1, context_compare_2, context_compare_3, context_compare_4, context_compare_n,
		context_compare_1a, context_compare_2a, context_compare_3a, context_compare_4a, context_compare_na);

pub fn context_compare_1a (_value : &Context, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn context_compare_2a (left : &Context, right : &Context, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Context::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x9296c028),
	}
}


def_fn_compare! (Binding,
		binding_compare_1, binding_compare_2, binding_compare_3, binding_compare_4, binding_compare_n,
		binding_compare_1a, binding_compare_2a, binding_compare_3a, binding_compare_4a, binding_compare_na);

pub fn binding_compare_1a (_value : &Binding, _comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true);
}

pub fn binding_compare_2a (left : &Binding, right : &Binding, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (Binding::is_self (left, right)),
		Comparison::Ordering (_, _, _) =>
			fail_unimplemented! (0x4466d4a7),
	}
}




def_fn_compare! (Value,
		number_compare_1, number_compare_2, number_compare_3, number_compare_4, number_compare_n,
		number_compare_1a, number_compare_2a, number_compare_3a, number_compare_4a, number_compare_na);

pub fn number_compare_1a (value : &Value, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		Comparison::Equivalence (_, _, _) =>
			succeed! (true),
		Comparison::Ordering (_, _, _) =>
			match value.class () {
				ValueClass::NumberInteger =>
					succeed! (true),
				ValueClass::NumberReal =>
					if NumberReal::as_ref (value) .is_nan () {
						succeed! (false);
					} else {
						succeed! (true);
					},
				_ =>
					fail! (0x0e6bfc4b),
			},
	}
}

pub fn number_compare_2a (left : &Value, right : &Value, comparison : Comparison) -> (Outcome<bool>) {
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




pub fn value_class_compare_2a_ordering (left : ValueClass, right : ValueClass, ordering : Ordering) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering (&left, &right, ordering);
}




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
					if ! try! (compare_2a (left_next, right_next, comparison_for_last)) {
						succeed! (false);
					}
				} else {
					if ! try! (compare_2a (left_next, right_next, comparison_for_non_last)) {
						if comparison_for_non_last == comparison_for_last {
							succeed! (false);
						}
						if try! (compare_2a (left_next, right_next, comparison_for_last)) {
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
						fail_unreachable! (0xee6ad119),
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
						fail_unreachable! (0xee6ad119),
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

