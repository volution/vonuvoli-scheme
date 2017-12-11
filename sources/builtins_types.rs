

use super::errors::exports::*;
use super::ports::exports::*;
use super::values::exports::*;




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
			
			is_number,
			
			is_number_all_2, is_number_all_3, is_number_all_4, is_number_all_n,
			
	};
	
	
	pub use super::{
			
			is_list, is_list_empty, is_list_or_empty,
			is_list_proper, is_list_proper_or_empty,
			is_list_dotted, is_list_dotted_or_empty,
			is_list_cyclic, is_list_cyclic_or_empty,
			
			is_list_empty_all_2, is_list_empty_all_3, is_list_empty_all_4, is_list_empty_all_n,
			is_list_empty_any_2, is_list_empty_any_3, is_list_empty_any_4, is_list_empty_any_n,
			
	};
	
	
	pub use super::{
			
			is_array, is_array_empty, is_array_not_empty,
			
			is_array_empty_all_2, is_array_empty_all_3, is_array_empty_all_4,
			is_array_empty_any_2, is_array_empty_any_3, is_array_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_bytes, is_bytes_empty, is_bytes_not_empty,
			
			is_bytes_empty_all_2, is_bytes_empty_all_3, is_bytes_empty_all_4,
			is_bytes_empty_any_2, is_bytes_empty_any_3, is_bytes_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_string, is_string_empty, is_string_not_empty,
			
			is_string_empty_all_2, is_string_empty_all_3, is_string_empty_all_4,
			is_string_empty_any_2, is_string_empty_any_3, is_string_empty_any_4,
			
	};
	
	
	pub use super::{
			
			is_values, is_values_empty, is_values_not_empty,
			
			is_values_empty_all_2, is_values_empty_all_3, is_values_empty_all_4,
			is_values_empty_any_2, is_values_empty_any_3, is_values_empty_any_4,
			
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
			
			is_port,
			is_port_input, is_port_output,
			is_port_binary, is_port_textual,
			is_port_eof,
			
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
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2);
		}
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2) && $predicate (value_3);
		}
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (value_1) && $predicate (value_2) && $predicate (value_3) && $predicate (value_4);
		}
		pub fn $predicate_n (values : &[Value]) -> (bool) {
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
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2);
		}
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2) || $predicate (value_3);
		}
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
			return $predicate (value_1) || $predicate (value_2) || $predicate (value_3) || $predicate (value_4);
		}
		pub fn $predicate_n (values : &[Value]) -> (bool) {
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
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			succeed! (outcome_1 && outcome_2);
		}
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			succeed! (outcome_1 && outcome_2 && outcome_3);
		}
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			let outcome_4 = try! ($predicate (value_4));
			succeed! (outcome_1 && outcome_2 && outcome_3 && outcome_4);
		}
		pub fn $predicate_n (values : &[Value]) -> (Outcome<bool>) {
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
		pub fn $predicate_2 (value_1 : &Value, value_2 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			succeed! (outcome_1 || outcome_2);
		}
		pub fn $predicate_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			succeed! (outcome_1 || outcome_2 || outcome_3);
		}
		pub fn $predicate_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<bool>) {
			let outcome_1 = try! ($predicate (value_1));
			let outcome_2 = try! ($predicate (value_2));
			let outcome_3 = try! ($predicate (value_3));
			let outcome_4 = try! ($predicate (value_4));
			succeed! (outcome_1 || outcome_2 || outcome_3 || outcome_4);
		}
		pub fn $predicate_n (values : &[Value]) -> (Outcome<bool>) {
			let mut outcome = true;
			for value_i in values {
				let outcome_i = try! ($predicate (value_i));
				outcome = outcome || outcome_i;
			}
			succeed! (outcome);
		}
	);
}




pub fn is_boolean (value : &Value) -> (bool) {
	return value.is (ValueClass::Boolean);
}

def_fn_predicate_all! (is_boolean, is_boolean_all_2, is_boolean_all_3, is_boolean_all_4, is_boolean_all_n);
def_fn_predicate_any! (is_boolean, is_boolean_any_2, is_boolean_any_3, is_boolean_any_4, is_boolean_any_n);




pub fn is_true (value : &Value) -> (bool) {
	if let Ok (value) = Boolean::try_as_ref (value) {
		return value.0 == true;
	} else {
		return false;
	}
}

