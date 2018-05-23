

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ListPrimitive0;
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitive3;
	pub use super::ListPrimitive4;
	pub use super::ListPrimitive5;
	pub use super::ListPrimitiveN;
	pub use super::ListPrimitiveV;
	
	pub use super::list_primitive_0_evaluate;
	pub use super::list_primitive_1_evaluate;
	pub use super::list_primitive_2_evaluate;
	pub use super::list_primitive_3_evaluate;
	pub use super::list_primitive_4_evaluate;
	pub use super::list_primitive_5_evaluate;
	pub use super::list_primitive_n_evaluate;
	
	pub use super::list_primitive_v_alternative_0;
	pub use super::list_primitive_v_alternative_1;
	pub use super::list_primitive_v_alternative_2;
	pub use super::list_primitive_v_alternative_3;
	pub use super::list_primitive_v_alternative_4;
	pub use super::list_primitive_v_alternative_5;
	pub use super::list_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::list_primitive_n_attributes;
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive0 {
	
	ListEmpty,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive1 {
	
	PairLeft,
	PairRight,
	
	ListFirstOfFirst,
	ListRestOfFirst,
	
	ListPairAt1,
	ListPairAt2,
	ListPairAt3,
	ListPairAt4,
	ListPairAt5,
	ListPairAt6,
	ListPairAt7,
	ListPairAt8,
	ListPairAt9,
	ListPairAt10,
	
	ListFirstAt1,
	ListFirstAt2,
	ListFirstAt3,
	ListFirstAt4,
	ListFirstAt5,
	ListFirstAt6,
	ListFirstAt7,
	ListFirstAt8,
	ListFirstAt9,
	ListFirstAt10,
	
	ListRestAt1,
	ListRestAt2,
	ListRestAt3,
	ListRestAt4,
	ListRestAt5,
	ListRestAt6,
	ListRestAt7,
	ListRestAt8,
	ListRestAt9,
	ListRestAt10,
	
	ListLength,
	ListClone,
	ListReverse,
	
	ListMake,
	
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListFill,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairToMutable,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListToMutable,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive2 {
	
	Pair,
	PairExchanged,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairLeftSet,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairRightSet,
	
	ListPairAt,
	ListFirstAt,
	ListRestAt,
	
	ListMake,
	
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListCopy,
	ListRangeClone,
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListMemberByIdentity,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListMemberByValue,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListMemberByValueRecursive,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListAssocByIdentity,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListAssocByValue,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ListAssocByValueRecursive,
	
	ListFind,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive3 {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListFirstAtSet,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRestAtSet,
	
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeCopy,
	ListRangeClone,
	
	ListMemberByComparator,
	ListAssocByComparator,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive4 {
	
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeCopy,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitive5 {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeCopy,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitiveN {
	
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ListPrimitiveV {
	
	ListMake,
	ListBuild,
	ListBuildDotted,
	ListAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ListRangeCopy,
	ListRangeClone,
	
	ListMember,
	ListAssoc,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_0_evaluate (primitive : ListPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive0::ListEmpty =>
			return list_empty () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive1::PairLeft =>
			return list_first (input_1),
		
		ListPrimitive1::PairRight =>
			return list_rest (input_1),
		
		ListPrimitive1::ListPairAt1 =>
			return list_pair_at (input_1, 0),
		
		ListPrimitive1::ListPairAt2 =>
			return list_pair_at (input_1, 1),
		
		ListPrimitive1::ListPairAt3 =>
			return list_pair_at (input_1, 2),
		
		ListPrimitive1::ListPairAt4 =>
			return list_pair_at (input_1, 3),
		
		ListPrimitive1::ListPairAt5 =>
			return list_pair_at (input_1, 4),
		
		ListPrimitive1::ListPairAt6 =>
			return list_pair_at (input_1, 5),
		
		ListPrimitive1::ListPairAt7 =>
			return list_pair_at (input_1, 6),
		
		ListPrimitive1::ListPairAt8 =>
			return list_pair_at (input_1, 7),
		
		ListPrimitive1::ListPairAt9 =>
			return list_pair_at (input_1, 8),
		
		ListPrimitive1::ListPairAt10 =>
			return list_pair_at (input_1, 9),
		
		ListPrimitive1::ListFirstAt1 =>
			return list_first_at (input_1, 0),
		
		ListPrimitive1::ListFirstAt2 =>
			return list_first_at (input_1, 1),
		
		ListPrimitive1::ListFirstAt3 =>
			return list_first_at (input_1, 2),
		
		ListPrimitive1::ListFirstAt4 =>
			return list_first_at (input_1, 3),
		
		ListPrimitive1::ListFirstAt5 =>
			return list_first_at (input_1, 4),
		
		ListPrimitive1::ListFirstAt6 =>
			return list_first_at (input_1, 5),
		
		ListPrimitive1::ListFirstAt7 =>
			return list_first_at (input_1, 6),
		
		ListPrimitive1::ListFirstAt8 =>
			return list_first_at (input_1, 7),
		
		ListPrimitive1::ListFirstAt9 =>
			return list_first_at (input_1, 8),
		
		ListPrimitive1::ListFirstAt10 =>
			return list_first_at (input_1, 9),
		
		ListPrimitive1::ListRestAt1 =>
			return list_rest_at (input_1, 0),
		
		ListPrimitive1::ListRestAt2 =>
			return list_rest_at (input_1, 1),
		
		ListPrimitive1::ListRestAt3 =>
			return list_rest_at (input_1, 2),
		
		ListPrimitive1::ListRestAt4 =>
			return list_rest_at (input_1, 3),
		
		ListPrimitive1::ListRestAt5 =>
			return list_rest_at (input_1, 4),
		
		ListPrimitive1::ListRestAt6 =>
			return list_rest_at (input_1, 5),
		
		ListPrimitive1::ListRestAt7 =>
			return list_rest_at (input_1, 6),
		
		ListPrimitive1::ListRestAt8 =>
			return list_rest_at (input_1, 7),
		
		ListPrimitive1::ListRestAt9 =>
			return list_rest_at (input_1, 8),
		
		ListPrimitive1::ListRestAt10 =>
			return list_rest_at (input_1, 9),
		
		ListPrimitive1::ListFirstOfFirst =>
			return list_first (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListRestOfFirst =>
			return list_rest (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListLength =>
			return list_length (input_1) .into_0 (),
		
		ListPrimitive1::ListClone =>
			return list_clone (input_1, None),
		
		ListPrimitive1::ListReverse =>
			return list_reverse (input_1, None),
		
		ListPrimitive1::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &UNDEFINED.into (), None),
		
		ListPrimitive1::ListBuild =>
			return list_build_1 (input_1, None, None) .into_0 (),
		
		ListPrimitive1::ListBuildDotted =>
			return input_1.clone () .into_0 (),
		
		ListPrimitive1::ListAppend =>
			return input_1.clone () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive1::ListFill =>
			return list_fill_range (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive1::PairToImmutable =>
			return try_as_pair_as_ref! (input_1) .to_immutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive1::PairToMutable =>
			return try_as_pair_as_ref! (input_1) .to_mutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive1::ListToImmutable =>
			fail_unimplemented! (0xaab9fe29, (github_issue, 37)),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive1::ListToMutable =>
			fail_unimplemented! (0xf0892d44, (github_issue, 37)),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive2::Pair =>
			return pair (input_1, input_2, None) .into_0 (),
		
		ListPrimitive2::PairExchanged =>
			return pair (input_2, input_1, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive2::PairLeftSet =>
			return pair_left_set (input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive2::PairRightSet =>
			return pair_right_set (input_1, input_2),
		
		ListPrimitive2::ListPairAt =>
			return list_pair_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListFirstAt =>
			return list_first_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListRestAt =>
			return list_rest_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2, None),
		
		ListPrimitive2::ListBuild =>
			return list_build_2 (input_1, input_2, None, None). into_0 (),
		
		ListPrimitive2::ListBuildDotted =>
			return list_build_1 (input_1, Some (input_2), None). into_0 (),
		
		ListPrimitive2::ListAppend =>
			return list_append_2 (input_1, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive2::ListFill =>
			return list_fill_range (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive2::ListCopy =>
			return list_copy_range (input_1, None, input_2, None, None),
		
		ListPrimitive2::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListMemberByIdentity =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false), false)),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListMemberByValue =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), false)),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListMemberByValueRecursive =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), false)),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListAssocByIdentity =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false), false)),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListAssocByValue =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), false)),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitive2::ListAssocByValueRecursive =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), false)),
		
		ListPrimitive2::ListFind =>
			return list_find (input_2, input_1, evaluator),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_3_evaluate (primitive : ListPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive3::ListFirstAtSet =>
			return list_first_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive3::ListRestAtSet =>
			return list_rest_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListBuild =>
			return list_build_3 (input_1, input_2, input_3, None, None) .into_0 (),
		
		ListPrimitive3::ListBuildDotted =>
			return list_build_2 (input_1, input_2, Some (input_3), None) .into_0 (),
		
		ListPrimitive3::ListAppend =>
			return list_append_3 (input_1, input_2, input_3, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive3::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive3::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, None, None),
		
		ListPrimitive3::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), Some (input_3), None),
		
		ListPrimitive3::ListMemberByComparator =>
			return list_member_by_comparator (input_2, input_1, input_3, evaluator),
		
		ListPrimitive3::ListAssocByComparator =>
			return list_assoc_by_comparator (input_2, input_1, input_3, evaluator),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_4_evaluate (primitive : ListPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive4::ListBuild =>
			return list_build_4 (input_1, input_2, input_3, input_4, None, None) .into_0 (),
		
		ListPrimitive4::ListBuildDotted =>
			return list_build_3 (input_1, input_2, input_3, Some (input_4), None) .into_0 (),
		
		ListPrimitive4::ListAppend =>
			return list_append_4 (input_1, input_2, input_3, input_4, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive4::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive4::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn list_primitive_5_evaluate (primitive : ListPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitive5::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitiveN::ListBuild =>
			return list_build_n (inputs, None, None) .into_0 (),
		
		ListPrimitiveN::ListBuildDotted =>
			return list_build_n_dotted (inputs, None) .into_0 (),
		
		ListPrimitiveN::ListAppend =>
			return list_append_n (inputs, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_0 (primitive : ListPrimitiveV) -> (Option<ListPrimitive0>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive0::ListEmpty),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitive0::ListEmpty),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive0::ListEmpty),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_1 (primitive : ListPrimitiveV) -> (Option<ListPrimitive1>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			Some (ListPrimitive1::ListMake),
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive1::ListBuild),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitive1::ListBuildDotted),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive1::ListAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive1::ListFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive1::ListClone),
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_2 (primitive : ListPrimitiveV) -> (Option<ListPrimitive2>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			Some (ListPrimitive2::ListMake),
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive2::ListBuild),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitive2::ListBuildDotted),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive2::ListAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive2::ListFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive2::ListCopy),
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive2::ListRangeClone),
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitiveV::ListMember =>
			Some (ListPrimitive2::ListMemberByValueRecursive),
		#[ cfg ( not ( feature = "vonuvoli_builtins_comparisons" ) ) ]
		ListPrimitiveV::ListMember =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ListPrimitiveV::ListAssoc =>
			Some (ListPrimitive2::ListAssocByValueRecursive),
		#[ cfg ( not ( feature = "vonuvoli_builtins_comparisons" ) ) ]
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_3 (primitive : ListPrimitiveV) -> (Option<ListPrimitive3>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive3::ListBuild),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitive3::ListBuildDotted),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive3::ListAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive3::ListRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive3::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive3::ListRangeClone),
		ListPrimitiveV::ListMember =>
			Some (ListPrimitive3::ListMemberByComparator),
		ListPrimitiveV::ListAssoc =>
			Some (ListPrimitive3::ListAssocByComparator),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_4 (primitive : ListPrimitiveV) -> (Option<ListPrimitive4>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive4::ListBuild),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitive4::ListBuildDotted),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive4::ListAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive4::ListRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive4::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_5 (primitive : ListPrimitiveV) -> (Option<ListPrimitive5>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			None,
		ListPrimitiveV::ListBuildDotted =>
			None,
		ListPrimitiveV::ListAppend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive5::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_n (primitive : ListPrimitiveV) -> (Option<ListPrimitiveN>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitiveN::ListBuild),
		ListPrimitiveV::ListBuildDotted =>
			Some (ListPrimitiveN::ListBuildDotted),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitiveN::ListAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_0_attributes (_primitive : ListPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_1_attributes (_primitive : ListPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_2_attributes (_primitive : ListPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_3_attributes (_primitive : ListPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_4_attributes (_primitive : ListPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_5_attributes (_primitive : ListPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_n_attributes (_primitive : ListPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

