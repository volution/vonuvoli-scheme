

// FIXME:  Remove this after exporting all public functions!
#![ allow (unreachable_pub) ]




use super::errors::exports::*;
use super::ports::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	
	pub use super::{
			
			is_boolean,
			is_true, is_false,
			is_not_true, is_not_false,
			is_true_or_equivalent, is_false_or_equivalent,
			
			is_boolean_all_2, is_true_all_2, is_false_all_2, is_not_true_all_2, is_not_false_all_2,
			is_boolean_all_3, is_true_all_3, is_false_all_3, is_not_true_all_3, is_not_false_all_3,
			is_boolean_all_4, is_true_all_4, is_false_all_4, is_not_true_all_4, is_not_false_all_4,
			is_boolean_all_n, is_true_all_n, is_false_all_n, is_not_true_all_n, is_not_false_all_n,
			
			is_boolean_any_2, is_true_any_2, is_false_any_2, is_not_true_any_2, is_not_false_any_2,
			is_boolean_any_3, is_true_any_3, is_false_any_3, is_not_true_any_3, is_not_false_any_3,
			is_boolean_any_4, is_true_any_4, is_false_any_4, is_not_true_any_4, is_not_false_any_4,
			is_boolean_any_n, is_true_any_n, is_false_any_n, is_not_true_any_n, is_not_false_any_n,
			
			is_true_or_equivalent_all_2, is_false_or_equivalent_any_2,
			is_true_or_equivalent_all_3, is_false_or_equivalent_any_3,
			is_true_or_equivalent_all_4, is_false_or_equivalent_any_4,
			is_true_or_equivalent_all_n, is_false_or_equivalent_any_n,
			
	};
	
	
	pub use super::{
			
			is_null, is_not_null,
			is_void, is_not_void,
			is_undefined, is_not_undefined,
			
			is_null_all_2, is_null_all_3, is_null_all_4, is_null_all_n,
			is_null_any_2, is_null_any_3, is_null_any_4, is_null_any_n,
			is_not_null_all_2, is_not_null_all_3, is_not_null_all_4, is_not_null_all_n,
			is_not_null_any_2, is_not_null_any_3, is_not_null_any_4, is_not_null_any_n,
			
			is_void_all_2, is_void_all_3, is_void_all_4, is_void_all_n,
			is_void_any_2, is_void_any_3, is_void_any_4, is_void_any_n,
			is_not_void_all_2, is_not_void_all_3, is_not_void_all_4, is_not_void_all_n,
			is_not_void_any_2, is_not_void_any_3, is_not_void_any_4, is_not_void_any_n,
			
			is_undefined_all_2, is_undefined_all_3, is_undefined_all_4, is_undefined_all_n,
			is_undefined_any_2, is_undefined_any_3, is_undefined_any_4, is_undefined_any_n,
			is_not_undefined_all_2, is_not_undefined_all_3, is_not_undefined_all_4, is_not_undefined_all_n,
			is_not_undefined_any_2, is_not_undefined_any_3, is_not_undefined_any_4, is_not_undefined_any_n,
			
	};
	
	
	pub use super::{
			
			is_number, is_number_integer, is_number_rational, is_number_real, is_number_complex,
			is_number_exact, is_number_exact_integer, is_number_inexact,
			is_number_zero, is_number_positive, is_number_negative, is_number_finite, is_number_infinite, is_number_nan,
			is_number_even, is_number_odd,
			
			is_number_all_2, is_number_all_3, is_number_all_4, is_number_all_n,
			is_number_integer_all_2, is_number_integer_all_3, is_number_integer_all_4, is_number_integer_all_n,
			is_number_rational_all_2, is_number_rational_all_3, is_number_rational_all_4, is_number_rational_all_n,
			is_number_real_all_2, is_number_real_all_3, is_number_real_all_4, is_number_real_all_n,
			is_number_complex_all_2, is_number_complex_all_3, is_number_complex_all_4, is_number_complex_all_n,
			is_number_exact_all_2, is_number_exact_all_3, is_number_exact_all_4, is_number_exact_all_n,
			is_number_exact_integer_all_2, is_number_exact_integer_all_3, is_number_exact_integer_all_4, is_number_exact_integer_all_n,
			is_number_inexact_all_2, is_number_inexact_all_3, is_number_inexact_all_4, is_number_inexact_all_n,
			
			is_number_any_2, is_number_any_3, is_number_any_4, is_number_any_n,
			is_number_integer_any_2, is_number_integer_any_3, is_number_integer_any_4, is_number_integer_any_n,
			is_number_rational_any_2, is_number_rational_any_3, is_number_rational_any_4, is_number_rational_any_n,
			is_number_real_any_2, is_number_real_any_3, is_number_real_any_4, is_number_real_any_n,
			is_number_complex_any_2, is_number_complex_any_3, is_number_complex_any_4, is_number_complex_any_n,
			is_number_exact_any_2, is_number_exact_any_3, is_number_exact_any_4, is_number_exact_any_n,
			is_number_exact_integer_any_2, is_number_exact_integer_any_3, is_number_exact_integer_any_4, is_number_exact_integer_any_n,
			is_number_inexact_any_2, is_number_inexact_any_3, is_number_inexact_any_4, is_number_inexact_any_n,
			
			// FIXME:  Add other variants for `is_number_*`!
	};
	
	
	pub use super::{
			
			is_character,
			
			is_character_all_2, is_character_all_3, is_character_all_4, is_character_all_n,
			is_character_any_2, is_character_any_3, is_character_any_4, is_character_any_n,
			
	};
	
	
	pub use super::{
			
			is_symbol, is_symbol_eq,
			
			is_symbol_all_2, is_symbol_all_3, is_symbol_all_4, is_symbol_all_n,
			is_symbol_any_2, is_symbol_any_3, is_symbol_any_4, is_symbol_any_n,
			
			is_symbol_eq_all_2, is_symbol_eq_all_3, is_symbol_eq_all_4, is_symbol_eq_all_n,
			is_symbol_eq_any_2, is_symbol_eq_any_3, is_symbol_eq_any_4, is_symbol_eq_any_n,
			
	};
	
	
	pub use super::{
			
			is_pair, is_pair_immutable, is_pair_mutable,
			
			is_pair_all_2, is_pair_all_3, is_pair_all_4, is_pair_all_n,
			is_pair_immutable_all_2, is_pair_immutable_all_3, is_pair_immutable_all_4, is_pair_immutable_all_n,
			is_pair_mutable_all_2, is_pair_mutable_all_3, is_pair_mutable_all_4, is_pair_mutable_all_n,
			
			is_pair_any_2, is_pair_any_3, is_pair_any_4, is_pair_any_n,
			is_pair_immutable_any_2, is_pair_immutable_any_3, is_pair_immutable_any_4, is_pair_immutable_any_n,
			is_pair_mutable_any_2, is_pair_mutable_any_3, is_pair_mutable_any_4, is_pair_mutable_any_n,
			
	};
	
	
	pub use super::{
			
			is_list, is_list_empty, is_list_or_empty,
			is_list_proper, is_list_proper_or_empty,
			is_list_dotted, is_list_dotted_or_empty,
			is_list_cyclic, is_list_cyclic_or_empty,
			
			is_list_empty_all_2, is_list_empty_all_3, is_list_empty_all_4, is_list_empty_all_n,
			is_list_empty_any_2, is_list_empty_any_3, is_list_empty_any_4, is_list_empty_any_n,
			
			// FIXME:  Add other variants for `is_list_*`!
	};
	
	
	pub use super::{
			
			is_array, is_array_empty, is_array_not_empty,
			is_array_immutable, is_array_immutable_empty, is_array_immutable_not_empty,
			is_array_mutable, is_array_mutable_empty, is_array_mutable_not_empty,
			
			is_array_all_2, is_array_all_3, is_array_all_4, is_array_all_n,
			is_array_immutable_all_2, is_array_immutable_all_3, is_array_immutable_all_4, is_array_immutable_all_n,
			is_array_mutable_all_2, is_array_mutable_all_3, is_array_mutable_all_4, is_array_mutable_all_n,
			
			is_array_any_2, is_array_any_3, is_array_any_4, is_array_any_n,
			is_array_immutable_any_2, is_array_immutable_any_3, is_array_immutable_any_4, is_array_immutable_any_n,
			is_array_mutable_any_2, is_array_mutable_any_3, is_array_mutable_any_4, is_array_mutable_any_n,
			
			is_array_empty_all_2, is_array_empty_all_3, is_array_empty_all_4,
			is_array_immutable_empty_all_2, is_array_immutable_empty_all_3, is_array_immutable_empty_all_4,
			is_array_mutable_empty_all_2, is_array_mutable_empty_all_3, is_array_mutable_empty_all_4,
			
			is_array_empty_any_2, is_array_empty_any_3, is_array_empty_any_4,
			is_array_immutable_empty_any_2, is_array_immutable_empty_any_3, is_array_immutable_empty_any_4,
			is_array_mutable_empty_any_2, is_array_mutable_empty_any_3, is_array_mutable_empty_any_4,
			
			is_array_not_empty_all_2, is_array_not_empty_all_3, is_array_not_empty_all_4,
			is_array_immutable_not_empty_all_2, is_array_immutable_not_empty_all_3, is_array_immutable_not_empty_all_4,
			is_array_mutable_not_empty_all_2, is_array_mutable_not_empty_all_3, is_array_mutable_not_empty_all_4,
			
			is_array_not_empty_any_2, is_array_not_empty_any_3, is_array_not_empty_any_4,
			is_array_immutable_not_empty_any_2, is_array_immutable_not_empty_any_3, is_array_immutable_not_empty_any_4,
			is_array_mutable_not_empty_any_2, is_array_mutable_not_empty_any_3, is_array_mutable_not_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_bytes, is_bytes_empty, is_bytes_not_empty,
			is_bytes_immutable, is_bytes_immutable_empty, is_bytes_immutable_not_empty,
			is_bytes_mutable, is_bytes_mutable_empty, is_bytes_mutable_not_empty,
			
			is_bytes_all_2, is_bytes_all_3, is_bytes_all_4, is_bytes_all_n,
			is_bytes_immutable_all_2, is_bytes_immutable_all_3, is_bytes_immutable_all_4, is_bytes_immutable_all_n,
			is_bytes_mutable_all_2, is_bytes_mutable_all_3, is_bytes_mutable_all_4, is_bytes_mutable_all_n,
			
			is_bytes_any_2, is_bytes_any_3, is_bytes_any_4, is_bytes_any_n,
			is_bytes_immutable_any_2, is_bytes_immutable_any_3, is_bytes_immutable_any_4, is_bytes_immutable_any_n,
			is_bytes_mutable_any_2, is_bytes_mutable_any_3, is_bytes_mutable_any_4, is_bytes_mutable_any_n,
			
			is_bytes_empty_all_2, is_bytes_empty_all_3, is_bytes_empty_all_4,
			is_bytes_immutable_empty_all_2, is_bytes_immutable_empty_all_3, is_bytes_immutable_empty_all_4,
			is_bytes_mutable_empty_all_2, is_bytes_mutable_empty_all_3, is_bytes_mutable_empty_all_4,
			
			is_bytes_empty_any_2, is_bytes_empty_any_3, is_bytes_empty_any_4,
			is_bytes_immutable_empty_any_2, is_bytes_immutable_empty_any_3, is_bytes_immutable_empty_any_4,
			is_bytes_mutable_empty_any_2, is_bytes_mutable_empty_any_3, is_bytes_mutable_empty_any_4,
			
			is_bytes_not_empty_all_2, is_bytes_not_empty_all_3, is_bytes_not_empty_all_4,
			is_bytes_immutable_not_empty_all_2, is_bytes_immutable_not_empty_all_3, is_bytes_immutable_not_empty_all_4,
			is_bytes_mutable_not_empty_all_2, is_bytes_mutable_not_empty_all_3, is_bytes_mutable_not_empty_all_4,
			
			is_bytes_not_empty_any_2, is_bytes_not_empty_any_3, is_bytes_not_empty_any_4,
			is_bytes_immutable_not_empty_any_2, is_bytes_immutable_not_empty_any_3, is_bytes_immutable_not_empty_any_4,
			is_bytes_mutable_not_empty_any_2, is_bytes_mutable_not_empty_any_3, is_bytes_mutable_not_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_string, is_string_empty, is_string_not_empty,
			is_string_immutable, is_string_immutable_empty, is_string_immutable_not_empty,
			is_string_mutable, is_string_mutable_empty, is_string_mutable_not_empty,
			
			is_string_all_2, is_string_all_3, is_string_all_4, is_string_all_n,
			is_string_immutable_all_2, is_string_immutable_all_3, is_string_immutable_all_4, is_string_immutable_all_n,
			is_string_mutable_all_2, is_string_mutable_all_3, is_string_mutable_all_4, is_string_mutable_all_n,
			
			is_string_any_2, is_string_any_3, is_string_any_4, is_string_any_n,
			is_string_immutable_any_2, is_string_immutable_any_3, is_string_immutable_any_4, is_string_immutable_any_n,
			is_string_mutable_any_2, is_string_mutable_any_3, is_string_mutable_any_4, is_string_mutable_any_n,
			
			is_string_empty_all_2, is_string_empty_all_3, is_string_empty_all_4,
			is_string_immutable_empty_all_2, is_string_immutable_empty_all_3, is_string_immutable_empty_all_4,
			is_string_mutable_empty_all_2, is_string_mutable_empty_all_3, is_string_mutable_empty_all_4,
			
			is_string_empty_any_2, is_string_empty_any_3, is_string_empty_any_4,
			is_string_immutable_empty_any_2, is_string_immutable_empty_any_3, is_string_immutable_empty_any_4,
			is_string_mutable_empty_any_2, is_string_mutable_empty_any_3, is_string_mutable_empty_any_4,
			
			is_string_not_empty_all_2, is_string_not_empty_all_3, is_string_not_empty_all_4,
			is_string_immutable_not_empty_all_2, is_string_immutable_not_empty_all_3, is_string_immutable_not_empty_all_4,
			is_string_mutable_not_empty_all_2, is_string_mutable_not_empty_all_3, is_string_mutable_not_empty_all_4,
			
			is_string_not_empty_any_2, is_string_not_empty_any_3, is_string_not_empty_any_4,
			is_string_immutable_not_empty_any_2, is_string_immutable_not_empty_any_3, is_string_immutable_not_empty_any_4,
			is_string_mutable_not_empty_any_2, is_string_mutable_not_empty_any_3, is_string_mutable_not_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_values, is_values_empty, is_values_not_empty,
			
			is_values_all_2, is_values_all_3, is_values_all_4,
			is_values_any_2, is_values_any_3, is_values_any_4,
			
			is_values_empty_all_2, is_values_empty_all_3, is_values_empty_all_4,
			is_values_empty_any_2, is_values_empty_any_3, is_values_empty_any_4,
			
			is_values_not_empty_all_2, is_values_not_empty_all_3, is_values_not_empty_all_4,
			is_values_not_empty_any_2, is_values_not_empty_any_3, is_values_not_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_procedure,
			
			is_procedure_all_2, is_procedure_all_3, is_procedure_all_4, is_procedure_all_n,
			is_procedure_any_2, is_procedure_any_3, is_procedure_any_4, is_procedure_any_n,
			
	};
	
	
	pub use super::{
			
			is_syntax,
			
			is_syntax_all_2, is_syntax_all_3, is_syntax_all_4, is_syntax_all_n,
			is_syntax_any_2, is_syntax_any_3, is_syntax_any_4, is_syntax_any_n,
			
	};
	
	
	pub use super::{
			
			is_error,
			is_error_syntax,
			is_error_file, is_error_port, is_error_port_input, is_error_port_output,
			
			// FIXME:  Add other variants for `is_error_*`!
	};
	
	
	pub use super::{
			
			is_port,
			is_port_input, is_port_output,
			is_port_binary, is_port_textual,
			is_port_eof,
			
			is_port_all_2, is_port_all_3, is_port_all_4, is_port_all_n,
			is_port_any_2, is_port_any_3, is_port_any_4, is_port_any_n,
			
			// FIXME:  Add other variants for `is_port_*`!
	};
	
	
	pub use super::{
			
			is_process,
			is_resource,
			is_opaque,
			
			is_process_all_2, is_process_all_3, is_process_all_4, is_process_all_n,
			is_process_any_2, is_process_any_3, is_process_any_4, is_process_any_n,
			
			is_resource_all_2, is_resource_all_3, is_resource_all_4, is_resource_all_n,
			is_resource_any_2, is_resource_any_3, is_resource_any_4, is_resource_any_n,
			
			is_opaque_all_2, is_opaque_all_3, is_opaque_all_4, is_opaque_all_n,
			is_opaque_any_2, is_opaque_any_3, is_opaque_any_4, is_opaque_any_n,
			
	};
	
	
	pub use super::{
			
			NumberClass, number_class,
			
			ListClass, list_class_o1, list_class_on,
			
			ProcedureClass, procedure_class,
			
			SyntaxClass, syntax_class,
			
	};
	
	
}




