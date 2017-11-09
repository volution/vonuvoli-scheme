

pub mod exports {
	pub use super::BitwisePrimitive1;
	pub use super::BitwisePrimitive2;
	pub use super::BitwisePrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive1 {
	
	Complement,
	
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive2 {
	
	ShiftLeft,
	ShiftRight,
	
	RotateLeft,
	RotateRight,
	
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitiveN {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}

