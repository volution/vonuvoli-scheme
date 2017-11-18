

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::operators::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::compile;
	pub use super::compile_vec;
	pub use super::compile_slice;
}




#[ inline (always) ]
pub fn compile (context : &Context, value : &Value) -> (Outcome<Expression>) {
	
	match value.class () {
		
		ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
			succeed! (value.clone ()),
		ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
			succeed! (value.clone ()),
		ValueClass::String | ValueClass::Bytes =>
			succeed! (value.clone ()),
		
		ValueClass::Symbol =>
			return compile_symbol (context, value.as_ref ()),
		ValueClass::Pair =>
			return compile_form (context, value.as_ref ()),
		ValueClass::Array =>
			fail_unimplemented! (0xe7db25d8),
		
		ValueClass::Error =>
			fail! (0x2aa7bc60),
		ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
			fail! (0xaf6f1288),
		
		ValueClass::Binding | ValueClass::Context =>
			fail! (0x5f0d7003),
		
		ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
			fail! (0x841d4d00),
		
	}
}




#[ inline (always) ]
pub fn compile_vec (context : &Context, values : ValueVec) -> (Outcome<ExpressionVec>) {
	values.into_iter () .map (|ref value| compile (context, value)) .collect ()
}

#[ inline (always) ]
pub fn compile_slice (context : &Context, values : &[Value]) -> (Outcome<ExpressionVec>) {
	values.iter () .map (|ref value| compile (context, value)) .collect ()
}




#[ inline (always) ]
pub fn compile_symbol (context : &Context, identifier : &Symbol) -> (Outcome<Expression>) {
	
	if let Ok (Some (binding)) = context.resolve (identifier) {
		succeed! (Expression::BindingGet (binding));
	} else {
		succeed! (Expression::ContextSelect (identifier.clone ()));
	}
}




#[ inline (always) ]
pub fn compile_form (context : &Context, form : &Pair) -> (Outcome<Expression>) {
	
	match try! (compile_form_1 (context, &form)) {
		
		Some ((ref primitive, ref arguments)) =>
			return compile_syntax_call (context, primitive, arguments),
		None =>
			return compile_procedure_call (context, form.left (), form.right ()),
	}
}


#[ inline (always) ]
fn compile_form_1 (context : &Context, value : &Pair) -> (Outcome<Option<(SyntaxPrimitive, Value)>>) {
	
	let callable = value.left ();
	let arguments = value.right ();
	
	match callable.class () {
		
		ValueClass::Symbol =>
			if let Ok (Some (binding)) = context.resolve (callable.clone () .as_ref () as &Symbol) {
				let callable = try! (binding.get ());
				match callable.class () {
					ValueClass::SyntaxPrimitive =>
						succeed! (Some ((callable.into (), arguments.clone ()))),
					_ =>
						succeed! (None),
				}
			} else {
				succeed! (None);
			},
		
		ValueClass::SyntaxPrimitive =>
			succeed! (Some ((callable.clone () .into (), arguments.clone ()))),
		
		_ =>
			succeed! (None),
		
	}
}




#[ inline (always) ]
pub fn compile_procedure_call (context : &Context, procedure : &Value, arguments : &Value) -> (Outcome<Expression>) {
	
	let procedure = try! (compile (context, procedure));
	
	let arguments = try! (vec_clone_list (arguments));
	let arguments = try! (compile_vec (context, arguments));
	
	succeed! (Expression::ProcedureCall (procedure.into (), arguments));
}




