

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::ArithmeticPrimitive1;
	pub use super::ArithmeticPrimitive2;
	pub use super::ArithmeticPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
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
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArithmeticPrimitive2 {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Remainder,
	
	Minimum,
	Maximum,
	
	Power,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArithmeticPrimitiveN {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
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




pub fn arithmetic_primitive_1_evaluate (primitive : ArithmeticPrimitive1, input : &Value) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive1::IsComplex =>
			TRUE.into (),
		
		ArithmeticPrimitive1::IsReal =>
			TRUE.into (),
		
		ArithmeticPrimitive1::IsRational =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsInteger =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsExact =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsExactInteger =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, TRUE,
					_value, FALSE),
		
		ArithmeticPrimitive1::IsInexact =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, FALSE,
					_value, TRUE),
		
		ArithmeticPrimitive1::IsZero =>
			arithmetic_primitive_1_delegate_call! (is_zero, input),
		
		ArithmeticPrimitive1::IsPositive =>
			arithmetic_primitive_1_delegate_call! (input,
					value, !value.is_zero () && value.is_positive (),
					value, !value.is_zero () && value.is_positive ()),
		
		ArithmeticPrimitive1::IsNegative =>
			arithmetic_primitive_1_delegate_call! (is_negative, input),
		
		ArithmeticPrimitive1::IsFinite =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, TRUE,
					value, value.is_finite ()),
		
		ArithmeticPrimitive1::IsInfinite =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, FALSE,
					value, value.is_infinite ()),
		
		ArithmeticPrimitive1::IsNan =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, FALSE,
					value, value.is_nan ()),
		
		ArithmeticPrimitive1::IsEven =>
			arithmetic_primitive_1_delegate_call! (is_even, input),
		
		ArithmeticPrimitive1::IsOdd =>
			arithmetic_primitive_1_delegate_call! (is_odd, input),
		
		ArithmeticPrimitive1::Negate =>
			arithmetic_primitive_1_delegate_call! (input,
					value, try! (value.neg ()),
					value, value.neg ()),
		
		ArithmeticPrimitive1::Absolute =>
			arithmetic_primitive_1_delegate_call! (input,
					value, try! (value.abs ()),
					value, value.abs ()),
		
		ArithmeticPrimitive1::Signum =>
			arithmetic_primitive_1_delegate_call! (signum, input),
		
		ArithmeticPrimitive1::Floor =>
			arithmetic_primitive_1_delegate_call! (input,
					value, value.clone (),
					value, value.floor ()),
		
		ArithmeticPrimitive1::Ceiling =>
			arithmetic_primitive_1_delegate_call! (input,
					value, value.clone (),
					value, value.ceil ()),
		
		ArithmeticPrimitive1::Round =>
			arithmetic_primitive_1_delegate_call! (input,
					value, value.clone (),
					value, value.round ()),
		
		ArithmeticPrimitive1::Truncate =>
			arithmetic_primitive_1_delegate_call! (input,
					value, value.clone (),
					value, value.trunc ()),
		
		ArithmeticPrimitive1::Fractional =>
			arithmetic_primitive_1_delegate_call! (input,
					_value, ZERO,
					value, value.fract ()),
		
		ArithmeticPrimitive1::Square =>
			arithmetic_primitive_1_delegate_call! (input,
					value, value.power (&2.into ()),
					value, value.power (&2.into ())),
		
		ArithmeticPrimitive1::SquareRoot =>
			arithmetic_primitive_1_delegate_call! (sqrt, input),
		
		ArithmeticPrimitive1::Exponential =>
			arithmetic_primitive_1_delegate_call! (exp, input),
		
		ArithmeticPrimitive1::Logarithm =>
			arithmetic_primitive_1_delegate_call! (ln, input),
		
		ArithmeticPrimitive1::Sin =>
			arithmetic_primitive_1_delegate_call! (sin, input),
		
		ArithmeticPrimitive1::Cos =>
			arithmetic_primitive_1_delegate_call! (cos, input),
		
		ArithmeticPrimitive1::Tan =>
			arithmetic_primitive_1_delegate_call! (tan, input),
		
		ArithmeticPrimitive1::Asin =>
			arithmetic_primitive_1_delegate_call! (asin, input),
		
		ArithmeticPrimitive1::Acos =>
			arithmetic_primitive_1_delegate_call! (acos, input),
		
		ArithmeticPrimitive1::Atan =>
			arithmetic_primitive_1_delegate_call! (atan, input),
		
	};
	
	return Ok (output);
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
		
		ArithmeticPrimitive2::Power =>
			arithmetic_primitive_2_delegate_call! ((input_1, input_2),
					(value_1, value_2), NumberReal::power (&<NumberReal>::from (*value_1), &<NumberReal>::from (*value_2)),
					(value_1, value_2), NumberReal::power (value_1, value_2)),
		
		ArithmeticPrimitive2::Minimum =>
			arithmetic_primitive_2_delegate_call! (min, (input_1, input_2)),
		
		ArithmeticPrimitive2::Maximum =>
			arithmetic_primitive_2_delegate_call! (max, (input_1, input_2)),
		
	};
	
	return Ok (output.into ());
}




pub fn arithmetic_primitive_n_evaluate (primitive : ArithmeticPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	let inputs_count = inputs.len ();
	
	if inputs_count == 1 {
		
		return Ok (inputs[0].clone ());
		
	} else if inputs_count == 0 {
		match primitive {
			
			ArithmeticPrimitiveN::Addition =>
				return Ok (ZERO.into ()),
			
			ArithmeticPrimitiveN::Multiplication =>
				return Ok (ONE.into ()),
			
			_ =>
				fail! (0x69d3b6cc),
		}
	}
	
	let mut output : Value = match try! (number_coerce_1 (&inputs[0])) {
		NumberCoercion1::Integer (value) =>
			value.clone () .into (),
		NumberCoercion1::Real (value) =>
			value.clone () .into (),
	};
	
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
			
			ArithmeticPrimitiveN::Minimum =>
				arithmetic_primitive_2_delegate_call! (min, (&output, input)),
			
			ArithmeticPrimitiveN::Maximum =>
				arithmetic_primitive_2_delegate_call! (max, (&output, input)),
			
		};
	}
	
	return Ok (output.into ());
}

