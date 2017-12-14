

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::FunctionsPrimitive0;
	pub use super::FunctionsPrimitive1;
	pub use super::FunctionsPrimitive2;
	pub use super::FunctionsPrimitive3;
	pub use super::FunctionsPrimitive4;
	pub use super::FunctionsPrimitive5;
	pub use super::FunctionsPrimitiveN;
	pub use super::FunctionsPrimitiveV;
	
	pub use super::functions_primitive_0_evaluate;
	pub use super::functions_primitive_1_evaluate;
	pub use super::functions_primitive_2_evaluate;
	pub use super::functions_primitive_3_evaluate;
	pub use super::functions_primitive_4_evaluate;
	pub use super::functions_primitive_5_evaluate;
	pub use super::functions_primitive_n_evaluate;
	
	pub use super::functions_primitive_v_alternative_0;
	pub use super::functions_primitive_v_alternative_1;
	pub use super::functions_primitive_v_alternative_2;
	pub use super::functions_primitive_v_alternative_3;
	pub use super::functions_primitive_v_alternative_4;
	pub use super::functions_primitive_v_alternative_5;
	pub use super::functions_primitive_v_alternative_n;
	
	pub use super::functions_primitive_0_attributes;
	pub use super::functions_primitive_1_attributes;
	pub use super::functions_primitive_2_attributes;
	pub use super::functions_primitive_3_attributes;
	pub use super::functions_primitive_4_attributes;
	pub use super::functions_primitive_5_attributes;
	pub use super::functions_primitive_n_attributes;
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FunctionsPrimitiveV {
	
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




pub fn functions_primitive_n_evaluate (primitive : FunctionsPrimitiveN, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




pub fn functions_primitive_v_alternative_0 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive0>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			None,
		FunctionsPrimitiveV::Apply =>
			None,
		FunctionsPrimitiveV::ListsMap =>
			None,
		FunctionsPrimitiveV::ListsIterate =>
			None,
		FunctionsPrimitiveV::ArraysMap =>
			None,
		FunctionsPrimitiveV::ArraysIterate =>
			None,
		FunctionsPrimitiveV::BytesMap =>
			None,
		FunctionsPrimitiveV::BytesIterate =>
			None,
		FunctionsPrimitiveV::StringsMap =>
			None,
		FunctionsPrimitiveV::StringsIterate =>
			None,
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive0::Values),
	}
}




pub fn functions_primitive_v_alternative_1 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive1>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive1::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive1::Apply),
		FunctionsPrimitiveV::ListsMap =>
			None,
		FunctionsPrimitiveV::ListsIterate =>
			None,
		FunctionsPrimitiveV::ArraysMap =>
			None,
		FunctionsPrimitiveV::ArraysIterate =>
			None,
		FunctionsPrimitiveV::BytesMap =>
			None,
		FunctionsPrimitiveV::BytesIterate =>
			None,
		FunctionsPrimitiveV::StringsMap =>
			None,
		FunctionsPrimitiveV::StringsIterate =>
			None,
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive1::Values),
	}
}




pub fn functions_primitive_v_alternative_2 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive2>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive2::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive2::Apply),
		FunctionsPrimitiveV::ListsMap =>
			Some (FunctionsPrimitive2::ListsMap),
		FunctionsPrimitiveV::ListsIterate =>
			Some (FunctionsPrimitive2::ListsIterate),
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive2::ArraysMap),
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive2::ArraysIterate),
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive2::BytesMap),
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive2::BytesIterate),
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive2::StringsMap),
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive2::StringsIterate),
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive2::Values),
	}
}




pub fn functions_primitive_v_alternative_3 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive3>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive3::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive3::Apply),
		FunctionsPrimitiveV::ListsMap =>
			Some (FunctionsPrimitive3::ListsMap),
		FunctionsPrimitiveV::ListsIterate =>
			Some (FunctionsPrimitive3::ListsIterate),
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive3::ArraysMap),
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive3::ArraysIterate),
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive3::BytesMap),
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive3::BytesIterate),
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive3::StringsMap),
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive3::StringsIterate),
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive3::Values),
	}
}




pub fn functions_primitive_v_alternative_4 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive4>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive4::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive4::Apply),
		FunctionsPrimitiveV::ListsMap =>
			Some (FunctionsPrimitive4::ListsMap),
		FunctionsPrimitiveV::ListsIterate =>
			Some (FunctionsPrimitive4::ListsIterate),
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive4::ArraysMap),
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive4::ArraysIterate),
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive4::BytesMap),
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive4::BytesIterate),
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive4::StringsMap),
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive4::StringsIterate),
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive4::Values),
	}
}




pub fn functions_primitive_v_alternative_5 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive5>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive5::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive5::Apply),
		FunctionsPrimitiveV::ListsMap =>
			Some (FunctionsPrimitive5::ListsMap),
		FunctionsPrimitiveV::ListsIterate =>
			Some (FunctionsPrimitive5::ListsIterate),
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive5::ArraysMap),
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive5::ArraysIterate),
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive5::BytesMap),
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive5::BytesIterate),
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive5::StringsMap),
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive5::StringsIterate),
		FunctionsPrimitiveV::Values =>
			None,
	}
}




pub fn functions_primitive_v_alternative_n (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitiveN>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitiveN::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitiveN::Apply),
		FunctionsPrimitiveV::ListsMap =>
			Some (FunctionsPrimitiveN::ListsMap),
		FunctionsPrimitiveV::ListsIterate =>
			Some (FunctionsPrimitiveN::ListsIterate),
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitiveN::ArraysMap),
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitiveN::ArraysIterate),
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitiveN::BytesMap),
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitiveN::BytesIterate),
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitiveN::StringsMap),
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitiveN::StringsIterate),
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitiveN::Values),
	}
}




pub fn functions_primitive_0_attributes (_primitive : FunctionsPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_1_attributes (_primitive : FunctionsPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_2_attributes (_primitive : FunctionsPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_3_attributes (_primitive : FunctionsPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_4_attributes (_primitive : FunctionsPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_5_attributes (_primitive : FunctionsPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

pub fn functions_primitive_n_attributes (_primitive : FunctionsPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

