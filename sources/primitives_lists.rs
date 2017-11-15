

use super::errors::exports::*;
use super::operators::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive1 {
	
	GetLeft,
	GetRight,
	
	List,
	Append,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	SetLeft,
	SetRight,
	
	List2,
	Append2,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitiveN {
	
	List,
	Append,
	
}




#[ inline (always) ]
pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input : &Value) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		ListPrimitive1::GetLeft =>
			(try! (input.try_as_ref ()) as &Pair) .left () .clone (),
		
		ListPrimitive1::GetRight =>
			(try! (input.try_as_ref ()) as &Pair) .right () .clone (),
		
		ListPrimitive1::List =>
			list_build_1 (input),
		
		ListPrimitive1::Append =>
			input.clone (),
		
	};
	
	return Ok (output.into ());
}




#[ inline (always) ]
pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		ListPrimitive2::Pair =>
			pair (input_1, input_2),
		
		ListPrimitive2::SetLeft =>
			fail_unimplemented! (0xa2ba6335),
		
		ListPrimitive2::SetRight =>
			fail_unimplemented! (0xadf82f55),
		
		ListPrimitive2::List2 =>
			list_build_2 (input_1, input_2),
		
		ListPrimitive2::Append2 =>
			try! (list_append_2 (input_1, input_2)),
		
	};
	
	return Ok (output.into ());
}




#[ inline (always) ]
pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		ListPrimitiveN::List =>
			list_build_n (inputs),
		
		ListPrimitiveN::Append =>
			try! (list_append_n (inputs)),
		
	};
	
	return Ok (output.into ());
}


