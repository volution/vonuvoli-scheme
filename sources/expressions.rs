

use super::contexts::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
use super::primitives::exports::*;

#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
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
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::ExpressionConditionalMatchClauses;
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::ExpressionConditionalMatchClause;
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::ExpressionConditionalMatchGuard;
	pub use super::ExpressionValueConsumer;
	
	pub use super::ExpressionForContexts;
	
	pub use super::ExpressionForProcedureGenericCall;
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	pub use super::ExpressionForProcedurePrimitiveCall;
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::ExpressionForProcedureExtendedCall;
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::ExpressionForProcedureLambdaCall;
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::ExpressionForProcedureNativeCall;
	
	pub use super::ExpressionSequenceOperator;
	
}




#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum Expression {
	
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Void,
	Value ( Value ),
	
	Sequence ( ExpressionSequenceOperator, StdBox<[Expression]> ),
	ConditionalIf ( ExpressionConditionalIfClauses ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ConditionalMatch ( ExpressionBox, ExpressionConditionalMatchClauses ),
	Loop ( Option<ExpressionBox>, Option<ExpressionBox>, Option<ExpressionBox>, Option<ExpressionConditionalIfClauses> ),
	
	Contexts ( ExpressionForContexts ),
	
	ProcedureGenericCall ( ExpressionForProcedureGenericCall ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedurePrimitiveCall ( ExpressionForProcedurePrimitiveCall ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtendedCall ( ExpressionForProcedureExtendedCall ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambdaCall ( ExpressionForProcedureLambdaCall ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
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
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalIfClauses {
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Void,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	TrueReturn,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ExpressionOnly ( ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Single ( StdBox<ExpressionConditionalIfClause> ),
	Multiple ( StdBox<[ExpressionConditionalIfClause]> ),
}

#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalIfClause {
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Void,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	TrueReturn,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ExpressionOnly ( Expression ),
	GuardOnly ( ExpressionConditionalIfGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalIfGuard, ExpressionValueConsumer, Expression ),
}

#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalIfGuard {
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	True,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	False,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Value ( Value, bool ),
	Expression ( Expression, bool ),
}




#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalMatchClauses {
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Void,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	TrueReturn,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ExpressionOnly ( ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Single ( StdBox<ExpressionConditionalMatchClause> ),
	Multiple ( StdBox<[ExpressionConditionalMatchClause]> ),
}

#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalMatchClause {
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Void,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	TrueReturn,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ExpressionOnly ( Expression ),
	GuardOnly ( ExpressionConditionalMatchGuard, ExpressionValueConsumer ),
	GuardAndExpression ( ExpressionConditionalMatchGuard, ExpressionValueConsumer, Expression ),
}

#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionConditionalMatchGuard {
	True,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	False,
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	Value ( Value, bool ),
	Values ( StdBox<[Value]>, bool ),
}




#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionValueConsumer {
	Ignore,
	Return,
	BindingInitialize ( Binding ),
	BindingSet ( Binding ),
	RegisterInitialize ( usize ),
	RegisterSet ( usize ),
}




#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
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
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionForProcedureGenericCall {
	
	ProcedureCall ( ExpressionBox, StdBox<[Expression]> ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall0 ( ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall1 ( ExpressionBox, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall2 ( ExpressionBox, ExpressionBox, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall3 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall4 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCall5 ( ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox, ExpressionBox ),
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	ProcedureCallN ( ExpressionBox, StdBox<[Expression]> ),
	
}




#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
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




#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
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
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ExpressionSequenceOperator {
	ReturnLast,
	ReturnFirst,
	And,
	Or,
}

