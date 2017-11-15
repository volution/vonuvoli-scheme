

//use super::constants::exports::*;
//use super::errors::exports::*;
//use super::runtime::exports::*;
//use super::values::exports::*;




pub mod exports {
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive1 {
	
	GetLeft,
	GetRight,
	
	List1,
	Append1,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	SetLeft,
	SetRight,
	
	List2,
	Append2,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitiveN {
	
	ListN,
	AppendN,
	
}

