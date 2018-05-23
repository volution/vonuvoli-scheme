

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::primitives_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

use super::prelude::*;




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
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::functions_primitive_n_attributes;
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive0 {
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive1 {
	
	Call,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	Identity,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Constant,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ConstantStar,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Not,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive2 {
	
	CallWithList,
	CallWithListBuilder,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	CallWithArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	CallWithArrayBuilder,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	CallWithValues,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	CallWithValuesBuilder,
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive3 {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive4 {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitive5 {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitiveN {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum FunctionsPrimitiveV {
	
	Call,
	Apply,
	
	ListsMap,
	ListsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysMap,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArraysIterate,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesMap,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesIterate,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsMap,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringsIterate,
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryLeft,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	CurryRight,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Compose1,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ComposeV,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_0_evaluate (primitive : FunctionsPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive0::Values =>
			return values_build_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_1_evaluate (primitive : FunctionsPrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive1::Call =>
			return call_0 (evaluator, input_1),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive1::Values =>
			return values_build_1 (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::CurryLeft =>
			succeed! (input_1.clone ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::CurryRight =>
			succeed! (input_1.clone ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::Compose1 =>
			succeed! (input_1.clone ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::ComposeV =>
			succeed! (input_1.clone ()),
		
		FunctionsPrimitive1::Identity =>
			succeed! (input_1.clone ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::Constant =>
			succeed! (ProcedureExtendedInternals::Constant (input_1.clone (), false) .into ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::ConstantStar =>
			succeed! (ProcedureExtendedInternals::Constant (input_1.clone (), true) .into ()),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive1::Not =>
			succeed! (ProcedureExtendedInternals::Not (input_1.clone ()) .into ()),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_2_evaluate (primitive : FunctionsPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FunctionsPrimitive2::CallWithList =>
			return call_with_list (evaluator, input_1, input_2),
		
		FunctionsPrimitive2::CallWithListBuilder =>
			return call_with_list_builder (evaluator, input_2, input_1),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive2::CallWithArray =>
			return call_with_array (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive2::CallWithArrayBuilder =>
			return call_with_array_builder (evaluator, input_2, input_1),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive2::CallWithValues =>
			return call_with_values (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive2::ArraysMap =>
			return arrays_map_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive2::ArraysIterate =>
			return arrays_iterate_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive2::BytesMap =>
			return bytes_map_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive2::BytesIterate =>
			return bytes_iterate_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive2::StringsMap =>
			return strings_map_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive2::StringsIterate =>
			return strings_iterate_1 (evaluator, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive2::Values =>
			return values_build_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive2::CurryLeft =>
			return curry_1 (input_1, input_2, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive2::CurryRight =>
			return curry_1 (input_1, input_2, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive2::Compose1 =>
			return compose_2 (input_1, input_2, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive2::ComposeV =>
			return compose_2 (input_1, input_2, true) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive3::ArraysMap =>
			return arrays_map_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive3::ArraysIterate =>
			return arrays_iterate_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive3::BytesMap =>
			return bytes_map_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive3::BytesIterate =>
			return bytes_iterate_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive3::StringsMap =>
			return strings_map_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive3::StringsIterate =>
			return strings_iterate_2 (evaluator, input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive3::Values =>
			return values_build_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive3::CurryLeft =>
			return curry_2 (input_1, input_2, input_3, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive3::CurryRight =>
			return curry_2 (input_1, input_2, input_3, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive3::Compose1 =>
			return compose_3 (input_1, input_2, input_3, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive3::ComposeV =>
			return compose_3 (input_1, input_2, input_3, true) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive4::ArraysMap =>
			return arrays_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive4::ArraysIterate =>
			return arrays_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive4::BytesMap =>
			return bytes_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive4::BytesIterate =>
			return bytes_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive4::StringsMap =>
			return strings_map_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive4::StringsIterate =>
			return strings_iterate_3 (evaluator, input_1, input_2, input_3, input_4),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitive4::Values =>
			return values_build_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive4::CurryLeft =>
			return curry_3 (input_1, input_2, input_3, input_4, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive4::CurryRight =>
			return curry_3 (input_1, input_2, input_3, input_4, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive4::Compose1 =>
			return compose_4 (input_1, input_2, input_3, input_4, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive4::ComposeV =>
			return compose_4 (input_1, input_2, input_3, input_4, true) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive5::ArraysMap =>
			return arrays_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitive5::ArraysIterate =>
			return arrays_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive5::BytesMap =>
			return bytes_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitive5::BytesIterate =>
			return bytes_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive5::StringsMap =>
			return strings_map_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitive5::StringsIterate =>
			return strings_iterate_4 (evaluator, input_1, input_2, input_3, input_4, input_5),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive5::CurryLeft =>
			return curry_4 (input_1, input_2, input_3, input_4, input_5, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitive5::CurryRight =>
			return curry_4 (input_1, input_2, input_3, input_4, input_5, true) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveN::ArraysMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xe284e2bf);
			return arrays_map_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveN::ArraysIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xc7077329);
			return arrays_iterate_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveN::BytesMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0x01e0c89f);
			return bytes_map_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveN::BytesIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xca0f78c1);
			return bytes_iterate_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveN::StringsMap => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xd751b843);
			return strings_map_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveN::StringsIterate => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0x5ca9746e);
			return strings_iterate_n (evaluator, callable, inputs);
		},
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveN::Values =>
			return values_build_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveN::CurryLeft => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0xd0ecd544);
			return curry_n (callable, inputs, false);
		},
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveN::CurryRight => {
			let (callable, inputs) = try_some! (inputs.split_first (), 0x1a8aea63);
			return curry_n (callable, inputs, true);
		},
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveN::Compose1 =>
			return compose_n (inputs, false),
		
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveN::ComposeV =>
			return compose_n (inputs, true),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive0::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			None,
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			None,
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			None,
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_v_alternative_1 (primitive : FunctionsPrimitiveV) -> (Option<FunctionsPrimitive1>) {
	match primitive {
		FunctionsPrimitiveV::Call =>
			Some (FunctionsPrimitive1::Call),
		FunctionsPrimitiveV::Apply =>
			Some (FunctionsPrimitive1::Call),
		FunctionsPrimitiveV::ListsMap =>
			None,
		FunctionsPrimitiveV::ListsIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive1::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitive1::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitive1::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			Some (FunctionsPrimitive1::Compose1),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			Some (FunctionsPrimitive1::ComposeV),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive2::ArraysMap),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive2::ArraysIterate),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive2::BytesMap),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive2::BytesIterate),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive2::StringsMap),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive2::StringsIterate),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive2::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitive2::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitive2::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			Some (FunctionsPrimitive2::Compose1),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			Some (FunctionsPrimitive2::ComposeV),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive3::ArraysMap),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive3::ArraysIterate),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive3::BytesMap),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive3::BytesIterate),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive3::StringsMap),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive3::StringsIterate),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive3::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitive3::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitive3::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			Some (FunctionsPrimitive3::Compose1),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			Some (FunctionsPrimitive3::ComposeV),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive4::ArraysMap),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive4::ArraysIterate),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive4::BytesMap),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive4::BytesIterate),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive4::StringsMap),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive4::StringsIterate),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitive4::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitive4::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitive4::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			Some (FunctionsPrimitive4::Compose1),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			Some (FunctionsPrimitive4::ComposeV),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitive5::ArraysMap),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitive5::ArraysIterate),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitive5::BytesMap),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitive5::BytesIterate),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitive5::StringsMap),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitive5::StringsIterate),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			None,
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitive5::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitive5::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			None,
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysMap =>
			Some (FunctionsPrimitiveN::ArraysMap),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		FunctionsPrimitiveV::ArraysIterate =>
			Some (FunctionsPrimitiveN::ArraysIterate),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesMap =>
			Some (FunctionsPrimitiveN::BytesMap),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FunctionsPrimitiveV::BytesIterate =>
			Some (FunctionsPrimitiveN::BytesIterate),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsMap =>
			Some (FunctionsPrimitiveN::StringsMap),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FunctionsPrimitiveV::StringsIterate =>
			Some (FunctionsPrimitiveN::StringsIterate),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		FunctionsPrimitiveV::Values =>
			Some (FunctionsPrimitiveN::Values),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryLeft =>
			Some (FunctionsPrimitiveN::CurryLeft),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::CurryRight =>
			Some (FunctionsPrimitiveN::CurryRight),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::Compose1 =>
			Some (FunctionsPrimitiveN::Compose1),
		#[ cfg ( feature = "vonuvoli_values_extended" ) ]
		FunctionsPrimitiveV::ComposeV =>
			Some (FunctionsPrimitiveN::ComposeV),
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_0_attributes (_primitive : FunctionsPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_1_attributes (_primitive : FunctionsPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_2_attributes (_primitive : FunctionsPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_3_attributes (_primitive : FunctionsPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_4_attributes (_primitive : FunctionsPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_5_attributes (_primitive : FunctionsPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn functions_primitive_n_attributes (_primitive : FunctionsPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

