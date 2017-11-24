

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::FunctionsPrimitive1;
	pub use super::FunctionsPrimitive2;
	pub use super::FunctionsPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive1 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum FunctionsPrimitiveN {
	
	Apply,
	
	ListsMap,
	ListsIterate,
	
	Values,
	
}




pub fn functions_primitive_1_evaluate (primitive : FunctionsPrimitive1, _input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn functions_primitive_2_evaluate (primitive : FunctionsPrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
		
		FunctionsPrimitiveN::Values =>
			succeed! (values_build_n (inputs)),
		
	}
}

