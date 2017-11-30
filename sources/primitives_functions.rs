

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
	pub use super::FunctionsPrimitive5;
	pub use super::FunctionsPrimitiveN;
	
	pub use super::functions_primitive_0_evaluate;
	pub use super::functions_primitive_1_evaluate;
	pub use super::functions_primitive_2_evaluate;
	pub use super::functions_primitive_3_evaluate;
	pub use super::functions_primitive_4_evaluate;
	pub use super::functions_primitive_5_evaluate;
	pub use super::functions_primitive_n_evaluate;
	
	pub use super::functions_primitive_n_alternative_0;
	pub use super::functions_primitive_n_alternative_1;
	pub use super::functions_primitive_n_alternative_2;
	pub use super::functions_primitive_n_alternative_3;
	pub use super::functions_primitive_n_alternative_4;
	pub use super::functions_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive0 {
	
	Values,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive1 {
	
	Call,
	Apply,
	
	Values,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive2 {
	
	CallWithValues,
	CallWithValuesBuilder,
	
	Call,
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive3 {
	
	Call,
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive4 {
	
	Call,
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitive5 {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	ArraysMap,
	ArraysIterate,
	
	BytesMap,
	BytesIterate,
	
	StringsMap,
	StringsIterate,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitiveN {
	
	Call,
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
	match primitive {
		
		FunctionsPrimitive0::Values =>
			succeed! (values_build_0 ()),
		
	}
}




pub fn functions_primitive_1_evaluate (primitive : FunctionsPrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive1::Call =>
			return call_0 (evaluator, input_1),
		
		FunctionsPrimitive1::Apply =>
			return apply_0 (evaluator, input_1),
		
		FunctionsPrimitive1::Values =>
			succeed! (values_build_1 (input_1)),
		
	}
}




pub fn functions_primitive_2_evaluate (primitive : FunctionsPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive2::CallWithValues =>
			return call_with_values (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::CallWithValuesBuilder =>
			return call_with_values_builder (evaluator, input_2, input_1),
		
		FunctionsPrimitive2::Call =>
			return call_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::Apply =>
			return apply_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::ListsMap =>
			return lists_map_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::ListsIterate =>
			return lists_iterate_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::ArraysMap =>
			return arrays_map_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::ArraysIterate =>
			return arrays_iterate_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::BytesMap =>
			return bytes_map_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::BytesIterate =>
			return bytes_iterate_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::StringsMap =>
			return strings_map_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::StringsIterate =>
			return strings_iterate_1 (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::Values =>
			succeed! (values_build_2 (input_1, input_2)),
		
	}
}




pub fn functions_primitive_3_evaluate (primitive : FunctionsPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive3::Call =>
			return call_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::Apply =>
			return apply_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::ListsMap =>
			return lists_map_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::ListsIterate =>
			return lists_iterate_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::ArraysMap =>
			return arrays_map_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::ArraysIterate =>
			return arrays_iterate_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::BytesMap =>
			return bytes_map_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::BytesIterate =>
			return bytes_iterate_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::StringsMap =>
			return strings_map_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::StringsIterate =>
			return strings_iterate_2 (evaluator, input_1, input_2, input_3),
		
		FunctionsPrimitive3::Values =>
			succeed! (values_build_3 (input_1, input_2, input_3)),
		
	}
}




pub fn functions_primitive_4_evaluate (primitive : FunctionsPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive4::Call =>
			return call_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::Apply =>
			return apply_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::ListsMap =>
			return lists_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::ListsIterate =>
			return lists_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::ArraysMap =>
			return arrays_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::ArraysIterate =>
			return arrays_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::BytesMap =>
			return bytes_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::BytesIterate =>
			return bytes_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::StringsMap =>
			return strings_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::StringsIterate =>
			return strings_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		FunctionsPrimitive4::Values =>
			succeed! (values_build_4 (input_1, input_2, input_3, input_4)),
		
	}
}




pub fn functions_primitive_5_evaluate (primitive : FunctionsPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive5::Call =>
			return call_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::Apply =>
			return apply_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::ListsMap =>
			return lists_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::ListsIterate =>
			return lists_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::ArraysMap =>
			return arrays_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::ArraysIterate =>
			return arrays_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::BytesMap =>
			return bytes_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::BytesIterate =>
			return bytes_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::StringsMap =>
			return strings_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		FunctionsPrimitive5::StringsIterate =>
			return strings_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
	}
}




pub fn functions_primitive_n_evaluate (primitive : FunctionsPrimitiveN, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitiveN::Call => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xf2ed3ec8);
			return call_n (evaluator, callable, inputs);
		},
		
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
		FunctionsPrimitiveN::Call =>
			None,
		FunctionsPrimitiveN::Apply =>
			None,
		FunctionsPrimitiveN::ListsMap =>
			None,
		FunctionsPrimitiveN::ListsIterate =>
			None,
		FunctionsPrimitiveN::ArraysMap =>
			None,
		FunctionsPrimitiveN::ArraysIterate =>
			None,
		FunctionsPrimitiveN::BytesMap =>
			None,
		FunctionsPrimitiveN::BytesIterate =>
			None,
		FunctionsPrimitiveN::StringsMap =>
			None,
		FunctionsPrimitiveN::StringsIterate =>
			None,
		FunctionsPrimitiveN::Values =>
			Some (FunctionsPrimitive0::Values),
	}
}


