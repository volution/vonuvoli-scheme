

use super::constants::exports::*;
use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::StringPrimitive0;
	pub use super::StringPrimitive1;
	pub use super::StringPrimitive2;
	pub use super::StringPrimitive3;
	pub use super::StringPrimitive4;
	pub use super::StringPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive0 {
	
	StringBuild,
	StringAppend,
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive1 {
	
	StringLength,
	StringClone,
	StringReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive2 {
	
	StringAt,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive3 {
	
	StringAtSet,
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive4 {
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitiveN {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringSliceFill,
	StringSliceCopy,
	StringSliceClone,
	
}




pub fn string_primitive_0_evaluate (primitive : StringPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive0::StringBuild =>
			succeed! (string_empty ()),
		
		StringPrimitive0::StringAppend =>
			succeed! (string_empty ()),
		
	}
}




pub fn string_primitive_1_evaluate (primitive : StringPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive1::StringLength => {
			let length = try! (string_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		StringPrimitive1::StringClone =>
			return string_clone (input_1),
		
		StringPrimitive1::StringReverse =>
			return string_reverse (input_1),
		
		StringPrimitive1::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &ZERO.into ()),
		
		StringPrimitive1::StringBuild =>
			return string_build_1 (input_1),
		
		StringPrimitive1::StringAppend =>
			return string_clone (input_1),
		
	}
}




pub fn string_primitive_2_evaluate (primitive : StringPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive2::StringAt =>
			return string_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		StringPrimitive2::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2),
		
		StringPrimitive2::StringBuild =>
			return string_build_2 (input_1, input_2),
		
		StringPrimitive2::StringAppend =>
			return string_append_2 (input_1, input_2),
		
	}
}




pub fn string_primitive_3_evaluate (primitive : StringPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive3::StringAtSet =>
			return string_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		StringPrimitive3::StringBuild =>
			return string_build_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringAppend =>
			return string_append_3 (input_1, input_2, input_3),
		
	}
}




pub fn string_primitive_4_evaluate (primitive : StringPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive4::StringBuild =>
			return string_build_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringAppend =>
			return string_append_4 (input_1, input_2, input_3, input_4),
		
	}
}




pub fn string_primitive_n_evaluate (primitive : StringPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		StringPrimitiveN::StringMake =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringMake, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x81290cad),
			},
		
		StringPrimitiveN::StringBuild =>
			match inputs_count {
				0 =>
					return string_primitive_0_evaluate (StringPrimitive0::StringBuild),
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringBuild, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringBuild, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return string_build_n (inputs),
			},
		
		StringPrimitiveN::StringAppend =>
			match inputs_count {
				0 =>
					return string_primitive_0_evaluate (StringPrimitive0::StringAppend),
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringAppend, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringAppend, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return string_append_n (inputs),
			},
		
		StringPrimitiveN::StringSliceFill =>
			fail_unimplemented! (0xac7a8a81),
		
		StringPrimitiveN::StringSliceCopy =>
			fail_unimplemented! (0xfa7dd889),
		
		StringPrimitiveN::StringSliceClone =>
			if inputs_count == 1 {
				return string_primitive_1_evaluate (StringPrimitive1::StringClone, &inputs[0]);
			} else {
				fail_unimplemented! (0xfa7dd889);
			},
		
	}
}