pub fn is_false (value : &Value) -> (bool) {
	if let Ok (value) = Boolean::try_as_ref (value) {
		return value.0 == false;
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_true, is_true_all_2, is_true_all_3, is_true_all_4, is_true_all_n);
def_fn_predicate_any! (is_true, is_true_any_2, is_true_any_3, is_true_any_4, is_true_any_n);

def_fn_predicate_all! (is_false, is_false_all_2, is_false_all_3, is_false_all_4, is_false_all_n);
def_fn_predicate_any! (is_false, is_false_any_2, is_false_any_3, is_false_any_4, is_false_any_n);


pub fn is_not_true (value : &Value) -> (bool) {
	return !is_true (value);
}

pub fn is_not_false (value : &Value) -> (bool) {
	return !is_false (value);
}

def_fn_predicate_all! (is_not_true, is_not_true_all_2, is_not_true_all_3, is_not_true_all_4, is_not_true_all_n);
def_fn_predicate_any! (is_not_true, is_not_true_any_2, is_not_true_any_3, is_not_true_any_4, is_not_true_any_n);

def_fn_predicate_all! (is_not_false, is_not_false_all_2, is_not_false_all_3, is_not_false_all_4, is_not_false_all_n);
def_fn_predicate_any! (is_not_false, is_not_false_any_2, is_not_false_any_3, is_not_false_any_4, is_not_false_any_n);




pub fn is_true_or_equivalent (value : &Value) -> (bool) {
	!is_false_or_equivalent (value)
}

pub fn is_false_or_equivalent (value : &Value) -> (bool) {
	match value.class () {
		ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
			return true,
		ValueClass::Boolean =>
			return Boolean::as_ref (value) .0 == false,
		ValueClass::Error =>
			return true,
		_ =>
			return false,
	}
}

def_fn_predicate_all! (is_true_or_equivalent, is_true_or_equivalent_all_2, is_true_or_equivalent_all_3, is_true_or_equivalent_all_4, is_true_or_equivalent_all_n);
def_fn_predicate_any! (is_true_or_equivalent, is_true_or_equivalent_any_2, is_true_or_equivalent_any_3, is_true_or_equivalent_any_4, is_true_or_equivalent_any_n);

def_fn_predicate_all! (is_false_or_equivalent, is_false_or_equivalent_all_2, is_false_or_equivalent_all_3, is_false_or_equivalent_all_4, is_false_or_equivalent_all_n);
def_fn_predicate_any! (is_false_or_equivalent, is_false_or_equivalent_any_2, is_false_or_equivalent_any_3, is_false_or_equivalent_any_4, is_false_or_equivalent_any_n);




pub fn is_null (value : &Value) -> (bool) {
	return value.is (ValueClass::Null);
}

pub fn is_void (value : &Value) -> (bool) {
	return value.is (ValueClass::Void);
}

pub fn is_undefined (value : &Value) -> (bool) {
	return value.is (ValueClass::Undefined);
}

def_fn_predicate_all! (is_null, is_null_all_2, is_null_all_3, is_null_all_4, is_null_all_n);
def_fn_predicate_any! (is_null, is_null_any_2, is_null_any_3, is_null_any_4, is_null_any_n);

def_fn_predicate_all! (is_void, is_void_all_2, is_void_all_3, is_void_all_4, is_void_all_n);
def_fn_predicate_any! (is_void, is_void_any_2, is_void_any_3, is_void_any_4, is_void_any_n);

def_fn_predicate_all! (is_undefined, is_undefined_all_2, is_undefined_all_3, is_undefined_all_4, is_undefined_all_n);
def_fn_predicate_any! (is_undefined, is_undefined_any_2, is_undefined_any_3, is_undefined_any_4, is_undefined_any_n);


pub fn is_not_null (value : &Value) -> (bool) {
	return !is_null (value);
}

pub fn is_not_void (value : &Value) -> (bool) {
	return !is_void (value);
}

pub fn is_not_undefined (value : &Value) -> (bool) {
	return !is_undefined (value);
}

def_fn_predicate_all! (is_not_null, is_not_null_all_2, is_not_null_all_3, is_not_null_all_4, is_not_null_all_n);
def_fn_predicate_any! (is_not_null, is_not_null_any_2, is_not_null_any_3, is_not_null_any_4, is_not_null_any_n);

def_fn_predicate_all! (is_not_void, is_not_void_all_2, is_not_void_all_3, is_not_void_all_4, is_not_void_all_n);
def_fn_predicate_any! (is_not_void, is_not_void_any_2, is_not_void_any_3, is_not_void_any_4, is_not_void_any_n);

def_fn_predicate_all! (is_not_undefined, is_not_undefined_all_2, is_not_undefined_all_3, is_not_undefined_all_4, is_not_undefined_all_n);
def_fn_predicate_any! (is_not_undefined, is_not_undefined_any_2, is_not_undefined_any_3, is_not_undefined_any_4, is_not_undefined_any_n);




pub fn is_number (value : &Value) -> (bool) {
	return number_class (value) .is_ok ();
}

def_fn_predicate_all! (is_number, is_number_all_2, is_number_all_3, is_number_all_4, is_number_all_n);
def_fn_predicate_any! (is_number, is_number_any_2, is_number_any_3, is_number_any_4, is_number_any_n);




pub fn is_list (value : &Value) -> (bool) {
	let class = list_class_o1 (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Cell;
	} else {
		return false;
	}
}

pub fn is_list_empty (value : &Value) -> (bool) {
	let class = list_class_o1 (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Empty;
	} else {
		return false;
	}
}

pub fn is_list_or_empty (value : &Value) -> (bool) {
	let class = list_class_o1 (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Cell) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list, is_list_all_2, is_list_all_3, is_list_all_4, is_list_all_n);
def_fn_predicate_any! (is_list, is_list_any_2, is_list_any_3, is_list_any_4, is_list_any_n);

def_fn_predicate_all! (is_list_empty, is_list_empty_all_2, is_list_empty_all_3, is_list_empty_all_4, is_list_empty_all_n);
def_fn_predicate_any! (is_list_empty, is_list_empty_any_2, is_list_empty_any_3, is_list_empty_any_4, is_list_empty_any_n);

def_fn_predicate_all! (is_list_or_empty, is_list_or_empty_all_2, is_list_or_empty_all_3, is_list_or_empty_all_4, is_list_or_empty_all_n);
def_fn_predicate_any! (is_list_or_empty, is_list_or_empty_any_2, is_list_or_empty_any_3, is_list_or_empty_any_4, is_list_or_empty_any_n);


pub fn is_list_proper (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Proper;
	} else {
		return false;
	}
}

