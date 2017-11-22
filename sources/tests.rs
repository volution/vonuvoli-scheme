

use super::compiler::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::languages::exports::*;
use super::parser::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::io::Write;



pub mod exports {
	pub use super::{TestCase, TestAction, TestVerbosity};
	pub use super::{execute_tests, execute_test};
	pub use super::parse_and_execute_tests;
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct TestCase {
	pub value : Value,
	pub action : TestAction,
}

#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum TestAction {
	Expect ( Value ),
	Debug,
	Ignore,
	Skip,
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum TestVerbosity {
	Quiet,
	Verbose,
	Debug,
}




pub fn parse_and_execute_tests (source : &str, transcript : &mut Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	let tests = try! (parse_tests (source));
	return execute_tests (&tests, transcript, verbosity);
}




pub fn execute_tests (tests : &StdVec<TestCase>, transcript : &mut Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	for test in tests {
		try! (execute_test (&context, test, transcript, verbosity));
	}
	
	succeed! (());
}


#[ allow (unused_assignments) ]
pub fn execute_test (context : &Context, test : &TestCase, transcript : &mut Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	
	fn header_emit (test : &TestCase, transcript : &mut Write, verbosity : TestVerbosity, emitted : bool, forced : bool) -> (Outcome<bool>) {
		if emitted {
			succeed! (true);
		}
		let forced = match verbosity {
			TestVerbosity::Verbose | TestVerbosity::Debug =>
				true,
			_ =>
				forced,
		};
		if forced {
			try_or_fail! (writeln! (transcript), 0x2ee85063);
			match test.action {
				TestAction::Expect (ref output) =>
					try_or_fail! (write! (transcript, ">> {} => {};\n", &test.value, output), 0xf1bf16d3),
				TestAction::Debug | TestAction::Ignore | TestAction::Skip =>
					try_or_fail! (write! (transcript, ">> {} => #ignore;\n", &test.value), 0x01fdbf40),
			}
			succeed! (true);
		} else {
			succeed! (false);
		}
	}
	
	
	let verbosity = match test.action {
		TestAction::Debug =>
			TestVerbosity::Debug,
		_ =>
			verbosity,
	};
	
	
	match test.action {
		TestAction::Skip =>
			succeed! (()),
		_ =>
			(),
	}
	
	
	let mut header_emitted = try! (header_emit (test, transcript, verbosity, false, false));
	
	
	let input_expression = match compile (&context, &test.value) {
		Ok (input_expression) =>
			input_expression,
		Err (error) => {
			header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
			try_or_fail! (write! (transcript, "!! compile !!\n{:#?}\n!! => !!\n{:#?}\n", &test.value, &error), 0x495036cb);
			return Err (error);
		},
	};
	
	match verbosity {
		TestVerbosity::Debug => {
			header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
			try_or_fail! (write! (transcript, "-- compile --\n{:#?}\n", &input_expression), 0xebf8f59e);
		},
		_ =>
			(),
	}
	
	
	let input_value = match evaluate (&context, &input_expression) {
		Ok (input_value) =>
			input_value,
		Err (error) => {
			header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
			try_or_fail! (write! (transcript, "!! evaluate !!\n{:#?}\n!! => !!\n{:#?}\n", &input_expression, &error), 0xd70f287e);
			return Err (error);
		},
	};
	
	match verbosity {
		TestVerbosity::Debug => {
			header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
			try_or_fail! (write! (transcript, "-- evaluate --\n{:#?}\n", &input_value), 0x4e1189d7);
		},
		_ =>
			(),
	}
	
	
	let output_value = match test.action {
		TestAction::Expect (ref output_syntax) => {
			let output_expression = try! (compile (&context, &output_syntax));
			let output_value = try! (evaluate (&context, &output_expression));
			Some (output_value)
		},
		_ =>
			None,
	};
	
	
	if let Some (output_value) = output_value {
		if input_value != output_value {
			header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
			try_or_fail! (write! (transcript, "!! assertion !! {} => {}\n", &input_value, &output_value), 0xb66640e5);
			try_or_fail! (write! (transcript, "!! assertion !!\n{:#?}\n!! => !!\n{:#?}\n", &input_value, &output_value), 0xe650c868);
			try_or_fail! (write! (transcript, "!! failed\n"), 0xf7d88757);
			fail! (0xe52ddb4f);
		} else {
			match verbosity {
				TestVerbosity::Debug => {
					header_emitted = try! (header_emit (test, transcript, verbosity, header_emitted, true));
					try_or_fail! (write! (transcript, "!! succeeded\n"), 0x15c96537);
					try_or_fail! (writeln! (transcript), 0xace6dd5b);
				},
				_ =>
					(),
			}
		}
	}
	
	
	succeed! (());
}

