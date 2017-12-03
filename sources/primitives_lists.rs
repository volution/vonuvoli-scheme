

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ListPrimitive0;
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitive3;
	pub use super::ListPrimitive4;
	pub use super::ListPrimitive5;
	pub use super::ListPrimitiveN;
	
	pub use super::list_primitive_0_evaluate;
	pub use super::list_primitive_1_evaluate;
	pub use super::list_primitive_2_evaluate;
	pub use super::list_primitive_3_evaluate;
	pub use super::list_primitive_4_evaluate;
	pub use super::list_primitive_5_evaluate;
	pub use super::list_primitive_n_evaluate;
	
	pub use super::list_primitive_n_alternative_0;
	pub use super::list_primitive_n_alternative_1;
	pub use super::list_primitive_n_alternative_2;
	pub use super::list_primitive_n_alternative_3;
	pub use super::list_primitive_n_alternative_4;
	pub use super::list_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive0 {
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive1 {
	
	PairLeft,
	PairRight,
	
	ListFirstOfFirst,
	ListRestOfFirst,
	
	ListFirstAt2,
	ListRestAt2,
	
	ListLength,
	ListClone,
	ListReverse,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
	ListFill,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	PairLeftSet,
	PairRightSet,
	
	ListPairAt,
	ListFirstAt,
	ListRestAt,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
	ListFill,
	ListCopy,
	ListRangeClone,
	
	ListMemberByIdentity,
	ListMemberByValue,
	ListMemberByValueRecursive,
	ListAssocByIdentity,
	ListAssocByValue,
	ListAssocByValueRecursive,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive3 {
	
	ListFirstAtSet,
	ListRestAtSet,
	
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	ListRangeClone,
	
	ListMemberByComparator,
	ListAssocByComparator,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive4 {
	
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive5 {
	
	ListRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitiveN {
	
	ListMake,
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	ListRangeClone,
	
	ListMember,
	ListAssoc,
	
}




pub fn list_primitive_0_evaluate (primitive : ListPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive0::ListBuild =>
			succeed! (list_empty ()),
		
		ListPrimitive0::ListAppend =>
			succeed! (list_empty ()),
		
	}
}




pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive1::PairLeft =>
			return list_first (input_1),
		
		ListPrimitive1::PairRight =>
			return list_rest (input_1),
		
		ListPrimitive1::ListFirstAt2 =>
			return list_first_at (input_1, 1),
		
		ListPrimitive1::ListRestAt2 =>
			return list_rest_at (input_1, 1),
		
		ListPrimitive1::ListFirstOfFirst =>
			return list_first (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListRestOfFirst =>
			return list_rest (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListLength => {
			let length = try! (list_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		ListPrimitive1::ListClone =>
			return list_clone (input_1),
		
		ListPrimitive1::ListReverse =>
			return list_reverse (input_1),
		
		ListPrimitive1::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &UNDEFINED.into ()),
		
		ListPrimitive1::ListBuild =>
			succeed! (list_build_1 (input_1)),
		
		ListPrimitive1::ListAppend =>
			succeed! (input_1.clone ()),
		
		ListPrimitive1::ListFill =>
			return list_fill_range (input_1, None, None, None),
		
	}
}




pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive2::Pair =>
			succeed! (pair (input_1, input_2)),
		
		ListPrimitive2::PairLeftSet =>
			return pair_left_set (input_1, input_2),
		
		ListPrimitive2::PairRightSet =>
			return pair_right_set (input_1, input_2),
		
		ListPrimitive2::ListPairAt =>
			return list_pair_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListFirstAt =>
			return list_first_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListRestAt =>
			return list_rest_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2),
		
		ListPrimitive2::ListBuild =>
			succeed! (list_build_2 (input_1, input_2)),
		
		ListPrimitive2::ListAppend =>
			return list_append_2 (input_1, input_2),
		
		ListPrimitive2::ListFill =>
			return list_fill_range (input_1, Some (input_2), None, None),
		
		ListPrimitive2::ListCopy =>
			return list_copy_range (input_1, None, input_2, None, None),
		
		ListPrimitive2::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), None),
		
		ListPrimitive2::ListMemberByIdentity =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false))),
		
		ListPrimitive2::ListMemberByValue =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))),
		
		ListPrimitive2::ListMemberByValueRecursive =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))),
		
		ListPrimitive2::ListAssocByIdentity =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false))),
		
		ListPrimitive2::ListAssocByValue =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))),
		
		ListPrimitive2::ListAssocByValueRecursive =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))),
		
	}
}




