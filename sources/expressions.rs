

use super::contexts::exports::*;
use super::extended_procedures::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::Expression;
	pub use super::ExpressionBox;
	pub use super::ExpressionVec;
	
	pub use super::ExpressionConditionalIfClauses;
	pub use super::ExpressionConditionalIfClause;
	pub use super::ExpressionConditionalIfGuard;
	pub use super::ExpressionConditionalMatchClauses;
	pub use super::ExpressionConditionalMatchClause;
	pub use super::ExpressionConditionalMatchGuard;
	pub use super::ExpressionValueConsumer;
	
	pub use super::ExpressionForContexts;
	pub use super::ExpressionForProcedureGenericCall;
	pub use super::ExpressionForProcedurePrimitiveCall;
	pub use super::ExpressionForProcedureExtendedCall;
	pub use super::ExpressionForProcedureLambdaCall;
	pub use super::ExpressionForProcedureNativeCall;
	
	pub use super::ExpressionSequenceOperator;
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum Expression {
	
	Void,
	Value ( Value ),
	
	Sequence ( ExpressionSequenceOperator, StdBox<[Expression]> ),
	ConditionalIf ( ExpressionConditionalIfClauses ),
	ConditionalMatch ( ExpressionBox, ExpressionConditionalMatchClauses ),
	Loop ( Option<ExpressionBox>, Option<ExpressionBox>, Option<ExpressionBox>, ExpressionConditionalIfClauses ),
	
	Contexts ( ExpressionForContexts ),
	
	ProcedureGenericCall ( ExpressionForProcedureGenericCall ),
	ProcedurePrimitiveCall ( ExpressionForProcedurePrimitiveCall ),
	ProcedureExtendedCall ( ExpressionForProcedureExtendedCall ),
	ProcedureLambdaCall ( ExpressionForProcedureLambdaCall ),
	ProcedureNativeCall ( ExpressionForProcedureNativeCall ),
	
	Lambda ( StdRc<LambdaTemplate>, StdRc<Expression>, StdBox<[RegisterTemplate]>, StdRc<[RegisterTemplate]> ),
	
	ErrorReturn ( ExpressionBox ),
	ErrorCatch ( ExpressionBox, ExpressionValueConsumer, ExpressionBox ),
	ErrorThrow ( ExpressionBox ),
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalIfClauses {
	Void,
	Single ( StdBox<ExpressionConditionalIfClause> ),
	Multiple ( StdBox<[ExpressionConditionalIfClause]> ),
}

#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalIfClause {
	Void,
	GuardOnly ( ExpressionConditionalIfGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalIfGuard, ExpressionValueConsumer, Expression ),
}

#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalIfGuard {
	True,
	False,
	Expression ( Expression, bool ),
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalMatchClauses {
	Void,
	Single ( StdBox<ExpressionConditionalMatchClause> ),
	Multiple ( StdBox<[ExpressionConditionalMatchClause]> ),
}

#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalMatchClause {
	Void,
	GuardOnly ( ExpressionConditionalMatchGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalMatchGuard, ExpressionValueConsumer, Expression ),
}

