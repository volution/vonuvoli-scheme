

pub mod exports {
	pub use super::ArithmeticPrimitive1;
	pub use super::ArithmeticPrimitive2;
	pub use super::ArithmeticPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArithmeticPrimitive1 {
	
	Negate,
	Absolute,
	Sign,
	
	Square,
	SquareRoot,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArithmeticPrimitive2 {
	
	Power,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArithmeticPrimitiveN {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	Minimum,
	Maximum,
	
	Average,
	
}