pub fn list_primitive_3_evaluate (primitive : ListPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive3::ListFirstAtSet =>
			return list_first_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListRestAtSet =>
			return list_rest_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListBuild =>
			succeed! (list_build_3 (input_1, input_2, input_3)),
		
		ListPrimitive3::ListAppend =>
			return list_append_3 (input_1, input_2, input_3),
		
		ListPrimitive3::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		ListPrimitive3::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, None, None),
		
		ListPrimitive3::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), Some (input_3)),
		
		ListPrimitive3::ListMemberByComparator =>
			return list_member_by_comparator (input_2, input_1, input_3),
		
		ListPrimitive3::ListAssocByComparator =>
			return list_assoc_by_comparator (input_2, input_1, input_3),
		
	}
}




pub fn list_primitive_4_evaluate (primitive : ListPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive4::ListBuild =>
			succeed! (list_build_4 (input_1, input_2, input_3, input_4)),
		
		ListPrimitive4::ListAppend =>
			return list_append_4 (input_1, input_2, input_3, input_4),
		
		ListPrimitive4::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		ListPrimitive4::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




pub fn list_primitive_5_evaluate (primitive : ListPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive5::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		ListPrimitiveN::ListMake =>
			match inputs_count {
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListMake, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0xdd5940d5),
			},
		
		ListPrimitiveN::ListBuild =>
			match inputs_count {
				0 =>
					return list_primitive_0_evaluate (ListPrimitive0::ListBuild),
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListBuild, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListBuild, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					succeed! (list_build_n (inputs)),
			},
		
		ListPrimitiveN::ListAppend =>
			match inputs_count {
				0 =>
					return list_primitive_0_evaluate (ListPrimitive0::ListAppend),
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListAppend, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListAppend, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return list_append_n (inputs),
			},
		
		ListPrimitiveN::ListRangeFill =>
			match inputs_count {
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListFill, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListFill, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListRangeFill, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListRangeFill, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					fail! (0x0634b1d5),
			},
		
		ListPrimitiveN::ListRangeCopy =>
			match inputs_count {
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListCopy, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListRangeCopy, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				5 =>
					return list_primitive_5_evaluate (ListPrimitive5::ListRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]),
				_ =>
					fail! (0x749c9d57),
			},
		
		ListPrimitiveN::ListRangeClone =>
			match inputs_count {
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListClone, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListRangeClone, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListRangeClone, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xf28cef92),
			},
		
		ListPrimitiveN::ListMember =>
			match inputs_count {
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListMemberByValueRecursive, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListMemberByComparator, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x0bac53c1),
			},
		
		ListPrimitiveN::ListAssoc =>
			match inputs_count {
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListAssocByValueRecursive, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListAssocByComparator, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x69aaa417),
			},
		
	}
}




pub fn list_primitive_n_alternative_0 (primitive : ListPrimitiveN) -> (Option<ListPrimitive0>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			None,
		ListPrimitiveN::ListBuild =>
			Some (ListPrimitive0::ListBuild),
		ListPrimitiveN::ListAppend =>
			Some (ListPrimitive0::ListAppend),
		ListPrimitiveN::ListRangeFill =>
			None,
		ListPrimitiveN::ListRangeCopy =>
			None,
		ListPrimitiveN::ListRangeClone =>
			None,
		ListPrimitiveN::ListMember =>
			None,
		ListPrimitiveN::ListAssoc =>
			None,
	}
}


