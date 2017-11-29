

use super::contexts::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
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
	
	Sequence ( ExpressionVec ),
	Conditional ( StdVec<(bool, Expression, Option<Expression>)> ),
	
	ContextDefine ( Symbol, ExpressionBox ),
	ContextUpdate ( Symbol, ExpressionBox ),
	ContextSelect ( Symbol ),
	
	RegisterClosure ( ExpressionBox, StdVec<RegistersBindingTemplate> ),
	RegisterInitialize1 ( usize, ExpressionBox ),
	RegisterInitializeN ( StdVec<(usize, Expression)>, bool ),
	RegisterSet ( usize, ExpressionBox ),
	RegisterGet ( usize ),
	
	BindingInitialize1 ( Binding, ExpressionBox ),
	BindingInitializeN ( StdVec<(Binding, Expression)>, bool ),
	BindingSet ( Binding, ExpressionBox ),
	BindingGet ( Binding ),
	
	ProcedureCall0 ( ExpressionBox ),
	ProcedureCall1 ( ExpressionBox, ExpressionBox ),
	ProcedureCall2 ( ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall3 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall4 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall5 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCallN ( ExpressionBox, ExpressionVec ),
	ProcedureCall ( ExpressionBox, ExpressionVec ),
	
	ProcedurePrimitiveCall0 ( ProcedurePrimitive0 ),
	ProcedurePrimitiveCall1 ( ProcedurePrimitive1, ExpressionBox ),
	ProcedurePrimitiveCall2 ( ProcedurePrimitive2, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall3 ( ProcedurePrimitive3, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall4 ( ProcedurePrimitive4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall5 ( ProcedurePrimitive5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCallN ( ProcedurePrimitiveN, ExpressionVec ),
	ProcedurePrimitiveCall ( ProcedurePrimitive, ExpressionVec ),
	
	SyntaxPrimitiveCall0 ( SyntaxPrimitive0 ),
	SyntaxPrimitiveCall1 ( SyntaxPrimitive1, ExpressionBox ),
	SyntaxPrimitiveCall2 ( SyntaxPrimitive2, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall3 ( SyntaxPrimitive3, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall4 ( SyntaxPrimitive4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall5 ( SyntaxPrimitive5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCallN ( SyntaxPrimitiveN, ExpressionVec ),
	SyntaxPrimitiveCall ( SyntaxPrimitive, ExpressionVec ),
	
	Lambda ( StdBox<LambdaTemplate>, ExpressionBox, StdVec<RegistersBindingTemplate>, StdVec<RegistersBindingTemplate> ),
	
}


pub type ExpressionBox = StdBox<Expression>;
pub type ExpressionVec = StdVec<Expression>;