macro_rules! def_fn_predicate_all {
	( $predicate : ident, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2) && $predicate (value_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2) && $predicate (value_3) && $predicate (value_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (values : &[&Value]) -> (bool) {
			for value_i in values {
				if !$predicate (value_i) {
					return false;
				}
			}
			return true;
		}
	);
}

macro_rules! def_fn_predicate_any {
	( $predicate : ident, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2) || $predicate (value_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2) || $predicate (value_3) || $predicate (value_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (values : &[&Value]) -> (bool) {
			for value_i in values {
				if $predicate (value_i) {
					return false;
				}
			}
			return false;
		}
	);
}


macro_rules! def_fn_try_predicate_all {
	( $predicate : ident, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			succeed! (outcome_1 && outcome_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			succeed! (outcome_1 && outcome_2 && outcome_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			let outcome_4 = try! ($predicate (value_4));
			succeed! (outcome_1 && outcome_2 && outcome_3 && outcome_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (values : &[&Value]) -> (Outcome<bool>) {
			let mut outcome = true;
			for value_i in values {
				let outcome_i = try! ($predicate (value_i));
				outcome = outcome && outcome_i;
			}
			succeed! (outcome);
		}
	);
}

macro_rules! def_fn_try_predicate_any {
	( $predicate : ident, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			succeed! (outcome_1 || outcome_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			succeed! (outcome_1 || outcome_2 || outcome_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			let outcome_4 = try! ($predicate (value_4));
			succeed! (outcome_1 || outcome_2 || outcome_3 || outcome_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (values : &[&Value]) -> (Outcome<bool>) {
			let mut outcome = true;
			for value_i in values {
				let outcome_i = try! ($predicate (value_i));
				outcome = outcome || outcome_i;
			}
			succeed! (outcome);
		}
	);
}




macro_rules! def_fn_predicate_all_x1 {
	( $predicate : ident, $extra_1_type : ty, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) && $predicate (extra_1, value_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) && $predicate (extra_1, value_2) && $predicate (extra_1, value_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) && $predicate (extra_1, value_2) && $predicate (extra_1, value_3) && $predicate (extra_1, value_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (extra_1 : $extra_1_type, values : &[&Value]) -> (bool) {
			for value_i in values {
				if !$predicate (extra_1, value_i) {
					return false;
				}
			}
			return true;
		}
	);
}

macro_rules! def_fn_predicate_any_x1 {
	( $predicate : ident, $extra_1_type : ty, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) || $predicate (extra_1, value_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) || $predicate (extra_1, value_2) || $predicate (extra_1, value_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (extra_1, value_1) || $predicate (extra_1, value_2) || $predicate (extra_1, value_3) || $predicate (extra_1, value_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (extra_1 : $extra_1_type, values : &[&Value]) -> (bool) {
			for value_i in values {
				if $predicate (extra_1, value_i) {
					return false;
				}
			}
			return false;
		}
	);
}


macro_rules! def_fn_try_predicate_all_x1 {
	( $predicate : ident, $extra_1_type : ty, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			succeed! (outcome_1 && outcome_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			let outcome_3 = try! ($predicate (extra_1, value_3));
			succeed! (outcome_1 && outcome_2 && outcome_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			let outcome_3 = try! ($predicate (extra_1, value_3));
			let outcome_4 = try! ($predicate (extra_1, value_4));
			succeed! (outcome_1 && outcome_2 && outcome_3 && outcome_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (extra_1 : $extra_1_type, values : &[&Value]) -> (Outcome<bool>) {
			let mut outcome = true;
			for value_i in values {
				let outcome_i = try! ($predicate (extra_1, value_i));
				outcome = outcome && outcome_i;
			}
			succeed! (outcome);
		}
	);
}

macro_rules! def_fn_try_predicate_any_x1 {
	( $predicate : ident, $extra_1_type : ty, $predicate_2 : ident, $predicate_3 : ident, $predicate_4 : ident, $predicate_n : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_2 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			succeed! (outcome_1 || outcome_2);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			let outcome_3 = try! ($predicate (extra_1, value_3));
			succeed! (outcome_1 || outcome_2 || outcome_3);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_4 (extra_1 : $extra_1_type, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (extra_1, value_1));
			let outcome_2 = try! ($predicate (extra_1, value_2));
			let outcome_3 = try! ($predicate (extra_1, value_3));
			let outcome_4 = try! ($predicate (extra_1, value_4));
			succeed! (outcome_1 || outcome_2 || outcome_3 || outcome_4);
		}
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate_n (extra_1 : $extra_1_type, values : &[&Value]) -> (Outcome<bool>) {
			let mut outcome = true;
			for value_i in values {
				let outcome_i = try! ($predicate (extra_1, value_i));
				outcome = outcome || outcome_i;
			}
			succeed! (outcome);
		}
	);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_boolean (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Boolean);
}

def_fn_predicate_all! (is_boolean, is_boolean_all_2, is_boolean_all_3, is_boolean_all_4, is_boolean_all_n);
def_fn_predicate_any! (is_boolean, is_boolean_any_2, is_boolean_any_3, is_boolean_any_4, is_boolean_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_true (value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) =>
			return value.value () == true,
		_ =>
			return false,
	}
}

def_fn_predicate_all! (is_true, is_true_all_2, is_true_all_3, is_true_all_4, is_true_all_n);
def_fn_predicate_any! (is_true, is_true_any_2, is_true_any_3, is_true_any_4, is_true_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_false (value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) =>
			return value.value () == false,
		_ =>
			return false,
	}
}

def_fn_predicate_all! (is_false, is_false_all_2, is_false_all_3, is_false_all_4, is_false_all_n);
def_fn_predicate_any! (is_false, is_false_any_2, is_false_any_3, is_false_any_4, is_false_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_not_true (value : &Value) -> (bool) {
	return ! is_true (value);
}

def_fn_predicate_all! (is_not_true, is_not_true_all_2, is_not_true_all_3, is_not_true_all_4, is_not_true_all_n);
def_fn_predicate_any! (is_not_true, is_not_true_any_2, is_not_true_any_3, is_not_true_any_4, is_not_true_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_not_false (value : &Value) -> (bool) {
	return ! is_false (value);
}

def_fn_predicate_all! (is_not_false, is_not_false_all_2, is_not_false_all_3, is_not_false_all_4, is_not_false_all_n);
def_fn_predicate_any! (is_not_false, is_not_false_any_2, is_not_false_any_3, is_not_false_any_4, is_not_false_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_true_or_equivalent (value : &Value) -> (bool) {
	return ! is_false_or_equivalent (value)
}

def_fn_predicate_all! (is_true_or_equivalent, is_true_or_equivalent_all_2, is_true_or_equivalent_all_3, is_true_or_equivalent_all_4, is_true_or_equivalent_all_n);
def_fn_predicate_any! (is_true_or_equivalent, is_true_or_equivalent_any_2, is_true_or_equivalent_any_3, is_true_or_equivalent_any_4, is_true_or_equivalent_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_false_or_equivalent (value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Null |
		ValueKindMatchAsRef::Void |
		ValueKindMatchAsRef::Undefined =>
			return true,
		ValueKindMatchAsRef::Boolean (value) =>
			return ! value.value (),
		ValueKindMatchAsRef::Error (_) =>
			return true,
		_ =>
			return false,
	}
}

def_fn_predicate_all! (is_false_or_equivalent, is_false_or_equivalent_all_2, is_false_or_equivalent_all_3, is_false_or_equivalent_all_4, is_false_or_equivalent_all_n);
def_fn_predicate_any! (is_false_or_equivalent, is_false_or_equivalent_any_2, is_false_or_equivalent_any_3, is_false_or_equivalent_any_4, is_false_or_equivalent_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_null (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Null);
}

def_fn_predicate_all! (is_null, is_null_all_2, is_null_all_3, is_null_all_4, is_null_all_n);
def_fn_predicate_any! (is_null, is_null_any_2, is_null_any_3, is_null_any_4, is_null_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_void (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Void);
}

def_fn_predicate_all! (is_void, is_void_all_2, is_void_all_3, is_void_all_4, is_void_all_n);
def_fn_predicate_any! (is_void, is_void_any_2, is_void_any_3, is_void_any_4, is_void_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_undefined (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Undefined);
}

def_fn_predicate_all! (is_undefined, is_undefined_all_2, is_undefined_all_3, is_undefined_all_4, is_undefined_all_n);
def_fn_predicate_any! (is_undefined, is_undefined_any_2, is_undefined_any_3, is_undefined_any_4, is_undefined_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_not_null (value : &Value) -> (bool) {
	return ! is_null (value);
}

def_fn_predicate_all! (is_not_null, is_not_null_all_2, is_not_null_all_3, is_not_null_all_4, is_not_null_all_n);
def_fn_predicate_any! (is_not_null, is_not_null_any_2, is_not_null_any_3, is_not_null_any_4, is_not_null_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_not_void (value : &Value) -> (bool) {
	return ! is_void (value);
}

def_fn_predicate_all! (is_not_void, is_not_void_all_2, is_not_void_all_3, is_not_void_all_4, is_not_void_all_n);
def_fn_predicate_any! (is_not_void, is_not_void_any_2, is_not_void_any_3, is_not_void_any_4, is_not_void_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_not_undefined (value : &Value) -> (bool) {
	return ! is_undefined (value);
}

def_fn_predicate_all! (is_not_undefined, is_not_undefined_all_2, is_not_undefined_all_3, is_not_undefined_all_4, is_not_undefined_all_n);
def_fn_predicate_any! (is_not_undefined, is_not_undefined_any_2, is_not_undefined_any_3, is_not_undefined_any_4, is_not_undefined_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Number);
}

def_fn_predicate_all! (is_number, is_number_all_2, is_number_all_3, is_number_all_4, is_number_all_n);
def_fn_predicate_any! (is_number, is_number_any_2, is_number_any_3, is_number_any_4, is_number_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_integer (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::NumberInteger);
}

def_fn_predicate_all! (is_number_integer, is_number_integer_all_2, is_number_integer_all_3, is_number_integer_all_4, is_number_integer_all_n);
def_fn_predicate_any! (is_number_integer, is_number_integer_any_2, is_number_integer_any_3, is_number_integer_any_4, is_number_integer_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_rational (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::NumberInteger);
}

def_fn_predicate_all! (is_number_rational, is_number_rational_all_2, is_number_rational_all_3, is_number_rational_all_4, is_number_rational_all_n);
def_fn_predicate_any! (is_number_rational, is_number_rational_any_2, is_number_rational_any_3, is_number_rational_any_4, is_number_rational_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_real (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::NumberReal);
}

def_fn_predicate_all! (is_number_real, is_number_real_all_2, is_number_real_all_3, is_number_real_all_4, is_number_real_all_n);
def_fn_predicate_any! (is_number_real, is_number_real_any_2, is_number_real_any_3, is_number_real_any_4, is_number_real_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_complex (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Number);
}

def_fn_predicate_all! (is_number_complex, is_number_complex_all_2, is_number_complex_all_3, is_number_complex_all_4, is_number_complex_all_n);
def_fn_predicate_any! (is_number_complex, is_number_complex_any_2, is_number_complex_any_3, is_number_complex_any_4, is_number_complex_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_exact (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true),
				NumberMatchAsRef::Real (_) =>
					succeed! (false),
			},
		_ =>
			fail! (0xf5982779),
	}
}

def_fn_try_predicate_all! (is_number_exact, is_number_exact_all_2, is_number_exact_all_3, is_number_exact_all_4, is_number_exact_all_n);
def_fn_try_predicate_any! (is_number_exact, is_number_exact_any_2, is_number_exact_any_3, is_number_exact_any_4, is_number_exact_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_exact_integer (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true),
				NumberMatchAsRef::Real (_) =>
					succeed! (false),
			},
		_ =>
			fail! (0xd8518d91),
	}
}

def_fn_try_predicate_all! (is_number_exact_integer, is_number_exact_integer_all_2, is_number_exact_integer_all_3, is_number_exact_integer_all_4, is_number_exact_integer_all_n);
def_fn_try_predicate_any! (is_number_exact_integer, is_number_exact_integer_any_2, is_number_exact_integer_any_3, is_number_exact_integer_any_4, is_number_exact_integer_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_inexact (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (false),
				NumberMatchAsRef::Real (_) =>
					succeed! (true),
			},
		_ =>
			fail! (0x676feef5),
	}
}

def_fn_try_predicate_all! (is_number_inexact, is_number_inexact_all_2, is_number_inexact_all_3, is_number_inexact_all_4, is_number_inexact_all_n);
def_fn_try_predicate_any! (is_number_inexact, is_number_inexact_any_2, is_number_inexact_any_3, is_number_inexact_any_4, is_number_inexact_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_zero (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (value.is_zero ()),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_zero ()),
			},
		_ =>
			fail! (0x71ac7e77),
	}
}

def_fn_try_predicate_all! (is_number_zero, is_number_zero_all_2, is_number_zero_all_3, is_number_zero_all_4, is_number_zero_all_n);
def_fn_try_predicate_any! (is_number_zero, is_number_zero_any_2, is_number_zero_any_3, is_number_zero_any_4, is_number_zero_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_positive (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (value.is_positive ()),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_positive ()),
			},
		_ =>
			fail! (0x7b86bd2d),
	}
}

def_fn_try_predicate_all! (is_number_positive, is_number_positive_all_2, is_number_positive_all_3, is_number_positive_all_4, is_number_positive_all_n);
def_fn_try_predicate_any! (is_number_positive, is_number_positive_any_2, is_number_positive_any_3, is_number_positive_any_4, is_number_positive_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_negative (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (value.is_negative ()),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_negative ()),
			},
		_ =>
			fail! (0xffbd419f),
	}
}

def_fn_try_predicate_all! (is_number_negative, is_number_negative_all_2, is_number_negative_all_3, is_number_negative_all_4, is_number_negative_all_n);
def_fn_try_predicate_any! (is_number_negative, is_number_negative_any_2, is_number_negative_any_3, is_number_negative_any_4, is_number_negative_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_finite (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (true),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_finite ()),
			},
		_ =>
			fail! (0xecdbce29),
	}
}

