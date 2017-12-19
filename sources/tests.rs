

use super::builtins::exports::*;
use super::compiler::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::languages::exports::*;
use super::parser::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::f64;
use std::io;

use test;




pub mod exports {
	
	pub use super::{TestCase, TestAction, TestVerbosity};
	
	pub use super::{compile_tests, compile_test, parse_and_compile_tests};
	pub use super::{execute_tests, execute_test, parse_and_execute_tests};
	pub use super::{benchmark_tests, parse_and_benchmark_tests};
	
}




#[ derive (Clone, Debug, Hash) ]
pub struct TestCase {
	pub expression : Value,
	pub action : TestAction,
	pub verbosity : TestVerbosity,
}

#[ derive (Clone, Debug, Hash) ]
pub enum TestAction {
	Expect ( Value ),
	Debug,
	Ignore,
	Skip,
}

#[ derive (Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd) ]
pub enum TestVerbosity {
	Quiet,
	Verbose,
	Debug,
	Default,
}




#[ derive (Debug, Hash) ]
pub struct TestCaseCompiled {
	expression_without_optimizations : Expression,
	expression_with_optimizations : Expression,
	context_without_optimizations : Context,
	context_with_optimizations : Context,
	action : TestAction,
	verbosity_without_optimizations : TestVerbosity,
	verbosity_with_optimizations : TestVerbosity,
	source : TestCase,
}




#[ inline (always) ]
pub fn parse_and_compile_tests (identifier : &str, source : &str, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<(StdVec<TestCaseCompiled>)>) {
	let tests = try! (parse_tests (source));
	return compile_tests (identifier, &tests, transcript, verbosity);
}


