

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::FileSystemPrimitive0;
	pub use super::FileSystemPrimitive1;
	pub use super::FileSystemPrimitive2;
	pub use super::FileSystemPrimitive3;
	pub use super::FileSystemPrimitive4;
	pub use super::FileSystemPrimitive5;
	pub use super::FileSystemPrimitiveN;
	pub use super::FileSystemPrimitiveV;
	
	pub use super::filesystem_primitive_0_evaluate;
	pub use super::filesystem_primitive_1_evaluate;
	pub use super::filesystem_primitive_2_evaluate;
	pub use super::filesystem_primitive_3_evaluate;
	pub use super::filesystem_primitive_4_evaluate;
	pub use super::filesystem_primitive_5_evaluate;
	pub use super::filesystem_primitive_n_evaluate;
	
	pub use super::filesystem_primitive_v_alternative_0;
	pub use super::filesystem_primitive_v_alternative_1;
	pub use super::filesystem_primitive_v_alternative_2;
	pub use super::filesystem_primitive_v_alternative_3;
	pub use super::filesystem_primitive_v_alternative_4;
	pub use super::filesystem_primitive_v_alternative_5;
	pub use super::filesystem_primitive_v_alternative_n;
	
	pub use super::filesystem_primitive_0_attributes;
	pub use super::filesystem_primitive_1_attributes;
	pub use super::filesystem_primitive_2_attributes;
	pub use super::filesystem_primitive_3_attributes;
	pub use super::filesystem_primitive_4_attributes;
	pub use super::filesystem_primitive_5_attributes;
	pub use super::filesystem_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive1 {
	
	FileExists,
	FileDelete,
	
	LinkResolve,
	PathCanonicalize,
	
	PathCoerce,
	PathParent,
	PathJoin,
	PathSplit,
	
	PathName,
	PathNameWithoutExtension,
	PathNameOnlyExtension,
	PathNameJoin,
	PathNameSplit,
	
	PathToString,
	StringToPath,
	PathToBytes,
	BytesToPath,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive2 {
	
	PathJoin,
	PathSplit,
	PathHasPrefix,
	PathHasSuffix,
	
	PathNameJoin,
	PathNameSplit,
	PathNameIs,
	PathNameHasPrefix,
	PathNameHasSuffix,
	
	LinkResolve,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive3 {
	
	PathJoin,
	PathNameJoin,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive4 {
	
	PathJoin,
	PathNameJoin,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive5 {
	
	PathJoin,
	PathNameJoin,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitiveN {
	
	PathJoin,
	PathNameJoin,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitiveV {
	
	PathJoin,
	PathSplit,
	PathNameJoin,
	PathNameSplit,
	
	LinkResolve,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_0_evaluate (primitive : FileSystemPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_1_evaluate (primitive : FileSystemPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive1::FileExists =>
			return filesystem_file_exists (input_1) .into_0 (),
		
		FileSystemPrimitive1::FileDelete =>
			return filesystem_file_delete (input_1) .into_0 (),
		
		FileSystemPrimitive1::LinkResolve =>
			return filesystem_link_resolve (input_1, false, false),
		
		FileSystemPrimitive1::PathCanonicalize =>
			return filesystem_path_canonicalize (input_1),
		
		FileSystemPrimitive1::PathCoerce =>
			return filesystem_path_coerce (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::PathParent =>
			return filesystem_path_parent (input_1),
		
		FileSystemPrimitive1::PathJoin =>
			return filesystem_path_join (&[input_1], true) .into_0 (),
		
		FileSystemPrimitive1::PathSplit =>
			return filesystem_path_split (input_1, false) .into_0 (),
		
		FileSystemPrimitive1::PathName =>
			return filesystem_path_name (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameWithoutExtension =>
			return filesystem_path_name_without_extension (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameOnlyExtension =>
			return filesystem_path_name_only_extension (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameJoin =>
			return filesystem_path_name_join (&[input_1]) .into_0 (),
		
		FileSystemPrimitive1::PathNameSplit =>
			return filesystem_path_name_split (input_1, false) .into_0 (),
		
		FileSystemPrimitive1::PathToString =>
			return filesystem_path_to_string (input_1, false) .into (),
		
		FileSystemPrimitive1::StringToPath =>
			return filesystem_string_to_path (input_1) .into (),
		
		FileSystemPrimitive1::PathToBytes =>
			return filesystem_path_to_bytes (input_1) .into (),
		
		FileSystemPrimitive1::BytesToPath =>
			return filesystem_bytes_to_path (input_1) .into (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_2_evaluate (primitive : FileSystemPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive2::PathJoin =>
			return filesystem_path_join (&[input_1, input_2], true) .into_0 (),
		
		FileSystemPrimitive2::PathSplit =>
			return filesystem_path_split (input_1, try! (boolean_coerce (Some (input_2))) .unwrap_or (false)) .into_0 (),
		
		FileSystemPrimitive2::PathHasPrefix =>
			return filesystem_path_has_prefix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathHasSuffix =>
			return filesystem_path_has_suffix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2]) .into_0 (),
		
		FileSystemPrimitive2::PathNameSplit =>
			return filesystem_path_name_split (input_1, try! (boolean_coerce (Some (input_2))) .unwrap_or (false)) .into_0 (),
		
		FileSystemPrimitive2::LinkResolve =>
			return filesystem_link_resolve (input_1, try! (boolean_coerce (Some (input_2))) .unwrap_or (false), false),
		
		FileSystemPrimitive2::PathNameIs =>
			return filesystem_path_name_is (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameHasPrefix =>
			return filesystem_path_name_has_prefix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameHasSuffix =>
			return filesystem_path_name_has_suffix (input_1, input_2) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_3_evaluate (primitive : FileSystemPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive3::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3], true) .into_0 (),
		
		FileSystemPrimitive3::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3]) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_4_evaluate (primitive : FileSystemPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive4::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3, input_4], true) .into_0 (),
		
		FileSystemPrimitive4::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3, input_4]) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_5_evaluate (primitive : FileSystemPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive5::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3, input_4, input_5], true) .into_0 (),
		
		FileSystemPrimitive5::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3, input_4, input_5]) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_n_evaluate (primitive : FileSystemPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitiveN::PathJoin =>
			return filesystem_path_join (inputs, true) .into_0 (),
		
		FileSystemPrimitiveN::PathNameJoin =>
			return filesystem_path_name_join (inputs) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_0 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive0>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			None,
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			None,
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::LinkResolve =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_1 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive1>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive1::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			Some (FileSystemPrimitive1::PathSplit),
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive1::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			Some (FileSystemPrimitive1::PathNameSplit),
		FileSystemPrimitiveV::LinkResolve =>
			Some (FileSystemPrimitive1::LinkResolve),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_2 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive2>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive2::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			Some (FileSystemPrimitive2::PathSplit),
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive2::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			Some (FileSystemPrimitive2::PathNameSplit),
		FileSystemPrimitiveV::LinkResolve =>
			Some (FileSystemPrimitive2::LinkResolve),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_3 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive3>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive3::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive3::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::LinkResolve =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_4 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive4>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive4::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive4::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::LinkResolve =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_5 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive5>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive5::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive5::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::LinkResolve =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_n (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitiveN>) {
	match primitive {
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitiveN::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitiveN::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::LinkResolve =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_0_attributes (_primitive : FileSystemPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_1_attributes (_primitive : FileSystemPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_2_attributes (_primitive : FileSystemPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_3_attributes (_primitive : FileSystemPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_4_attributes (_primitive : FileSystemPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_5_attributes (_primitive : FileSystemPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_n_attributes (_primitive : FileSystemPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