pub fn list_primitive_n_alternative_1 (primitive : ListPrimitiveN) -> (Option<ListPrimitive1>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			Some (ListPrimitive1::ListMake),
		ListPrimitiveN::ListBuild =>
			Some (ListPrimitive1::ListBuild),
		ListPrimitiveN::ListAppend =>
			Some (ListPrimitive1::ListAppend),
		ListPrimitiveN::ListRangeFill =>
			Some (ListPrimitive1::ListFill),
		ListPrimitiveN::ListRangeCopy =>
			None,
		ListPrimitiveN::ListRangeClone =>
			Some (ListPrimitive1::ListClone),
		ListPrimitiveN::ListMember =>
			None,
		ListPrimitiveN::ListAssoc =>
			None,
	}
}


pub fn list_primitive_n_alternative_2 (primitive : ListPrimitiveN) -> (Option<ListPrimitive2>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			Some (ListPrimitive2::ListMake),
		ListPrimitiveN::ListBuild =>
			Some (ListPrimitive2::ListBuild),
		ListPrimitiveN::ListAppend =>
			Some (ListPrimitive2::ListAppend),
		ListPrimitiveN::ListRangeFill =>
			Some (ListPrimitive2::ListFill),
		ListPrimitiveN::ListRangeCopy =>
			Some (ListPrimitive2::ListCopy),
		ListPrimitiveN::ListRangeClone =>
			Some (ListPrimitive2::ListRangeClone),
		ListPrimitiveN::ListMember =>
			Some (ListPrimitive2::ListMemberByValueRecursive),
		ListPrimitiveN::ListAssoc =>
			Some (ListPrimitive2::ListAssocByValueRecursive),
	}
}


pub fn list_primitive_n_alternative_3 (primitive : ListPrimitiveN) -> (Option<ListPrimitive3>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			None,
		ListPrimitiveN::ListBuild =>
			Some (ListPrimitive3::ListBuild),
		ListPrimitiveN::ListAppend =>
			Some (ListPrimitive3::ListAppend),
		ListPrimitiveN::ListRangeFill =>
			Some (ListPrimitive3::ListRangeFill),
		ListPrimitiveN::ListRangeCopy =>
			Some (ListPrimitive3::ListRangeCopy),
		ListPrimitiveN::ListRangeClone =>
			Some (ListPrimitive3::ListRangeClone),
		ListPrimitiveN::ListMember =>
			Some (ListPrimitive3::ListMemberByComparator),
		ListPrimitiveN::ListAssoc =>
			Some (ListPrimitive3::ListAssocByComparator),
	}
}


pub fn list_primitive_n_alternative_4 (primitive : ListPrimitiveN) -> (Option<ListPrimitive4>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			None,
		ListPrimitiveN::ListBuild =>
			Some (ListPrimitive4::ListBuild),
		ListPrimitiveN::ListAppend =>
			Some (ListPrimitive4::ListAppend),
		ListPrimitiveN::ListRangeFill =>
			Some (ListPrimitive4::ListRangeFill),
		ListPrimitiveN::ListRangeCopy =>
			Some (ListPrimitive4::ListRangeCopy),
		ListPrimitiveN::ListRangeClone =>
			None,
		ListPrimitiveN::ListMember =>
			None,
		ListPrimitiveN::ListAssoc =>
			None,
	}
}


pub fn list_primitive_n_alternative_5 (primitive : ListPrimitiveN) -> (Option<ListPrimitive5>) {
	match primitive {
		ListPrimitiveN::ListMake =>
			None,
		ListPrimitiveN::ListBuild =>
			None,
		ListPrimitiveN::ListAppend =>
			None,
		ListPrimitiveN::ListRangeFill =>
			None,
		ListPrimitiveN::ListRangeCopy =>
			Some (ListPrimitive5::ListRangeCopy),
		ListPrimitiveN::ListRangeClone =>
			None,
		ListPrimitiveN::ListMember =>
			None,
		ListPrimitiveN::ListAssoc =>
			None,
	}
}