pub fn is_list_proper_or_empty (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Proper) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_proper, is_list_proper_all_2, is_list_proper_all_3, is_list_proper_all_4, is_list_proper_all_n);
def_fn_predicate_any! (is_list_proper, is_list_proper_any_2, is_list_proper_any_3, is_list_proper_any_4, is_list_proper_any_n);

def_fn_predicate_all! (is_list_proper_or_empty, is_list_proper_or_empty_all_2, is_list_proper_or_empty_all_3, is_list_proper_or_empty_all_4, is_list_proper_or_empty_all_n);
def_fn_predicate_any! (is_list_proper_or_empty, is_list_proper_or_empty_any_2, is_list_proper_or_empty_any_3, is_list_proper_or_empty_any_4, is_list_proper_or_empty_any_n);


pub fn is_list_dotted (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Dotted;
	} else {
		return false;
	}
}

pub fn is_list_dotted_or_empty (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Dotted) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_dotted, is_list_dotted_all_2, is_list_dotted_all_3, is_list_dotted_all_4, is_list_dotted_all_n);
def_fn_predicate_any! (is_list_dotted, is_list_dotted_any_2, is_list_dotted_any_3, is_list_dotted_any_4, is_list_dotted_any_n);

def_fn_predicate_all! (is_list_dotted_or_empty, is_list_dotted_or_empty_all_2, is_list_dotted_or_empty_all_3, is_list_dotted_or_empty_all_4, is_list_dotted_or_empty_all_n);
def_fn_predicate_any! (is_list_dotted_or_empty, is_list_dotted_or_empty_any_2, is_list_dotted_or_empty_any_3, is_list_dotted_or_empty_any_4, is_list_dotted_or_empty_any_n);


pub fn is_list_cyclic (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Cyclic;
	} else {
		return false;
	}
}

pub fn is_list_cyclic_or_empty (value : &Value) -> (bool) {
	let class = list_class_on (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Cyclic) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

def_fn_predicate_all! (is_list_cyclic, is_list_cyclic_all_2, is_list_cyclic_all_3, is_list_cyclic_all_4, is_list_cyclic_all_n);
def_fn_predicate_any! (is_list_cyclic, is_list_cyclic_any_2, is_list_cyclic_any_3, is_list_cyclic_any_4, is_list_cyclic_any_n);

def_fn_predicate_all! (is_list_cyclic_or_empty, is_list_cyclic_or_empty_all_2, is_list_cyclic_or_empty_all_3, is_list_cyclic_or_empty_all_4, is_list_cyclic_or_empty_all_n);
def_fn_predicate_any! (is_list_cyclic_or_empty, is_list_cyclic_or_empty_any_2, is_list_cyclic_or_empty_any_3, is_list_cyclic_or_empty_any_4, is_list_cyclic_or_empty_any_n);




pub fn is_array (value : &Value) -> (bool) {
	return value.is (ValueClass::Array);
}

def_fn_predicate_all! (is_array, is_array_all_2, is_array_all_3, is_array_all_4, is_array_all_n);
def_fn_predicate_any! (is_array, is_array_any_2, is_array_any_3, is_array_any_4, is_array_any_n);


pub fn is_array_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_array_ref! (value) .values_is_empty ());
}

