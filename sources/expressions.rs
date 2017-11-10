

use super::contexts::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::Expression;
	pub use super::ExpressionBox;
	pub use super::ExpressionVec;
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Expression {
	
	Void,
	Value ( Value ),
	
	ContextDefine ( Symbol, ExpressionBox ),
	ContextUpdate ( Symbol, ExpressionBox ),
	ContextSelect ( Symbol ),
	
	RegisterGet ( usize ),
	RegisterSet ( usize, ExpressionBox ),
	
	BindingGet ( Binding ),
	BindingSet ( Binding, ExpressionBox ),
	
	ProcedurePrimitiveCall0 ( ProcedurePrimitive0 ),
	ProcedurePrimitiveCall1 ( ProcedurePrimitive1, ExpressionBox ),
	ProcedurePrimitiveCall2 ( ProcedurePrimitive2, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCallN ( ProcedurePrimitiveN, ExpressionVec ),
	ProcedurePrimitiveCall ( ProcedurePrimitive, ExpressionVec ),
	
	SyntaxPrimitiveCall1 ( SyntaxPrimitive1, ExpressionBox ),
	SyntaxPrimitiveCall2 ( SyntaxPrimitive2, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCallN ( SyntaxPrimitiveN, ExpressionVec ),
	SyntaxPrimitiveCall ( SyntaxPrimitive, ExpressionVec ),
	
}


pub type ExpressionBox = StdBox<Expression>;
pub type ExpressionVec = StdVec<Expression>;

