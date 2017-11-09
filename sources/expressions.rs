

use super::errors::exports::*;
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


impl StdFrom<Value> for Expression { fn from (value : Value) -> (Self) { Expression::Value (value) } }

impl StdFrom<Boolean> for Expression { fn from (value : Boolean) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<NumberInteger> for Expression { fn from (value : NumberInteger) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<NumberReal> for Expression { fn from (value : NumberReal) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Character> for Expression { fn from (value : Character) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Symbol> for Expression { fn from (value : Symbol) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<String> for Expression { fn from (value : String) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Bytes> for Expression { fn from (value : Bytes) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Pair> for Expression { fn from (value : Pair) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Array> for Expression { fn from (value : Array) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<Error> for Expression { fn from (value : Error) -> (Self) { Expression::Value (value.into ()) } }

impl StdFrom<bool> for Expression { fn from (value : bool) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<i64> for Expression { fn from (value : i64) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<f64> for Expression { fn from (value : f64) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<char> for Expression { fn from (value : char) -> (Self) { Expression::Value (value.into ()) } }

impl StdFrom<StdString> for Expression { fn from (value : StdString) -> (Self) { Expression::Value (value.into ()) } }
impl StdFrom<&'static str> for Expression { fn from (value : &'static str) -> (Self) { Expression::Value (value.into ()) } }


impl StdFrom<Value> for ExpressionBox { fn from (value : Value) -> (Self) { StdBox::new (value.into ()) } }

impl StdFrom<Boolean> for ExpressionBox { fn from (value : Boolean) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<NumberInteger> for ExpressionBox { fn from (value : NumberInteger) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<NumberReal> for ExpressionBox { fn from (value : NumberReal) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Character> for ExpressionBox { fn from (value : Character) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Symbol> for ExpressionBox { fn from (value : Symbol) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<String> for ExpressionBox { fn from (value : String) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Bytes> for ExpressionBox { fn from (value : Bytes) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Pair> for ExpressionBox { fn from (value : Pair) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Array> for ExpressionBox { fn from (value : Array) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<Error> for ExpressionBox { fn from (value : Error) -> (Self) { StdBox::new (value.into ()) } }

impl StdFrom<bool> for ExpressionBox { fn from (value : bool) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<i64> for ExpressionBox { fn from (value : i64) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<f64> for ExpressionBox { fn from (value : f64) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<char> for ExpressionBox { fn from (value : char) -> (Self) { StdBox::new (value.into ()) } }

impl StdFrom<StdString> for ExpressionBox { fn from (value : StdString) -> (Self) { StdBox::new (value.into ()) } }
impl StdFrom<&'static str> for ExpressionBox { fn from (value : &'static str) -> (Self) { StdBox::new (value.into ()) } }