pub fn is_array_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_array_ref! (value) .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_array_empty, is_array_empty_all_2, is_array_empty_all_3, is_array_empty_all_4, is_array_empty_all_n);
def_fn_try_predicate_any! (is_array_empty, is_array_empty_any_2, is_array_empty_any_3, is_array_empty_any_4, is_array_empty_any_n);

def_fn_try_predicate_all! (is_array_not_empty, is_array_not_empty_all_2, is_array_not_empty_all_3, is_array_not_empty_all_4, is_array_not_empty_all_n);
def_fn_try_predicate_any! (is_array_not_empty, is_array_not_empty_any_2, is_array_not_empty_any_3, is_array_not_empty_any_4, is_array_not_empty_any_n);




pub fn is_bytes (value : &Value) -> (bool) {
	return value.is (ValueClass::Bytes);
}

def_fn_predicate_all! (is_bytes, is_bytes_all_2, is_bytes_all_3, is_bytes_all_4, is_bytes_all_n);
def_fn_predicate_any! (is_bytes, is_bytes_any_2, is_bytes_any_3, is_bytes_any_4, is_bytes_any_n);


pub fn is_bytes_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_bytes_ref! (value) .values_is_empty ());
}

pub fn is_bytes_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_bytes_ref! (value) .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_bytes_empty, is_bytes_empty_all_2, is_bytes_empty_all_3, is_bytes_empty_all_4, is_bytes_empty_all_n);
def_fn_try_predicate_any! (is_bytes_empty, is_bytes_empty_any_2, is_bytes_empty_any_3, is_bytes_empty_any_4, is_bytes_empty_any_n);

def_fn_try_predicate_all! (is_bytes_not_empty, is_bytes_not_empty_all_2, is_bytes_not_empty_all_3, is_bytes_not_empty_all_4, is_bytes_not_empty_all_n);
def_fn_try_predicate_any! (is_bytes_not_empty, is_bytes_not_empty_any_2, is_bytes_not_empty_any_3, is_bytes_not_empty_any_4, is_bytes_not_empty_any_n);




pub fn is_string (value : &Value) -> (bool) {
	return value.is (ValueClass::String);
}

def_fn_predicate_all! (is_string, is_string_all_2, is_string_all_3, is_string_all_4, is_string_all_n);
def_fn_predicate_any! (is_string, is_string_any_2, is_string_any_3, is_string_any_4, is_string_any_n);


pub fn is_string_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_string_ref! (value) .string_is_empty ());
}

pub fn is_string_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_string_ref! (value) .string_is_not_empty ());
}

def_fn_try_predicate_all! (is_string_empty, is_string_empty_all_2, is_string_empty_all_3, is_string_empty_all_4, is_string_empty_all_n);
def_fn_try_predicate_any! (is_string_empty, is_string_empty_any_2, is_string_empty_any_3, is_string_empty_any_4, is_string_empty_any_n);

def_fn_try_predicate_all! (is_string_not_empty, is_string_not_empty_all_2, is_string_not_empty_all_3, is_string_not_empty_all_4, is_string_not_empty_all_n);
def_fn_try_predicate_any! (is_string_not_empty, is_string_not_empty_any_2, is_string_not_empty_any_3, is_string_not_empty_any_4, is_string_not_empty_any_n);




pub fn is_values (value : &Value) -> (bool) {
	return value.is (ValueClass::Values);
}

def_fn_predicate_all! (is_values, is_values_all_2, is_values_all_3, is_values_all_4, is_values_all_n);
def_fn_predicate_any! (is_values, is_values_any_2, is_values_any_3, is_values_any_4, is_values_any_n);


pub fn is_values_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_values_ref! (value) .values_is_empty ());
}

pub fn is_values_not_empty (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_values_ref! (value) .values_is_not_empty ());
}

