

extern crate rust_scheme;

use rust_scheme::exports::*;
use rust_scheme::runtime::exports::*;




macro_rules! assert_compare_2 {
	(
			$comparator : ident,
			(
				$( ( $left : expr, $right : expr ), )*
			),
			(
					$equivalent_by_identity : expr,
					$equivalent_by_value_strict : expr,
					$equivalent_by_value_strict_recursive : expr,
					$equivalent_by_value_coerced : expr,
					$equivalent_by_value_coerced_recursive : expr
			)
	) => (
		$(
			assert_compare_outcome! ($comparator ($left, $right, Comparison::Equivalence (Equivalence::ByIdentity, None, None)), $equivalent_by_identity);
		)*
	);
}

macro_rules! assert_compare_outcome {
	( $comparator : expr, $expected : expr ) => (
		let outcome = $comparator;
		let expected = $expected;
		let mut comparator_expression = StdString::from (stringify! ($comparator)) .replace ('\n', " ") .replace ('\t', " ");
		loop {
			let comparator_expression_0 = comparator_expression.replace ("  ", " ");
			if comparator_expression == comparator_expression_0 {
				break;
			} else {
				comparator_expression = comparator_expression_0;
			}
		}
		match outcome {
			Ok (output) => {
				if output == expected {
					eprintln! ("succeeded executing `{}`;", comparator_expression);
				} else {
					eprintln! ("failed with wrong output `{}`, expected `{}`, executing `{}`!", output, expected, comparator_expression);
					panic! ("3b3ffddf");
				}
			},
			Err (error) => {
				eprintln! ("failed with error `{:?}` executing `{}`!", error, comparator_expression);
				panic! ("a5dfe9c3");
			},
		}
	);
}




#[ test ]
fn test () -> () {
	
	assert_compare_2! (
			boolean_compare_2,
			(
				(&TRUE, &TRUE),
				(&FALSE, &FALSE),
			),
			(true, true, true, true, true)
	);
	
}

