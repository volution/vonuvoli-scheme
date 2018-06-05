

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
use super::contexts::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_values_native" ) ]
use super::native_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
use super::native_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
use super::ports::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
use super::processes::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			Equivalence,
			Ordering,
			Comparison,
			
	};
	
	pub use super::{
			
			equivalent_by_identity_2,
			equivalent_by_value_strict_2,
			equivalent_by_value_strict_recursive_2,
			equivalent_by_value_coerced_2,
			equivalent_by_value_coerced_recursive_2,
			
	};
	
	pub use super::{
			
			compare_1,
			compare_2,
			compare_3,
			compare_4,
			compare_n,
			
			value_singleton_compare_1, value_singleton_compare_1a,
			value_singleton_compare_2, value_singleton_compare_2a,
			value_singleton_compare_3, value_singleton_compare_3a,
			value_singleton_compare_4, value_singleton_compare_4a,
			value_singleton_compare_n, value_singleton_compare_na,
			
			boolean_compare_1, boolean_compare_1a,
			boolean_compare_2, boolean_compare_2a,
			boolean_compare_3, boolean_compare_3a,
			boolean_compare_4, boolean_compare_4a,
			boolean_compare_n, boolean_compare_na,
			
			number_compare_1,
			number_compare_2,
			number_compare_3,
			number_compare_4,
			number_compare_n,
			
			number_integer_compare_1, number_integer_compare_1a,
			number_integer_compare_2, number_integer_compare_2a,
			number_integer_compare_3, number_integer_compare_3a,
			number_integer_compare_4, number_integer_compare_4a,
			number_integer_compare_n, number_integer_compare_na,
			
			number_real_compare_1, number_real_compare_1a,
			number_real_compare_2, number_real_compare_2a,
			number_real_compare_3, number_real_compare_3a,
			number_real_compare_4, number_real_compare_4a,
			number_real_compare_n, number_real_compare_na,
			
			symbol_compare_1, symbol_compare_1a,
			symbol_compare_2, symbol_compare_2a,
			symbol_compare_3, symbol_compare_3a,
			symbol_compare_4, symbol_compare_4a,
			symbol_compare_n, symbol_compare_na,
			
			pair_compare_1,
			pair_compare_2,
			pair_compare_3,
			pair_compare_4,
			pair_compare_n,
			
			pair_immutable_compare_1, pair_immutable_compare_1a,
			pair_immutable_compare_2, pair_immutable_compare_2a,
			pair_immutable_compare_3, pair_immutable_compare_3a,
			pair_immutable_compare_4, pair_immutable_compare_4a,
			pair_immutable_compare_n, pair_immutable_compare_na,
			
			procedure_primitive_compare_1, procedure_primitive_compare_1a,
			procedure_primitive_compare_2, procedure_primitive_compare_2a,
			procedure_primitive_compare_3, procedure_primitive_compare_3a,
			procedure_primitive_compare_4, procedure_primitive_compare_4a,
			procedure_primitive_compare_n, procedure_primitive_compare_na,
			
			syntax_primitive_compare_1, syntax_primitive_compare_1a,
			syntax_primitive_compare_2, syntax_primitive_compare_2a,
			syntax_primitive_compare_3, syntax_primitive_compare_3a,
			syntax_primitive_compare_4, syntax_primitive_compare_4a,
			syntax_primitive_compare_n, syntax_primitive_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			pair_mutable_compare_1, pair_mutable_compare_1a,
			pair_mutable_compare_2, pair_mutable_compare_2a,
			pair_mutable_compare_3, pair_mutable_compare_3a,
			pair_mutable_compare_4, pair_mutable_compare_4a,
			pair_mutable_compare_n, pair_mutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
			
			character_compare_1, character_compare_1a,
			character_compare_2, character_compare_2a,
			character_compare_3, character_compare_3a,
			character_compare_4, character_compare_4a,
			character_compare_n, character_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
			
			string_compare_1,
			string_compare_2,
			string_compare_3,
			string_compare_4,
			string_compare_n,
			
			string_immutable_compare_1, string_immutable_compare_1a,
			string_immutable_compare_2, string_immutable_compare_2a,
			string_immutable_compare_3, string_immutable_compare_3a,
			string_immutable_compare_4, string_immutable_compare_4a,
			string_immutable_compare_n, string_immutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			string_mutable_compare_1, string_mutable_compare_1a,
			string_mutable_compare_2, string_mutable_compare_2a,
			string_mutable_compare_3, string_mutable_compare_3a,
			string_mutable_compare_4, string_mutable_compare_4a,
			string_mutable_compare_n, string_mutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
			
			bytes_compare_1,
			bytes_compare_2,
			bytes_compare_3,
			bytes_compare_4,
			bytes_compare_n,
			
			bytes_immutable_compare_1, bytes_immutable_compare_1a,
			bytes_immutable_compare_2, bytes_immutable_compare_2a,
			bytes_immutable_compare_3, bytes_immutable_compare_3a,
			bytes_immutable_compare_4, bytes_immutable_compare_4a,
			bytes_immutable_compare_n, bytes_immutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			bytes_mutable_compare_1, bytes_mutable_compare_1a,
			bytes_mutable_compare_2, bytes_mutable_compare_2a,
			bytes_mutable_compare_3, bytes_mutable_compare_3a,
			bytes_mutable_compare_4, bytes_mutable_compare_4a,
			bytes_mutable_compare_n, bytes_mutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::{
			
			array_compare_1,
			array_compare_2,
			array_compare_3,
			array_compare_4,
			array_compare_n,
			
			array_immutable_compare_1, array_immutable_compare_1a,
			array_immutable_compare_2, array_immutable_compare_2a,
			array_immutable_compare_3, array_immutable_compare_3a,
			array_immutable_compare_4, array_immutable_compare_4a,
			array_immutable_compare_n, array_immutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			array_mutable_compare_1, array_mutable_compare_1a,
			array_mutable_compare_2, array_mutable_compare_2a,
			array_mutable_compare_3, array_mutable_compare_3a,
			array_mutable_compare_4, array_mutable_compare_4a,
			array_mutable_compare_n, array_mutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::{
			
			values_compare_1, values_compare_1a,
			values_compare_2, values_compare_2a,
			values_compare_3, values_compare_3a,
			values_compare_4, values_compare_4a,
			values_compare_n, values_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::{
			
			procedure_lambda_compare_1, procedure_lambda_compare_1a,
			procedure_lambda_compare_2, procedure_lambda_compare_2a,
			procedure_lambda_compare_3, procedure_lambda_compare_3a,
			procedure_lambda_compare_4, procedure_lambda_compare_4a,
			procedure_lambda_compare_n, procedure_lambda_compare_na,
			
	};
	
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::{
			
			syntax_lambda_compare_1, syntax_lambda_compare_1a,
			syntax_lambda_compare_2, syntax_lambda_compare_2a,
			syntax_lambda_compare_3, syntax_lambda_compare_3a,
			syntax_lambda_compare_4, syntax_lambda_compare_4a,
			syntax_lambda_compare_n, syntax_lambda_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::{
			
			procedure_extended_compare_1, procedure_extended_compare_1a,
			procedure_extended_compare_2, procedure_extended_compare_2a,
			procedure_extended_compare_3, procedure_extended_compare_3a,
			procedure_extended_compare_4, procedure_extended_compare_4a,
			procedure_extended_compare_n, procedure_extended_compare_na,
			
	};
	
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::{
			
			syntax_extended_compare_1, syntax_extended_compare_1a,
			syntax_extended_compare_2, syntax_extended_compare_2a,
			syntax_extended_compare_3, syntax_extended_compare_3a,
			syntax_extended_compare_4, syntax_extended_compare_4a,
			syntax_extended_compare_n, syntax_extended_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::{
			
			procedure_native_compare_1, procedure_native_compare_1a,
			procedure_native_compare_2, procedure_native_compare_2a,
			procedure_native_compare_3, procedure_native_compare_3a,
			procedure_native_compare_4, procedure_native_compare_4a,
			procedure_native_compare_n, procedure_native_compare_na,
			
	};
	
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::{
			
			syntax_native_compare_1, syntax_native_compare_1a,
			syntax_native_compare_2, syntax_native_compare_2a,
			syntax_native_compare_3, syntax_native_compare_3a,
			syntax_native_compare_4, syntax_native_compare_4a,
			syntax_native_compare_n, syntax_native_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	pub use super::{
			
			error_compare_1, error_compare_1a,
			error_compare_2, error_compare_2a,
			error_compare_3, error_compare_3a,
			error_compare_4, error_compare_4a,
			error_compare_n, error_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	pub use super::{
			
			keyword_compare_1, keyword_compare_1a,
			keyword_compare_2, keyword_compare_2a,
			keyword_compare_3, keyword_compare_3a,
			keyword_compare_4, keyword_compare_4a,
			keyword_compare_n, keyword_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	pub use super::{
			
			unique_compare_1, unique_compare_1a,
			unique_compare_2, unique_compare_2a,
			unique_compare_3, unique_compare_3a,
			unique_compare_4, unique_compare_4a,
			unique_compare_n, unique_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	pub use super::{
			
			opaque_compare_1, opaque_compare_1a,
			opaque_compare_2, opaque_compare_2a,
			opaque_compare_3, opaque_compare_3a,
			opaque_compare_4, opaque_compare_4a,
			opaque_compare_n, opaque_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	pub use super::{
			
			context_compare_1, context_compare_1a,
			context_compare_2, context_compare_2a,
			context_compare_3, context_compare_3a,
			context_compare_4, context_compare_4a,
			context_compare_n, context_compare_na,
			
			binding_compare_1, binding_compare_1a,
			binding_compare_2, binding_compare_2a,
			binding_compare_3, binding_compare_3a,
			binding_compare_4, binding_compare_4a,
			binding_compare_n, binding_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	pub use super::{
			
			record_kind_compare_1, record_kind_compare_1a,
			record_kind_compare_2, record_kind_compare_2a,
			record_kind_compare_3, record_kind_compare_3a,
			record_kind_compare_4, record_kind_compare_4a,
			record_kind_compare_n, record_kind_compare_na,
			
			record_compare_1,
			record_compare_2,
			record_compare_3,
			record_compare_4,
			record_compare_n,
			
			record_immutable_compare_1, record_immutable_compare_1a,
			record_immutable_compare_2, record_immutable_compare_2a,
			record_immutable_compare_3, record_immutable_compare_3a,
			record_immutable_compare_4, record_immutable_compare_4a,
			record_immutable_compare_n, record_immutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			record_mutable_compare_1, record_mutable_compare_1a,
			record_mutable_compare_2, record_mutable_compare_2a,
			record_mutable_compare_3, record_mutable_compare_3a,
			record_mutable_compare_4, record_mutable_compare_4a,
			record_mutable_compare_n, record_mutable_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	pub use super::{
			
			parameters_compare_1, parameters_compare_1a,
			parameters_compare_2, parameters_compare_2a,
			parameters_compare_3, parameters_compare_3a,
			parameters_compare_4, parameters_compare_4a,
			parameters_compare_n, parameters_compare_na,
			
			parameter_compare_1, parameter_compare_1a,
			parameter_compare_2, parameter_compare_2a,
			parameter_compare_3, parameter_compare_3a,
			parameter_compare_4, parameter_compare_4a,
			parameter_compare_n, parameter_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	pub use super::{
			
			promise_compare_1, promise_compare_1a,
			promise_compare_2, promise_compare_2a,
			promise_compare_3, promise_compare_3a,
			promise_compare_4, promise_compare_4a,
			promise_compare_n, promise_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
			
			string_regex_compare_1, string_regex_compare_1a,
			string_regex_compare_2, string_regex_compare_2a,
			string_regex_compare_3, string_regex_compare_3a,
			string_regex_compare_4, string_regex_compare_4a,
			string_regex_compare_n, string_regex_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
			
			bytes_regex_compare_1, bytes_regex_compare_1a,
			bytes_regex_compare_2, bytes_regex_compare_2a,
			bytes_regex_compare_3, bytes_regex_compare_3a,
			bytes_regex_compare_4, bytes_regex_compare_4a,
			bytes_regex_compare_n, bytes_regex_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::{
			
			port_compare_1, port_compare_1a,
			port_compare_2, port_compare_2a,
			port_compare_3, port_compare_3a,
			port_compare_4, port_compare_4a,
			port_compare_n, port_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::{
			
			path_compare_1, path_compare_1a,
			path_compare_2, path_compare_2a,
			path_compare_3, path_compare_3a,
			path_compare_4, path_compare_4a,
			path_compare_n, path_compare_na,
			
	};
	
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	pub use super::{
			
			process_compare_1, process_compare_1a,
			process_compare_2, process_compare_2a,
			process_compare_3, process_compare_3a,
			process_compare_4, process_compare_4a,
			process_compare_n, process_compare_na,
			
	};
	
	
	pub use super::{
			
			vec_compare_2,
			
	};
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum Equivalence {
	
	ByIdentity,
	ByValue,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum Ordering {
	
	Lesser,
	LesserOrEqual,
	Equal,
	GreaterOrEqual,
	Greater,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum Comparison {
	Equivalence ( Equivalence, Option<bool>, Option<bool>, bool ), // ( equivalence, coercion, recursive, negated )
	Ordering ( Ordering, Option<bool>, Option<bool>, bool ), // ( ordering, case_sensitivity, recursive, negated )
}


impl Comparison {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_aggregated (&self, last : bool) -> (Comparison) {
		match *self {
			
			Comparison::Equivalence (equivalence, coercion, recursive, negated) =>
				match equivalence {
					Equivalence::ByIdentity =>
						Comparison::Equivalence (Equivalence::ByIdentity, coercion, Some (false), negated),
					Equivalence::ByValue =>
						match recursive {
							None | Some (false) =>
								Comparison::Equivalence (Equivalence::ByIdentity, coercion, Some (false), negated),
							Some (true) =>
								Comparison::Equivalence (Equivalence::ByValue, coercion, Some (true), negated),
						},
				},
			
			Comparison::Ordering (ordering, case_sensitivity, recursive, negated) => {
				let ordering = if ! last {
					Ordering::Equal
				} else {
					ordering
				};
				match recursive {
					None | Some (false) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (false), negated),
					Some (true) =>
						Comparison::Ordering (ordering, case_sensitivity, Some (true), negated),
				}
			},
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn negated (&self) -> (bool) {
		match *self {
			Comparison::Equivalence (_, _, _, negated) =>
				negated,
			Comparison::Ordering (_, _, _, negated) =>
				negated,
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_identity_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, negated : bool) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_strict_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, negated : bool) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_strict_recursive_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, negated : bool) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_coerced_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, negated : bool) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn equivalent_by_value_coerced_recursive_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, negated : bool) -> (Outcome<bool>) {
	return compare_2 (left, right, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated));
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
			let negated = comparison.negated ();
			if ! try! ($compare_2 (input_1, input_2, comparison)) ^ negated {
				succeed! (false ^ negated);
			}
			return $compare_2 (input_2, input_3, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_4 <ValueRef : StdAsRef<$type>> (input_1 : ValueRef, input_2 : ValueRef, input_3 : ValueRef, input_4 : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
			let input_1 = input_1.as_ref ();
			let input_2 = input_2.as_ref ();
			let input_3 = input_3.as_ref ();
			let input_4 = input_4.as_ref ();
			let negated = comparison.negated ();
			if ! try! ($compare_2 (input_1, input_2, comparison)) ^ negated {
				succeed! (false ^ negated);
			}
			if ! try! ($compare_2 (input_2, input_3, comparison)) ^ negated {
				succeed! (false ^ negated);
			}
			return $compare_2 (input_3, input_4, comparison);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $compare_n <ValueRef : StdAsRef<$type>> (inputs : &[ValueRef], comparison : Comparison) -> (Outcome<bool>) {
			let inputs_count = inputs.len ();
			let negated = comparison.negated ();
			match inputs_count {
				0 =>
					succeed! (true ^ negated),
				1 =>
					return $compare_1 (inputs[0].as_ref (), comparison),
				_ =>
					(),
			}
			let mut inputs_iterator = inputs.iter ();
			let mut input_previous = try_some_or_panic! (inputs_iterator.next (), 0xe68d235b, github_issue_new);
			for input_current in inputs_iterator {
				if ! try! ($compare_2 (input_previous, input_current, comparison)) ^ negated {
					succeed! (false ^ negated);
				}
				input_previous = input_current;
			}
			succeed! (true ^ negated);
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
			let negated = comparison.negated ();
			match inputs_count {
				0 =>
					succeed! (true ^ negated),
				1 =>
					return $compare_1 (inputs[0].as_ref (), comparison),
				_ =>
					(),
			}
			TODO! ("try to eliminate extra vector creation");
			let inputs = try! (inputs.iter () .map (|input| StdTryAsRef0::<$type>::try_as_ref_0 (input.as_ref ())) .collect::<Outcome<StdVec<_>>> ());
			let mut inputs_iterator = inputs.iter ();
			let mut input_previous = try_some_or_panic! (inputs_iterator.next (), 0x47173388, github_issue_new);
			for input_current in inputs_iterator {
				if ! try! ($compare_2a (input_previous, input_current, comparison)) ^ negated {
					succeed! (false ^ negated);
				}
				input_previous = input_current;
			}
			succeed! (true ^ negated);
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::Character (value) =>
			return character_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::Symbol (value) =>
			return symbol_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueKindMatchAsRef::Keyword (value) =>
			return keyword_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ValueKindMatchAsRef::Unique (value) =>
			return unique_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringImmutable (value) =>
			return string_immutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::StringMutable (value) =>
			return string_mutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesImmutable (value) =>
			return bytes_immutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::BytesMutable (value) =>
			return bytes_mutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringRegex (value) =>
			return string_regex_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesRegex (value) =>
			return bytes_regex_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::PairImmutable (value) =>
			return pair_immutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::PairMutable (value) =>
			return pair_mutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueKindMatchAsRef::ArrayImmutable (value) =>
			return array_immutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::ArrayMutable (value) =>
			return array_mutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueKindMatchAsRef::Values (value) =>
			return values_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueKindMatchAsRef::RecordKind (value) =>
			return record_kind_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueKindMatchAsRef::RecordImmutable (value) =>
			return record_immutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::RecordMutable (value) =>
			return record_mutable_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		ValueKindMatchAsRef::Error (value) =>
			return error_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::ProcedurePrimitive (value) =>
			return procedure_primitive_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		ValueKindMatchAsRef::ProcedureExtended (value) =>
			return procedure_extended_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_native" ) ]
		ValueKindMatchAsRef::ProcedureNative (value) =>
			return procedure_native_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
		ValueKindMatchAsRef::ProcedureLambda (value) =>
			return procedure_lambda_compare_1a (value, comparison),
		
		ValueKindMatchAsRef::SyntaxPrimitive (value) =>
			return syntax_primitive_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		ValueKindMatchAsRef::SyntaxExtended (value) =>
			return syntax_extended_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_native" ) ]
		ValueKindMatchAsRef::SyntaxNative (value) =>
			return syntax_native_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
		ValueKindMatchAsRef::SyntaxLambda (value) =>
			return syntax_lambda_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef::Path (value) =>
			return path_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ValueKindMatchAsRef::Port (value) =>
			return port_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		ValueKindMatchAsRef::Process (value) =>
			return process_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		ValueKindMatchAsRef::Context (value) =>
			return context_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		ValueKindMatchAsRef::Binding (value) =>
			return binding_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		ValueKindMatchAsRef::Parameters (value) =>
			return parameters_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		ValueKindMatchAsRef::Parameter (value) =>
			return parameter_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		ValueKindMatchAsRef::Promise (value) =>
			return promise_compare_1a (value, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
		ValueKindMatchAsRef::Opaque (value) =>
			return opaque_compare_1a (value, comparison),
		
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef2::Character (left, right) =>
			return character_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Symbol (left, right) =>
			return symbol_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueKindMatchAsRef2::Keyword (left, right) =>
			return keyword_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ValueKindMatchAsRef2::Unique (left, right) =>
			return unique_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef2::StringImmutable (left, right) =>
			return string_immutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef2::StringMutable (left, right) =>
			return string_mutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef2::BytesImmutable (left, right) =>
			return bytes_immutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef2::BytesMutable (left, right) =>
			return bytes_mutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef2::StringRegex (left, right) =>
			return string_regex_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef2::BytesRegex (left, right) =>
			return bytes_regex_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::PairImmutable (left, right) =>
			return pair_immutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef2::PairMutable (left, right) =>
			return pair_mutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueKindMatchAsRef2::ArrayImmutable (left, right) =>
			return array_immutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef2::ArrayMutable (left, right) =>
			return array_mutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueKindMatchAsRef2::Values (left, right) =>
			return values_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueKindMatchAsRef2::RecordKind (left, right) =>
			return record_kind_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueKindMatchAsRef2::RecordImmutable (left, right) =>
			return record_immutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef2::RecordMutable (left, right) =>
			return record_mutable_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		ValueKindMatchAsRef2::Error (left, right) =>
			return error_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::ProcedurePrimitive (left, right) =>
			return procedure_primitive_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		ValueKindMatchAsRef2::ProcedureExtended (left, right) =>
			return procedure_extended_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_native" ) ]
		ValueKindMatchAsRef2::ProcedureNative (left, right) =>
			return procedure_native_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
		ValueKindMatchAsRef2::ProcedureLambda (left, right) =>
			return procedure_lambda_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::SyntaxPrimitive (left, right) =>
			return syntax_primitive_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		ValueKindMatchAsRef2::SyntaxExtended (left, right) =>
			return syntax_extended_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_native" ) ]
		ValueKindMatchAsRef2::SyntaxNative (left, right) =>
			return syntax_native_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		#[ cfg ( feature = "vonuvoli_compiler" ) ]
		#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
		ValueKindMatchAsRef2::SyntaxLambda (left, right) =>
			return syntax_lambda_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef2::Path (left, right) =>
			return path_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ValueKindMatchAsRef2::Port (left, right) =>
			return port_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		ValueKindMatchAsRef2::Process (left, right) =>
			return process_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		ValueKindMatchAsRef2::Context (left, right) =>
			return context_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		ValueKindMatchAsRef2::Binding (left, right) =>
			return binding_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		ValueKindMatchAsRef2::Parameters (left, right) =>
			return parameters_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		ValueKindMatchAsRef2::Parameter (left, right) =>
			return parameter_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		ValueKindMatchAsRef2::Promise (left, right) =>
			return promise_compare_2a (left, right, comparison),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
		ValueKindMatchAsRef2::Opaque (left, right) =>
			return opaque_compare_2a (left, right, comparison),
		
		ValueKindMatchAsRef2::Missmatched =>
			match Value::class_match_as_ref_2 (left, right) {
				
				ValueClassMatchAsRef2::Number (ref class) =>
					return number_match_as_ref_compare_2a (class, comparison),
				
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ValueClassMatchAsRef2::String (ref class) => {
					let (left, right) = try! (class.string_ref ());
					return string_ref_compare_2a (&left, &right, comparison);
				},
				
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ValueClassMatchAsRef2::Bytes (ref class) => {
					let (left, right) = try! (class.bytes_ref ());
					return bytes_ref_compare_2a (&left, &right, comparison);
				},
				
				ValueClassMatchAsRef2::Pair (ref class) => {
					let (left, right) = try! (class.pair_ref ());
					return pair_ref_compare_2a (&left, &right, comparison);
				},
				
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ValueClassMatchAsRef2::Array (ref class) => {
					let (left, right) = try! (class.array_ref ());
					return array_ref_compare_2a (&left, &right, comparison);
				},
				
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ValueClassMatchAsRef2::Record (ref class) => {
					let (left, right) = try! (class.record_ref ());
					return record_ref_compare_2a (&left, &right, comparison);
				},
				
				_ =>
					match comparison {
						Comparison::Equivalence (_, _, _, negated) =>
							succeed! (false ^ negated),
						Comparison::Ordering (ordering, _, _, negated) => {
							let left_kind = left.kind ();
							let right_kind = right.kind ();
							return value_kind_compare_2a_ordering (left_kind, right_kind, ordering, negated);
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
				Comparison::Equivalence (_, _, _, negated) =>
					succeed! (true ^ negated),
				Comparison::Ordering (_, _, _, _) =>
					fail! (0x534ee60c),
			},
		_ =>
			succeed! (true ^ comparison.negated ()),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn value_singleton_compare_2a <ValueRef : StdAsRef<ValueSingleton>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match (*left, *right) {
		(ValueSingleton::Undefined, ValueSingleton::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _, negated) =>
					succeed! (true ^ negated),
				Comparison::Ordering (_, _, _, _) =>
					fail! (0xec7931c0),
			},
		(ValueSingleton::Undefined, _) =>
			match comparison {
				Comparison::Equivalence (_, _, _, negated) =>
					succeed! (false ^ negated),
				Comparison::Ordering (_, _, _, _) =>
					fail! (0xa7c9f145),
			},
		(_, ValueSingleton::Undefined) =>
			match comparison {
				Comparison::Equivalence (_, _, _, negated) =>
					succeed! (false ^ negated),
				Comparison::Ordering (_, _, _, _) =>
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
pub fn boolean_compare_1a <ValueRef : StdAsRef<Boolean>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn boolean_compare_2a <ValueRef : StdAsRef<Boolean>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}




def_fn_compare! (NumberInteger,
		number_integer_compare_1, number_integer_compare_2, number_integer_compare_3, number_integer_compare_4, number_integer_compare_n,
		number_integer_compare_1a, number_integer_compare_2a, number_integer_compare_3a, number_integer_compare_4a, number_integer_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_integer_compare_1a <ValueRef : StdAsRef<NumberInteger>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
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
			Comparison::Equivalence (_, _, _, negated) =>
				succeed! (true ^ negated),
			Comparison::Ordering (_, _, _, negated) =>
				succeed! (false ^ negated),
		}
	} else {
		succeed! (true ^ comparison.negated ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_real_compare_2a <ValueRef : StdAsRef<NumberReal>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .value ();
	let right = right.as_ref () .value ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			if left.is_nan () && right.is_nan () {
				succeed! (true ^ negated);
			} else {
				succeed! (f64::eq (&left, &right) ^ negated);
			},
		Comparison::Ordering (ordering, _, _, negated) =>
			if left.is_nan () || right.is_nan () {
				succeed! (false ^ negated);
			} else {
				return std_ord_compare_2_ordering_val (left, right, ordering, negated);
			},
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
def_fn_compare! (Character,
		character_compare_1, character_compare_2, character_compare_3, character_compare_4, character_compare_n,
		character_compare_1a, character_compare_2a, character_compare_3a, character_compare_4a, character_compare_na);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_compare_1a <ValueRef : StdAsRef<Character>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_compare_2a <ValueRef : StdAsRef<Character>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .value ();
	let right = right.as_ref () .value ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (char::eq (&left, &right) ^ negated),
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_val (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0xea3c51f1, (github_issue, 35)),
			},
	}
}




def_fn_compare! (Symbol,
		symbol_compare_1, symbol_compare_2, symbol_compare_3, symbol_compare_4, symbol_compare_n,
		symbol_compare_1a, symbol_compare_2a, symbol_compare_3a, symbol_compare_4a, symbol_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_compare_1a <ValueRef : StdAsRef<Symbol>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_compare_2a <ValueRef : StdAsRef<Symbol>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (Symbol::eq (left, right) ^ negated),
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_val (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0xc4ef7065, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
def_fn_compare! (Keyword,
		keyword_compare_1, keyword_compare_2, keyword_compare_3, keyword_compare_4, keyword_compare_n,
		keyword_compare_1a, keyword_compare_2a, keyword_compare_3a, keyword_compare_4a, keyword_compare_na);

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_compare_1a <ValueRef : StdAsRef<Keyword>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_compare_2a <ValueRef : StdAsRef<Keyword>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (Keyword::eq (left, right) ^ negated),
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_val (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0x3a2cf6b7, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_values_unique" ) ]
def_fn_compare! (Unique,
		unique_compare_1, unique_compare_2, unique_compare_3, unique_compare_4, unique_compare_n,
		unique_compare_1a, unique_compare_2a, unique_compare_3a, unique_compare_4a, unique_compare_na);

#[ cfg ( feature = "vonuvoli_values_unique" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unique_compare_1a <ValueRef : StdAsRef<Unique>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_unique" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unique_compare_2a <ValueRef : StdAsRef<Unique>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (Unique::eq (left, right) ^ negated),
		Comparison::Ordering (ordering, _, _, negated) =>
			return std_ord_compare_2_ordering_val (left, right, ordering, negated),
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
def_fn_compare! (StringImmutable,
		string_immutable_compare_1, string_immutable_compare_2, string_immutable_compare_3, string_immutable_compare_4, string_immutable_compare_n,
		string_immutable_compare_1a, string_immutable_compare_2a, string_immutable_compare_3a, string_immutable_compare_4a, string_immutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_compare_1a <ValueRef : StdAsRef<StringImmutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_compare_2a <ValueRef : StdAsRef<StringImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .string_ref ();
	let right = right.as_ref () .string_ref ();
	return string_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
def_fn_compare! (StringMutable,
		string_mutable_compare_1, string_mutable_compare_2, string_mutable_compare_3, string_mutable_compare_4, string_mutable_compare_n,
		string_mutable_compare_1a, string_mutable_compare_2a, string_mutable_compare_3a, string_mutable_compare_4a, string_mutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_compare_1a <ValueRef : StdAsRef<StringMutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_compare_2a <ValueRef : StdAsRef<StringMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (left.as_ref () .string_ref ());
	let right = try! (right.as_ref () .string_ref ());
	return string_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn string_ref_compare_2a <'a, ValueRef : StdAsRef<StringRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (StringRef::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (StringRef::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0x2736b1f6, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
def_fn_compare! (BytesImmutable,
		bytes_immutable_compare_1, bytes_immutable_compare_2, bytes_immutable_compare_3, bytes_immutable_compare_4, bytes_immutable_compare_n,
		bytes_immutable_compare_1a, bytes_immutable_compare_2a, bytes_immutable_compare_3a, bytes_immutable_compare_4a, bytes_immutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_compare_1a <ValueRef : StdAsRef<BytesImmutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_compare_2a <ValueRef : StdAsRef<BytesImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .bytes_ref ();
	let right = right.as_ref () .bytes_ref ();
	return bytes_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
def_fn_compare! (BytesMutable,
		bytes_mutable_compare_1, bytes_mutable_compare_2, bytes_mutable_compare_3, bytes_mutable_compare_4, bytes_mutable_compare_n,
		bytes_mutable_compare_1a, bytes_mutable_compare_2a, bytes_mutable_compare_3a, bytes_mutable_compare_4a, bytes_mutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_compare_1a <ValueRef : StdAsRef<BytesMutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_compare_2a <ValueRef : StdAsRef<BytesMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (left.as_ref () .bytes_ref ());
	let right = try! (right.as_ref () .bytes_ref ());
	return bytes_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn bytes_ref_compare_2a <'a, ValueRef : StdAsRef<BytesRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (BytesRef::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (BytesRef::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0x5be9ea91, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
def_fn_compare! (StringRegex,
		string_regex_compare_1, string_regex_compare_2, string_regex_compare_3, string_regex_compare_4, string_regex_compare_n,
		string_regex_compare_1a, string_regex_compare_2a, string_regex_compare_3a, string_regex_compare_4a, string_regex_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_compare_1a <ValueRef : StdAsRef<StringRegex>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_compare_2a <ValueRef : StdAsRef<StringRegex>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (StringRegex::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (StringRegex::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0x86b3116e, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
def_fn_compare! (BytesRegex,
		bytes_regex_compare_1, bytes_regex_compare_2, bytes_regex_compare_3, bytes_regex_compare_4, bytes_regex_compare_n,
		bytes_regex_compare_1a, bytes_regex_compare_2a, bytes_regex_compare_3a, bytes_regex_compare_4a, bytes_regex_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_compare_1a <ValueRef : StdAsRef<BytesRegex>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_compare_2a <ValueRef : StdAsRef<BytesRegex>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (BytesRegex::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (BytesRegex::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref (left, right, ordering, negated),
				_ =>
					fail_unimplemented! (0x3d0884f6, (github_issue, 35)),
			},
	}
}




def_fn_compare! (PairImmutable,
		pair_immutable_compare_1, pair_immutable_compare_2, pair_immutable_compare_3, pair_immutable_compare_4, pair_immutable_compare_n,
		pair_immutable_compare_1a, pair_immutable_compare_2a, pair_immutable_compare_3a, pair_immutable_compare_4a, pair_immutable_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_compare_1a <ValueRef : StdAsRef<PairImmutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_compare_2a <ValueRef : StdAsRef<PairImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .pair_ref ();
	let right = right.as_ref () .pair_ref ();
	return pair_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
def_fn_compare! (PairMutable,
		pair_mutable_compare_1, pair_mutable_compare_2, pair_mutable_compare_3, pair_mutable_compare_4, pair_mutable_compare_n,
		pair_mutable_compare_1a, pair_mutable_compare_2a, pair_mutable_compare_3a, pair_mutable_compare_4a, pair_mutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_compare_1a <ValueRef : StdAsRef<PairMutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_compare_2a <ValueRef : StdAsRef<PairMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (left.as_ref () .pair_ref ());
	let right = try! (right.as_ref () .pair_ref ());
	return pair_ref_compare_2a (&left, &right, comparison);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn pair_ref_compare_2a <'a, ValueRef : StdAsRef<PairRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	{ if false {
		// NOTE:  This is a semantically equivalent implementation;  although less efficient!
		match comparison {
			Comparison::Equivalence (equivalence, _, _, negated) =>
				match equivalence {
					Equivalence::ByIdentity =>
						succeed! (PairRef::value_is_self (left, right) ^ negated),
					Equivalence::ByValue =>
						return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
				},
			Comparison::Ordering (_, _, _, _) =>
				return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
		}
	} }
	
	match comparison {
		
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (PairRef::value_is_self (left, right) ^ negated),
				Equivalence::ByValue => {
					let comparison = comparison.for_aggregated (false);
					if ! negated {
						succeed! (
								try! (compare_2 (left.left (), right.left (), comparison)) &&
								try! (compare_2 (left.right (), right.right (), comparison)));
					} else {
						succeed! (
								try! (compare_2 (left.left (), right.left (), comparison)) ||
								try! (compare_2 (left.right (), right.right (), comparison)));
					}
				},
			},
		
		Comparison::Ordering (_, _, _, negated) => {
			let comparison_for_last = comparison.for_aggregated (true);
			let comparison_for_non_last = comparison.for_aggregated (false);
			
			if ! try! (compare_2 (left.left (), right.left (), comparison_for_non_last)) ^ negated {
				if comparison_for_last == comparison_for_non_last {
					succeed! (false ^ negated);
				} else {
					if ! try! (compare_2 (left.left (), right.left (), comparison_for_last)) ^ negated {
						succeed! (false ^ negated);
					} else {
						succeed! (true ^ negated);
					}
				}
			}
			
			if ! try! (compare_2 (left.right (), right.right (), comparison_for_last)) ^ negated {
				succeed! (false ^ negated);
			} else {
				succeed! (true ^ negated);
			}
			
		},
		
	}
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
def_fn_compare! (ArrayImmutable,
		array_immutable_compare_1, array_immutable_compare_2, array_immutable_compare_3, array_immutable_compare_4, array_immutable_compare_n,
		array_immutable_compare_1a, array_immutable_compare_2a, array_immutable_compare_3a, array_immutable_compare_4a, array_immutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_compare_1a <ValueRef : StdAsRef<ArrayImmutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_compare_2a <ValueRef : StdAsRef<ArrayImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .array_ref ();
	let right = right.as_ref () .array_ref ();
	return array_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
def_fn_compare! (ArrayMutable,
		array_mutable_compare_1, array_mutable_compare_2, array_mutable_compare_3, array_mutable_compare_4, array_mutable_compare_n,
		array_mutable_compare_1a, array_mutable_compare_2a, array_mutable_compare_3a, array_mutable_compare_4a, array_mutable_compare_na);

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_compare_1a <ValueRef : StdAsRef<ArrayMutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_compare_2a <ValueRef : StdAsRef<ArrayMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (left.as_ref () .array_ref ());
	let right = try! (right.as_ref () .array_ref ());
	return array_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn array_ref_compare_2a <'a, ValueRef : StdAsRef<ArrayRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (ArrayRef::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
			},
		Comparison::Ordering (_, _, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
	}
}




#[ cfg ( feature = "vonuvoli_values_values" ) ]
def_fn_compare! (Values,
		values_compare_1, values_compare_2, values_compare_3, values_compare_4, values_compare_n,
		values_compare_1a, values_compare_2a, values_compare_3a, values_compare_4a, values_compare_na);

#[ cfg ( feature = "vonuvoli_values_values" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_compare_1a <ValueRef : StdAsRef<Values>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_values" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_compare_2a <ValueRef : StdAsRef<Values>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Values::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
			},
		Comparison::Ordering (_, _, _, _) =>
			return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
def_fn_compare! (RecordKind,
		record_kind_compare_1, record_kind_compare_2, record_kind_compare_3, record_kind_compare_4, record_kind_compare_n,
		record_kind_compare_1a, record_kind_compare_2a, record_kind_compare_3a, record_kind_compare_4a, record_kind_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_compare_1a <ValueRef : StdAsRef<RecordKind>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_compare_2a <ValueRef : StdAsRef<RecordKind>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (RecordKind::is_self (left, right) ^ negated),
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
def_fn_compare! (RecordImmutable,
		record_immutable_compare_1, record_immutable_compare_2, record_immutable_compare_3, record_immutable_compare_4, record_immutable_compare_n,
		record_immutable_compare_1a, record_immutable_compare_2a, record_immutable_compare_3a, record_immutable_compare_4a, record_immutable_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_immutable_compare_1a <ValueRef : StdAsRef<RecordImmutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_immutable_compare_2a <ValueRef : StdAsRef<RecordImmutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref () .record_ref ();
	let right = right.as_ref () .record_ref ();
	return record_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
def_fn_compare! (RecordMutable,
		record_mutable_compare_1, record_mutable_compare_2, record_mutable_compare_3, record_mutable_compare_4, record_mutable_compare_n,
		record_mutable_compare_1a, record_mutable_compare_2a, record_mutable_compare_3a, record_mutable_compare_4a, record_mutable_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_mutable_compare_1a <ValueRef : StdAsRef<RecordMutable>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_mutable_compare_2a <ValueRef : StdAsRef<RecordMutable>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (left.as_ref () .record_ref ());
	let right = try! (right.as_ref () .record_ref ());
	return record_ref_compare_2a (&left, &right, comparison);
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn record_ref_compare_2a <'a, ValueRef : StdAsRef<RecordRef<'a>>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (RecordRef::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					if RecordKind::is_self (left.kind (), right.kind ()) {
						return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison);
					} else {
						succeed! (false ^ negated);
					},
			},
		Comparison::Ordering (_, _, _, _) => {
			let left_kind = left.kind ();
			let right_kind = right.kind ();
			if RecordKind::is_self (left_kind, right_kind) {
				return vec_compare_2 (left.values_as_slice (), right.values_as_slice (), comparison);
			} else {
				return std_ord_compare_2_ref (left_kind, right_kind, comparison);
			}
		},
	}
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
def_fn_compare! (Error,
		error_compare_1, error_compare_2, error_compare_3, error_compare_4, error_compare_n,
		error_compare_1a, error_compare_2a, error_compare_3a, error_compare_4a, error_compare_na);

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_compare_1a <ValueRef : StdAsRef<Error>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_compare_2a <ValueRef : StdAsRef<Error>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Error::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Error::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, _, _, negated) =>
			return std_ord_compare_2_ordering_val (left.code (), right.code (), ordering, negated),
	}
}




def_fn_compare! (ProcedurePrimitive,
		procedure_primitive_compare_1, procedure_primitive_compare_2, procedure_primitive_compare_3, procedure_primitive_compare_4, procedure_primitive_compare_n,
		procedure_primitive_compare_1a, procedure_primitive_compare_2a, procedure_primitive_compare_3a, procedure_primitive_compare_4a, procedure_primitive_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_primitive_compare_1a <ValueRef : StdAsRef<ProcedurePrimitive>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_primitive_compare_2a <ValueRef : StdAsRef<ProcedurePrimitive>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}


#[ cfg ( feature = "vonuvoli_values_extended" ) ]
def_fn_compare! (ProcedureExtended,
		procedure_extended_compare_1, procedure_extended_compare_2, procedure_extended_compare_3, procedure_extended_compare_4, procedure_extended_compare_n,
		procedure_extended_compare_1a, procedure_extended_compare_2a, procedure_extended_compare_3a, procedure_extended_compare_4a, procedure_extended_compare_na);

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_compare_1a <ValueRef : StdAsRef<ProcedureExtended>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_compare_2a <ValueRef : StdAsRef<ProcedureExtended>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (ProcedureExtended::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (ProcedureExtended::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


#[ cfg ( feature = "vonuvoli_values_native" ) ]
def_fn_compare! (ProcedureNative,
		procedure_native_compare_1, procedure_native_compare_2, procedure_native_compare_3, procedure_native_compare_4, procedure_native_compare_n,
		procedure_native_compare_1a, procedure_native_compare_2a, procedure_native_compare_3a, procedure_native_compare_4a, procedure_native_compare_na);

#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_native_compare_1a <ValueRef : StdAsRef<ProcedureNative>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_native_compare_2a <ValueRef : StdAsRef<ProcedureNative>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (ProcedureNative::is_self (left, right) ^ negated),
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
def_fn_compare! (ProcedureLambda,
		procedure_lambda_compare_1, procedure_lambda_compare_2, procedure_lambda_compare_3, procedure_lambda_compare_4, procedure_lambda_compare_n,
		procedure_lambda_compare_1a, procedure_lambda_compare_2a, procedure_lambda_compare_3a, procedure_lambda_compare_4a, procedure_lambda_compare_na);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_lambda_compare_1a <ValueRef : StdAsRef<ProcedureLambda>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_lambda_compare_2a <ValueRef : StdAsRef<ProcedureLambda>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (ProcedureLambda::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (ProcedureLambda::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




def_fn_compare! (SyntaxPrimitive,
		syntax_primitive_compare_1, syntax_primitive_compare_2, syntax_primitive_compare_3, syntax_primitive_compare_4, syntax_primitive_compare_n,
		syntax_primitive_compare_1a, syntax_primitive_compare_2a, syntax_primitive_compare_3a, syntax_primitive_compare_4a, syntax_primitive_compare_na);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_primitive_compare_1a <ValueRef : StdAsRef<SyntaxPrimitive>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_primitive_compare_2a <ValueRef : StdAsRef<SyntaxPrimitive>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	return std_ord_compare_2_ref (left, right, comparison);
}


#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
def_fn_compare! (SyntaxExtended,
		syntax_extended_compare_1, syntax_extended_compare_2, syntax_extended_compare_3, syntax_extended_compare_4, syntax_extended_compare_n,
		syntax_extended_compare_1a, syntax_extended_compare_2a, syntax_extended_compare_3a, syntax_extended_compare_4a, syntax_extended_compare_na);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_extended_compare_1a <ValueRef : StdAsRef<SyntaxExtended>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_extended_compare_2a <ValueRef : StdAsRef<SyntaxExtended>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (SyntaxExtended::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (SyntaxExtended::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
def_fn_compare! (SyntaxNative,
		syntax_native_compare_1, syntax_native_compare_2, syntax_native_compare_3, syntax_native_compare_4, syntax_native_compare_n,
		syntax_native_compare_1a, syntax_native_compare_2a, syntax_native_compare_3a, syntax_native_compare_4a, syntax_native_compare_na);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_native_compare_1a <ValueRef : StdAsRef<SyntaxNative>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_native_compare_2a <ValueRef : StdAsRef<SyntaxNative>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (SyntaxNative::is_self (left, right) ^ negated),
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}


#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
def_fn_compare! (SyntaxLambda,
		syntax_lambda_compare_1, syntax_lambda_compare_2, syntax_lambda_compare_3, syntax_lambda_compare_4, syntax_lambda_compare_n,
		syntax_lambda_compare_1a, syntax_lambda_compare_2a, syntax_lambda_compare_3a, syntax_lambda_compare_4a, syntax_lambda_compare_na);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_lambda_compare_1a <ValueRef : StdAsRef<SyntaxLambda>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_lambda_compare_2a <ValueRef : StdAsRef<SyntaxLambda>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (SyntaxLambda::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (SyntaxLambda::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
def_fn_compare! (Path,
		path_compare_1, path_compare_2, path_compare_3, path_compare_4, path_compare_n,
		path_compare_1a, path_compare_2a, path_compare_3a, path_compare_4a, path_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn path_compare_1a <ValueRef : StdAsRef<Path>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn path_compare_2a <ValueRef : StdAsRef<Path>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Path::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Path::eq (left, right) ^ negated),
			},
		Comparison::Ordering (ordering, case_sensitivity, _, negated) =>
			match case_sensitivity {
				None | Some (true) =>
					return std_ord_compare_2_ordering_ref::<fs_path::Path, _> (left.path_ref (), right.path_ref (), ordering, negated),
				_ =>
					fail_unimplemented! (0x17fe4dd5, (github_issue, 35)),
			},
	}
}




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
def_fn_compare! (Port,
		port_compare_1, port_compare_2, port_compare_3, port_compare_4, port_compare_n,
		port_compare_1a, port_compare_2a, port_compare_3a, port_compare_4a, port_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_compare_1a <ValueRef : StdAsRef<Port>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_compare_2a <ValueRef : StdAsRef<Port>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Port::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Port::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
def_fn_compare! (Process,
		process_compare_1, process_compare_2, process_compare_3, process_compare_4, process_compare_n,
		process_compare_1a, process_compare_2a, process_compare_3a, process_compare_4a, process_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_compare_1a <ValueRef : StdAsRef<Process>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_compare_2a <ValueRef : StdAsRef<Process>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Process::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Process::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
def_fn_compare! (Context,
		context_compare_1, context_compare_2, context_compare_3, context_compare_4, context_compare_n,
		context_compare_1a, context_compare_2a, context_compare_3a, context_compare_4a, context_compare_na);

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn context_compare_1a <ValueRef : StdAsRef<Context>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn context_compare_2a <ValueRef : StdAsRef<Context>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Context::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Context::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
def_fn_compare! (Binding,
		binding_compare_1, binding_compare_2, binding_compare_3, binding_compare_4, binding_compare_n,
		binding_compare_1a, binding_compare_2a, binding_compare_3a, binding_compare_4a, binding_compare_na);

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn binding_compare_1a <ValueRef : StdAsRef<Binding>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn binding_compare_2a <ValueRef : StdAsRef<Binding>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Binding::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Binding::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
def_fn_compare! (Parameters,
		parameters_compare_1, parameters_compare_2, parameters_compare_3, parameters_compare_4, parameters_compare_n,
		parameters_compare_1a, parameters_compare_2a, parameters_compare_3a, parameters_compare_4a, parameters_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameters_compare_1a <ValueRef : StdAsRef<Parameters>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameters_compare_2a <ValueRef : StdAsRef<Parameters>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Parameters::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Parameters::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
def_fn_compare! (Parameter,
		parameter_compare_1, parameter_compare_2, parameter_compare_3, parameter_compare_4, parameter_compare_n,
		parameter_compare_1a, parameter_compare_2a, parameter_compare_3a, parameter_compare_4a, parameter_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_compare_1a <ValueRef : StdAsRef<Parameter>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_compare_2a <ValueRef : StdAsRef<Parameter>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Parameter::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Parameter::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
def_fn_compare! (Promise,
		promise_compare_1, promise_compare_2, promise_compare_3, promise_compare_4, promise_compare_n,
		promise_compare_1a, promise_compare_2a, promise_compare_3a, promise_compare_4a, promise_compare_na);

#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn promise_compare_1a <ValueRef : StdAsRef<Promise>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn promise_compare_2a <ValueRef : StdAsRef<Promise>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Promise::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Promise::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
			return std_ord_compare_2_ref (left, right, comparison),
	}
}




#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
def_fn_compare! (Opaque,
		opaque_compare_1, opaque_compare_2, opaque_compare_3, opaque_compare_4, opaque_compare_n,
		opaque_compare_1a, opaque_compare_2a, opaque_compare_3a, opaque_compare_4a, opaque_compare_na);

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn opaque_compare_1a <ValueRef : StdAsRef<Opaque>> (_value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn opaque_compare_2a <ValueRef : StdAsRef<Opaque>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = left.as_ref ();
	let right = right.as_ref ();
	match comparison {
		Comparison::Equivalence (equivalence, _, _, negated) =>
			match equivalence {
				Equivalence::ByIdentity =>
					succeed! (Opaque::is_self (left, right) ^ negated),
				Equivalence::ByValue =>
					succeed! (Opaque::eq (left, right) ^ negated),
			},
		Comparison::Ordering (_, _, _, _) =>
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
pub(crate) fn number_match_as_ref_compare_1a (class : &NumberMatchAsRef, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (_, _, _, negated) =>
			match *class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true ^ negated),
				NumberMatchAsRef::Real (_) =>
					succeed! (true ^ negated),
			},
		
		Comparison::Ordering (_, _, _, negated) =>
			match *class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true ^ negated),
				NumberMatchAsRef::Real (value) => {
					let value = value.value ();
					if value.is_nan () {
						succeed! (false ^ negated);
					} else {
						succeed! (true ^ negated);
					}
				},
			},
		
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn number_match_as_ref_compare_2a (class : &NumberMatchAsRef2, comparison : Comparison) -> (Outcome<bool>) {
	match comparison {
		
		Comparison::Equivalence (_, coercion, _, negated) =>
			match coercion {
				
				None | Some (false) =>
					match *class {
						NumberMatchAsRef2::IntegerBoth (left, right) => {
							let left = left.value ();
							let right = right.value ();
							succeed! (i64::eq (&left, &right) ^ negated);
						},
						NumberMatchAsRef2::RealBoth (left, right) => {
							let left = left.value ();
							let right = right.value ();
							if left.is_nan () && right.is_nan () {
								succeed! (true ^ negated);
							} else {
								succeed! (f64::eq (&left, &right) ^ negated);
							}
						},
						NumberMatchAsRef2::IntegerAndReal (_, _) =>
							succeed! (false ^ negated),
						NumberMatchAsRef2::RealAndInteger (_, _) =>
							succeed! (false ^ negated),
					},
				
				Some (true) =>
					match number_coerce_2e (class) {
						NumberCoercion2::Integer (left, right) =>
							succeed! (i64::eq (&left, &right) ^ negated),
						NumberCoercion2::Real (left, right) =>
							if left.is_nan () && right.is_nan () {
								succeed! (true ^ negated);
							} else {
								succeed! (f64::eq (&left, &right) ^ negated);
							},
					},
				
			},
		
		Comparison::Ordering (ordering, _, _, negated) =>
			match number_coerce_2e (class) {
				NumberCoercion2::Integer (left, right) =>
					return std_ord_compare_2_ordering_val (left, right, ordering, negated),
				NumberCoercion2::Real (left, right) =>
					if left.is_nan () || right.is_nan () {
						succeed! (false ^ negated);
					} else {
						return std_ord_compare_2_ordering_val (left, right, ordering, negated);
					},
			},
		
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
def_fn_compare! (Value,
		string_compare_1, string_compare_2, string_compare_3, string_compare_4, string_compare_n);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (StringRef::try (value.as_ref ()));
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (StringRef::try (left.as_ref ()));
	let right = try! (StringRef::try (right.as_ref ()));
	return string_ref_compare_2a (&left, &right, comparison);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
def_fn_compare! (Value,
		bytes_compare_1, bytes_compare_2, bytes_compare_3, bytes_compare_4, bytes_compare_n);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (BytesRef::try (value.as_ref ()));
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (BytesRef::try (left.as_ref ()));
	let right = try! (BytesRef::try (right.as_ref ()));
	return bytes_ref_compare_2a (&left, &right, comparison);
}




def_fn_compare! (Value,
		pair_compare_1, pair_compare_2, pair_compare_3, pair_compare_4, pair_compare_n);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (PairRef::try_ref (value.as_ref ()));
	succeed! (true ^ comparison.negated ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (PairRef::try_ref (left.as_ref ()));
	let right = try! (PairRef::try_ref (right.as_ref ()));
	return pair_ref_compare_2a (&left, &right, comparison);
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
def_fn_compare! (Value,
		array_compare_1, array_compare_2, array_compare_3, array_compare_4, array_compare_n);

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (ArrayRef::try (value.as_ref ()));
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (ArrayRef::try (left.as_ref ()));
	let right = try! (ArrayRef::try (right.as_ref ()));
	return array_ref_compare_2a (&left, &right, comparison);
}




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
def_fn_compare! (Value,
		record_compare_1, record_compare_2, record_compare_3, record_compare_4, record_compare_n);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_compare_1 <ValueRef : StdAsRef<Value>> (value : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let _value = try! (RecordRef::try (value.as_ref ()));
	succeed! (true ^ comparison.negated ());
}

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_compare_2 <ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>) {
	let left = try! (RecordRef::try (left.as_ref ()));
	let right = try! (RecordRef::try (right.as_ref ()));
	return record_ref_compare_2a (&left, &right, comparison);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn value_kind_compare_2a_ordering (left : ValueKind, right : ValueKind, ordering : Ordering, negated : bool) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering_val (left, right, ordering, negated);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn value_class_compare_2a_ordering (left : ValueClass, right : ValueClass, ordering : Ordering, negated : bool) -> (Outcome<bool>) {
	return std_ord_compare_2_ordering_val (left, right, ordering, negated);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_compare_2 (left : &[Value], right : &[Value], comparison : Comparison) -> (Outcome<bool>) {
	
	let left_length = left.len ();
	let right_length = right.len ();
	
	match comparison {
		
		Comparison::Equivalence (_, _, _, negated) =>
			if left_length != right_length {
				succeed! (false ^ negated);
			} else if (left_length == 0) && (right_length == 0) {
				succeed! (true ^ negated);
			},
		
		Comparison::Ordering (ordering, _, _, negated) =>
			if (left_length == 0) && (right_length == 0) {
				match ordering {
					Ordering::LesserOrEqual | Ordering::Equal | Ordering::GreaterOrEqual =>
						succeed! (true ^ negated),
					Ordering::Lesser | Ordering::Greater =>
						succeed! (false ^ negated),
				}
			},
		
	}
	
	let mut left_iterator = left.iter ();
	let mut right_iterator = right.iter ();
	let comparison_for_last = comparison.for_aggregated (true);
	let comparison_for_non_last = comparison.for_aggregated (false);
	let negated = comparison.negated ();
	let index_last = usize::max (left_length, right_length) - 1;
	let mut index_next = 0;
	loop {
		let left_next = left_iterator.next ();
		let right_next = right_iterator.next ();
		match (left_next, right_next) {
			
			(Some (left_next), Some (right_next)) =>
				if index_next == index_last {
					if ! try! (compare_2 (left_next, right_next, comparison_for_last)) ^ negated {
						succeed! (false ^ negated);
					}
				} else {
					if ! try! (compare_2 (left_next, right_next, comparison_for_non_last)) ^ negated {
						if comparison_for_last == comparison_for_non_last {
							succeed! (false ^ negated);
						}
						if ! try! (compare_2 (left_next, right_next, comparison_for_last)) ^ negated {
							succeed! (false ^ negated);
						} else {
							succeed! (true ^ negated);
						}
					}
				},
			
			(None, None) =>
				match comparison {
					Comparison::Equivalence (_, _, _, negated) =>
						succeed! (true ^ negated),
					Comparison::Ordering (_, _, _, negated) =>
						succeed! (true ^ negated),
				},
			
			(None, Some (_)) =>
				match comparison {
					Comparison::Equivalence (_, _, _, _) =>
						fail_unreachable! (0x97e8a0c7, github_issue_new),
					Comparison::Ordering (ordering, _, _, negated) =>
						match ordering {
							Ordering::Lesser | Ordering::LesserOrEqual =>
								succeed! (true ^ negated),
							Ordering::Equal | Ordering::GreaterOrEqual | Ordering::Greater =>
								succeed! (false ^ negated),
						},
				},
			
			(Some (_), None) =>
				match comparison {
					Comparison::Equivalence (_, _, _, _) =>
						fail_unreachable! (0x2558c7ee, github_issue_new),
					Comparison::Ordering (ordering, _, _, negated) =>
						match ordering {
							Ordering::Greater | Ordering::GreaterOrEqual =>
								succeed! (true ^ negated),
							Ordering::Equal | Ordering::LesserOrEqual | Ordering::Lesser =>
								succeed! (false ^ negated),
						},
				},
			
		}
		
		index_next += 1;
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn std_ord_compare_2_ref <Value : ?Sized, ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, comparison : Comparison) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (Value::eq (left.as_ref (), right.as_ref ()) ^ negated),
		Comparison::Ordering (ordering, _, _, negated) =>
			return std_ord_compare_2_ordering_ref (left, right, ordering, negated),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn std_ord_compare_2_ordering_ref <Value : ?Sized, ValueRef : StdAsRef<Value>> (left : ValueRef, right : ValueRef, ordering : Ordering, negated : bool) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	let left = left.as_ref ();
	let right = right.as_ref ();
	let output = match ordering {
		Ordering::Lesser =>
			Value::lt (left, right),
		Ordering::LesserOrEqual =>
			Value::le (left, right),
		Ordering::Equal =>
			Value::eq (left, right),
		Ordering::GreaterOrEqual =>
			Value::ge (left, right),
		Ordering::Greater =>
			Value::gt (left, right),
	};
	succeed! (output ^ negated);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn std_ord_compare_2_val <Value> (left : Value, right : Value, comparison : Comparison) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	match comparison {
		Comparison::Equivalence (_, _, _, negated) =>
			succeed! (Value::eq (&left, &right) ^ negated),
		Comparison::Ordering (ordering, _, _, negated) =>
			return std_ord_compare_2_ordering_val (left, right, ordering, negated),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn std_ord_compare_2_ordering_val <Value> (left : Value, right : Value, ordering : Ordering, negated : bool) -> (Outcome<bool>)
		where Value : cmp::PartialOrd
{
	let output = match ordering {
		Ordering::Lesser =>
			Value::lt (&left, &right),
		Ordering::LesserOrEqual =>
			Value::le (&left, &right),
		Ordering::Equal =>
			Value::eq (&left, &right),
		Ordering::GreaterOrEqual =>
			Value::ge (&left, &right),
		Ordering::Greater =>
			Value::gt (&left, &right),
	};
	succeed! (output ^ negated);
}

