

use super::errors::exports::*;
use super::operators::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitive3;
	pub use super::ListPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive1 {
	
	PairLeft,
	PairRight,
	
	Length,
	Reverse,
	
	ListFirst,
	ListFirstOfFirst,
	
	ListRest,
	ListRestOfFirst,
	
	ListFirstAt2,
	ListRestAt2,
	
	List,
	Append,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	PairLeftSet,
	PairRightSet,
	
	ListFirstAt,
	ListRestAt,
	
	List,
	Append,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive3 {
	
	ListFirstAtSet,
	
	List,
	Append,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitiveN {
	
	List,
	Append,
	
}




pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive1::PairLeft =>
			succeed! ((try! (input.try_as_ref ()) as &Pair) .left () .clone ()),
		
		ListPrimitive1::PairRight =>
			succeed! ((try! (input.try_as_ref ()) as &Pair) .right () .clone ()),
		
		ListPrimitive1::ListFirst =>
			fail_unimplemented! (0x15b5099a),
		
		ListPrimitive1::ListFirstAt2 =>
			fail_unimplemented! (0xffdaecc7),
		
		ListPrimitive1::ListRest =>
			fail_unimplemented! (0x87b73c9a),
		
		ListPrimitive1::ListRestAt2 =>
			fail_unimplemented! (0x14584ee7),
		
		ListPrimitive1::ListFirstOfFirst =>
			fail_unimplemented! (0x3bd9af62),
		
		ListPrimitive1::ListRestOfFirst =>
			fail_unimplemented! (0xe37e31a9),
		
		ListPrimitive1::Length =>
			fail_unimplemented! (0x73feb843),
		
		ListPrimitive1::Reverse =>
			fail_unimplemented! (0x398ecefa),
		
		ListPrimitive1::List =>
			succeed! (list_build_1 (input)),
		
		ListPrimitive1::Append =>
			succeed! (input.clone ()),
		
	}
}




pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive2::Pair =>
			succeed! (pair (input_1, input_2)),
		
		ListPrimitive2::PairLeftSet =>
			fail_unimplemented! (0xa2ba6335),
		
		ListPrimitive2::PairRightSet =>
			fail_unimplemented! (0xadf82f55),
		
		ListPrimitive2::ListFirstAt =>
			fail_unimplemented! (0x3437ff0a),
		
		ListPrimitive2::ListRestAt =>
			fail_unimplemented! (0x1260c5d7),
		
		ListPrimitive2::List =>
			succeed! (list_build_2 (input_1, input_2)),
		
		ListPrimitive2::Append =>
			return list_append_2 (input_1, input_2),
		
	}
}




pub fn list_primitive_3_evaluate (primitive : ListPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive3::ListFirstAtSet =>
			fail_unimplemented! (0x52b3e12d),
		
		ListPrimitive3::List =>
			succeed! (list_build_3 (input_1, input_2, input_3)),
		
		ListPrimitive3::Append =>
			return list_append_3 (input_1, input_2, input_3),
		
	}
}




pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitiveN::List =>
			succeed! (list_build_n (inputs)),
		
		ListPrimitiveN::Append =>
			return list_append_n (inputs),
		
	}
}