def_fn_try_predicate_all! (is_values_empty, is_values_empty_all_2, is_values_empty_all_3, is_values_empty_all_4, is_values_empty_all_n);
def_fn_try_predicate_any! (is_values_empty, is_values_empty_any_2, is_values_empty_any_3, is_values_empty_any_4, is_values_empty_any_n);

def_fn_try_predicate_all! (is_values_not_empty, is_values_not_empty_all_2, is_values_not_empty_all_3, is_values_not_empty_all_4, is_values_not_empty_all_n);
def_fn_try_predicate_any! (is_values_not_empty, is_values_not_empty_any_2, is_values_not_empty_any_3, is_values_not_empty_any_4, is_values_not_empty_any_n);




pub fn is_procedure (value : &Value) -> (bool) {
	return procedure_class (value) .is_ok ();
}

def_fn_predicate_all! (is_procedure, is_procedure_all_2, is_procedure_all_3, is_procedure_all_4, is_procedure_all_n);
def_fn_predicate_any! (is_procedure, is_procedure_any_2, is_procedure_any_3, is_procedure_any_4, is_procedure_any_n);




pub fn is_syntax (value : &Value) -> (bool) {
	return syntax_class (value) .is_ok ();
}

def_fn_predicate_all! (is_syntax, is_syntax_all_2, is_syntax_all_3, is_syntax_all_4, is_syntax_all_n);
def_fn_predicate_any! (is_syntax, is_syntax_any_2, is_syntax_any_3, is_syntax_any_4, is_syntax_any_n);




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum NumberClass {
	Integer,
	Real,
}


pub fn number_class (value : &Value) -> (Outcome<NumberClass>) {
	match value.class () {
		
		ValueClass::NumberInteger =>
			succeed! (NumberClass::Integer),
		
		ValueClass::NumberReal =>
			succeed! (NumberClass::Real),
		
		_ =>
			fail! (0x7a6c3f3e),
		
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


pub fn list_class_o1 (value : &Value) -> (Outcome<ListClass>) {
	match value.class () {
		
		ValueClass::Null =>
			succeed! (ListClass::Empty),
		
		ValueClass::Pair =>
			succeed! (ListClass::Cell),
		
		_ =>
			fail! (0x355bf0c9),
		
	}
}


pub fn list_class_on (value : &Value) -> (Outcome<ListClass>) {
	match value.class () {
		
		ValueClass::Null =>
			succeed! (ListClass::Empty),
		
		ValueClass::Pair => {
			let mut cursor = Pair::as_ref (value) .right ();
			loop {
				match cursor.class () {
					ValueClass::Pair =>
						cursor = Pair::as_ref (cursor) .right (),
					ValueClass::Null =>
						succeed! (ListClass::Proper),
					_ =>
						succeed! (ListClass::Dotted),
				}
				if value.is_self (cursor) {
					succeed! (ListClass::Cyclic);
				}
			}
		},
		
		_ =>
			fail! (0xf9bfa236),
		
	}
}




pub fn is_port (value : &Value) -> (bool) {
	return value.is (ValueClass::Port);
}

pub fn is_port_input (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_read_implemented ());
}

pub fn is_port_output (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_write_implemented ());
}

pub fn is_port_binary (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_byte_implemented ());
}

pub fn is_port_textual (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (value) .is_char_implemented ());
}

pub fn is_port_eof (value : &Value) -> (bool) {
	return value.is (ValueClass::Singleton) && (*ValueSingleton::as_ref (value) == ValueSingleton::PortEof);
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedureClass {
	Primitive,
	Extended,
	Lambda,
}


pub fn procedure_class (value : &Value) -> (Outcome<ProcedureClass>) {
	match value.class () {
		ValueClass::ProcedurePrimitive =>
			succeed! (ProcedureClass::Primitive),
		ValueClass::ProcedureExtended =>
			succeed! (ProcedureClass::Extended),
		ValueClass::ProcedureLambda =>
			succeed! (ProcedureClass::Lambda),
		_ =>
			fail! (0xef418db1),
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxClass {
	Primitive,
	Extended,
	Lambda,
}


pub fn syntax_class (value : &Value) -> (Outcome<SyntaxClass>) {
	match value.class () {
		ValueClass::SyntaxPrimitive =>
			succeed! (SyntaxClass::Primitive),
		ValueClass::SyntaxExtended =>
			succeed! (SyntaxClass::Extended),
		ValueClass::SyntaxLambda =>
			succeed! (SyntaxClass::Lambda),
		_ =>
			fail! (0x97144c3b),
	}
}

