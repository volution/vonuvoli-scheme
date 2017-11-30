

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ArithmeticPrimitive0;
	pub use super::ArithmeticPrimitive1;
	pub use super::ArithmeticPrimitive2;
	pub use super::ArithmeticPrimitive3;
	pub use super::ArithmeticPrimitive4;
	pub use super::ArithmeticPrimitive5;
	pub use super::ArithmeticPrimitiveN;
	
	pub use super::arithmetic_primitive_0_evaluate;
	pub use super::arithmetic_primitive_1_evaluate;
	pub use super::arithmetic_primitive_2_evaluate;
	pub use super::arithmetic_primitive_3_evaluate;
	pub use super::arithmetic_primitive_4_evaluate;
	pub use super::arithmetic_primitive_5_evaluate;
	pub use super::arithmetic_primitive_n_evaluate;
	
	pub use super::arithmetic_primitive_n_alternative_0;
	pub use super::arithmetic_primitive_n_alternative_1;
	pub use super::arithmetic_primitive_n_alternative_2;
	pub use super::arithmetic_primitive_n_alternative_3;
	pub use super::arithmetic_primitive_n_alternative_4;
	pub use super::arithmetic_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive0 {
	
	Addition,
	Multiplication,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive1 {
	
	IsComplex,
	IsReal,
	IsRational,
	IsInteger,
	IsExact,
	IsExactInteger,
	IsInexact,
	
	IsZero,
	IsPositive,
	IsNegative,
	IsFinite,
	IsInfinite,
	IsNan,
	IsEven,
	IsOdd,
	
	Negate,
	Absolute,
	Signum,
	
	Floor,
	Ceiling,
	Round,
	Truncate,
	Fractional,
	
	Square,
	SquareRoot,
	Exponential,
	Logarithm,
	
	Sin,
	Cos,
	Tan,
	Asin,
	Acos,
	Atan,
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive2 {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Remainder,
	
	DivisionFloor,
	DivisionFloorQuotient,
	DivisionFloorRemainder,
	
	DivisionTruncate,
	DivisionTruncateQuotient,
	DivisionTruncateRemainder,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
	Power,
	
}



#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitiveN {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
}




macro_rules! arithmetic_primitive_1_delegate_call {
	( $input : expr, $value_integer : ident, $for_integer : expr, $value_real : ident, $for_real : expr ) => (
		match try! (number_coerce_1 ($input)) {
			NumberCoercion1::Integer (ref $value_integer) =>
				$for_integer.into (),
			NumberCoercion1::Real (ref $value_real) =>
				$for_real.into (),
		}
	);
	( $delegate : ident, $input : expr ) => (
		arithmetic_primitive_1_delegate_call! (
				$input,
				value, NumberInteger::$delegate (value),
				value, NumberReal::$delegate (value)
			);
	);
}


macro_rules! arithmetic_primitive_2_delegate_call {
	( ($input_1 : expr, $input_2 : expr), ($value_1_integer : ident, $value_2_integer : ident), $for_integer : expr, ($value_1_real : ident, $value_2_real : ident), $for_real : expr ) => (
		match try! (number_coerce_2a ($input_1, $input_2)) {
			NumberCoercion2::Integer (ref $value_1_integer, ref $value_2_integer) =>
				$for_integer.into (),
			NumberCoercion2::Real (ref $value_1_real, ref $value_2_real) =>
				$for_real.into (),
		}
	);
	( $delegate : ident, ($input_1 : expr, $input_2 : expr) ) => (
		arithmetic_primitive_2_delegate_call! (
				($input_1, $input_2),
				(value_1, value_2), NumberInteger::$delegate (value_1, value_2),
				(value_1, value_2), NumberReal::$delegate (value_1, value_2)
			);
	);
}




pub fn arithmetic_primitive_0_evaluate (primitive : ArithmeticPrimitive0) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive0::Addition =>
			ZERO.into (),
		
		ArithmeticPrimitive0::Multiplication =>
			ONE.into (),
		
	};
	
	succeed! (output);
}