pub fn functions_primitive_n_alternative_1 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive1>) {
	match primitive {
		FunctionsPrimitiveN::Call =>
			Some (FunctionsPrimitive1::Call),
		FunctionsPrimitiveN::Apply =>
			Some (FunctionsPrimitive1::Apply),
		FunctionsPrimitiveN::ListsMap =>
			None,
		FunctionsPrimitiveN::ListsIterate =>
			None,
		FunctionsPrimitiveN::ArraysMap =>
			None,
		FunctionsPrimitiveN::ArraysIterate =>
			None,
		FunctionsPrimitiveN::BytesMap =>
			None,
		FunctionsPrimitiveN::BytesIterate =>
			None,
		FunctionsPrimitiveN::StringsMap =>
			None,
		FunctionsPrimitiveN::StringsIterate =>
			None,
		FunctionsPrimitiveN::Values =>
			Some (FunctionsPrimitive1::Values),
	}
}


pub fn functions_primitive_n_alternative_2 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive2>) {
	match primitive {
		FunctionsPrimitiveN::Call =>
			Some (FunctionsPrimitive2::Call),
		FunctionsPrimitiveN::Apply =>
			Some (FunctionsPrimitive2::Apply),
		FunctionsPrimitiveN::ListsMap =>
			Some (FunctionsPrimitive2::ListsMap),
		FunctionsPrimitiveN::ListsIterate =>
			Some (FunctionsPrimitive2::ListsIterate),
		FunctionsPrimitiveN::ArraysMap =>
			Some (FunctionsPrimitive2::ArraysMap),
		FunctionsPrimitiveN::ArraysIterate =>
			Some (FunctionsPrimitive2::ArraysIterate),
		FunctionsPrimitiveN::BytesMap =>
			Some (FunctionsPrimitive2::BytesMap),
		FunctionsPrimitiveN::BytesIterate =>
			Some (FunctionsPrimitive2::BytesIterate),
		FunctionsPrimitiveN::StringsMap =>
			Some (FunctionsPrimitive2::StringsMap),
		FunctionsPrimitiveN::StringsIterate =>
			Some (FunctionsPrimitive2::StringsIterate),
		FunctionsPrimitiveN::Values =>
			Some (FunctionsPrimitive2::Values),
	}
}


