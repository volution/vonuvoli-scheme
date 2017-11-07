

extern crate rust_scheme;

use rust_scheme::*;

fn main () -> ()
{
	
	if false {
		
		println! ("## size-of `Boolean`: {}", std::mem::size_of::<values::Boolean> ());
		println! ("## size-of `NumberInteger`: {}", std::mem::size_of::<values::NumberInteger> ());
		println! ("## size-of `NumberReal`: {}", std::mem::size_of::<values::NumberReal> ());
		println! ("## size-of `Character`: {}", std::mem::size_of::<values::Character> ());
		println! ("## size-of `String`: {}", std::mem::size_of::<values::String> ());
		println! ("## size-of `Symbol`: {}", std::mem::size_of::<values::Symbol> ());
		println! ("## size-of `Bytes`: {}", std::mem::size_of::<values::Bytes> ());
		println! ("## size-of `Pair`: {}", std::mem::size_of::<values::Pair> ());
		println! ("## size-of `Value`: {}", std::mem::size_of::<values::Value> ());
		
		println! ("## size-of `()`: {}", std::mem::size_of::<()> ());
		println! ("## size-of `Option<()>`: {}", std::mem::size_of::<Option<()>> ());
		println! ("## size-of `Option<i8>`: {}", std::mem::size_of::<Option<i8>> ());
		println! ("## size-of `Option<i16>`: {}", std::mem::size_of::<Option<i16>> ());
		println! ("## size-of `Option<i32>`: {}", std::mem::size_of::<Option<i32>> ());
		println! ("## size-of `Option<(i32, i32)>`: {}", std::mem::size_of::<Option<(i32, i32)>> ());
		println! ("## size-of `Option<i64>`: {}", std::mem::size_of::<Option<i64>> ());
		
	}
	
	let tests = vec! [
			
			/*== booleans ==*/
			"#t", "#true", "#f", "#false",
			
			/*== numbers ==*/
			"0", "+0", "-0",
			"01", "+01", "-01",
			"1234567890", "+1234567890", "-1234567890",
			"#b0", "#b01", "#b10", "#B0", "#B01", "#B10",
			"#o0", "#o07", "#o70", "#O0", "#O07", "#O70",
			"#d0", "#d01", "#d10", "#D0", "#D01", "#D10",
			"#x0", "#x0f", "#xf0", "#X0", "#X0f", "#Xf0",
			"0.0", "0.", ".0", "+0.0", "+0.", "+.0", "-0.0", "-0.", "-.0",
			"1.2", "1.", ".2", "+1.2", "+1.", "+.2", "-1.2", "-1.", "-.2",
			"0.0e0", "0.e+0", ".0e-0", "+0.0e0", "+0.e+0", "+.0e-0", "-0.0e0", "-0.e+0", "-.0e-0",
			"1.2e3", "1.e+3", ".2e-3", "+1.2e3", "+1.e+3", "+.2e-3", "-1.2e3", "-1.e+3", "-.2e-3",
			"+inf", "+inf.0", "-inf", "-inf.0", "+nan", "+nan.0", "-nan", "-nan.0",
			
			/*== symbols ==*/
			"a", "A", "ab", "aB", "Ab", "AB",
			"a1", "1a",
			"+", "-", "+a", "-a", ".a", "a+", "a-", "a.",
			
			/*== characters ==*/
			"#\\x", "#\\x0", "#\\x00", "#\\x000", "#\\null",
			/*== strings ==*/
			r#""""#, r#""a""#, r#""0""#, r#""a0""#, r#""0a""#, r#"" ""#, r#""a a""#, r#""\n""#, r#""\x0;""#, r#""\x00;""#, r#""\x000;""#,
			
			/*== bytes ==*/
			"#u8()", "#U8()", "#u8( )", "#u8(0)", "#u8( 0 )", "#u8(0 1)", "#u8(0 1 2)", "#u8(0 1 2 3)",
			
			/*== lists ==*/
			"()", "#null",
			"(0)", "( 0 )", "(0 1)", "(0 1 2)",
			"(0 . 1)", "( 0 . 1 )",
			"(0 1 . 2)", "(0 1 2 . 3)",
			
			/*== vectors ==*/
			"#()",
			"#(0)", "#( 0 )", "#(0 1)", "#(0 1 2)",
			
		];
	
	for input_1 in tests {
		println! (">> `{}`", input_1);
		let output_1 = parser::parse_value (input_1);
		let input_2 = output_1.to_string ();
		let output_2 = parser::parse_value (input_2.as_ref ());
		println! ("<< `{}` -> `{}` -> `{}`", input_1, output_1, output_2);
	}
	
}