pub fn arithmetic_primitive_1_evaluate (primitive : ArithmeticPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive1::IsComplex =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, TRUE),
		
		ArithmeticPrimitive1::IsReal =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, TRUE),
		
		ArithmeticPrimitive1::IsRational =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsInteger =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsExact =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsExactInteger =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsInexact =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, FALSE,
					_value, TRUE),
		
		ArithmeticPrimitive1::IsZero =>
			arithmetic_primitive_1_delegate_call! (is_zero, input_1),
		
		ArithmeticPrimitive1::IsPositive =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, !value.is_zero () && value.is_positive (),
					value, !value.is_zero () && value.is_positive ()),
		
		ArithmeticPrimitive1::IsNegative =>
			arithmetic_primitive_1_delegate_call! (is_negative, input_1),
		
		ArithmeticPrimitive1::IsFinite =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, TRUE,
					value, value.is_finite ()),
		
		ArithmeticPrimitive1::IsInfinite =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, FALSE,
					value, value.is_infinite ()),
		
		ArithmeticPrimitive1::IsNan =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, FALSE,
					value, value.is_nan ()),
		
		ArithmeticPrimitive1::IsEven =>
			arithmetic_primitive_1_delegate_call! (is_even, input_1),
		
		ArithmeticPrimitive1::IsOdd =>
			arithmetic_primitive_1_delegate_call! (is_odd, input_1),
		
		ArithmeticPrimitive1::Negate =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (value.neg ()),
					value, value.neg ()),
		
		ArithmeticPrimitive1::Absolute =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (value.abs ()),
					value, value.abs ()),
		
		ArithmeticPrimitive1::Signum =>
			arithmetic_primitive_1_delegate_call! (signum, input_1),
		
		ArithmeticPrimitive1::Floor =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.floor ()),
		
		ArithmeticPrimitive1::Ceiling =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.ceil ()),
		
		ArithmeticPrimitive1::Round =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.round ()),
		
		ArithmeticPrimitive1::Truncate =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.trunc ()),
		
		ArithmeticPrimitive1::Fractional =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, ZERO,
					value, value.fract ()),
		
		ArithmeticPrimitive1::Square =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.power (&2.into ()),
					value, value.power (&2.into ())),
		
		ArithmeticPrimitive1::SquareRoot =>
			arithmetic_primitive_1_delegate_call! (sqrt, input_1),
		
		ArithmeticPrimitive1::Exponential =>
			arithmetic_primitive_1_delegate_call! (exp, input_1),
		
		ArithmeticPrimitive1::Logarithm =>
			arithmetic_primitive_1_delegate_call! (ln, input_1),
		
		ArithmeticPrimitive1::Sin =>
			arithmetic_primitive_1_delegate_call! (sin, input_1),
		
		ArithmeticPrimitive1::Cos =>
			arithmetic_primitive_1_delegate_call! (cos, input_1),
		
		ArithmeticPrimitive1::Tan =>
			arithmetic_primitive_1_delegate_call! (tan, input_1),
		
		ArithmeticPrimitive1::Asin =>
			arithmetic_primitive_1_delegate_call! (asin, input_1),
		
		ArithmeticPrimitive1::Acos =>
			arithmetic_primitive_1_delegate_call! (acos, input_1),
		
		ArithmeticPrimitive1::Atan =>
			arithmetic_primitive_1_delegate_call! (atan, input_1),
		
		ArithmeticPrimitive1::Addition =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Subtraction =>
			arithmetic_primitive_2_delegate_call! (
					(&ZERO.into (), &input_1),
					(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
					(value_1, value_2), NumberReal::sub (value_1, value_2)),
		
		ArithmeticPrimitive1::Multiplication =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Division =>
			arithmetic_primitive_2_delegate_call! (
					(&ONE.into (), &input_1),
					(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
					(value_1, value_2), NumberReal::div (value_1, value_2)),
		
		ArithmeticPrimitive1::GreatestCommonDivisor =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
		ArithmeticPrimitive1::LeastCommonMultiple =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Minimum =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Maximum =>
			try! (number_coerce_1 (input_1)) .into_value (),
		
	};
	
	succeed! (output);
}