pub fn functions_primitive_n_alternative_3 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive3>) {
	match primitive {
		FunctionsPrimitiveN::Call =>
			Some (FunctionsPrimitive3::Call),
		FunctionsPrimitiveN::Apply =>
			Some (FunctionsPrimitive3::Apply),
		FunctionsPrimitiveN::ListsMap =>
			Some (FunctionsPrimitive3::ListsMap),
		FunctionsPrimitiveN::ListsIterate =>
			Some (FunctionsPrimitive3::ListsIterate),
		FunctionsPrimitiveN::ArraysMap =>
			Some (FunctionsPrimitive3::ArraysMap),
		FunctionsPrimitiveN::ArraysIterate =>
			Some (FunctionsPrimitive3::ArraysIterate),
		FunctionsPrimitiveN::BytesMap =>
			Some (FunctionsPrimitive3::BytesMap),
		FunctionsPrimitiveN::BytesIterate =>
			Some (FunctionsPrimitive3::BytesIterate),
		FunctionsPrimitiveN::StringsMap =>
			Some (FunctionsPrimitive3::StringsMap),
		FunctionsPrimitiveN::StringsIterate =>
			Some (FunctionsPrimitive3::StringsIterate),
		FunctionsPrimitiveN::Values =>
			Some (FunctionsPrimitive3::Values),
	}
}


pub fn functions_primitive_n_alternative_4 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive4>) {
	match primitive {
		FunctionsPrimitiveN::Call =>
			Some (FunctionsPrimitive4::Call),
		FunctionsPrimitiveN::Apply =>
			Some (FunctionsPrimitive4::Apply),
		FunctionsPrimitiveN::ListsMap =>
			Some (FunctionsPrimitive4::ListsMap),
		FunctionsPrimitiveN::ListsIterate =>
			Some (FunctionsPrimitive4::ListsIterate),
		FunctionsPrimitiveN::ArraysMap =>
			Some (FunctionsPrimitive4::ArraysMap),
		FunctionsPrimitiveN::ArraysIterate =>
			Some (FunctionsPrimitive4::ArraysIterate),
		FunctionsPrimitiveN::BytesMap =>
			Some (FunctionsPrimitive4::BytesMap),
		FunctionsPrimitiveN::BytesIterate =>
			Some (FunctionsPrimitive4::BytesIterate),
		FunctionsPrimitiveN::StringsMap =>
			Some (FunctionsPrimitive4::StringsMap),
		FunctionsPrimitiveN::StringsIterate =>
			Some (FunctionsPrimitive4::StringsIterate),
		FunctionsPrimitiveN::Values =>
			Some (FunctionsPrimitive4::Values),
	}
}


pub fn functions_primitive_n_alternative_5 (primitive : FunctionsPrimitiveN) -> (Option<FunctionsPrimitive5>) {
	match primitive {
		FunctionsPrimitiveN::Call =>
			Some (FunctionsPrimitive5::Call),
		FunctionsPrimitiveN::Apply =>
			Some (FunctionsPrimitive5::Apply),
		FunctionsPrimitiveN::ListsMap =>
			Some (FunctionsPrimitive5::ListsMap),
		FunctionsPrimitiveN::ListsIterate =>
			Some (FunctionsPrimitive5::ListsIterate),
		FunctionsPrimitiveN::ArraysMap =>
			Some (FunctionsPrimitive5::ArraysMap),
		FunctionsPrimitiveN::ArraysIterate =>
			Some (FunctionsPrimitive5::ArraysIterate),
		FunctionsPrimitiveN::BytesMap =>
			Some (FunctionsPrimitive5::BytesMap),
		FunctionsPrimitiveN::BytesIterate =>
			Some (FunctionsPrimitive5::BytesIterate),
		FunctionsPrimitiveN::StringsMap =>
			Some (FunctionsPrimitive5::StringsMap),
		FunctionsPrimitiveN::StringsIterate =>
			Some (FunctionsPrimitive5::StringsIterate),
		FunctionsPrimitiveN::Values =>
			None,
	}
}