#[ inline (never) ]
pub fn compile_tests (identifier : &str, tests : &StdVec<TestCase>, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<(StdVec<TestCaseCompiled>)>) {
	
	try_or_fail! (write! (transcript, "## compiling `{}`...\n", identifier), 0xb1d307bd);
	
	let context_without_optimizations = Context::new (None);
	try! (context_without_optimizations.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context_without_optimizations.define_all_with_prefix (try! (language_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let context_with_optimizations = Context::new (None);
	try! (context_with_optimizations.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context_with_optimizations.define_all_with_prefix (try! (language_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let tests = vec_filter_into! (tests.clone (), test,
		match test.action {
			TestAction::Skip => false,
			_ => true
		});
	
	let tests = try_vec_map_into! (tests, test, compile_test (&context_without_optimizations, &context_with_optimizations, &test, transcript, verbosity));
	
	succeed! (tests);
}




#[ inline (always) ]
pub fn parse_and_execute_tests (identifier : &str, source : &str, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	let tests = try! (parse_and_compile_tests (identifier, source, transcript, verbosity));
	return execute_tests (identifier, &tests, transcript, verbosity);
}


#[ inline (never) ]
pub fn execute_tests (identifier : &str, tests : &StdVec<TestCaseCompiled>, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	try_or_fail! (write! (transcript, "## executing `{}`...\n", identifier), 0x450c3e03);
	
	let mut tests_succeeded = 0;
	let mut tests_failed = 0;
	let mut tests_error = None;
	
	for test in tests {
		match execute_test (test, transcript, verbosity) {
			Ok (()) =>
				tests_succeeded += 1,
			Err (error) => {
				tests_failed += 1;
				if tests_error.is_none () {
					tests_error = Some (error);
				}
			},
		}
	}
	
	try_or_fail! (write! (transcript, "## executed `{}`: succeeded {} / failed {}!\n", identifier, tests_succeeded, tests_failed), 0xbf6a7cd1);
	
	if tests_error.is_none () {
		succeed! (());
	} else {
		return Err (tests_error.unwrap ());
	}
}






#[ inline (always) ]
pub fn parse_and_benchmark_tests (identifier : &str, source : &str, bencher : &mut test::Bencher, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	let tests = try! (parse_and_compile_tests (identifier, source, transcript, verbosity));
	return benchmark_tests (identifier, &tests, bencher, transcript, verbosity);
}


#[ inline (never) ]
pub fn benchmark_tests (identifier : &str, tests : &StdVec<TestCaseCompiled>, bencher : &mut test::Bencher, transcript : &mut io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	try_or_fail! (write! (transcript, "## benchmarking `{}`...\n", identifier), 0x0930df0d);
	
	let iterations_warmup = 1;
	let iterations_bencher = 1;
	let iterations_bests_with_optimizations = 5;
	let iterations_bests_without_optimizations = iterations_bests_with_optimizations * 5;
	let summary_factor = 1.0 / iterations_bencher as f64;
	let memory_leak_threshold = 128 * 1024;
	
	let resources_at_start = libc_getrusage_for_thread ();
	let memory_at_start = resources_at_start.ru_maxrss;
	
	for test in tests {
		for _iteration in 0 .. iterations_warmup {
			try! (execute_test (test, transcript, verbosity));
		}
	}
	
	let resources_after_warmup = libc_getrusage_for_thread ();
	let memory_after_warmup = resources_after_warmup.ru_maxrss;
	
	let mut summary_without_optimizations = None;
	let mut summary_without_optimizations_best = f64::MAX;
	let mut memory_leaks_for_without_optimizations = false;
	for _iteration in 0 .. iterations_bests_without_optimizations {
		
		let resources_before = libc_getrusage_for_thread ();
		
		let summary = bencher.bench (|ref mut bencher| bencher.iter (||
			if iterations_bencher == 1 {
				for test in tests {
					benchmark_test_without_optimizations (test) .expect ("2754c9b4");
				}
			} else {
				for test in tests {
					for _iteration in 0 .. iterations_bencher {
						benchmark_test_without_optimizations (test) .expect ("4d0ddf23");
					}
				}
			}
		));
		
		let resources_after = libc_getrusage_for_thread ();
		let memory_delta = resources_after.ru_maxrss - resources_before.ru_maxrss;
		if memory_delta > memory_leak_threshold {
			memory_leaks_for_without_optimizations = true;
		}
		
		if let Some (summary) = summary {
			if summary.median < summary_without_optimizations_best {
				summary_without_optimizations_best = summary.median;
				summary_without_optimizations = Some (summary);
			}
		} else {
			break;
		}
	}
	
	let resources_after_without_optimizations = libc_getrusage_for_thread ();
	let memory_after_without_optimizations = resources_after_without_optimizations.ru_maxrss;
	let memory_delta_for_without_optimizations = memory_after_without_optimizations - memory_after_warmup;
	if memory_delta_for_without_optimizations > memory_leak_threshold {
		memory_leaks_for_without_optimizations = true;
	}
	
	let mut summary_with_optimizations = None;
	let mut summary_with_optimizations_best = f64::MAX;
	let mut memory_leaks_for_with_optimizations = false;
	for _iteration in 0 .. iterations_bests_with_optimizations {
		
		let resources_before = libc_getrusage_for_thread ();
		
		let summary = bencher.bench (|ref mut bencher| bencher.iter (||
			if iterations_bencher == 1 {
				for test in tests {
					benchmark_test_with_optimizations (test) .expect ("a434c507");
				}
			} else {
				for test in tests {
					for _iteration in 0 .. iterations_bencher {
						benchmark_test_with_optimizations (test) .expect ("690bb327");
					}
				}
			}
		));
		
		let resources_after = libc_getrusage_for_thread ();
		let memory_delta = resources_after.ru_maxrss - resources_before.ru_maxrss;
		if memory_delta > memory_leak_threshold {
			memory_leaks_for_with_optimizations = true;
		}
		
		if let Some (summary) = summary {
			if summary.median < summary_with_optimizations_best {
				summary_with_optimizations_best = summary.median;
				summary_with_optimizations = Some (summary);
			}
		} else {
			break;
		}
	}
	
	let resources_after_with_optimizations = libc_getrusage_for_thread ();
	let memory_after_with_optimizations = resources_after_with_optimizations.ru_maxrss;
	let memory_delta_for_with_optimizations = memory_after_with_optimizations - memory_after_without_optimizations;
	if memory_delta_for_with_optimizations > memory_leak_threshold {
		memory_leaks_for_with_optimizations = true;
	}
	
	let _memory_delta_for_wamup = memory_after_warmup - memory_at_start;
	let memory_delta_for_without_optimizations = memory_delta_for_without_optimizations as f64 / 1024.0;
	let memory_delta_for_with_optimizations = memory_delta_for_with_optimizations as f64 / 1024.0;
	
	try_or_fail! (write! (transcript, "## benchmarked `{}`!\n", identifier), 0xedd3605c);
	if let Some (summary_without_optimizations) = summary_without_optimizations {
		try! (benchmark_report (
				&format! ("without optimizations:"), "     ",
				&summary_without_optimizations, None, summary_factor,
				transcript, verbosity));
	}
	if memory_leaks_for_without_optimizations {
		try_or_fail! (write! (transcript, "       mem-leaks : {:10.0} KB (!!!! DETECTED !!!!)\n", memory_delta_for_without_optimizations), 0x3c756d6f);
	} else {
		try_or_fail! (write! (transcript, "       mem-leaks : {:10.0} KB\n", memory_delta_for_without_optimizations), 0xb8b463fe);
	}
	if let Some (summary_with_optimizations) = summary_with_optimizations {
		try! (benchmark_report (
				&format! ("with optimizations:"), "     ",
				&summary_with_optimizations, summary_without_optimizations.as_ref (), summary_factor,
				transcript, verbosity));
	}
	if memory_leaks_for_with_optimizations {
		try_or_fail! (write! (transcript, "       mem-leaks : {:10.0} KB (!!!! DETECTED !!!!)\n", memory_delta_for_with_optimizations), 0x1c319cd7);
	} else {
		try_or_fail! (write! (transcript, "       mem-leaks : {:10.0} KB\n", memory_delta_for_with_optimizations), 0x5460baae);
	}
	
	succeed! (());
}

#[ inline (always) ]
fn benchmark_report (header : &str, prefix : &str, summary : &test::stats::Summary, reference : Option<&test::stats::Summary>, factor : f64, transcript : &mut io::Write, _verbosity : TestVerbosity) -> (Outcome<()>) {
	let mut report = StdString::new ();
	report.push_str (&format! ("{}{}\n", prefix, header));
	if let Some (reference) = reference {
		let speedup_factor = reference.mean / summary.mean;
		let speedup_percent = (1.0 - summary.mean / reference.mean) * 100.0;
		report.push_str (&format! ("{}  average   : {:10.0} / {:+5.2}% / {:.2}*\n", prefix, summary.mean * factor, speedup_percent, speedup_factor));
	} else {
		report.push_str (&format! ("{}  average   : {:10.0}\n", prefix, summary.mean * factor));
	}
	report.push_str (&format! ("{}    stdev   : {:10.0} / {:6.2}%\n", prefix, summary.std_dev * factor, summary.std_dev_pct));
	report.push_str (&format! ("{}    median  : {:10.0} / {:6.2}%\n", prefix, summary.median * factor, summary.median_abs_dev_pct));
	report.push_str (&format! ("{}  min / max : {:10.0} / {:.0} / {:.0} / {:.0} / {:.0}\n", prefix, summary.min * factor, summary.quartiles.0 * factor, summary.quartiles.1 * factor, summary.quartiles.2 * factor, summary.max * factor));
	try_or_fail! (write! (transcript, "{}", report), 0x9b631c5f);
	succeed! (());
}




#[ inline (always) ]
#[ allow (unused_assignments) ] // FIXME:  Why does the compiler think we are not using `header_emitted`?
pub fn compile_test (context_without_optimizations : &Context, context_with_optimizations : &Context, test : &TestCase, transcript : &mut io::Write, verbosity_global : TestVerbosity) -> (Outcome<TestCaseCompiled>) {
	
	
	let (verbosity_without_optimizations, verbosity_with_optimizations) = match (&test.action, verbosity_global) {
		(&TestAction::Debug, _) |
		(_, TestVerbosity::Debug) =>
			(TestVerbosity::Debug, TestVerbosity::Debug),
		_ =>
			(verbosity_global, verbosity_global),
	};
	let verbosity_generic = verbosity_global;
	
	
	let mut header_emitted = try! (test_case_header_emit (test, transcript, verbosity_generic, false, false));
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_generic, header_emitted, true));
			try_or_fail! (write! (transcript, "-- parse --\n{:#?}\n", &test.expression), 0xa70973e0);
		},
		_ =>
			(),
	}
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_generic, header_emitted, true));
			try_or_fail! (write! (transcript, "-- compile-without-optimizations ...\n"), 0x72ea2b99);
		},
		_ =>
			(),
	}
	
	let expression_without_optimizations = match compile (&context_without_optimizations, &test.expression) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_without_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! compile-without-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &test.expression, &error), 0x495036cb);
			return Err (error);
		},
	};
	
	match verbosity_without_optimizations {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_without_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "-- compile-without-optimizations --\n{:#?}\n", &expression_without_optimizations), 0xebf8f59e);
		},
		_ =>
			(),
	}
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_generic, header_emitted, true));
			try_or_fail! (write! (transcript, "-- compile-with-optimizations ...\n"), 0xe66eecff);
		},
		_ =>
			(),
	}
	
	let expression_with_optimizations = match compile (&context_with_optimizations, &test.expression) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! compile-with-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &test.expression, &error), 0x1241fb62);
			return Err (error);
		},
	};
	
	let expression_with_optimizations = match optimize (expression_with_optimizations) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! compile-with-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &test.expression, &error), 0x4a0986b8);
			return Err (error);
		},
	};
	
	match verbosity_with_optimizations {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (test, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "-- compile-with-optimizations --\n{:#?}\n", &expression_with_optimizations), 0xaacf1561);
		},
		_ =>
			(),
	}
	
	
	try! (test_case_footer_emit (test, transcript, verbosity_generic, header_emitted, false));
	
	
	let test = TestCaseCompiled {
			expression_without_optimizations,
			expression_with_optimizations,
			context_without_optimizations : context_without_optimizations.clone (),
			context_with_optimizations : context_with_optimizations.clone (),
			action : test.action.clone (),
			verbosity_without_optimizations,
			verbosity_with_optimizations,
			source : test.clone (),
		};
	
	succeed! (test);
}