def_fn_try_predicate_all! (is_number_finite, is_number_finite_all_2, is_number_finite_all_3, is_number_finite_all_4, is_number_finite_all_n);
def_fn_try_predicate_any! (is_number_finite, is_number_finite_any_2, is_number_finite_any_3, is_number_finite_any_4, is_number_finite_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_infinite (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (false),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_infinite ()),
			},
		_ =>
			fail! (0x348e9928),
	}
}

def_fn_try_predicate_all! (is_number_infinite, is_number_infinite_all_2, is_number_infinite_all_3, is_number_infinite_all_4, is_number_infinite_all_n);
def_fn_try_predicate_any! (is_number_infinite, is_number_infinite_any_2, is_number_infinite_any_3, is_number_infinite_any_4, is_number_infinite_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_nan (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					succeed! (false),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_nan ()),
			},
		_ =>
			fail! (0xa3ce47bf),
	}
}

def_fn_try_predicate_all! (is_number_nan, is_number_nan_all_2, is_number_nan_all_3, is_number_nan_all_4, is_number_nan_all_n);
def_fn_try_predicate_any! (is_number_nan, is_number_nan_any_2, is_number_nan_any_3, is_number_nan_any_4, is_number_nan_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_even (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (value.is_even ()),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_even ()),
			},
		_ =>
			fail! (0x4baca78b),
	}
}

