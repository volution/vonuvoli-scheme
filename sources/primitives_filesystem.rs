

use super::builtins::exports::*;
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
	
	PathCoerce,
	PathParent,
	
	PathToString,
	StringToPath,
	PathToBytes,
	BytesToPath,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitiveN {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemPrimitiveV {}




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
		
		FileSystemPrimitive1::PathCoerce =>
			return filesystem_path_coerce (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathParent =>
			return filesystem_path_parent (input_1),
		
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
pub fn filesystem_primitive_2_evaluate (primitive : FileSystemPrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_3_evaluate (primitive : FileSystemPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_4_evaluate (primitive : FileSystemPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_5_evaluate (primitive : FileSystemPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_n_evaluate (primitive : FileSystemPrimitiveN, _inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_0 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive0>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_1 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive1>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_2 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive2>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_3 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive3>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_4 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive4>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_5 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive5>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_primitive_v_alternative_n (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitiveN>) {
	match primitive {}
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