#[ inline (always) ]
#[ allow (unused_assignments) ] // FIXME:  Why does the compiler think we are not using `header_emitted`?
pub fn execute_test (test : &TestCaseCompiled, transcript : &mut io::Write, verbosity_global : TestVerbosity) -> (Outcome<()>) {
	
	let (verbosity_without_optimizations, verbosity_with_optimizations) = match (&test.action, verbosity_global) {
		(&TestAction::Debug, _) |
		(_, TestVerbosity::Debug) =>
			(TestVerbosity::Debug, TestVerbosity::Debug),
		_ =>
			(test.verbosity_without_optimizations, test.verbosity_with_optimizations),
	};
	let verbosity_generic = verbosity_global;
	
	
	let mut header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_generic, false, false));
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_generic, header_emitted, true));
			try_or_fail! (write! (transcript, "-- evaluate-without-optimizations ...\n"), 0xad17507e);
		},
		_ =>
			(),
	}
	
	let output_value_without_optimizations = match evaluate (&test.context_without_optimizations, &test.expression_without_optimizations) {
		Ok (output_value) =>
			output_value,
		Err (error) => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_without_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! evaluate-without-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &test.expression_without_optimizations, &error), 0xd70f287e);
			return Err (error);
		},
	};
	
	match verbosity_without_optimizations {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_without_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "-- evaluate-without-optimizations --\n{:#?}\n", &output_value_without_optimizations), 0x4e1189d7);
		},
		_ =>
			(),
	}
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_generic, header_emitted, true));
			try_or_fail! (write! (transcript, "-- evaluate-with-optimizations ...\n"), 0xecf07fc4);
		},
		_ =>
			(),
	}
	
	let output_value_with_optimizations = match evaluate (&test.context_with_optimizations, &test.expression_with_optimizations) {
		Ok (output_value) =>
			output_value,
		Err (error) => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! evaluate-with-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &test.expression_with_optimizations, &error), 0x4dfaa3fd);
			return Err (error);
		},
	};
	
	match verbosity_with_optimizations {
		TestVerbosity::Debug => {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "-- evaluate-with-optimizations --\n{:#?}\n", &output_value_with_optimizations), 0xa410b78a);
		},
		_ =>
			(),
	}
	
	
	let expected_value_without_optimizations = match test.action {
		TestAction::Expect (ref expected_expression) => {
			// FIXME:  Add error reporting for these!
			let expected_expression = try! (compile (&test.context_without_optimizations, expected_expression));
			let expected_value = try! (evaluate (&test.context_without_optimizations, &expected_expression));
			Some (expected_value)
		},
		_ =>
			None,
	};
	
	if let Some (ref expected_value) = expected_value_without_optimizations {
		let output_matched = try! (equivalent_by_value_strict_recursive_2 (&output_value_without_optimizations, expected_value));
		if !output_matched {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_without_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! assertion-without-optimizations !! {} => {}\n", &output_value_without_optimizations, expected_value), 0xdcdf61a8);
			try_or_fail! (write! (transcript, "!! assertion-without-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &output_value_without_optimizations, &expected_value), 0x64d71b63);
			try_or_fail! (write! (transcript, "!! failed\n"), 0xd2c3f278);
			fail! (0x67419241);
		}
	}
	
	
	let expected_value_with_optimizations = match test.action {
		TestAction::Expect (ref expected_expression) => {
			// FIXME:  Add error reporting for these!
			let expected_expression = try! (compile (&test.context_with_optimizations, expected_expression));
			let expected_value = try! (evaluate (&test.context_with_optimizations, &expected_expression));
			Some (expected_value)
		},
		_ =>
			None,
	};
	
	if let Some (ref expected_value) = expected_value_with_optimizations {
		let output_matched = try! (equivalent_by_value_strict_recursive_2 (&output_value_with_optimizations, expected_value));
		if !output_matched {
			header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_with_optimizations, header_emitted, true));
			try_or_fail! (write! (transcript, "!! assertion-with-optimizations !! {} => {}\n", &output_value_with_optimizations, expected_value), 0xb66640e5);
			try_or_fail! (write! (transcript, "!! assertion-with-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", &output_value_with_optimizations, &expected_value), 0xe650c868);
			try_or_fail! (write! (transcript, "!! failed\n"), 0xf7d88757);
			fail! (0xe52ddb4f);
		}
	}
	
	
	match (expected_value_without_optimizations, expected_value_with_optimizations) {
		(None, None) =>
			(),
		(Some (ref expected_value_without_optimizations), Some (ref expected_value_with_optimizations)) =>
			match (expected_value_without_optimizations.class (), expected_value_with_optimizations.class ()) {
				(ValueClass::ProcedureLambda, ValueClass::ProcedureLambda) |
				(ValueClass::SyntaxLambda, ValueClass::SyntaxLambda) |
				(ValueClass::Port, ValueClass::Port) =>
					(),
				(_, _) => {
					let output_matched = try! (equivalent_by_value_strict_recursive_2 (expected_value_without_optimizations, expected_value_with_optimizations));
					if !output_matched {
						header_emitted = try! (test_case_header_emit (&test.source, transcript, verbosity_generic, header_emitted, true));
						try_or_fail! (write! (transcript, "!! assertion-with/without-optimizations !! {} => {}\n", expected_value_without_optimizations, expected_value_with_optimizations), 0xe003610f);
						try_or_fail! (write! (transcript, "!! assertion-with/without-optimizations !!\n{:#?}\n!! => !!\n{:#?}\n", expected_value_without_optimizations, expected_value_with_optimizations), 0x91335537);
						try_or_fail! (write! (transcript, "!! failed\n"), 0x8459f31b);
						fail! (0xc8a2813a);
					}
				},
			},
		(_, _) =>
			fail_panic! (0x2f1f97f3),
	}
	
	
	try! (test_case_footer_emit (&test.source, transcript, verbosity_global, header_emitted, false));
	
	succeed! (());
}