#[ inline (always) ]
pub fn compile_syntax_call (context : &Context, syntax : &SyntaxPrimitive, arguments : &Value) -> (Outcome<Expression>) {
	
	let arguments = try! (vec_clone_list (arguments));
	let arguments_count = arguments.len ();
	
	match *syntax {
		
		SyntaxPrimitive::Primitive1 (syntax) =>
			if arguments_count == 1 {
				let arguments = &arguments[0];
				match syntax {
					
					SyntaxPrimitive1::Quote =>
						succeed! (Expression::Value (arguments.clone ())),
					
					SyntaxPrimitive1::QuasiQuote =>
						return compile_syntax_quasy_quote_value (context, arguments, false),
					
					SyntaxPrimitive1::UnQuote | SyntaxPrimitive1::UnQuoteSplicing =>
						fail! (0x99b4857b),
					
				}
			} else {
				fail! (0x421da1f1);
			},
		
		SyntaxPrimitive::Primitive2 (syntax) =>
			if arguments_count == 2 {
				match syntax {
					
					SyntaxPrimitive2::Define => {
						let identifier = &arguments[0];
						let value = &arguments[1];
						match identifier.class () {
							ValueClass::Symbol => {
								let identifier = try_as_symbol_ref! (identifier);
								let binding = try! (context.define (identifier));
								let value = try! (compile (context, value));
								succeed! (Expression::BindingInitialize (binding, value.into ()));
							},
							ValueClass::Pair =>
								fail_unimplemented! (0xfc72467c),
							_ =>
								fail! (0x404d24c7),
						}
					},
					
					SyntaxPrimitive2::DefineValues =>
						fail_unimplemented! (0x2f87acf0),
					
					SyntaxPrimitive2::Set =>
						fail_unimplemented! (0x19d7ebad),
					
					SyntaxPrimitive2::SetValues =>
						fail_unimplemented! (0xeea43320),
					
				}
			} else {
				fail! (0x9d9b6a94);
			},
		
		SyntaxPrimitive::Primitive3 (syntax) =>
			if arguments_count == 3 {
				match syntax {
					
					SyntaxPrimitive3::If => {
						let arguments = try! (compile_vec (context, arguments));
						succeed! (Expression::SyntaxPrimitiveCall (SyntaxPrimitive3::If.into (), arguments));
					},
					
				}
			} else {
				fail! (0xd76f0ad2);
			},
		
		SyntaxPrimitive::PrimitiveN (syntax) =>
			match syntax {
				
				SyntaxPrimitiveN::Begin => {
					let arguments = try! (compile_vec (context, arguments));
					succeed! (Expression::SyntaxPrimitiveCall (SyntaxPrimitiveN::Begin.into (), arguments));
				},
				
				SyntaxPrimitiveN::And | SyntaxPrimitiveN::Or => {
					let arguments = try! (compile_vec (context, arguments));
					succeed! (Expression::SyntaxPrimitiveCall (syntax.into (), arguments));
				},
				
				SyntaxPrimitiveN::When | SyntaxPrimitiveN::Unless =>
					if arguments_count >= 2 {
						let arguments = try! (compile_vec (context, arguments));
						succeed! (Expression::SyntaxPrimitiveCall (syntax.into (), arguments));
					} else {
						fail! (0x3c364a9f);
					},
				
				SyntaxPrimitiveN::Local => {
					let context = Context::new (Some (context));
					let arguments = try! (compile_vec (&context, arguments));
					succeed! (Expression::SyntaxPrimitiveCall (SyntaxPrimitiveN::Begin.into (), arguments));
				},
				
				_ =>
					fail_unimplemented! (0x73d95eb5),
				
			},
		
		SyntaxPrimitive::Auxiliary =>
			fail! (0x1aed14f3),
		
		SyntaxPrimitive::Reserved =>
			fail! (0x1aed14f3),
		
		SyntaxPrimitive::Unimplemented =>
			fail_unimplemented! (0xa4e41f62),
		
	}
}


#[ inline (always) ]
pub fn compile_syntax_quasy_quote_value (context : &Context, value : &Value, spliceable : bool) -> (Outcome<Expression>) {
	
	#[ inline (always) ]
	fn splice <ExpressionInto : StdInto<Expression>> (expression : ExpressionInto, spliceable : bool) -> (Expression) {
		let expression = expression.into ();
		if spliceable {
			Expression::ProcedureCall (ListPrimitive1::List.into (), vec! [expression])
		} else {
			expression
		}
	}
	
	match value.class () {
		
		ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
			succeed! (splice (value.clone (), spliceable)),
		ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
			succeed! (splice (value.clone (), spliceable)),
		ValueClass::String | ValueClass::Bytes =>
			succeed! (splice (value.clone (), spliceable)),
		
		ValueClass::Symbol =>
			succeed! (splice (value.clone (), spliceable)),
		ValueClass::Array =>
			fail_unimplemented! (0x0d99c57b),
		
		ValueClass::Error =>
			fail! (0x9681733a),
		ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
			fail! (0x251a7fd0),
		
		ValueClass::Binding | ValueClass::Context =>
			fail! (0xfa7ef6f6),
		
		ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
			fail! (0x841d4d00),
		
		ValueClass::Pair => {
			match try! (compile_form_1 (context, value.as_ref ())) {
				
				Some ((ref primitive, ref arguments)) => {
					let arguments = try! (vec_clone_list (arguments));
					let arguments_count = arguments.len ();
					match *primitive {
						
						SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuote) =>
							if arguments_count == 1 {
								let element = try! (compile_syntax_quasy_quote_value (context, &arguments[0], false));
								succeed! (splice (element, spliceable));
							} else {
								fail! (0x9dc44267);
							},
						
						SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuoteSplicing) =>
							if arguments_count == 1 {
								if spliceable {
									let element = try! (compile_syntax_quasy_quote_value (context, &arguments[0], false));
									succeed! (element);
								} else {
									fail! (0x47356961);
								}
							} else {
								fail! (0xe0c45124);
							},
						
						_ =>
							{},
					}
				},
				
				None =>
					{},
				
			}
			
			let mut elements = ExpressionVec::new ();
			let mut cursor = value;
			loop {
				match cursor.class () {
					
					ValueClass::Pair => {
						let pair = cursor.as_ref () as &Pair;
						let element = try! (compile_syntax_quasy_quote_value (context, pair.left (), true));
						elements.push (element);
						cursor = pair.right ();
					},
					
					ValueClass::Null =>
						break,
					
					_ => {
						let element = try! (compile_syntax_quasy_quote_value (context, cursor, true));
						elements.push (element);
						break;
					},
					
				}
			}
			
			succeed! (Expression::ProcedureCall (ListPrimitiveN::Append.into (), elements));
		},
		
	}
}

