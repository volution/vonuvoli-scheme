

use super::constants::exports::*;
use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::BytesPrimitive0;
	pub use super::BytesPrimitive1;
	pub use super::BytesPrimitive2;
	pub use super::BytesPrimitive3;
	pub use super::BytesPrimitive4;
	pub use super::BytesPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive0 {
	
	BytesBuild,
	BytesAppend,
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive1 {
	
	BytesLength,
	BytesClone,
	BytesReverse,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive2 {
	
	BytesAt,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive3 {
	
	BytesAtSet,
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive4 {
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitiveN {
	
	BytesMake,
	BytesBuild,
	BytesAppend,
	
	BytesSliceFill,
	BytesSliceCopy,
	
}




pub fn bytes_primitive_0_evaluate (primitive : BytesPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive0::BytesBuild =>
			succeed! (bytes_empty ()),
		
		BytesPrimitive0::BytesAppend =>
			succeed! (bytes_empty ()),
		
	}
}




pub fn bytes_primitive_1_evaluate (primitive : BytesPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive1::BytesLength => {
			let length = try! (bytes_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		BytesPrimitive1::BytesClone =>
			return bytes_clone (input_1),
		
		BytesPrimitive1::BytesReverse =>
			return bytes_reverse (input_1),
		
		BytesPrimitive1::BytesMake =>
			return bytes_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &ZERO.into ()),
		
		BytesPrimitive1::BytesBuild =>
			return bytes_build_1 (input_1),
		
		BytesPrimitive1::BytesAppend =>
			return bytes_clone (input_1),
		
	}
}




pub fn bytes_primitive_2_evaluate (primitive : BytesPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive2::BytesAt =>
			return bytes_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		BytesPrimitive2::BytesMake =>
			return bytes_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2),
		
		BytesPrimitive2::BytesBuild =>
			return bytes_build_2 (input_1, input_2),
		
		BytesPrimitive2::BytesAppend =>
			return bytes_append_2 (input_1, input_2),
		
	}
}




pub fn bytes_primitive_3_evaluate (primitive : BytesPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive3::BytesAtSet =>
			return bytes_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		BytesPrimitive3::BytesBuild =>
			return bytes_build_3 (input_1, input_2, input_3),
		
		BytesPrimitive3::BytesAppend =>
			return bytes_append_3 (input_1, input_2, input_3),
		
	}
}




pub fn bytes_primitive_4_evaluate (primitive : BytesPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive4::BytesBuild =>
			return bytes_build_4 (input_1, input_2, input_3, input_4),
		
		BytesPrimitive4::BytesAppend =>
			return bytes_append_4 (input_1, input_2, input_3, input_4),
		
	}
}




pub fn bytes_primitive_n_evaluate (primitive : BytesPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		BytesPrimitiveN::BytesMake =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesMake, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x670a044a),
			},
		
		BytesPrimitiveN::BytesBuild =>
			match inputs_count {
				0 =>
					return bytes_primitive_0_evaluate (BytesPrimitive0::BytesBuild),
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesBuild, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesBuild, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return bytes_build_n (inputs),
			},
		
		BytesPrimitiveN::BytesAppend =>
			match inputs_count {
				0 =>
					return bytes_primitive_0_evaluate (BytesPrimitive0::BytesAppend),
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesAppend, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesAppend, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return bytes_append_n (inputs),
			},
		
		BytesPrimitiveN::BytesSliceFill =>
			fail_unimplemented! (0xdb534eb6),
		
		BytesPrimitiveN::BytesSliceCopy =>
			fail_unimplemented! (0xe7a2e534),
		
	}
}

