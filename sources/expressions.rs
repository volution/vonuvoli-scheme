

use super::contexts::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_values_native" ) ]
use super::native_procedures::exports::*;

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
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::ExpressionForProcedureExtendedCall;
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::ExpressionForProcedureLambdaCall;
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::ExpressionForProcedureNativeCall;
	
	pub use super::ExpressionSequenceOperator;
	
}




#[ derive ( Clone ) ] // OK ~~
pub enum Expression {
	
	Void,
	Value ( Value ),
	
	Sequence ( ExpressionSequenceOperator, StdBox<[Expression]> ),
	ConditionalIf ( ExpressionConditionalIfClauses ),
	ConditionalMatch ( ExpressionBox, ExpressionConditionalMatchClauses ),
	Loop ( Option<ExpressionBox>, Option<ExpressionBox>, Option<ExpressionBox>, Option<ExpressionConditionalIfClauses> ),
	
	Contexts ( ExpressionForContexts ),
	
	ProcedureGenericCall ( ExpressionForProcedureGenericCall ),
	ProcedurePrimitiveCall ( ExpressionForProcedurePrimitiveCall ),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtendedCall ( ExpressionForProcedureExtendedCall ),
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambdaCall ( ExpressionForProcedureLambdaCall ),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNativeCall ( ExpressionForProcedureNativeCall ),
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda ( StdRc<LambdaTemplate>, StdRc<Expression>, StdBox<[RegisterTemplate]>, StdRc<[RegisterTemplate]> ),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorReturn ( ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorCatch ( ExpressionBox, ExpressionValueConsumer, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorThrow ( ExpressionBox ),
	
}




#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalIfClauses {
	Void,
	TrueReturn,
	ExpressionOnly ( ExpressionBox ),
	Single ( StdBox<ExpressionConditionalIfClause> ),
	Multiple ( StdBox<[ExpressionConditionalIfClause]> ),
}

#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalIfClause {
	Void,
	TrueReturn,
	ExpressionOnly ( Expression ),
	GuardOnly ( ExpressionConditionalIfGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalIfGuard, ExpressionValueConsumer, Expression ),
}

#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalIfGuard {
	True,
	False,
	Value ( Value, bool ),
	Expression ( Expression, bool ),
}




#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalMatchClauses {
	Void,
	TrueReturn,
	ExpressionOnly ( ExpressionBox ),
	Single ( StdBox<ExpressionConditionalMatchClause> ),
	Multiple ( StdBox<[ExpressionConditionalMatchClause]> ),
}

#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalMatchClause {
	Void,
	TrueReturn,
	ExpressionOnly ( Expression ),
	GuardOnly ( ExpressionConditionalMatchGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalMatchGuard, ExpressionValueConsumer, Expression ),
}

#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionConditionalMatchGuard {
	True,
	False,
	Value ( Value, bool ),
	Values ( StdBox<[Value]>, bool ),
}




#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionValueConsumer {
	Ignore,
	Return,
	BindingInitialize ( Binding ),
	BindingSet ( Binding ),
	RegisterInitialize ( usize ),
	RegisterSet ( usize ),
}




#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionForContexts {
	
	ContextDefine ( Symbol, ExpressionBox ),
	ContextUpdate ( Symbol, ExpressionBox ),
	ContextSelect ( Symbol ),
	
	BindingInitialize1 ( Binding, ExpressionBox ),
	BindingInitializeN ( StdBox<[(Binding, Expression)]>, bool ),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	BindingInitializeValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingSet1 ( Binding, ExpressionBox ),
	BindingSetN ( StdBox<[(Binding, Expression)]>, bool ),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	BindingSetValues ( StdBox<[Binding]>, ExpressionBox ),
	BindingGet1 ( Binding ),
	
	RegisterClosure ( ExpressionBox, StdBox<[RegisterTemplate]> ),
	RegisterInitialize1 ( usize, ExpressionBox ),
	RegisterInitializeN ( StdBox<[(usize, Expression)]>, bool ),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RegisterInitializeValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterSet1 ( usize, ExpressionBox ),
	RegisterSetN ( StdBox<[(usize, Expression)]>, bool ),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	RegisterSetValues ( StdBox<[usize]>, ExpressionBox ),
	RegisterGet1 ( usize ),
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterClosure ( ExpressionBox ),
	
}




#[ derive ( Clone ) ] // OK ~~
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




#[ derive ( Clone ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ derive ( Clone ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ derive ( Clone ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ derive ( Clone ) ] // OK ~~
pub enum ExpressionForProcedureNativeCall {
	
	ProcedureNativeCall ( ProcedureNative, StdBox<[Expression]> ),
	
	ProcedureNativeCall0 ( ProcedureNative0 ),
	ProcedureNativeCall1 ( ProcedureNative1, ExpressionBox ),
	ProcedureNativeCall2 ( ProcedureNative2, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall3 ( ProcedureNative3, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall4 ( ProcedureNative4, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall5 ( ProcedureNative5, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCallN ( ProcedureNativeN, StdBox<[Expression]> ),
	
	ProcedureNativeCall0E ( ProcedureNative0E ),
	ProcedureNativeCall1E ( ProcedureNative1E, ExpressionBox ),
	ProcedureNativeCall2E ( ProcedureNative2E, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall3E ( ProcedureNative3E, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall4E ( ProcedureNative4E, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCall5E ( ProcedureNative5E, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	ProcedureNativeCallNE ( ProcedureNativeNE, StdBox<[Expression]> ),
	
}




pub type ExpressionBox = StdBox<Expression>;
pub type ExpressionVec = StdVec<Expression>;




#[ derive ( Copy, Clone ) ] // OK
pub enum ExpressionSequenceOperator {
	ReturnLast,
	ReturnFirst,
	And,
	Or,
}

