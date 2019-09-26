

#![ feature (test) ]
#![ no_implicit_prelude ]
#![ feature (slice_concat_trait) ]
include! ("prelude.in");




def_test! (test__values, {
	
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
			"#x0", "#x0F", "#xF0", "#X0", "#X0F", "#XF0",
			"#b-0", "#b-01", "#b-10", "#B-0", "#B-01", "#B-10",
			"#o-0", "#o-07", "#o-70", "#O-0", "#O-07", "#O-70",
			"#d-0", "#d-01", "#d-10", "#D-0", "#D-01", "#D-10",
			"#x-0", "#x-0f", "#x-f0", "#X-0", "#X-0f", "#X-f0",
			"#x-0", "#x-0F", "#x-F0", "#X-0", "#X-0F", "#X-F0",
			"#b+0", "#b+01", "#b+10", "#B+0", "#B+01", "#B+10",
			"#o+0", "#o+07", "#o+70", "#O+0", "#O+07", "#O+70",
			"#d+0", "#d+01", "#d+10", "#D+0", "#D+01", "#D+10",
			"#x+0", "#x+0f", "#x+f0", "#X+0", "#X+0f", "#X+f0",
			"#x+0", "#x+0F", "#x+F0", "#X+0", "#X+0F", "#X+F0",
			"0.0", "0.", ".0", "+0.0", "+0.", "+.0", "-0.0", "-0.", "-.0",
			"1.2", "1.", ".2", "+1.2", "+1.", "+.2", "-1.2", "-1.", "-.2",
			"0.0e0", "0.e+0", ".0e-0", "+0.0e0", "+0.e+0", "+.0e-0", "-0.0e0", "-0.e+0", "-.0e-0",
			"0.0E0", "0.E+0", ".0E-0", "+0.0E0", "+0.E+0", "+.0E-0", "-0.0E0", "-0.E+0", "-.0E-0",
			"1.2e3", "1.e+3", ".2e-3", "+1.2e3", "+1.e+3", "+.2e-3", "-1.2e3", "-1.e+3", "-.2e-3",
			"1.2E3", "1.E+3", ".2E-3", "+1.2E3", "+1.E+3", "+.2E-3", "-1.2E3", "-1.E+3", "-.2E-3",
			"inf", "+inf", "+inf.0", "-inf", "-inf.0",
			"nan", "+nan", "+nan.0", "-nan", "-nan.0",
			"epsilon", "+epsilon", "+epsilon.0", "-epsilon", "-epsilon.0",
			
			/*== symbols 1 ==*/
			"a", "A", "ab", "aB", "Ab", "AB",
			"a1", "1a",
			"+", "++", "-", "--", "+-", "-+",
			"+a", "-a", ".a", "a+", "a-", "a.",
			"|.|", "+.", "-.", ".-", ".+", "..", "...",
			
			/*== symbols 2 ==*/
			"|#t|", "|#true|", "|#f|", "|#false|",
			"|0|", "|+0|", "|-0|", "|.0|", "|+.0|", "|-.0|", "|+0.|", "|-0.|",
			"|0+|", "|0-|", "|+0+|", "|-0-|",
			"|++0|", "|--0|", "|0++|", "|0--|", "|++0++|", "|--0--|",
			"|()|", "|#null|",
			r#"||"#, r#"|a|"#, r#"|0|"#, r#"|a0|"#, r#"|0a|"#, r#"| |"#, r#"|a a|"#, r#"|\n|"#, r#"|\x0;|"#, r#"|\x00;|"#, r#"|\x000;|"#,
			
			/*== characters ==*/
			"#\\newline", "#\\return", "#\\tab", "#\\null",
			"#\\n", "#\\r", "#\\t",
			"#\\x", "#\\X",
			"#\\x0", "#\\x00", "#\\x000", "#\\x0000", "#\\x00000", "#\\x000000", "#\\x0000000",
			"#\\x0", "#\\x00", "#\\x000", "#\\x0000", "#\\x00000", "#\\x000000", "#\\x0000000",
			"#\\x1", "#\\x10", "#\\x01", "#\\x001", "#\\x0001", "#\\x00001", "#\\x000001", "#\\x0000001", "#\\x00000001",
			"#\\X1", "#\\X10", "#\\x01", "#\\x001", "#\\x0001", "#\\x00001", "#\\x000001", "#\\x0000001", "#\\x00000001",
			"#\\xf", "#\\xf0", "#\\x0f", "#\\x00f", "#\\x000f", "#\\x0000f", "#\\x00000f", "#\\x000000f", "#\\x0000000f",
			"#\\Xf", "#\\Xf0", "#\\x0f", "#\\x00f", "#\\x000f", "#\\x0000f", "#\\x00000f", "#\\x000000f", "#\\x0000000f",
			"#\\xF", "#\\xF0", "#\\x0F", "#\\x00F", "#\\x000F", "#\\x0000F", "#\\x00000f", "#\\x000000F", "#\\x0000000F",
			"#\\XF", "#\\XF0", "#\\x0F", "#\\x00F", "#\\x000F", "#\\x0000F", "#\\x00000f", "#\\x000000F", "#\\x0000000F",
			
			/*== strings ==*/
			r#""""#, r#""a""#, r#""0""#, r#""a0""#, r#""0a""#, r#"" ""#, r#""a a""#, r#""\n""#, r#""\t""#, r#""\x0;""#, r#""\x00;""#, r#""\x000;""#,
			"#string()", "#string( )", "#string(0)", "#string( 0 )", "#string(0 1)", "#string(0 1 2)", "#string(0 1 2 3)",
			
			/*== bytes ==*/
			"#u8()", "#u8( )", "#u8(0)", "#u8( 0 )", "#u8(0 1)", "#u8(0 1 2)", "#u8(0 1 2 3)",
			"#U8()", "#U8( )", "#U8(0)", "#U8( 0 )", "#U8(0 1)", "#U8(0 1 2)", "#U8(0 1 2 3)",
			"#bytes()", "#bytes( )", "#bytes(0)", "#bytes( 0 )", "#bytes(0 1)", "#bytes(0 1 2)", "#bytes(0 1 2 3)",
			
			/*== lists ==*/
			"()", "#null",
			"(0)", "( 0 )", "(0 1)", "(0 1 2)",
			"(0 . 1)", "( 0 . 1 )",
			"(0 1 . 2)", "(0 1 2 . 3)",
			
			/*== vectors ==*/
			"#()", "#( )", "#(0)", "#( 0 )", "#(0 1)", "#(0 1 2)", "#(0 1 2 3)",
			"#array()", "#array( )", "#array(0)", "#array( 0 )", "#array(0 1)", "#array(0 1 2)", "#array(0 1 2 3)",
			
			/*== values ==*/
			"#values()", "#values( )", "#values(0)", "#values( 0 )", "#values(0 1)", "#values(0 1 2)", "#values(0 1 2 3)",
			
		];
	
	
	for input_1 in tests {
		
		eprintln! (">> `{:?}`", input_1);
		
		let output_1 = parse_value (input_1, None) .expect ("9f292a07");
		let input_2 = output_1.to_string ();
		let output_2 = parse_value (input_2.as_ref (), None) .expect ("3652725f");
		
		if output_1 != output_2 {
			eprintln! ("== `{:?}` -> `{:?}` -> `{:?}` -> `{:?}`", input_1, output_1, input_2, output_2);
			assert_eq! (output_1, output_2);
		}
		
		eprintln! ("== `{:?}` -> `{}`", input_1, output_1);
		if false {
			eprintln! ("## {:#?}", output_1);
		}
		eprintln! ();
	}
	
});