pub fn arithmetic_primitive_2_evaluate (primitive : ArithmeticPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive2::Addition =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::add (value_1, value_2)),
					(value_1, value_2), NumberReal::add (value_1, value_2)),
		
		ArithmeticPrimitive2::Subtraction =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
					(value_1, value_2), NumberReal::sub (value_1, value_2)),
		
		ArithmeticPrimitive2::Multiplication =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::mul (value_1, value_2)),
					(value_1, value_2), NumberReal::mul (value_1, value_2)),
		
		ArithmeticPrimitive2::Division =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
					(value_1, value_2), NumberReal::div (value_1, value_2)),
		
		ArithmeticPrimitive2::Remainder =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::rem (value_1, value_2)),
					(value_1, value_2), NumberReal::rem (value_1, value_2)),
		
		ArithmeticPrimitive2::DivisionFloor =>
			fail_unimplemented! (0x738acdd6),
		
		ArithmeticPrimitive2::DivisionFloorQuotient =>
			fail_unimplemented! (0x2f425d22),
		
		ArithmeticPrimitive2::DivisionFloorRemainder =>
			fail_unimplemented! (0x8b709e6a),
		
		ArithmeticPrimitive2::DivisionTruncate =>
			fail_unimplemented! (0xbbf7f471),
			
		ArithmeticPrimitive2::DivisionTruncateQuotient =>
			fail_unimplemented! (0xd6bb8165),
		
		ArithmeticPrimitive2::DivisionTruncateRemainder =>
			fail_unimplemented! (0xfba74cd9),
		
		ArithmeticPrimitive2::Power =>
			arithmetic_primitive_2_delegate_call! ((input_1, input_2),
					(value_1, value_2), NumberReal::power (&<NumberReal>::from (*value_1), &<NumberReal>::from (*value_2)),
					(value_1, value_2), NumberReal::power (value_1, value_2)),
		
		ArithmeticPrimitive2::GreatestCommonDivisor =>
			fail_unimplemented! (0x79f53d20),
		
		ArithmeticPrimitive2::LeastCommonMultiple =>
			fail_unimplemented! (0x79f53d20),
		
		ArithmeticPrimitive2::Minimum =>
			arithmetic_primitive_2_delegate_call! (min, (input_1, input_2)),
		
		ArithmeticPrimitive2::Maximum =>
			arithmetic_primitive_2_delegate_call! (max, (input_1, input_2)),
		
	};
	
	succeed! (output);
}




pub fn arithmetic_primitive_3_evaluate (primitive : ArithmeticPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn arithmetic_primitive_4_evaluate (primitive : ArithmeticPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn arithmetic_primitive_5_evaluate (primitive : ArithmeticPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn arithmetic_primitive_n_evaluate (primitive : ArithmeticPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	match primitive {
		
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			fail_unimplemented! (0xeefd593c),
		
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			fail_unimplemented! (0x4bc0a9ad),
		
		_ =>
			(),
		
	}
	
	let inputs_count = inputs.len ();
	
	if inputs_count == 0 {
		match primitive {
			
			ArithmeticPrimitiveN::Addition =>
				succeed! (ZERO.into ()),
			
			ArithmeticPrimitiveN::Multiplication =>
				succeed! (ONE.into ()),
			
			_ =>
				fail! (0x69d3b6cc),
		}
	}
	
	let mut output : Value = try! (number_coerce_1 (&inputs[0])) .into_value ();
	
	if inputs_count == 1 {
		output = match primitive {
			
			ArithmeticPrimitiveN::Subtraction =>
				arithmetic_primitive_2_delegate_call! (
						(&ZERO.into (), &output),
						(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
						(value_1, value_2), NumberReal::sub (value_1, value_2)),
			
			ArithmeticPrimitiveN::Division =>
				arithmetic_primitive_2_delegate_call! (
						(&ONE.into (), &output),
						(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
						(value_1, value_2), NumberReal::div (value_1, value_2)),
			
			_ =>
				output,
			
		};
		succeed! (output);
	}
	
	for input in &inputs[1..] {
		output = match primitive {
			
			ArithmeticPrimitiveN::Addition =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::add (value_1, value_2)),
						(value_1, value_2), NumberReal::add (value_1, value_2)),
			
			ArithmeticPrimitiveN::Subtraction =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
						(value_1, value_2), NumberReal::sub (value_1, value_2)),
			
			ArithmeticPrimitiveN::Multiplication =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::mul (value_1, value_2)),
						(value_1, value_2), NumberReal::mul (value_1, value_2)),
			
			ArithmeticPrimitiveN::Division =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
						(value_1, value_2), NumberReal::div (value_1, value_2)),
			
			ArithmeticPrimitiveN::GreatestCommonDivisor =>
				fail_panic! (0x38fce646),
			
			ArithmeticPrimitiveN::LeastCommonMultiple =>
				fail_panic! (0x5c07f7c2),
			
			ArithmeticPrimitiveN::Minimum =>
				arithmetic_primitive_2_delegate_call! (min, (&output, input)),
			
			ArithmeticPrimitiveN::Maximum =>
				arithmetic_primitive_2_delegate_call! (max, (&output, input)),
			
		};
	}
	
	succeed! (output);
}