#[ inline (always) ]
pub fn benchmark_test_without_optimizations (test : &TestCaseCompiled) -> (Outcome<()>) {
	
	try! (evaluate (&test.context_without_optimizations, &test.expression_without_optimizations));
	
	succeed! (());
}

#[ inline (always) ]
pub fn benchmark_test_with_optimizations (test : &TestCaseCompiled) -> (Outcome<()>) {
	
	try! (evaluate (&test.context_with_optimizations, &test.expression_with_optimizations));
	
	succeed! (());
}




#[ inline (always) ]
fn test_case_header_emit (test : &TestCase, transcript : &mut io::Write, verbosity : TestVerbosity, emitted : bool, forced : bool) -> (Outcome<bool>) {
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
		try_or_fail! (write! (transcript, "\n--------------------------------------------------------------------------------\n"), 0xccf80728);
		match test.action {
			TestAction::Expect (ref expected) =>
				try_or_fail! (write! (transcript, ">> {} => {};\n", &test.expression, expected), 0xf1bf16d3),
			TestAction::Debug | TestAction::Ignore | TestAction::Skip =>
				try_or_fail! (write! (transcript, ">> {} => #ignore;\n", &test.expression), 0x01fdbf40),
		}
		succeed! (true);
	} else {
		succeed! (false);
	}
}


#[ inline (always) ]
fn test_case_footer_emit (test : &TestCase, transcript : &mut io::Write, verbosity : TestVerbosity, emitted : bool, forced : bool) -> (Outcome<bool>) {
	let emitted = try! (test_case_header_emit (test, transcript, verbosity, emitted, forced));
	if emitted {
		try_or_fail! (write! (transcript, "--------------------------------------------------------------------------------\n\n\n\n"), 0x3d63834c);
	}
	succeed! (emitted);
}

