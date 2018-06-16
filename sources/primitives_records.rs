

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::RecordPrimitive0;
	pub use super::RecordPrimitive1;
	pub use super::RecordPrimitive2;
	pub use super::RecordPrimitive3;
	pub use super::RecordPrimitive4;
	pub use super::RecordPrimitive5;
	pub use super::RecordPrimitiveN;
	pub use super::RecordPrimitiveV;
	
	pub use super::record_primitive_0_evaluate;
	pub use super::record_primitive_1_evaluate;
	pub use super::record_primitive_2_evaluate;
	pub use super::record_primitive_3_evaluate;
	pub use super::record_primitive_4_evaluate;
	pub use super::record_primitive_5_evaluate;
	pub use super::record_primitive_n_evaluate;
	
	pub use super::record_primitive_v_alternative_0;
	pub use super::record_primitive_v_alternative_1;
	pub use super::record_primitive_v_alternative_2;
	pub use super::record_primitive_v_alternative_3;
	pub use super::record_primitive_v_alternative_4;
	pub use super::record_primitive_v_alternative_5;
	pub use super::record_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::record_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (RecordPrimitive0, (procedure, 0), {});


def_primitives_enum! (RecordPrimitive1, (procedure, 1), {
	
	RecordKindBuild,
	RecordKindIdentifier,
	RecordKindSize,
	
	RecordKindIsFn,
	RecordGetFn,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSetFn,
	RecordBuildFnN,
	RecordBuildFnC,
	
	RecordKindIs,
	RecordKindGet,
	
	RecordBuild,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordToMutable,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordFromArray,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordToValues,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
});


def_primitives_enum! (RecordPrimitive2, (procedure, 2), {
	
	RecordKindBuild,
	
	RecordKindIsFn,
	RecordGetFn,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSetFn,
	RecordBuildFnN,
	RecordBuildFnC,
	
	RecordKindIs,
	RecordGet,
	
	RecordBuild,
	RecordBuildC,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordFromArray,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordToValues,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
});


def_primitives_enum! (RecordPrimitive3, (procedure, 3), {
	
	RecordBuildFnN,
	RecordBuildFnC,
	
	RecordKindIs,
	RecordGet,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSet,
	
	RecordBuild,
	RecordBuildC,
	
});


def_primitives_enum! (RecordPrimitive4, (procedure, 4), {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSet,
	
	RecordBuild,
	
});


def_primitives_enum! (RecordPrimitive5, (procedure, 5), {
	
	RecordBuild,
	
});


def_primitives_enum! (RecordPrimitiveN, (procedure, n), {
	
	RecordBuild,
	
});


def_primitives_enum! (RecordPrimitiveV, (procedure, v), {
	
	RecordKindBuild,
	
	RecordKindIsFn,
	RecordGetFn,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSetFn,
	RecordBuildFnN,
	RecordBuildFnC,
	
	RecordKindIs,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSet,
	RecordGet,
	
	RecordBuild,
	RecordBuildC,
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	RecordFromArray,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordToValues,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
});




