

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::FunctionsPrimitive0;
	pub use super::FunctionsPrimitive1;
	pub use super::FunctionsPrimitive2;
	pub use super::FunctionsPrimitive3;
	pub use super::FunctionsPrimitive4;
	pub use super::FunctionsPrimitiveN;
	
	pub use super::functions_primitive_0_evaluate;
	pub use super::functions_primitive_1_evaluate;
	pub use super::functions_primitive_2_evaluate;
	pub use super::functions_primitive_3_evaluate;
	pub use super::functions_primitive_4_evaluate;
	pub use super::functions_primitive_n_evaluate;
	
	pub use super::functions_primitive_n_alternative_0;
	pub use super::functions_primitive_n_alternative_1;
	pub use super::functions_primitive_n_alternative_2;
	pub use super::functions_primitive_n_alternative_3;
	pub use super::functions_primitive_n_alternative_4;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive1 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitiveN {
	
	Apply,
	
	ListsMap,
	ListsIterate,
	
	ArraysMap,
	ArraysIterate,
	
	BytesMap,
	BytesIterate,
	
	StringsMap,
	StringsIterate,
	
	Values,
	
}




pub fn functions_primitive_0_evaluate (primitive : FunctionsPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_1_evaluate (primitive : FunctionsPrimitive1, _input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_2_evaluate (primitive : FunctionsPrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_3_evaluate (primitive : FunctionsPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_4_evaluate (primitive : FunctionsPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_n_evaluate (primitive : FunctionsPrimitiveN, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitiveN::Apply => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xa03c75aa);
			return apply_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::ListsMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xc82da0ae);
			return lists_map_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::ListsIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xc9671a04);
			return lists_iterate_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::ArraysMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xe284e2bf);
			return arrays_map_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::ArraysIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xc7077329);
			return arrays_iterate_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::BytesMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0x01e0c89f);
			return bytes_map_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::BytesIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xca0f78c1);
			return bytes_iterate_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::StringsMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xd751b843);
			return strings_map_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::StringsIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0x5ca9746e);
			return strings_iterate_n (evaluator, callable, inputs);
		},
		
		FunctionsPrimitiveN::Values =>
			succeed! (values_build_n (inputs)),
		
	}
}




pub fn functions_primitive_n_alternative_0 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive0>) {
	match primitive {
		_ => None,
	}
}


pub fn functions_primitive_n_alternative_1 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive1>) {
	match primitive {
		_ => None,
	}
}


pub fn functions_primitive_n_alternative_2 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive2>) {
	match primitive {
		_ => None,
	}
}


pub fn functions_primitive_n_alternative_3 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive3>) {
	match primitive {
		_ => None,
	}
}


pub fn functions_primitive_n_alternative_4 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive4>) {
	match primitive {
		_ => None,
	}
}


