

pub mod exports {
	pub use super::TypePrimitive1;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum TypePrimitive1 {
	
	IsNull,
	IsVoid,
	IsUndefined,
	
	IsBoolean,
	IsNumberInteger,
	IsNumberReal,
	IsCharacter,
	IsSymbol,
	IsString,
	IsBytes,
	IsPair,
	IsArray,
	IsError,
	
	IsNumber,
	IsList,
	
}