impl_procedure_primitive_enum_matrix! (
		(RecordPrimitive0, record_primitive_0_evaluate, record_primitive_0_attributes, record_primitive_v_alternative_0),
		(RecordPrimitive1, record_primitive_1_evaluate, record_primitive_1_attributes, record_primitive_v_alternative_1),
		(RecordPrimitive2, record_primitive_2_evaluate, record_primitive_2_attributes, record_primitive_v_alternative_2),
		(RecordPrimitive3, record_primitive_3_evaluate, record_primitive_3_attributes, record_primitive_v_alternative_3),
		(RecordPrimitive4, record_primitive_4_evaluate, record_primitive_4_attributes, record_primitive_v_alternative_4),
		(RecordPrimitive5, record_primitive_5_evaluate, record_primitive_5_attributes, record_primitive_v_alternative_5),
		(RecordPrimitiveN, record_primitive_n_evaluate, record_primitive_n_attributes, record_primitive_v_alternative_n),
		(RecordPrimitiveV, record_primitive_v_evaluate, record_primitive_v_attributes),
	);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_0_evaluate (primitive : RecordPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_1_evaluate (primitive : RecordPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RecordPrimitive1::RecordKindBuild =>
			return record_kind_build (None, input_1) .into_0 (),
		
		RecordPrimitive1::RecordKindSize =>
			return record_kind_size (input_1) .into_0 (),
		
		RecordPrimitive1::RecordKindIdentifier =>
			return record_kind_identifier (input_1),
		
		RecordPrimitive1::RecordKindIsFn =>
			return record_kind_is_fn (try_as_record_kind_ref! (input_1), None) .into_0 (),
		
		RecordPrimitive1::RecordGetFn =>
			return record_get_x_fn (None, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive1::RecordSetFn =>
			return record_set_x_fn (None, input_1) .into_0 (),
		
		RecordPrimitive1::RecordBuildFnN =>
			return record_build_fn_n (try_as_record_kind_ref! (input_1), None, None) .into_0 (),
		
		RecordPrimitive1::RecordBuildFnC =>
			return record_build_fn_c (try_as_record_kind_ref! (input_1), None, None) .into_0 (),
		
		RecordPrimitive1::RecordKindIs =>
			return is_record (input_1) .into_0 (),
		
		RecordPrimitive1::RecordKindGet =>
			return record_kind_get (input_1) .into_0 (),
		
		RecordPrimitive1::RecordBuild =>
			return record_build_0 (try_as_record_kind_ref! (input_1), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive1::RecordToImmutable =>
			return try_as_record_as_ref! (input_1) .to_immutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive1::RecordToMutable =>
			return try_as_record_as_ref! (input_1) .to_mutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitive1::RecordToArray =>
			return record_to_array (None, input_1, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitive1::RecordFromArray =>
			return record_from_array (None, input_1, None),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitive1::RecordToValues =>
			return record_to_values (None, input_1),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitive1::RecordFromValues =>
			return record_from_values (None, input_1, None),
		
		RecordPrimitive1::RecordToList =>
			return record_to_list (None, input_1, None),
		
		RecordPrimitive1::RecordFromList =>
			return record_from_list (None, input_1, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_2_evaluate (primitive : RecordPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RecordPrimitive2::RecordKindBuild =>
			return record_kind_build (Some (input_1), input_2) .into_0 (),
		
		RecordPrimitive2::RecordKindIsFn =>
			return record_kind_is_fn (try_as_record_kind_ref! (input_1), Some (try_as_boolean_ref! (input_2) .value ())) .into_0 (),
		
		RecordPrimitive2::RecordGetFn =>
			return record_get_x_fn (Some (try_as_record_kind_ref! (input_1)), input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive2::RecordSetFn =>
			return record_set_x_fn (Some (try_as_record_kind_ref! (input_1)), input_2) .into_0 (),
		
		RecordPrimitive2::RecordBuildFnN =>
			return record_build_fn_n (try_as_record_kind_ref! (input_1), Some (input_2), None) .into_0 (),
		
		RecordPrimitive2::RecordBuildFnC =>
			return record_build_fn_c (try_as_record_kind_ref! (input_1), Some (input_2), None) .into_0 (),
		
		RecordPrimitive2::RecordKindIs =>
			return record_kind_is (try_as_record_kind_ref! (input_1), input_2, None) .into_0 (),
		
		RecordPrimitive2::RecordGet =>
			return record_get_x (None, input_1, input_2),
		
		RecordPrimitive2::RecordBuild =>
			return record_build_1 (try_as_record_kind_ref! (input_1), None, input_2, None),
		
		RecordPrimitive2::RecordBuildC =>
			return record_build (try_as_record_kind_ref! (input_1), None, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitive2::RecordToArray =>
			return record_to_array (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitive2::RecordFromArray =>
			return record_from_array (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitive2::RecordToValues =>
			return record_to_values (Some (try_as_record_kind_ref! (input_1)), input_2),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitive2::RecordFromValues =>
			return record_from_values (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RecordPrimitive2::RecordToList =>
			return record_to_list (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RecordPrimitive2::RecordFromList =>
			return record_from_list (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_3_evaluate (primitive : RecordPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RecordPrimitive3::RecordBuildFnN =>
			return record_build_fn_n (try_as_record_kind_ref! (input_1), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())) .into_0 (),
		
		RecordPrimitive3::RecordBuildFnC =>
			return record_build_fn_c (try_as_record_kind_ref! (input_1), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())) .into_0 (),
		
		RecordPrimitive3::RecordKindIs =>
			return record_kind_is (try_as_record_kind_ref! (input_1), input_2, Some (try_as_boolean_ref! (input_3) .value ())) .into_0 (),
		
		RecordPrimitive3::RecordGet =>
			return record_get_x (Some (try_as_record_kind_ref! (input_1)), input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive3::RecordSet =>
			return record_set_x (None, input_1, input_2, input_3),
		
		RecordPrimitive3::RecordBuild =>
			return record_build_2 (try_as_record_kind_ref! (input_1), None, input_2, input_3, None),
		
		RecordPrimitive3::RecordBuildC =>
			return record_build (try_as_record_kind_ref! (input_1), None, input_2, Some (try_as_boolean_ref! (input_3) .value ())),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_4_evaluate (primitive : RecordPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitive4::RecordSet =>
			return record_set_x (Some (try_as_record_kind_ref! (input_1)), input_2, input_3, input_4),
		
		RecordPrimitive4::RecordBuild =>
			return record_build_3 (try_as_record_kind_ref! (input_1), None, input_2, input_3, input_4, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_5_evaluate (primitive : RecordPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RecordPrimitive5::RecordBuild =>
			return record_build_4 (try_as_record_kind_ref! (input_1), None, input_2, input_3, input_4, input_5, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_n_evaluate (primitive : RecordPrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RecordPrimitiveN::RecordBuild => {
			let (kind, inputs) = try_some! (inputs.split_first (), 0xa34efcb8);
			return record_build_n (try_as_record_kind_ref! (kind.as_ref ()), None, inputs, None);
		},
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_0 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive0>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			None,
		RecordPrimitiveV::RecordKindIsFn =>
			None,
		RecordPrimitiveV::RecordGetFn =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			None,
		RecordPrimitiveV::RecordBuildFnN =>
			None,
		RecordPrimitiveV::RecordBuildFnC =>
			None,
		RecordPrimitiveV::RecordKindIs =>
			None,
		RecordPrimitiveV::RecordGet =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			None,
		RecordPrimitiveV::RecordBuild =>
			None,
		RecordPrimitiveV::RecordBuildC =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			None,
		RecordPrimitiveV::RecordToList =>
			None,
		RecordPrimitiveV::RecordFromList =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_1 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive1>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			Some (RecordPrimitive1::RecordKindBuild),
		RecordPrimitiveV::RecordKindIsFn =>
			Some (RecordPrimitive1::RecordKindIsFn),
		RecordPrimitiveV::RecordGetFn =>
			Some (RecordPrimitive1::RecordGetFn),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			Some (RecordPrimitive1::RecordSetFn),
		RecordPrimitiveV::RecordBuildFnN =>
			Some (RecordPrimitive1::RecordBuildFnN),
		RecordPrimitiveV::RecordBuildFnC =>
			Some (RecordPrimitive1::RecordBuildFnC),
		RecordPrimitiveV::RecordKindIs =>
			Some (RecordPrimitive1::RecordKindIs),
		RecordPrimitiveV::RecordGet =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			None,
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitive1::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			Some (RecordPrimitive1::RecordToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			Some (RecordPrimitive1::RecordFromArray),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			Some (RecordPrimitive1::RecordToValues),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			Some (RecordPrimitive1::RecordFromValues),
		RecordPrimitiveV::RecordToList =>
			Some (RecordPrimitive1::RecordToList),
		RecordPrimitiveV::RecordFromList =>
			Some (RecordPrimitive1::RecordFromList),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_2 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive2>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			Some (RecordPrimitive2::RecordKindBuild),
		RecordPrimitiveV::RecordKindIsFn =>
			Some (RecordPrimitive2::RecordKindIsFn),
		RecordPrimitiveV::RecordGetFn =>
			Some (RecordPrimitive2::RecordGetFn),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			Some (RecordPrimitive2::RecordSetFn),
		RecordPrimitiveV::RecordBuildFnN =>
			Some (RecordPrimitive2::RecordBuildFnN),
		RecordPrimitiveV::RecordBuildFnC =>
			Some (RecordPrimitive2::RecordBuildFnC),
		RecordPrimitiveV::RecordKindIs =>
			Some (RecordPrimitive2::RecordKindIs),
		RecordPrimitiveV::RecordGet =>
			Some (RecordPrimitive2::RecordGet),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			None,
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitive2::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			Some (RecordPrimitive2::RecordBuildC),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			Some (RecordPrimitive2::RecordToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			Some (RecordPrimitive2::RecordFromArray),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			Some (RecordPrimitive2::RecordToValues),
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			Some (RecordPrimitive2::RecordFromValues),
		RecordPrimitiveV::RecordToList =>
			Some (RecordPrimitive2::RecordToList),
		RecordPrimitiveV::RecordFromList =>
			Some (RecordPrimitive2::RecordFromList),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_3 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive3>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			None,
		RecordPrimitiveV::RecordKindIsFn =>
			None,
		RecordPrimitiveV::RecordGetFn =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			None,
		RecordPrimitiveV::RecordBuildFnN =>
			Some (RecordPrimitive3::RecordBuildFnN),
		RecordPrimitiveV::RecordBuildFnC =>
			Some (RecordPrimitive3::RecordBuildFnC),
		RecordPrimitiveV::RecordKindIs =>
			Some (RecordPrimitive3::RecordKindIs),
		RecordPrimitiveV::RecordGet =>
			Some (RecordPrimitive3::RecordGet),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			Some (RecordPrimitive3::RecordSet),
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitive3::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			Some (RecordPrimitive3::RecordBuildC),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			None,
		RecordPrimitiveV::RecordToList =>
			None,
		RecordPrimitiveV::RecordFromList =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_4 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive4>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			None,
		RecordPrimitiveV::RecordKindIsFn =>
			None,
		RecordPrimitiveV::RecordGetFn =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			None,
		RecordPrimitiveV::RecordBuildFnN =>
			None,
		RecordPrimitiveV::RecordBuildFnC =>
			None,
		RecordPrimitiveV::RecordKindIs =>
			None,
		RecordPrimitiveV::RecordGet =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			Some (RecordPrimitive4::RecordSet),
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitive4::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			None,
		RecordPrimitiveV::RecordToList =>
			None,
		RecordPrimitiveV::RecordFromList =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_5 (primitive : RecordPrimitiveV) -> (Option<RecordPrimitive5>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			None,
		RecordPrimitiveV::RecordKindIsFn =>
			None,
		RecordPrimitiveV::RecordGetFn =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			None,
		RecordPrimitiveV::RecordBuildFnN =>
			None,
		RecordPrimitiveV::RecordBuildFnC =>
			None,
		RecordPrimitiveV::RecordKindIs =>
			None,
		RecordPrimitiveV::RecordGet =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			None,
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitive5::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			None,
		RecordPrimitiveV::RecordToList =>
			None,
		RecordPrimitiveV::RecordFromList =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_v_alternative_n (primitive : RecordPrimitiveV) -> (Option<RecordPrimitiveN>) {
	match primitive {
		RecordPrimitiveV::RecordKindBuild =>
			None,
		RecordPrimitiveV::RecordKindIsFn =>
			None,
		RecordPrimitiveV::RecordGetFn =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSetFn =>
			None,
		RecordPrimitiveV::RecordBuildFnN =>
			None,
		RecordPrimitiveV::RecordBuildFnC =>
			None,
		RecordPrimitiveV::RecordKindIs =>
			None,
		RecordPrimitiveV::RecordGet =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		RecordPrimitiveV::RecordSet =>
			None,
		RecordPrimitiveV::RecordBuild =>
			Some (RecordPrimitiveN::RecordBuild),
		RecordPrimitiveV::RecordBuildC =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RecordPrimitiveV::RecordFromArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordToValues =>
			None,
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RecordPrimitiveV::RecordFromValues =>
			None,
		RecordPrimitiveV::RecordToList =>
			None,
		RecordPrimitiveV::RecordFromList =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_0_attributes (_primitive : RecordPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_1_attributes (_primitive : RecordPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_2_attributes (_primitive : RecordPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_3_attributes (_primitive : RecordPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_4_attributes (_primitive : RecordPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_5_attributes (_primitive : RecordPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_primitive_n_attributes (_primitive : RecordPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