def_fn_try_predicate_all! (is_number_even, is_number_even_all_2, is_number_even_all_3, is_number_even_all_4, is_number_even_all_n);
def_fn_try_predicate_any! (is_number_even, is_number_even_any_2, is_number_even_any_3, is_number_even_any_4, is_number_even_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_number_odd (value : &Value) -> (Outcome<bool>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (value.is_odd ()),
				NumberMatchAsRef::Real (value) =>
					succeed! (value.is_odd ()),
			},
		_ =>
			fail! (0x8b6565ee),
	}
}

def_fn_try_predicate_all! (is_number_odd, is_number_odd_all_2, is_number_odd_all_3, is_number_odd_all_4, is_number_odd_all_n);
def_fn_try_predicate_any! (is_number_odd, is_number_odd_any_2, is_number_odd_any_3, is_number_odd_any_4, is_number_odd_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_character (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Character);
}

def_fn_predicate_all! (is_character, is_character_all_2, is_character_all_3, is_character_all_4, is_character_all_n);
def_fn_predicate_any! (is_character, is_character_any_2, is_character_any_3, is_character_any_4, is_character_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_symbol (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Symbol);
}

def_fn_predicate_all! (is_symbol, is_symbol_all_2, is_symbol_all_3, is_symbol_all_4, is_symbol_all_n);
def_fn_predicate_any! (is_symbol, is_symbol_any_2, is_symbol_any_3, is_symbol_any_4, is_symbol_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_symbol_eq (expected : &str, value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Symbol (symbol) =>
			return symbol.string_eq (expected),
		_ =>
			return false,
	}
}

