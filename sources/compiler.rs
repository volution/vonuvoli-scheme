

use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::compile;
}




#[ inline (always) ]
pub fn compile (context : &Context, value : Value) -> (Outcome<Expression>) {
	
	match value.class () {
		
		ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
			Ok (value.into ()),
		ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
			Ok (value.into ()),
		ValueClass::String | ValueClass::Bytes =>
			Ok (value.into ()),
		
		ValueClass::Symbol =>
			compile_symbol (context, value.into ()),
		ValueClass::Pair =>
			compile_form (context, value.into ()),
		ValueClass::Array =>
			failed_unimplemented! (0xe7db25d8),
		
		ValueClass::Error =>
			failed! (0x2aa7bc60),
		ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
			failed! (0xaf6f1288),
		
		ValueClass::Binding =>
			Ok (Expression::BindingGet (value.into ())),
		
		ValueClass::Context =>
			failed! (0x5f0d7003),
		
		ValueClass::Number | ValueClass::List | ValueClass::Procedure | ValueClass::Syntax =>
			failed! (0x841d4d00),
		
	}
}




#[ inline (always) ]
fn compile_symbol (context : &Context, identifier : Symbol) -> (Outcome<Expression>) {
	
	if let Ok (binding) = context.resolve (&identifier) {
		Ok (Expression::BindingGet (binding))
	} else {
		Ok (Expression::ContextSelect (identifier))
	}
}




#[ inline (always) ]
fn compile_form (context : &Context, list : Pair) -> (Outcome<Expression>) {
	
	let callable = list.left () .clone ();
	let arguments = try! (vec_from_list (list.right ()));
	
	return compile_form_0 (context, callable, arguments);
}


#[ inline (always) ]
fn compile_form_0 (context : &Context, callable : Value, arguments : ValueVec) -> (Outcome<Expression>) {
	
	match callable.class () {
		
		ValueClass::Symbol =>
			if let Ok (binding) = context.resolve (callable.clone () .as_ref () as &Symbol) {
				let callable = try! (binding.get ());
				match callable.class () {
					ValueClass::SyntaxPrimitive =>
						return compile_form_0 (context, callable, arguments),
					_ =>
						return compile_procedure_call (context, binding.into (), arguments),
				}
			} else {
				return compile_procedure_call (context, callable, arguments)
			},
		
		ValueClass::SyntaxPrimitive =>
			return compile_syntax_call (context, SyntaxPrimitive::from (callable), arguments),
		
		_ =>
			return compile_procedure_call (context, callable, arguments),
	}
}


#[ inline (always) ]
fn compile_syntax_call (_context : &Context, _syntax : SyntaxPrimitive, _arguments : ValueVec) -> (Outcome<Expression>) {
	return failed_unimplemented! (0x23f98ca2);
}


#[ inline (always) ]
fn compile_procedure_call (context : &Context, procedure : Value, arguments : ValueVec) -> (Outcome<Expression>) {
	
	let procedure = try! (compile (context, procedure));
	let arguments : ExpressionVec = try! (arguments.into_iter () .map (|value| compile (context, value)) .collect ());
	
	return Ok (Expression::ProcedureCall (procedure.into (), arguments.into ()));
}