pub fn arithmetic_primitive_n_alternative_0 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive0>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			Some (ArithmeticPrimitive0::Addition),
		ArithmeticPrimitiveN::Subtraction =>
			None,
		ArithmeticPrimitiveN::Multiplication =>
			Some (ArithmeticPrimitive0::Multiplication),
		ArithmeticPrimitiveN::Division =>
			None,
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveN::Minimum =>
			None,
		ArithmeticPrimitiveN::Maximum =>
			None,
	}
}


pub fn arithmetic_primitive_n_alternative_1 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive1>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			Some (ArithmeticPrimitive1::Addition),
		ArithmeticPrimitiveN::Subtraction =>
			Some (ArithmeticPrimitive1::Subtraction),
		ArithmeticPrimitiveN::Multiplication =>
			Some (ArithmeticPrimitive1::Multiplication),
		ArithmeticPrimitiveN::Division =>
			Some (ArithmeticPrimitive1::Division),
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			Some (ArithmeticPrimitive1::GreatestCommonDivisor),
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			Some (ArithmeticPrimitive1::LeastCommonMultiple),
		ArithmeticPrimitiveN::Minimum =>
			Some (ArithmeticPrimitive1::Minimum),
		ArithmeticPrimitiveN::Maximum =>
			Some (ArithmeticPrimitive1::Maximum),
	}
}


pub fn arithmetic_primitive_n_alternative_2 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive2>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			Some (ArithmeticPrimitive2::Addition),
		ArithmeticPrimitiveN::Subtraction =>
			Some (ArithmeticPrimitive2::Subtraction),
		ArithmeticPrimitiveN::Multiplication =>
			Some (ArithmeticPrimitive2::Multiplication),
		ArithmeticPrimitiveN::Division =>
			Some (ArithmeticPrimitive2::Division),
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			Some (ArithmeticPrimitive2::GreatestCommonDivisor),
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			Some (ArithmeticPrimitive2::LeastCommonMultiple),
		ArithmeticPrimitiveN::Minimum =>
			Some (ArithmeticPrimitive2::Minimum),
		ArithmeticPrimitiveN::Maximum =>
			Some (ArithmeticPrimitive2::Maximum),
	}
}


pub fn arithmetic_primitive_n_alternative_3 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive3>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			None,
		ArithmeticPrimitiveN::Subtraction =>
			None,
		ArithmeticPrimitiveN::Multiplication =>
			None,
		ArithmeticPrimitiveN::Division =>
			None,
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveN::Minimum =>
			None,
		ArithmeticPrimitiveN::Maximum =>
			None,
	}
}


pub fn arithmetic_primitive_n_alternative_4 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive4>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			None,
		ArithmeticPrimitiveN::Subtraction =>
			None,
		ArithmeticPrimitiveN::Multiplication =>
			None,
		ArithmeticPrimitiveN::Division =>
			None,
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveN::Minimum =>
			None,
		ArithmeticPrimitiveN::Maximum =>
			None,
	}
}


pub fn arithmetic_primitive_n_alternative_5 (primitive : ArithmeticPrimitiveN) -> (Option<ArithmeticPrimitive5>) {
	match primitive {
		ArithmeticPrimitiveN::Addition =>
			None,
		ArithmeticPrimitiveN::Subtraction =>
			None,
		ArithmeticPrimitiveN::Multiplication =>
			None,
		ArithmeticPrimitiveN::Division =>
			None,
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveN::Minimum =>
			None,
		ArithmeticPrimitiveN::Maximum =>
			None,
	}
}