def_fn_predicate_all_x1! (is_symbol_eq, &str, is_symbol_eq_all_2, is_symbol_eq_all_3, is_symbol_eq_all_4, is_symbol_eq_all_n);
def_fn_predicate_any_x1! (is_symbol_eq, &str, is_symbol_eq_any_2, is_symbol_eq_any_3, is_symbol_eq_any_4, is_symbol_eq_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_pair (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Pair);
}

def_fn_predicate_all! (is_pair, is_pair_all_2, is_pair_all_3, is_pair_all_4, is_pair_all_n);
def_fn_predicate_any! (is_pair, is_pair_any_2, is_pair_any_3, is_pair_any_4, is_pair_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_pair_immutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::PairImmutable);
}

def_fn_predicate_all! (is_pair_immutable, is_pair_immutable_all_2, is_pair_immutable_all_3, is_pair_immutable_all_4, is_pair_immutable_all_n);
def_fn_predicate_any! (is_pair_immutable, is_pair_immutable_any_2, is_pair_immutable_any_3, is_pair_immutable_any_4, is_pair_immutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_pair_mutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::PairMutable);
}

def_fn_predicate_all! (is_pair_mutable, is_pair_mutable_all_2, is_pair_mutable_all_3, is_pair_mutable_all_4, is_pair_mutable_all_n);
def_fn_predicate_any! (is_pair_mutable, is_pair_mutable_any_2, is_pair_mutable_any_3, is_pair_mutable_any_4, is_pair_mutable_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list (value : &Value) -> (bool) {
	if let Some (class) = list_class_o1 (value) {
		return class == ListClass::Cell;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list, is_list_all_2, is_list_all_3, is_list_all_4, is_list_all_n);
def_fn_predicate_any! (is_list, is_list_any_2, is_list_any_3, is_list_any_4, is_list_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_empty (value : &Value) -> (bool) {
	if let Some (class) = list_class_o1 (value) {
		return class == ListClass::Empty;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_empty, is_list_empty_all_2, is_list_empty_all_3, is_list_empty_all_4, is_list_empty_all_n);
def_fn_predicate_any! (is_list_empty, is_list_empty_any_2, is_list_empty_any_3, is_list_empty_any_4, is_list_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_or_empty (value : &Value) -> (bool) {
	if let Some (class) = list_class_o1 (value) {
		return (class == ListClass::Cell) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_or_empty, is_list_or_empty_all_2, is_list_or_empty_all_3, is_list_or_empty_all_4, is_list_or_empty_all_n);
def_fn_predicate_any! (is_list_or_empty, is_list_or_empty_any_2, is_list_or_empty_any_3, is_list_or_empty_any_4, is_list_or_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_proper (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return class == ListClass::Proper;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_proper, is_list_proper_all_2, is_list_proper_all_3, is_list_proper_all_4, is_list_proper_all_n);
def_fn_predicate_any! (is_list_proper, is_list_proper_any_2, is_list_proper_any_3, is_list_proper_any_4, is_list_proper_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_proper_or_empty (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return (class == ListClass::Proper) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_proper_or_empty, is_list_proper_or_empty_all_2, is_list_proper_or_empty_all_3, is_list_proper_or_empty_all_4, is_list_proper_or_empty_all_n);
def_fn_predicate_any! (is_list_proper_or_empty, is_list_proper_or_empty_any_2, is_list_proper_or_empty_any_3, is_list_proper_or_empty_any_4, is_list_proper_or_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_dotted (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return class == ListClass::Dotted;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_dotted, is_list_dotted_all_2, is_list_dotted_all_3, is_list_dotted_all_4, is_list_dotted_all_n);
def_fn_predicate_any! (is_list_dotted, is_list_dotted_any_2, is_list_dotted_any_3, is_list_dotted_any_4, is_list_dotted_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_dotted_or_empty (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return (class == ListClass::Dotted) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_dotted_or_empty, is_list_dotted_or_empty_all_2, is_list_dotted_or_empty_all_3, is_list_dotted_or_empty_all_4, is_list_dotted_or_empty_all_n);
def_fn_predicate_any! (is_list_dotted_or_empty, is_list_dotted_or_empty_any_2, is_list_dotted_or_empty_any_3, is_list_dotted_or_empty_any_4, is_list_dotted_or_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_cyclic (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return class == ListClass::Cyclic;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_cyclic, is_list_cyclic_all_2, is_list_cyclic_all_3, is_list_cyclic_all_4, is_list_cyclic_all_n);
def_fn_predicate_any! (is_list_cyclic, is_list_cyclic_any_2, is_list_cyclic_any_3, is_list_cyclic_any_4, is_list_cyclic_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_list_cyclic_or_empty (value : &Value) -> (bool) {
	if let Some (class) = list_class_on (value) {
		return (class == ListClass::Cyclic) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_cyclic_or_empty, is_list_cyclic_or_empty_all_2, is_list_cyclic_or_empty_all_3, is_list_cyclic_or_empty_all_4, is_list_cyclic_or_empty_all_n);
def_fn_predicate_any! (is_list_cyclic_or_empty, is_list_cyclic_or_empty_any_2, is_list_cyclic_or_empty_any_3, is_list_cyclic_or_empty_any_4, is_list_cyclic_or_empty_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Array);
}

def_fn_predicate_all! (is_array, is_array_all_2, is_array_all_3, is_array_all_4, is_array_all_n);
def_fn_predicate_any! (is_array, is_array_any_2, is_array_any_3, is_array_any_4, is_array_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_immutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::ArrayImmutable);
}

def_fn_predicate_all! (is_array_immutable, is_array_immutable_all_2, is_array_immutable_all_3, is_array_immutable_all_4, is_array_immutable_all_n);
def_fn_predicate_any! (is_array_immutable, is_array_immutable_any_2, is_array_immutable_any_3, is_array_immutable_any_4, is_array_immutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_mutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::ArrayMutable);
}

def_fn_predicate_all! (is_array_mutable, is_array_mutable_all_2, is_array_mutable_all_3, is_array_mutable_all_4, is_array_mutable_all_n);
def_fn_predicate_any! (is_array_mutable, is_array_mutable_any_2, is_array_mutable_any_3, is_array_mutable_any_4, is_array_mutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_array_ref! (value) .values_is_empty ());
}

def_fn_try_predicate_all! (is_array_empty, is_array_empty_all_2, is_array_empty_all_3, is_array_empty_all_4, is_array_empty_all_n);
def_fn_try_predicate_any! (is_array_empty, is_array_empty_any_2, is_array_empty_any_3, is_array_empty_any_4, is_array_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_immutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_array_immutable_ref! (value);
	succeed! (value.array_ref () .values_is_empty ());
}

def_fn_try_predicate_all! (is_array_immutable_empty, is_array_immutable_empty_all_2, is_array_immutable_empty_all_3, is_array_immutable_empty_all_4, is_array_immutable_empty_all_n);
def_fn_try_predicate_any! (is_array_immutable_empty, is_array_immutable_empty_any_2, is_array_immutable_empty_any_3, is_array_immutable_empty_any_4, is_array_immutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_mutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_array_mutable_ref! (value);
	succeed! (value.array_ref () .values_is_empty ());
}

def_fn_try_predicate_all! (is_array_mutable_empty, is_array_mutable_empty_all_2, is_array_mutable_empty_all_3, is_array_mutable_empty_all_4, is_array_mutable_empty_all_n);
def_fn_try_predicate_any! (is_array_mutable_empty, is_array_mutable_empty_any_2, is_array_mutable_empty_any_3, is_array_mutable_empty_any_4, is_array_mutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_array_ref! (value) .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_array_not_empty, is_array_not_empty_all_2, is_array_not_empty_all_3, is_array_not_empty_all_4, is_array_not_empty_all_n);
def_fn_try_predicate_any! (is_array_not_empty, is_array_not_empty_any_2, is_array_not_empty_any_3, is_array_not_empty_any_4, is_array_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_immutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_array_immutable_ref! (value);
	succeed! (value.array_ref () .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_array_immutable_not_empty, is_array_immutable_not_empty_all_2, is_array_immutable_not_empty_all_3, is_array_immutable_not_empty_all_4, is_array_immutable_not_empty_all_n);
def_fn_try_predicate_any! (is_array_immutable_not_empty, is_array_immutable_not_empty_any_2, is_array_immutable_not_empty_any_3, is_array_immutable_not_empty_any_4, is_array_immutable_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_array_mutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_array_mutable_ref! (value);
	succeed! (value.array_ref () .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_array_mutable_not_empty, is_array_mutable_not_empty_all_2, is_array_mutable_not_empty_all_3, is_array_mutable_not_empty_all_4, is_array_mutable_not_empty_all_n);
def_fn_try_predicate_any! (is_array_mutable_not_empty, is_array_mutable_not_empty_any_2, is_array_mutable_not_empty_any_3, is_array_mutable_not_empty_any_4, is_array_mutable_not_empty_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Bytes);
}

def_fn_predicate_all! (is_bytes, is_bytes_all_2, is_bytes_all_3, is_bytes_all_4, is_bytes_all_n);
def_fn_predicate_any! (is_bytes, is_bytes_any_2, is_bytes_any_3, is_bytes_any_4, is_bytes_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_immutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::BytesImmutable);
}

def_fn_predicate_all! (is_bytes_immutable, is_bytes_immutable_all_2, is_bytes_immutable_all_3, is_bytes_immutable_all_4, is_bytes_immutable_all_n);
def_fn_predicate_any! (is_bytes_immutable, is_bytes_immutable_any_2, is_bytes_immutable_any_3, is_bytes_immutable_any_4, is_bytes_immutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_mutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::BytesMutable);
}

def_fn_predicate_all! (is_bytes_mutable, is_bytes_mutable_all_2, is_bytes_mutable_all_3, is_bytes_mutable_all_4, is_bytes_mutable_all_n);
def_fn_predicate_any! (is_bytes_mutable, is_bytes_mutable_any_2, is_bytes_mutable_any_3, is_bytes_mutable_any_4, is_bytes_mutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_bytes_ref! (value) .bytes_is_empty ());
}

def_fn_try_predicate_all! (is_bytes_empty, is_bytes_empty_all_2, is_bytes_empty_all_3, is_bytes_empty_all_4, is_bytes_empty_all_n);
def_fn_try_predicate_any! (is_bytes_empty, is_bytes_empty_any_2, is_bytes_empty_any_3, is_bytes_empty_any_4, is_bytes_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_immutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_bytes_immutable_ref! (value);
	succeed! (value.bytes_ref () .bytes_is_empty ());
}

def_fn_try_predicate_all! (is_bytes_immutable_empty, is_bytes_immutable_empty_all_2, is_bytes_immutable_empty_all_3, is_bytes_immutable_empty_all_4, is_bytes_immutable_empty_all_n);
def_fn_try_predicate_any! (is_bytes_immutable_empty, is_bytes_immutable_empty_any_2, is_bytes_immutable_empty_any_3, is_bytes_immutable_empty_any_4, is_bytes_immutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_mutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_bytes_mutable_ref! (value);
	succeed! (value.bytes_ref () .bytes_is_empty ());
}

def_fn_try_predicate_all! (is_bytes_mutable_empty, is_bytes_mutable_empty_all_2, is_bytes_mutable_empty_all_3, is_bytes_mutable_empty_all_4, is_bytes_mutable_empty_all_n);
def_fn_try_predicate_any! (is_bytes_mutable_empty, is_bytes_mutable_empty_any_2, is_bytes_mutable_empty_any_3, is_bytes_mutable_empty_any_4, is_bytes_mutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_bytes_ref! (value) .bytes_is_not_empty ());
}

def_fn_try_predicate_all! (is_bytes_not_empty, is_bytes_not_empty_all_2, is_bytes_not_empty_all_3, is_bytes_not_empty_all_4, is_bytes_not_empty_all_n);
def_fn_try_predicate_any! (is_bytes_not_empty, is_bytes_not_empty_any_2, is_bytes_not_empty_any_3, is_bytes_not_empty_any_4, is_bytes_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_immutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_bytes_immutable_ref! (value);
	succeed! (value.bytes_ref () .bytes_is_not_empty ());
}

def_fn_try_predicate_all! (is_bytes_immutable_not_empty, is_bytes_immutable_not_empty_all_2, is_bytes_immutable_not_empty_all_3, is_bytes_immutable_not_empty_all_4, is_bytes_immutable_not_empty_all_n);
def_fn_try_predicate_any! (is_bytes_immutable_not_empty, is_bytes_immutable_not_empty_any_2, is_bytes_immutable_not_empty_any_3, is_bytes_immutable_not_empty_any_4, is_bytes_immutable_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_bytes_mutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_bytes_mutable_ref! (value);
	succeed! (value.bytes_ref () .bytes_is_not_empty ());
}

def_fn_try_predicate_all! (is_bytes_mutable_not_empty, is_bytes_mutable_not_empty_all_2, is_bytes_mutable_not_empty_all_3, is_bytes_mutable_not_empty_all_4, is_bytes_mutable_not_empty_all_n);
def_fn_try_predicate_any! (is_bytes_mutable_not_empty, is_bytes_mutable_not_empty_any_2, is_bytes_mutable_not_empty_any_3, is_bytes_mutable_not_empty_any_4, is_bytes_mutable_not_empty_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string (value : &Value) -> (bool) {
	return value.is_class (ValueClass::String);
}

def_fn_predicate_all! (is_string, is_string_all_2, is_string_all_3, is_string_all_4, is_string_all_n);
def_fn_predicate_any! (is_string, is_string_any_2, is_string_any_3, is_string_any_4, is_string_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_immutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::StringImmutable);
}

def_fn_predicate_all! (is_string_immutable, is_string_immutable_all_2, is_string_immutable_all_3, is_string_immutable_all_4, is_string_immutable_all_n);
def_fn_predicate_any! (is_string_immutable, is_string_immutable_any_2, is_string_immutable_any_3, is_string_immutable_any_4, is_string_immutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_mutable (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::StringMutable);
}

def_fn_predicate_all! (is_string_mutable, is_string_mutable_all_2, is_string_mutable_all_3, is_string_mutable_all_4, is_string_mutable_all_n);
def_fn_predicate_any! (is_string_mutable, is_string_mutable_any_2, is_string_mutable_any_3, is_string_mutable_any_4, is_string_mutable_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_string_ref! (value) .string_is_empty ());
}

def_fn_try_predicate_all! (is_string_empty, is_string_empty_all_2, is_string_empty_all_3, is_string_empty_all_4, is_string_empty_all_n);
def_fn_try_predicate_any! (is_string_empty, is_string_empty_any_2, is_string_empty_any_3, is_string_empty_any_4, is_string_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_immutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_string_immutable_ref! (value);
	succeed! (value.string_ref () .string_is_empty ());
}

def_fn_try_predicate_all! (is_string_immutable_empty, is_string_immutable_empty_all_2, is_string_immutable_empty_all_3, is_string_immutable_empty_all_4, is_string_immutable_empty_all_n);
def_fn_try_predicate_any! (is_string_immutable_empty, is_string_immutable_empty_any_2, is_string_immutable_empty_any_3, is_string_immutable_empty_any_4, is_string_immutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_mutable_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_string_mutable_ref! (value);
	succeed! (value.string_ref () .string_is_empty ());
}

def_fn_try_predicate_all! (is_string_mutable_empty, is_string_mutable_empty_all_2, is_string_mutable_empty_all_3, is_string_mutable_empty_all_4, is_string_mutable_empty_all_n);
def_fn_try_predicate_any! (is_string_mutable_empty, is_string_mutable_empty_any_2, is_string_mutable_empty_any_3, is_string_mutable_empty_any_4, is_string_mutable_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_string_ref! (value) .string_is_not_empty ());
}

def_fn_try_predicate_all! (is_string_not_empty, is_string_not_empty_all_2, is_string_not_empty_all_3, is_string_not_empty_all_4, is_string_not_empty_all_n);
def_fn_try_predicate_any! (is_string_not_empty, is_string_not_empty_any_2, is_string_not_empty_any_3, is_string_not_empty_any_4, is_string_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_immutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_string_immutable_ref! (value);
	succeed! (value.string_ref () .string_is_not_empty ());
}

def_fn_try_predicate_all! (is_string_immutable_not_empty, is_string_immutable_not_empty_all_2, is_string_immutable_not_empty_all_3, is_string_immutable_not_empty_all_4, is_string_immutable_not_empty_all_n);
def_fn_try_predicate_any! (is_string_immutable_not_empty, is_string_immutable_not_empty_any_2, is_string_immutable_not_empty_any_3, is_string_immutable_not_empty_any_4, is_string_immutable_not_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_string_mutable_not_empty (value : &Value) -> (Outcome<bool>) {
	let value = try_as_string_mutable_ref! (value);
	succeed! (value.string_ref () .string_is_not_empty ());
}

def_fn_try_predicate_all! (is_string_mutable_not_empty, is_string_mutable_not_empty_all_2, is_string_mutable_not_empty_all_3, is_string_mutable_not_empty_all_4, is_string_mutable_not_empty_all_n);
def_fn_try_predicate_any! (is_string_mutable_not_empty, is_string_mutable_not_empty_any_2, is_string_mutable_not_empty_any_3, is_string_mutable_not_empty_any_4, is_string_mutable_not_empty_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_values (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Values);
}

def_fn_predicate_all! (is_values, is_values_all_2, is_values_all_3, is_values_all_4, is_values_all_n);
def_fn_predicate_any! (is_values, is_values_any_2, is_values_any_3, is_values_any_4, is_values_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_values_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_values_ref! (value) .values_is_empty ());
}

def_fn_try_predicate_all! (is_values_empty, is_values_empty_all_2, is_values_empty_all_3, is_values_empty_all_4, is_values_empty_all_n);
def_fn_try_predicate_any! (is_values_empty, is_values_empty_any_2, is_values_empty_any_3, is_values_empty_any_4, is_values_empty_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_values_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_values_ref! (value) .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_values_not_empty, is_values_not_empty_all_2, is_values_not_empty_all_3, is_values_not_empty_all_4, is_values_not_empty_all_n);
def_fn_try_predicate_any! (is_values_not_empty, is_values_not_empty_any_2, is_values_not_empty_any_3, is_values_not_empty_any_4, is_values_not_empty_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_procedure (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Procedure);
}

def_fn_predicate_all! (is_procedure, is_procedure_all_2, is_procedure_all_3, is_procedure_all_4, is_procedure_all_n);
def_fn_predicate_any! (is_procedure, is_procedure_any_2, is_procedure_any_3, is_procedure_any_4, is_procedure_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_syntax (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Syntax);
}

def_fn_predicate_all! (is_syntax, is_syntax_all_2, is_syntax_all_3, is_syntax_all_4, is_syntax_all_n);
def_fn_predicate_any! (is_syntax, is_syntax_any_2, is_syntax_any_3, is_syntax_any_4, is_syntax_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Error);
}

def_fn_predicate_all! (is_error, is_error_all_2, is_error_all_3, is_error_all_4, is_error_all_n);
def_fn_predicate_any! (is_error, is_error_any_2, is_error_any_3, is_error_any_4, is_error_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error_syntax (value : &Value) -> (Outcome<bool>) {
	let _value = try_as_error_ref! (value);
	fail_unimplemented! (0x18d9951d);
}

def_fn_try_predicate_all! (is_error_syntax, is_error_syntax_all_2, is_error_syntax_all_3, is_error_syntax_all_4, is_error_syntax_all_n);
def_fn_try_predicate_any! (is_error_syntax, is_error_syntax_any_2, is_error_syntax_any_3, is_error_syntax_any_4, is_error_syntax_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error_file (value : &Value) -> (Outcome<bool>) {
	let _value = try_as_error_ref! (value);
	fail_unimplemented! (0xdc61fd91);
}

def_fn_try_predicate_all! (is_error_file, is_error_file_all_2, is_error_file_all_3, is_error_file_all_4, is_error_file_all_n);
def_fn_try_predicate_any! (is_error_file, is_error_file_any_2, is_error_file_any_3, is_error_file_any_4, is_error_file_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error_port (value : &Value) -> (Outcome<bool>) {
	let _value = try_as_error_ref! (value);
	fail_unimplemented! (0xc1084d3e);
}

def_fn_try_predicate_all! (is_error_port, is_error_port_all_2, is_error_port_all_3, is_error_port_all_4, is_error_port_all_n);
def_fn_try_predicate_any! (is_error_port, is_error_port_any_2, is_error_port_any_3, is_error_port_any_4, is_error_port_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error_port_input (value : &Value) -> (Outcome<bool>) {
	let _value = try_as_error_ref! (value);
	fail_unimplemented! (0xb0f9d9e5);
}

def_fn_try_predicate_all! (is_error_port_input, is_error_port_input_all_2, is_error_port_input_all_3, is_error_port_input_all_4, is_error_port_input_all_n);
def_fn_try_predicate_any! (is_error_port_input, is_error_port_input_any_2, is_error_port_input_any_3, is_error_port_input_any_4, is_error_port_input_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_error_port_output (value : &Value) -> (Outcome<bool>) {
	let _value = try_as_error_ref! (value);
	fail_unimplemented! (0x2ec6d6b7);
}

def_fn_try_predicate_all! (is_error_port_output, is_error_port_output_all_2, is_error_port_output_all_3, is_error_port_output_all_4, is_error_port_output_all_n);
def_fn_try_predicate_any! (is_error_port_output, is_error_port_output_any_2, is_error_port_output_any_3, is_error_port_output_any_4, is_error_port_output_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Port);
}

def_fn_predicate_all! (is_port, is_port_all_2, is_port_all_3, is_port_all_4, is_port_all_n);
def_fn_predicate_any! (is_port, is_port_any_2, is_port_any_3, is_port_any_4, is_port_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_input (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_read_implemented ());
}

def_fn_try_predicate_all! (is_port_input, is_port_input_all_2, is_port_input_all_3, is_port_input_all_4, is_port_input_all_n);
def_fn_try_predicate_any! (is_port_input, is_port_input_any_2, is_port_input_any_3, is_port_input_any_4, is_port_input_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_output (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_write_implemented ());
}

def_fn_try_predicate_all! (is_port_output, is_port_output_all_2, is_port_output_all_3, is_port_output_all_4, is_port_output_all_n);
def_fn_try_predicate_any! (is_port_output, is_port_output_any_2, is_port_output_any_3, is_port_output_any_4, is_port_output_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_binary (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_byte_implemented ());
}

def_fn_try_predicate_all! (is_port_binary, is_port_binary_all_2, is_port_binary_all_3, is_port_binary_all_4, is_port_binary_all_n);
def_fn_try_predicate_any! (is_port_binary, is_port_binary_any_2, is_port_binary_any_3, is_port_binary_any_4, is_port_binary_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_textual (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_char_implemented ());
}

def_fn_try_predicate_all! (is_port_textual, is_port_textual_all_2, is_port_textual_all_3, is_port_textual_all_4, is_port_textual_all_n);
def_fn_try_predicate_any! (is_port_textual, is_port_textual_any_2, is_port_textual_any_3, is_port_textual_any_4, is_port_textual_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_eof (value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Singleton (ValueSingleton::PortEof) =>
			return true,
		_ =>
			return false,
	}
}

def_fn_predicate_all! (is_port_eof, is_port_eof_all_2, is_port_eof_all_3, is_port_eof_all_4, is_port_eof_all_n);
def_fn_predicate_any! (is_port_eof, is_port_eof_any_2, is_port_eof_any_3, is_port_eof_any_4, is_port_eof_any_n);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_process (value : &Value) -> (bool) {
	return value.is_kind (ValueKind::Process);
}

def_fn_predicate_all! (is_process, is_process_all_2, is_process_all_3, is_process_all_4, is_process_all_n);
def_fn_predicate_any! (is_process, is_process_any_2, is_process_any_3, is_process_any_4, is_process_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_resource (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Resource);
}

def_fn_predicate_all! (is_resource, is_resource_all_2, is_resource_all_3, is_resource_all_4, is_resource_all_n);
def_fn_predicate_any! (is_resource, is_resource_any_2, is_resource_any_3, is_resource_any_4, is_resource_any_n);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_opaque (value : &Value) -> (bool) {
	return value.is_class (ValueClass::Opaque);
}

def_fn_predicate_all! (is_opaque, is_opaque_all_2, is_opaque_all_3, is_opaque_all_4, is_opaque_all_n);
def_fn_predicate_any! (is_opaque, is_opaque_any_2, is_opaque_any_3, is_opaque_any_4, is_opaque_any_n);




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum NumberClass {
	Integer,
	Real,
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_class (value : &Value) -> (Option<NumberClass>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (_) =>
					return Some (NumberClass::Integer),
				NumberMatchAsRef::Real (_) =>
					return Some (NumberClass::Real),
			},
		_ =>
			return None
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListClass {
	Empty,
	Proper,
	Dotted,
	Cyclic,
	Cell,
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_class_o1 (value : &Value) -> (Option<ListClass>) {
	match value.list_match_as_ref () {
		ListMatchAsRef::Null =>
			return Some (ListClass::Empty),
		ListMatchAsRef::PairImmutable (_) =>
			return Some (ListClass::Cell),
		ListMatchAsRef::PairMutable (_) =>
			return Some (ListClass::Cell),
		ListMatchAsRef::Value (_) =>
			return None,
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_class_on (value : &Value) -> (Option<ListClass>) {
	match value.list_match_as_ref () {
		ListMatchAsRef::Null =>
			return Some (ListClass::Empty),
		ListMatchAsRef::PairImmutable (pair) =>
			return Some (list_class_on_0 (value, pair.right ())),
		ListMatchAsRef::PairMutable (pair) =>
			return Some (list_class_on_0 (value, pair.pair_ref () .right ())),
		ListMatchAsRef::Value (_) =>
			return None,
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn list_class_on_0 (value : &Value, cursor : &Value) -> (ListClass) {
	let mut cursor = cursor;
	loop {
		if value.is_self (cursor) {
			return ListClass::Cyclic;
		}
		match cursor.list_match_as_ref () {
			ListMatchAsRef::Null =>
				return ListClass::Proper,
			ListMatchAsRef::PairImmutable (pair) =>
				cursor = pair.right (),
			ListMatchAsRef::PairMutable (pair) =>
				return list_class_on_0 (value, pair.pair_ref () .right ()),
			ListMatchAsRef::Value (_) =>
				return ListClass::Dotted,
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedureClass {
	Primitive,
	Extended,
	Native,
	Lambda,
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_class (value : &Value) -> (Option<ProcedureClass>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Procedure (class) =>
			match class {
				ProcedureMatchAsRef::Primitive (_) =>
					return Some (ProcedureClass::Primitive),
				ProcedureMatchAsRef::Extended (_) =>
					return Some (ProcedureClass::Extended),
				ProcedureMatchAsRef::Native (_) =>
					return Some (ProcedureClass::Native),
				ProcedureMatchAsRef::Lambda (_) =>
					return Some (ProcedureClass::Lambda),
			},
		_ =>
			return None,
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxClass {
	Primitive,
	Extended,
	Native,
	Lambda,
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn syntax_class (value : &Value) -> (Option<SyntaxClass>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Syntax (class) =>
			match class {
				SyntaxMatchAsRef::Primitive (_) =>
					return Some (SyntaxClass::Primitive),
				SyntaxMatchAsRef::Extended (_) =>
					return Some (SyntaxClass::Extended),
				SyntaxMatchAsRef::Native (_) =>
					return Some (SyntaxClass::Native),
				SyntaxMatchAsRef::Lambda (_) =>
					return Some (SyntaxClass::Lambda),
			},
		_ =>
			return None,
	}
}