#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionConditionalMatchGuard {
	True,
	False,
	Value ( Value, bool ),
	Values ( StdBox<[Value]>, bool ),
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionValueConsumer {
	Ignore,
	Return,
	BindingInitialize ( Binding ),
	BindingSet ( Binding ),
	RegisterInitialize ( usize ),
	RegisterSet ( usize ),
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionForContexts {
	
	ContextDefine ( Symbol, ExpressionBox ),
	ContextUpdate ( Symbol, ExpressionBox ),
	ContextSelect ( Symbol ),
	
	BindingInitialize1 ( Binding, ExpressionBox ),
	BindingInitializeN ( StdBox<[(Binding, Expression)]>, bool ),
	BindingInitializeValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingSet1 ( Binding, ExpressionBox ),
	BindingSetN ( StdBox<[(Binding, Expression)]>, bool ),
	BindingSetValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingGet1 ( Binding ),
	
	RegisterClosure ( ExpressionBox, StdBox<[RegisterTemplate]> ),
	RegisterInitialize1 ( usize, ExpressionBox ),
	RegisterInitializeN ( StdBox<[(usize, Expression)]>, bool ),
	RegisterInitializeValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterSet1 ( usize, ExpressionBox ),
	RegisterSetN ( StdBox<[(usize, Expression)]>, bool ),
	RegisterSetValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterGet1 ( usize ),
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionForProcedureGenericCall {
	
	ProcedureCall ( ExpressionBox, StdBox<[Expression]> ),
	ProcedureCall0 ( ExpressionBox ),
	ProcedureCall1 ( ExpressionBox, ExpressionBox ),
	ProcedureCall2 ( ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall3 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall4 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCall5 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureCallN ( ExpressionBox, StdBox<[Expression]> ),
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionForProcedurePrimitiveCall {
	
	ProcedurePrimitiveCall ( ProcedurePrimitive, StdBox<[Expression]> ),
	ProcedurePrimitiveCall0 ( ProcedurePrimitive0 ),
	ProcedurePrimitiveCall1 ( ProcedurePrimitive1, ExpressionBox ),
	ProcedurePrimitiveCall2 ( ProcedurePrimitive2, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall3 ( ProcedurePrimitive3, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall4 ( ProcedurePrimitive4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCall5 ( ProcedurePrimitive5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedurePrimitiveCallN ( ProcedurePrimitiveN, StdBox<[Expression]> ),
	ProcedurePrimitiveCallV ( ProcedurePrimitiveV, StdBox<[Expression]> ),
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionForProcedureExtendedCall {
	
	ProcedureExtendedCall ( ProcedureExtended, StdBox<[Expression]> ),
	ProcedureExtendedCall0 ( ProcedureExtended ),
	ProcedureExtendedCall1 ( ProcedureExtended, ExpressionBox ),
	ProcedureExtendedCall2 ( ProcedureExtended, ExpressionBox, ExpressionBox ),
	ProcedureExtendedCall3 ( ProcedureExtended, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureExtendedCall4 ( ProcedureExtended, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureExtendedCall5 ( ProcedureExtended, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureExtendedCallN ( ProcedureExtended, StdBox<[Expression]> ),
	
}




#[ derive (Clone, Debug, Hash) ]
pub enum ExpressionForProcedureLambdaCall {
	
	ProcedureLambdaCall ( StdRc<LambdaInternals>, StdBox<[Expression]> ),
	ProcedureLambdaCall0 ( StdRc<LambdaInternals> ),
	ProcedureLambdaCall1 ( StdRc<LambdaInternals>, ExpressionBox ),
	ProcedureLambdaCall2 ( StdRc<LambdaInternals>, ExpressionBox, ExpressionBox ),
	ProcedureLambdaCall3 ( StdRc<LambdaInternals>, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureLambdaCall4 ( StdRc<LambdaInternals>, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureLambdaCall5 ( StdRc<LambdaInternals>, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureLambdaCallN ( StdRc<LambdaInternals>, StdBox<[Expression]> ),
	
}




#[ derive (Clone) ]
pub enum ExpressionForProcedureNativeCall {
	
	ProcedureNativeCall ( ProcedureNative, StdBox<[Expression]> ),
	ProcedureNativeCall0 ( ProcedureNative0 ),
	ProcedureNativeCall1 ( ProcedureNative1, ExpressionBox ),
	ProcedureNativeCall2 ( ProcedureNative2, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall3 ( ProcedureNative3, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall4 ( ProcedureNative4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall5 ( ProcedureNative5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCallN ( ProcedureNativeN, StdBox<[Expression]> ),
	
}




pub type ExpressionBox = StdBox<Expression>;
pub type ExpressionVec = StdVec<Expression>;




#[ derive (Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd) ]
pub enum ExpressionSequenceOperator {
	ReturnLast,
	ReturnFirst,
	And,
	Or,
}

