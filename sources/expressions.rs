

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




#[ derive (Clone, Debug, Hash) ]
pub enum Expression {
	
	Void,
	Value ( Value ),
	
	Sequence ( StdBox<[Expression]> ),
	ConditionalIf ( StdBox<[(Option<(Expression, bool)>, Option<Expression>)]> ),
	ConditionalMatch ( StdBox<Expression>, StdBox<[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]> ),
	
	ContextDefine ( Symbol, ExpressionBox ),
	ContextUpdate ( Symbol, ExpressionBox ),
	ContextSelect ( Symbol ),
	
	RegisterClosure ( ExpressionBox, StdBox<[RegistersBindingTemplate]> ),
	RegisterInitialize1 ( usize, ExpressionBox ),
	RegisterInitializeN ( StdBox<[(usize, Expression)]>, bool ),
	RegisterInitializeValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterSet1 ( usize, ExpressionBox ),
	RegisterSetValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterGet1 ( usize ),
	
	BindingInitialize1 ( Binding, ExpressionBox ),
	BindingInitializeN ( StdBox<[(Binding, Expression)]>, bool ),
	BindingInitializeValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingSet1 ( Binding, ExpressionBox ),
	BindingSetValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingGet1 ( Binding ),
	
	ProcedureCall0 ( ExpressionBox ),
	ProcedureCall1 ( ExpressionBox, ExpressionBox ),
	ProcedureCall2 ( ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall3 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall4 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall5 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCallN ( ExpressionBox, StdBox<[Expression]> ),
	ProcedureCall ( ExpressionBox, StdBox<[Expression]> ),
	
	ProcedurePrimitiveCall0 ( ProcedurePrimitive0 ),
	ProcedurePrimitiveCall1 ( ProcedurePrimitive1, ExpressionBox ),
	ProcedurePrimitiveCall2 ( ProcedurePrimitive2, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall3 ( ProcedurePrimitive3, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall4 ( ProcedurePrimitive4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall5 ( ProcedurePrimitive5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCallN ( ProcedurePrimitiveN, StdBox<[Expression]> ),
	ProcedurePrimitiveCall ( ProcedurePrimitive, StdBox<[Expression]> ),
	
	SyntaxPrimitiveCall0 ( SyntaxPrimitive0 ),
	SyntaxPrimitiveCall1 ( SyntaxPrimitive1, ExpressionBox ),
	SyntaxPrimitiveCall2 ( SyntaxPrimitive2, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall3 ( SyntaxPrimitive3, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall4 ( SyntaxPrimitive4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCall5 ( SyntaxPrimitive5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	SyntaxPrimitiveCallN ( SyntaxPrimitiveN, StdBox<[Expression]> ),
	SyntaxPrimitiveCall ( SyntaxPrimitive, StdBox<[Expression]> ),
	
	Lambda ( StdBox<LambdaTemplate>, ExpressionBox, StdBox<[RegistersBindingTemplate]>, StdBox<[RegistersBindingTemplate]> ),
	
}


pub type ExpressionBox = StdBox<Expression>;
pub type ExpressionVec = StdVec<Expression>;

