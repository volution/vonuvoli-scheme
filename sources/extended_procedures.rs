

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;
use super::runtime::exports::*;
use super::conversions::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ProcedureExtended;
	pub use super::ProcedureExtendedInternals;
	
	pub use super::procedure_extended_evaluate_0;
	pub use super::procedure_extended_evaluate_1;
	pub use super::procedure_extended_evaluate_2;
	pub use super::procedure_extended_evaluate_3;
	pub use super::procedure_extended_evaluate_4;
	pub use super::procedure_extended_evaluate_5;
	pub use super::procedure_extended_evaluate_n;
	
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
pub struct ProcedureExtended ( StdRc<ProcedureExtendedInternals> );


#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ProcedureExtendedInternals {
	
	ComposedPrimitive1 (StdBox<[ProcedurePrimitive1]>),
	Composed1 (StdBox<[Value]>),
	ComposedV (StdBox<[Value]>),
	
	CurryLeft (Value, StdBox<[Value]>),
	CurryRight (Value, StdBox<[Value]>),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKindIs (RecordKind, Option<bool>),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordBuildN (RecordKind, Option<StdBox<[usize]>>, Option<bool>),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordBuildC (RecordKind, Option<StdBox<[usize]>>, Option<bool>),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordGet (Option<RecordKind>, usize),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordGetX (Option<RecordKind>, Value),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSet (Option<RecordKind>, usize),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordSetX (Option<RecordKind>, Value),
	
	Constant (Value, bool),
	
	Not (Value),
	
}


impl ProcedureExtended {
	
	pub fn new (internals : ProcedureExtendedInternals) -> (ProcedureExtended) {
		return ProcedureExtended (StdRc::new (internals));
	}
	
	pub fn internals_ref (&self) -> (&ProcedureExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn is_self (&self, other : &ProcedureExtended) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}




pub fn procedure_extended_evaluate_0 (extended : &ProcedureExtended, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	const INPUTS_EMPTY : &[&Value] = &[];
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), INPUTS_EMPTY),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, INPUTS_EMPTY, values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_0 (kind, option_box_as_ref (fields), immutable),
		
		ProcedureExtendedInternals::Constant (ref value, _) =>
			succeed! (value.clone ()),
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_0 (callable))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, INPUTS_EMPTY, evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_1 (extended : &ProcedureExtended, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (ref primitives) =>
			return call_primitives_1 (evaluator, primitives.as_ref (), input_1),
		
		ProcedureExtendedInternals::Composed1 (ref callables) =>
			return call_composed_1_1 (evaluator, callables.as_ref (), input_1),
		
		ProcedureExtendedInternals::ComposedV (ref callables) =>
			return call_composed_v_1 (evaluator, callables.as_ref (), input_1),
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), &[input_1]),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, &[input_1], values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordKindIs (ref kind, immutable) =>
			return record_kind_is (kind, input_1, immutable) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_1 (kind, option_box_as_ref (fields), input_1, immutable),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildC (ref kind, ref fields, immutable) =>
			return record_build (kind, option_box_as_ref (fields), input_1, immutable),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordGet (ref kind, field) =>
			return record_get (kind.as_ref (), field, input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordGetX (ref kind, ref field) =>
			return record_get_x (kind.as_ref (), field, input_1),
		
		ProcedureExtendedInternals::Constant (ref value, ignore) =>
			if ignore {
				succeed! (value.clone ());
			} else {
				fail! (0x478a25fd);
			},
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_1 (callable, input_1))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, &[input_1], evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_2 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), &[input_1, input_2]),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, &[input_1, input_2], values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_2 (kind, option_box_as_ref (fields), input_1, input_2, immutable),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ProcedureExtendedInternals::RecordSet (ref kind, field) =>
			return record_set (kind.as_ref (), field, input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ProcedureExtendedInternals::RecordSetX (ref kind, ref field) =>
			return record_set_x (kind.as_ref (), field, input_1, input_2),
		
		ProcedureExtendedInternals::Constant (ref value, ignore) =>
			if ignore {
				succeed! (value.clone ());
			} else {
				fail! (0xafb82b34);
			},
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_2 (callable, input_1, input_2))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, &[input_1, input_2], evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_3 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), &[input_1, input_2, input_3]),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, &[input_1, input_2, input_3], values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_3 (kind, option_box_as_ref (fields), input_1, input_2, input_3, immutable),
		
		ProcedureExtendedInternals::Constant (ref value, ignore) =>
			if ignore {
				succeed! (value.clone ());
			} else {
				fail! (0xbbbd9722);
			},
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_3 (callable, input_1, input_2, input_3))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, &[input_1, input_2, input_3], evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_4 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), &[input_1, input_2, input_3, input_4]),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, &[input_1, input_2, input_3, input_4], values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_4 (kind, option_box_as_ref (fields), input_1, input_2, input_3, input_4, immutable),
		
		ProcedureExtendedInternals::Constant (ref value, ignore) =>
			if ignore {
				succeed! (value.clone ());
			} else {
				fail! (0x58bf628b);
			},
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_4 (callable, input_1, input_2, input_3, input_4))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, &[input_1, input_2, input_3, input_4], evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_5 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::CurryLeft (ref callable, ref values) =>
			return call_n_n (evaluator, callable, values.as_ref (), &[input_1, input_2, input_3, input_4, input_5]),
		
		ProcedureExtendedInternals::CurryRight (ref callable, ref values) =>
			return call_n_n (evaluator, callable, &[input_1, input_2, input_3, input_4, input_5], values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable) =>
			return record_build_n (kind, option_box_as_ref (fields), &[input_1, input_2, input_3, input_4, input_5], immutable),
		
		ProcedureExtendedInternals::Constant (ref value, ignore) =>
			if ignore {
				succeed! (value.clone ());
			} else {
				fail! (0xdf30d703);
			},
		
		ProcedureExtendedInternals::Not (ref callable) =>
			return is_false (& r#try! (evaluator.evaluate_procedure_call_5 (callable, input_1, input_2, input_3, input_4, input_5))) .into_0 (),
		
		_ => {
			FIXME! ("refactor this", (github_issue, 80));
			return procedure_extended_evaluate_n (extended, &[input_1, input_2, input_3, input_4, input_5], evaluator);
		},
		
	}
}