def_test! (test__comments, {
	
	let tests = vec! [
			
			"",
			" ", "  ", "   ",
			"\t", "\t\t", "\t\t\t",
			"\n", "\n\n", "\n\n\n",
			"\t\n", "\t\n\t\n", "\t\n\t\n\t\n",
			"\n\t", "\n\t\n\t", "\n\t\n\t\n\t",
			
			"0", "0 ", " 0", " 0 ", "0  ", "  0", "  0  ",
			"0", "0\t", "\t0", "\t0\t", "0\t\t", "\t\t0", "\t\t0\t\t",
			"0", "0\n", "\n0", "\n0\n", "0\n\n", "\n\n0", "\n\n0\n\n",
			
			"0 1", "0 1 ", " 0 1", " 0 1 ", "0  1  ", "  0  1", "  0  1  ",
			"0\t1", "0\t1\t", "\t0\t1", "\t0\t1\t", "0\t\t1\t\t", "\t\t0\t\t1", "\t\t0\t\t1\t\t",
			"0\n1", "0\n1\n", "\n0\n1", "\n0\n1\n", "0\n\n1\n\n", "\n\n0\n\n1", "\n\n0\n\n1\n\n",
			
			";\n", "; \n", ";\t\n",
			";0\n", ";0 \n", "; 0\n", ";0\t\n", ";\t0\n",
			
			"1;\n", "1; \n", "1;\t\n",
			"1;0\n", "1;0 \n", "1; 0\n", "1;0\t\n", "1;\t0\n",
			
			"#||#", "#|#||#|#", "#|#|#||#|#|#", "#|#|#|#||#|#|#|#",
			"#| |#", "#| #| |# |#", "#| #| #| |# |# |#", "#| #| #| #| |# |# |# |#",
			"#|  |#", "#|  #|  |#  |#", "#|  #|  #|  |#  |#  |#", "#|  #|  #|  #|  |#  |#  |#  |#",
			"#|\t|#", "#|\t#|\t|#\t|#", "#|\t#|\t#|\t|#\t|#\t|#", "#|\t#|\t#|\t#|\t|#\t|#\t|#\t|#",
			"#|\t\t|#", "#|\t\t#|\t\t|#\t\t|#", "#|\t\t#|\t\t#|\t\t|#\t\t|#\t\t|#", "#|\t\t#|\t\t#|\t\t#|\t\t|#\t\t|#\t\t|#\t\t|#",
			"#|\t\t\t|#", "#|\t\t\t#|\t\t\t|#\t\t\t|#", "#|\t\t\t#|\t\t\t#|\t\t\t|#\t\t\t|#\t\t\t|#", "#|\t\t\t#|\t\t\t#|\t\t\t#|\t\t\t|#\t\t\t|#\t\t\t|#\t\t\t|#",
			"#|\n|#", "#|\n#|\n|#\n|#", "#|\n#|\n#|\n|#\n|#\n|#", "#|\n#|\n#|\n#|\n|#\n|#\n|#\n|#",
			"#|\n\n|#", "#|\n\n#|\n\n|#\n\n|#", "#|\n\n#|\n\n#|\n\n|#\n\n|#\n\n|#", "#|\n\n#|\n\n#|\n\n#|\n\n|#\n\n|#\n\n|#\n\n|#",
			"#|\n\n\n|#", "#|\n\n\n#|\n\n\n|#\n\n\n|#", "#|\n\n\n#|\n\n\n#|\n\n\n|#\n\n\n|#\n\n\n|#", "#|\n\n\n#|\n\n\n#|\n\n\n#|\n\n\n|#\n\n\n|#\n\n\n|#\n\n\n|#",
			
			"#|;|#", "#|;#|;|#;|#", "#|;#|;#|;|#;|#;|#", "#|;#|;#|;#|;|#;|#;|#;|#",
			"#|_#_|#", "#|_#_#|_#_|#_#_|#", "#|_#_#|_#_#|_#_|#_#_|#_#_|#", "#|_#_#|_#_#|_#_#|_#_|#_#_|#_#_|#_#_|#",
			"#|_|_|#", "#|_|_#|_|_|#_|_|#", "#|_|_#|_|_#|_|_|#_|_|#_|_|#", "#|_|_#|_|_#|_|_#|_|_|#_|_|#_|_|#_|_|#",
			
			"#;0",
			"#; 0", "#;\t0", "#;\n0",
			"#;  0", "#;\t\t0", "#;\n\n0",
			
		];
	
	
	for input_1 in tests {
		
		eprintln! (">> `{:?}`", input_1);
		
		let output_1 = parse_script (input_1, None) .expect ("6167400f");
		let input_2 = output_1.iter () .map (|value| value.to_string ()) .collect::<StdVec<StdString>> ();
		let input_2 = slice::Join::join (input_2.as_slice (), " ");
		let output_2 = parse_script (input_2.as_ref (), None) .expect ("3773d406");
		
		if output_1 != output_2 {
			eprintln! ("== `{:?}` -> `{:?}` -> `{:?}` -> `{:?}`", input_1, output_1, input_2, output_2);
			assert_eq! (output_1, output_2);
		}
		
		eprintln! ("== `{:?}` -> `{:?}`", input_1, output_1);
		if false {
			eprintln! ("## {:#?}", output_1);
		}
		eprintln! ();
	}
	
});