pub fn procedure_extended_evaluate_n (extended : &ProcedureExtended, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match (inputs_count, extended.internals_ref ()) {
		
		(1, &ProcedureExtendedInternals::ComposedPrimitive1 (ref primitives)) =>
			return call_primitives_1 (evaluator, primitives.as_ref (), inputs[0].as_ref ()),
		
		(_, &ProcedureExtendedInternals::Composed1 (ref callables)) =>
			return call_composed_1_n (evaluator, callables.as_ref (), inputs),
		
		(_, &ProcedureExtendedInternals::ComposedV (ref callables)) =>
			return call_composed_v_n (evaluator, callables.as_ref (), inputs),
		
		(_, &ProcedureExtendedInternals::CurryLeft (ref callable, ref values)) =>
			return call_n_n (evaluator, callable, values.as_ref (), inputs),
		
		(_, &ProcedureExtendedInternals::CurryRight (ref callable, ref values)) =>
			return call_n_n (evaluator, callable, inputs, values.as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		(1, &ProcedureExtendedInternals::RecordKindIs (ref kind, immutable)) =>
			return record_kind_is (kind, inputs[0].as_ref (), immutable) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		(1, &ProcedureExtendedInternals::RecordGet (ref kind, field)) =>
			return record_get (kind.as_ref (), field, inputs[0].as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		(1, &ProcedureExtendedInternals::RecordGetX (ref kind, ref field)) =>
			return record_get_x (kind.as_ref (), field, inputs[0].as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		(2, &ProcedureExtendedInternals::RecordSet (ref kind, field)) =>
			return record_set (kind.as_ref (), field, inputs[0].as_ref (), inputs[1].as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		(2, &ProcedureExtendedInternals::RecordSetX (ref kind, ref field)) =>
			return record_set_x (kind.as_ref (), field, inputs[0].as_ref (), inputs[1].as_ref ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		(_, &ProcedureExtendedInternals::RecordBuildN (ref kind, ref fields, immutable)) =>
			return record_build_n (kind, option_box_as_ref (fields), inputs, immutable),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		(1, &ProcedureExtendedInternals::RecordBuildC (ref kind, ref fields, immutable)) =>
			return record_build (kind, option_box_as_ref (fields), inputs[0].as_ref (), immutable),
		
		(_, &ProcedureExtendedInternals::Constant (ref value, ignore)) =>
			if ignore || inputs_count == 0 {
				succeed! (value.clone ());
			} else {
				fail! (0x59dc9d37);
			},
		
		(_, &ProcedureExtendedInternals::Not (ref callable)) => {
			let inputs = vec_slice_to_ref (inputs);
			return is_false (& r#try! (evaluator.evaluate_procedure_call_n (callable, &inputs))) .into_0 ();
		},
		
		_ =>
			fail! (0x7b179cf1),
		
	}
}

