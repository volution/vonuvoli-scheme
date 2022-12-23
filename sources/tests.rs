

use super::compiler::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::libraries::exports::*;
use super::parser::exports::*;
use super::runtime::exports::*;
use super::transcript::exports::*;
use super::values::exports::*;
use super::values_tests::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
use super::builtins_comparisons::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{compile_tests, compile_test, parse_and_compile_tests};
	pub use super::{execute_tests, execute_test, parse_and_execute_tests};
	pub use super::{benchmark_tests, parse_and_benchmark_tests};
	
	pub use super::{execute_tests_main, benchmark_tests_main};
	
	pub use super::{benchmark_generic, benchmark_generic_main};
	
	pub use super::super::values_tests::exports::{TestCase, TestAction, TestVerbosity};
	
}




def_transcript! (transcript);




#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
type Parameters = !;




#[ derive ( Clone ) ] // OK ??
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub struct TestCaseCompiled {
	expression_without_optimizations : Expression,
	expression_with_optimizations : Expression,
	context_without_optimizations : Option<Context>,
	context_with_optimizations : Option<Context>,
	parameters_without_optimizations : Option<Parameters>,
	parameters_with_optimizations : Option<Parameters>,
	action : TestAction,
	verbosity_without_optimizations : TestVerbosity,
	verbosity_with_optimizations : TestVerbosity,
	source : TestCase,
}




pub fn parse_and_compile_tests (identifier : &str, source : &str, context : Option<&Context>, transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity) -> (Outcome<(StdVec<TestCaseCompiled>)>) {
	let tests = r#try! (parse_tests (source, None));
	return compile_tests (identifier, &tests, context, transcript_backend, verbosity);
}


pub fn compile_tests (identifier : &str, tests : &[TestCase], context_template : Option<&Context>, transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity) -> (Outcome<(StdVec<TestCaseCompiled>)>) {
	
	trace_information! (transcript, 0xb1d307bd => "compiling `{}`..." => (identifier), backend = transcript_backend);
	
	let context_template = if let Some (context) = context_template {
		context.clone ()
	} else {
		let context = Context::new (None);
		r#try! (context.define_all (r#try! (library_r7rs_generate_binding_templates ()) .as_ref ()));
		r#try! (context.define_all_with_prefix (r#try! (library_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
		context
	};
	let (context_without_optimizations, context_with_optimizations) = (context_template.fork (), context_template.fork ());
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let parameters_without_optimization = Some (r#try! (Parameters::new_empty ()));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let parameters_with_optimization = Some (r#try! (Parameters::new_empty ()));
	
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let parameters_without_optimization = None;
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let parameters_with_optimization = None;
	
	let tests = vec_filter! (tests.iter (), test,
		match test.action {
			TestAction::Skip => false,
			_ => true
		});
	
	let tests = try_vec_map_into! (tests, test, compile_test (test, &context_without_optimizations, &context_with_optimizations, parameters_without_optimization.as_ref (), parameters_with_optimization.as_ref (), transcript_backend, verbosity));
	
	succeed! (tests);
}




pub fn parse_and_execute_tests (identifier : &str, source : &str, context : Option<&Context>, transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity) -> (Outcome<()>) {
	let tests = r#try! (parse_and_compile_tests (identifier, source, context, transcript_backend, verbosity));
	return execute_tests (identifier, &tests, transcript_backend, verbosity);
}


pub fn execute_tests (identifier : &str, tests : &[TestCaseCompiled], transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	trace_information! (transcript, 0x450c3e03 => "executing `{}`..." => (identifier), backend = transcript_backend);
	
	let mut tests_succeeded = 0;
	let mut tests_failed = 0;
	let mut tests_error = None;
	
	for test in tests {
		match execute_test (test, transcript_backend, verbosity) {
			Ok (()) =>
				tests_succeeded += 1,
			Err (error) => {
				tests_failed += 1;
				if tests_error.is_none () {
					tests_error = Some (error);
				}
				if TESTS_FAIL_ON_FIRST_ERROR {
					break;
				}
			},
		}
	}
	
	if tests_failed == 0 {
		trace_notice! (transcript, 0xbf6a7cd1 => "executed `{}`: succeeded {};" => (identifier, tests_succeeded), backend = transcript_backend);
	} else {
		if ! TESTS_FAIL_ON_FIRST_ERROR {
			trace_warning! (transcript, 0x89d2290b => "executed `{}`: succeeded {} / failed {};" => (identifier, tests_succeeded, tests_failed), backend = transcript_backend);
		} else {
			trace_warning! (transcript, 0x9f1870a7 => "executed `{}`: aborted on first failure!" => (identifier), backend = transcript_backend);
		}
	}
	
	if let Some (error) = tests_error {
		return Err (error);
	} else {
		succeed! (());
	}
}




pub fn parse_and_benchmark_tests (identifier : &str, source : &str, context : Option<&Context>, bencher : &mut ext::test::Bencher, transcript_backend : &dyn TranscriptBackend, output : &mut dyn io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	let tests = r#try! (parse_and_compile_tests (identifier, source, context, transcript_backend, verbosity));
	return benchmark_tests (identifier, &tests, bencher, transcript_backend, output, verbosity);
}


pub fn benchmark_tests (identifier : &str, tests : &[TestCaseCompiled], bencher : &mut ext::test::Bencher, transcript_backend : &dyn TranscriptBackend, output : &mut dyn io::Write, verbosity : TestVerbosity) -> (Outcome<()>) {
	
	trace_information! (transcript, 0x0930df0d => "benchmarking `{}`..." => (identifier), backend = transcript_backend);
	
	let iterations_base = 20;
	let iterations_warmup = usize::max (iterations_base / 4, 2);
	let iterations_without_optimizations = usize::max (iterations_base / 2, 2);
	let iterations_with_optimizations = usize::max (iterations_base * 2, 4);
	
	let memory_leak_threshold = 128 * 1024;
	let summary_factor = 1.0;
	
	for _ in 0 .. iterations_warmup {
		for test in tests {
			r#try! (execute_test (test, transcript_backend, verbosity));
		}
	}
	
	let (summary_without_optimizations, memory_delta_without_optimizations) =
			r#try! (benchmark_bencher_iterate (bencher, iterations_without_optimizations,
					|| {
						#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
						let parameters = Some (Parameters::new_empty () .expect ("5c37298f"));
						#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
						let parameters = None;
						let evaluator = Evaluator::new (EvaluatorConfiguration::default ());
						let mut evaluation = evaluator.fork (None, parameters);
						for test in tests {
							benchmark_test_without_optimizations (test, &mut evaluation) .expect ("68669f56");
						}
						Ok (())
					}));
	
	let (summary_with_optimizations, memory_delta_with_optimizations) =
			r#try! (benchmark_bencher_iterate (bencher, iterations_with_optimizations,
					|| {
						#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
						let parameters = Some (Parameters::new_empty () .expect ("5326fbea"));
						#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
						let parameters = None;
						let evaluator = Evaluator::new (EvaluatorConfiguration::default ());
						let mut evaluation = evaluator.fork (None, parameters);
						for test in tests {
							benchmark_test_with_optimizations (test, &mut evaluation) .expect ("fffb0313");
						}
						Ok (())
					}));
	
	let memory_leaks_without_optimizations = memory_delta_without_optimizations > memory_leak_threshold;
	let memory_leaks_with_optimizations = memory_delta_with_optimizations > memory_leak_threshold;
	
	trace_notice! (transcript, 0x3748dc5d => "benchmarked `{}`;" => (identifier), backend = transcript_backend);
	if let Some (summary_without_optimizations) = summary_without_optimizations {
		r#try! (benchmark_bencher_report (
				Some ("without optimizations:"), "     ",
				&summary_without_optimizations, None, summary_factor,
				memory_delta_without_optimizations, memory_leaks_without_optimizations,
				transcript_backend, output, verbosity));
	}
	
	if let Some (summary_with_optimizations) = summary_with_optimizations {
		r#try! (benchmark_bencher_report (
				Some ("with optimizations:"), "     ",
				&summary_with_optimizations, summary_without_optimizations.as_ref (), summary_factor,
				memory_delta_with_optimizations, memory_leaks_with_optimizations,
				transcript_backend, output, verbosity));
	}
	
	succeed! (());
}




pub fn benchmark_generic <Setup, Iteration, SetupOutput, IterationOutput> (identifier : &str, setup : Setup, iteration : Iteration, bencher : &mut ext::test::Bencher, transcript_backend : &dyn TranscriptBackend, output : &mut dyn io::Write, verbosity : TestVerbosity) -> (Outcome<()>)
		where
			Setup : Fn () -> (Outcome<SetupOutput>),
			Iteration : Fn (&SetupOutput) -> (Outcome<IterationOutput>)
{
	
	trace_information! (transcript, 0xf25e5c5b => "benchmarking `{}`..." => (identifier), backend = transcript_backend);
	
	let iterations_base = 5;
	let iterations_warmup = iterations_base / 2;
	let iterations_benchmark = iterations_base * 2;
	
	let memory_leak_threshold = 128 * 1024;
	let summary_factor = 1.0;
	
	let setup = r#try! (setup ());
	
	for _ in 0 .. iterations_warmup {
		ext::test::black_box (iteration (&setup)) .expect ("8b1a47ad");
	}
	
	let (summary, memory_delta) = r#try! (benchmark_bencher_iterate (bencher, iterations_benchmark, || iteration (&setup)));
	
	let memory_leaks = memory_delta > memory_leak_threshold;
	
	trace_notice! (transcript, 0xb5993dbd => "benchmarked `{}`;" => (identifier), backend = transcript_backend);
	if let Some (summary) = summary {
		r#try! (benchmark_bencher_report (
				None, "     ",
				&summary, None, summary_factor,
				memory_delta, memory_leaks,
				transcript_backend, output, verbosity));
	}
	
	succeed! (());
}




fn benchmark_bencher_iterate <Iteration, Output> (bencher : &mut ext::test::Bencher, iterations_count : usize, iteration : Iteration) -> (Outcome<(Option<ext::test::stats::Summary>, usize)>)
		where Iteration : Fn () -> (Outcome<Output>)
{
	
	let resources_before = libc_getrusage_for_thread ();
	
	let mut summary_best = None;
	let mut summary_best_median = f64::MAX;
	
	for _ in 0 .. iterations_count {
		
		let summary = bencher.bench (|ref mut bencher| Ok (bencher.iter (&iteration))) .expect ("6824a31d");
		
		if let Some (summary) = summary {
			if summary.median < summary_best_median {
				summary_best_median = summary.median;
				summary_best = Some (summary);
			}
		} else {
			break;
		}
	}
	
	let resources_after = libc_getrusage_for_thread ();
	let memory_delta = (resources_after.ru_maxrss - resources_before.ru_maxrss) as usize;
	
	succeed! ((summary_best, memory_delta));
}


fn benchmark_bencher_report (header : Option<&str>, prefix : &str, summary : &ext::test::stats::Summary, reference : Option<&ext::test::stats::Summary>, factor : f64, memory_delta : usize, memory_leaks : bool, _transcript_backend : &dyn TranscriptBackend, output : &mut dyn io::Write, _verbosity : TestVerbosity) -> (Outcome<()>) {
	let mut report = StdString::new ();
	if let Some (header) = header {
		report.push_str (&format! ("{}{}\n", prefix, header));
	}
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
	if memory_leaks {
		report.push_str (&format! ("{}  mem-leaks : {:10.0} KB (!!!! DETECTED !!!!)\n", prefix, (memory_delta as f64) / 1024.0 * factor));
	} else {
		report.push_str (&format! ("{}  mem-leaks : {:10.0} KB\n", prefix, (memory_delta as f64) / 1024.0 * factor));
	}
	try_or_fail! (output.write_all (report.as_bytes ()), 0x9b631c5f);
	succeed! (());
}




#[ allow (unused_assignments) ]   // NOTE:  For some reason the compiler emits a warning...
pub fn compile_test (test : &TestCase, context_without_optimizations : &Context, context_with_optimizations : &Context, parameters_without_optimizations : Option<&Parameters>, parameters_with_optimizations : Option<&Parameters>, transcript_backend : &dyn TranscriptBackend, verbosity_global : TestVerbosity) -> (Outcome<TestCaseCompiled>) {
	
	let (verbosity_without_optimizations, verbosity_with_optimizations) = match (&test.action, verbosity_global) {
		(&TestAction::Debug, _) |
		(_, TestVerbosity::Debug) =>
			(TestVerbosity::Debug, TestVerbosity::Debug),
		_ =>
			(verbosity_global, verbosity_global),
	};
	let verbosity_generic = verbosity_global;
	
	
	let mut header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_generic, false, false));
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_generic, header_emitted, true));
			trace_internal! (transcript, 0x72ea2b99 => "compiling without optimizations..." => (), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	let expression_without_optimizations = match compile (context_without_optimizations, &test.expression, None) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_without_optimizations, header_emitted, true));
			trace_error! (transcript, 0x495036cb => "failed compiling without optimizations!\u{1e}{:#?}" => (&test.expression), error = &error, backend = transcript_backend);
			error.backtrace_report (tracer_error! (transcript, transcript_backend, 0xdb9b3cac));
			return Err (error);
		},
	};
	
	match verbosity_without_optimizations {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_without_optimizations, header_emitted, true));
			trace_internal! (transcript, 0xebf8f59e => "succeeded compiling without optimizations;\u{1e}{:#?}" => (&expression_without_optimizations), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_generic, header_emitted, true));
			trace_internal! (transcript, 0xe66eecff => "compiling with optimizations..." => (), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	let expression_with_optimizations = match compile (context_with_optimizations, &test.expression, None) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_error! (transcript, 0x1241fb62 => "failed compiling with optimizations:  compilation error!\u{1e}{:#?}" => (&test.expression), error = &error, backend = transcript_backend);
			error.backtrace_report (tracer_error! (transcript, transcript_backend, 0xaec53595));
			return Err (error);
		},
	};
	
	let expression_with_optimizations = match optimize (expression_with_optimizations, None) {
		Ok (expression) =>
			expression,
		Err (error) => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_error! (transcript, 0x4a0986b8 => "failed compiling with optimizations:  optimization error!\u{1e}{:#?}" => (&test.expression), error = &error, backend = transcript_backend);
			error.backtrace_report (tracer_error! (transcript, transcript_backend, 0xe88f6bf0));
			return Err (error);
		},
	};
	
	match verbosity_with_optimizations {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_internal! (transcript, 0xaacf1561 => "succeeded compiling with optimizations;\u{1e}{:#?}" => (&expression_with_optimizations), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	
	r#try! (test_case_footer_emit (test, transcript_backend, verbosity_generic, header_emitted, false));
	
	
	let test = TestCaseCompiled {
			expression_without_optimizations,
			expression_with_optimizations,
			context_without_optimizations : Some (context_without_optimizations.clone ()),
			context_with_optimizations : Some (context_with_optimizations.clone ()),
			parameters_without_optimizations : parameters_without_optimizations.cloned (),
			parameters_with_optimizations : parameters_with_optimizations.cloned (),
			action : test.action.clone (),
			verbosity_without_optimizations,
			verbosity_with_optimizations,
			source : test.clone (),
		};
	
	succeed! (test);
}




#[ allow (unused_assignments) ]   // NOTE:  For some reason the compiler emits a warning...
pub fn execute_test (test : &TestCaseCompiled, transcript_backend : &dyn TranscriptBackend, verbosity_global : TestVerbosity) -> (Outcome<()>) {
	
	let (verbosity_without_optimizations, verbosity_with_optimizations) = match (&test.action, verbosity_global) {
		(&TestAction::Debug, _) |
		(_, TestVerbosity::Debug) =>
			(TestVerbosity::Debug, TestVerbosity::Debug),
		_ =>
			(test.verbosity_without_optimizations, test.verbosity_with_optimizations),
	};
	let verbosity_generic = verbosity_global;
	
	
	let mut header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_generic, false, false));
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_generic, header_emitted, true));
			trace_internal! (transcript, 0xad17507e => "evaluating without optimizations..." => (), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	let output_value_without_optimizations = match evaluate (&test.expression_without_optimizations, test.context_without_optimizations.as_ref (), test.parameters_without_optimizations.as_ref (), None) {
		Ok (output_value) =>
			output_value,
		Err (error) => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_without_optimizations, header_emitted, true));
			trace_error! (transcript, 0xd70f287e => "failed evaluating without optimizations!\u{1e}{:#?}" => (&test.expression_without_optimizations), error = &error, backend = transcript_backend);
			error.backtrace_report (tracer_error! (transcript, transcript_backend, 0x78002618));
			return Err (error);
		},
	};
	
	match verbosity_without_optimizations {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_without_optimizations, header_emitted, true));
			trace_internal! (transcript, 0x4e1189d7 => "succeeded evaluating without optimizations;\u{1e}{:#?}" => (&output_value_without_optimizations), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	
	match verbosity_generic {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_generic, header_emitted, true));
			trace_internal! (transcript, 0xecf07fc4 => "evaluating with optimizations..." => (), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	let output_value_with_optimizations = match evaluate (&test.expression_with_optimizations, test.context_with_optimizations.as_ref (), test.parameters_with_optimizations.as_ref (), None) {
		Ok (output_value) =>
			output_value,
		Err (error) => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_error! (transcript, 0x4dfaa3fd => "failed evaluating with optimizations!\u{1e}{:#?}" => (&test.expression_with_optimizations), error = &error, backend = transcript_backend);
			error.backtrace_report (tracer_error! (transcript, transcript_backend, 0x430cfc58));
			return Err (error);
		},
	};
	
	match verbosity_with_optimizations {
		TestVerbosity::Debug => {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_internal! (transcript, 0xa410b78a => "succeeded evaluating with optimizations!\u{1e}{:#?}" => (&output_value_with_optimizations), backend = transcript_backend);
		},
		_ =>
			(),
	}
	
	
	let expected_value_without_optimizations = match test.action {
		TestAction::Expect (ref expected_expression) => {
			TODO! ("add error reporting for these");
			let context = try_some_ref! (test.context_without_optimizations, 0xa65fb508);
			let expected_expression = r#try! (compile (context, expected_expression, None));
			let expected_value = r#try! (evaluate (&expected_expression, test.context_without_optimizations.as_ref (), test.parameters_without_optimizations.as_ref (), None));
			Some (expected_value)
		},
		_ =>
			None,
	};
	
	if let Some (ref expected_value) = expected_value_without_optimizations {
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		let output_matched = r#try! (equivalent_by_value_strict_recursive_2 (&output_value_without_optimizations, expected_value, false));
		#[ cfg ( not ( feature = "vonuvoli_builtins_comparisons" ) ) ]
		let output_matched = Value::eq (&output_value_without_optimizations, expected_value);
		if !output_matched {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_without_optimizations, header_emitted, true));
			trace_error! (transcript, 0xdcdf61a8 => "failed assertion without optimizations!\u{1e}{0}\u{1e}{1}\u{1e}{0:#?}\u{1e}{1:#?}" => (&output_value_without_optimizations, expected_value), backend = transcript_backend);
			fail! (0x67419241);
		}
	}
	
	
	let expected_value_with_optimizations = match test.action {
		TestAction::Expect (ref expected_expression) => {
			TODO! ("add error reporting for these");
			let context = try_some_ref! (test.context_with_optimizations, 0x0042a4ed);
			let expected_expression = r#try! (compile (context, expected_expression, None));
			let expected_value = r#try! (evaluate (&expected_expression, test.context_with_optimizations.as_ref (), test.parameters_with_optimizations.as_ref (), None));
			Some (expected_value)
		},
		_ =>
			None,
	};
	
	if let Some (ref expected_value) = expected_value_with_optimizations {
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		let output_matched = r#try! (equivalent_by_value_strict_recursive_2 (&output_value_with_optimizations, expected_value, false));
		#[ cfg ( not ( feature = "vonuvoli_builtins_comparisons" ) ) ]
		let output_matched = Value::eq (&output_value_with_optimizations, expected_value);
		if !output_matched {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_with_optimizations, header_emitted, true));
			trace_error! (transcript, 0xb66640e5 => "failed assertion with optimizations!\u{1e}{0}\u{1e}{1}\u{1e}{0:#?}\u{1e}{1:#?}" => (&output_value_with_optimizations, expected_value), backend = transcript_backend);
			fail! (0xe52ddb4f);
		}
	}
	
	
	if match (expected_value_without_optimizations.as_ref (), &expected_value_with_optimizations.as_ref ()) {
		(None, None) =>
			false,
		(Some (expected_value_without_optimizations), Some (expected_value_with_optimizations)) =>
			match Value::kind_match_as_ref_2 (expected_value_without_optimizations, expected_value_with_optimizations) {
				
				ValueKindMatchAsRef2::Null |
				ValueKindMatchAsRef2::Void |
				ValueKindMatchAsRef2::Undefined |
				ValueKindMatchAsRef2::Singleton (_) |
				ValueKindMatchAsRef2::Boolean (_, _) |
				ValueKindMatchAsRef2::NumberInteger (_, _) |
				ValueKindMatchAsRef2::NumberReal (_, _) |
				ValueKindMatchAsRef2::Symbol (_, _) |
				ValueKindMatchAsRef2::PairImmutable (_, _) |
				ValueKindMatchAsRef2::ProcedurePrimitive (_, _) |
				ValueKindMatchAsRef2::SyntaxPrimitive (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				ValueKindMatchAsRef2::PairMutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ValueKindMatchAsRef2::Character (_, _) |
				ValueKindMatchAsRef2::StringImmutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				ValueKindMatchAsRef2::StringMutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ValueKindMatchAsRef2::BytesImmutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				ValueKindMatchAsRef2::BytesMutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ValueKindMatchAsRef2::ArrayImmutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				ValueKindMatchAsRef2::ArrayMutable (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ValueKindMatchAsRef2::Values (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_native" ) ]
				ValueKindMatchAsRef2::ProcedureNative (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_expressions" ) ]
				#[ cfg ( feature = "vonuvoli_compiler" ) ]
				#[ cfg ( feature = "vonuvoli_values_native" ) ]
				ValueKindMatchAsRef2::SyntaxNative (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ValueKindMatchAsRef2::Keyword (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ValueKindMatchAsRef2::StringRegex (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ValueKindMatchAsRef2::BytesRegex (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ValueKindMatchAsRef2::Path (_, _) =>
					true,
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				ValueKindMatchAsRef2::Error (_, _) =>
					true,
				
				#[ cfg ( feature = "vonuvoli_expressions" ) ]
				#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
				ValueKindMatchAsRef2::ProcedureLambda (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_expressions" ) ]
				#[ cfg ( feature = "vonuvoli_compiler" ) ]
				#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
				ValueKindMatchAsRef2::SyntaxLambda (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_values_extended" ) ]
				ValueKindMatchAsRef2::ProcedureExtended (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_expressions" ) ]
				#[ cfg ( feature = "vonuvoli_compiler" ) ]
				#[ cfg ( feature = "vonuvoli_values_extended" ) ]
				ValueKindMatchAsRef2::SyntaxExtended (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ValueKindMatchAsRef2::Unique (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ValueKindMatchAsRef2::RecordKind (_, _) |
				ValueKindMatchAsRef2::RecordImmutable (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				ValueKindMatchAsRef2::RecordMutable (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
				ValueKindMatchAsRef2::Context (_, _) |
				ValueKindMatchAsRef2::Binding (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
				ValueKindMatchAsRef2::Parameters (_, _) |
				ValueKindMatchAsRef2::Parameter (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
				ValueKindMatchAsRef2::Promise (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ValueKindMatchAsRef2::Port (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
				ValueKindMatchAsRef2::Process (_, _) =>
					false,
				#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
				ValueKindMatchAsRef2::Opaque (_, _) =>
					false,
				
				ValueKindMatchAsRef2::Missmatched =>
					fail! (0x670c12cb),
				
			},
		(_, _) =>
			fail_panic! (0x2f1f97f3, github_issue_new),
	} {
		let expected_value_without_optimizations = try_some_or_panic! (expected_value_without_optimizations.as_ref (), 0x3d5461ef, github_issue_new);
		let expected_value_with_optimizations = try_some_or_panic! (expected_value_with_optimizations.as_ref (), 0x150d106d, github_issue_new);
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		let output_matched = r#try! (equivalent_by_value_strict_recursive_2 (expected_value_without_optimizations, expected_value_with_optimizations, false));
		#[ cfg ( not ( feature = "vonuvoli_builtins_comparisons" ) ) ]
		let output_matched = Value::eq (expected_value_without_optimizations, expected_value_with_optimizations);
		if !output_matched {
			header_emitted = r#try! (test_case_header_emit (&test.source, transcript_backend, verbosity_generic, header_emitted, true));
			trace_error! (transcript, 0xe003610f => "failed assertion between with and without optimizations!\u{1e}{0}\u{1e}{1}\u{1e}{0:#?}\u{1e}{1:#?}" => (expected_value_without_optimizations, expected_value_with_optimizations), backend = transcript_backend);
			fail! (0xc8a2813a);
		}
	}
	
	r#try! (test_case_footer_emit (&test.source, transcript_backend, verbosity_global, header_emitted, false));
	
	succeed! (());
}




pub(crate) fn benchmark_test_without_optimizations (test : &TestCaseCompiled, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	
	r#try! (evaluator.evaluate (&test.expression_without_optimizations));
	
	succeed! (());
}

pub(crate) fn benchmark_test_with_optimizations (test : &TestCaseCompiled, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	
	r#try! (evaluator.evaluate (&test.expression_with_optimizations));
	
	succeed! (());
}




fn test_case_header_emit (test : &TestCase, transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity, emitted : bool, forced : bool) -> (Outcome<bool>) {
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
		TODO! ("replace the following with a transcript cut");
		// trace_internal! (transcript, 0xccf80728 => "--------------------------------------------------------------------------------" => (), backend = transcript_backend);
		match test.action {
			TestAction::Expect (ref expected) =>
				trace_internal! (transcript, 0xf1bf16d3 => ">> {} => {}" => (&test.expression, expected), backend = transcript_backend),
			TestAction::Debug | TestAction::Ignore | TestAction::Skip =>
				trace_internal! (transcript, 0x01fdbf40 => ">> {} => #ignore" => (&test.expression), backend = transcript_backend),
		}
		succeed! (true);
	} else {
		succeed! (false);
	}
}


fn test_case_footer_emit (test : &TestCase, transcript_backend : &dyn TranscriptBackend, verbosity : TestVerbosity, emitted : bool, forced : bool) -> (Outcome<bool>) {
	let emitted = r#try! (test_case_header_emit (test, transcript_backend, verbosity, emitted, forced));
	if emitted {
		TODO! ("replace the following with a transcript cut");
		// trace_internal! (transcript, 0x3d63834c => "--------------------------------------------------------------------------------" => (), backend = transcript_backend);
	}
	succeed! (emitted);
}




pub fn execute_tests_main (identifier : &str, source : &str, context : Option<&Context>, transcript_backend : Option<&dyn TranscriptBackend>, verbosity : Option<TestVerbosity>) -> (Outcome<()>) {
	
	let transcript_backend = if let Some (transcript_backend) = transcript_backend { transcript_backend } else { transcript.backend () };
	let verbosity = if let Some (verbosity) = verbosity { verbosity } else {
		let verbosity = env::var ("VONUVOLI_SCHEME_TESTS_DEBUG") .unwrap_or_else (|_| string::String::from ("false"));
		let verbosity = if verbosity == "true" { TestVerbosity::Debug } else { TestVerbosity::Quiet };
		verbosity
	};
	
	let outcome = parse_and_execute_tests (identifier, source, context, transcript_backend, verbosity);
	
	return outcome;
}




pub fn benchmark_tests_main (identifier : &str, source : &str, context : Option<&Context>, bencher : Option<&mut ext::test::Bencher>, transcript_backend : Option<&dyn TranscriptBackend>, output : Option<&mut dyn io::Write>, verbosity : Option<TestVerbosity>) -> (Outcome<()>) {
	benchmark_main (
			identifier,
			|identifier, bencher, transcript_backend, output, verbosity|
					parse_and_benchmark_tests (identifier, source, context, bencher, transcript_backend, output, verbosity),
			bencher, transcript_backend, output, verbosity)
}


pub fn benchmark_generic_main <Setup, Iteration, SetupOutput, IterationOutput> (identifier : &str, setup : Setup, iteration : Iteration, bencher : Option<&mut ext::test::Bencher>, transcript_backend : Option<&dyn TranscriptBackend>, output : Option<&mut dyn io::Write>, verbosity : Option<TestVerbosity>) -> (Outcome<()>)
		where
			Setup : Fn () -> (Outcome<SetupOutput>),
			Iteration : Fn (&SetupOutput) -> (Outcome<IterationOutput>)
{
	benchmark_main (
			identifier,
			|identifier, bencher, transcript_backend, output, verbosity|
					benchmark_generic (identifier, &setup, &iteration, bencher, transcript_backend, output, verbosity),
			bencher, transcript_backend, output, verbosity)
}


pub(crate) fn benchmark_main <Benchmark> (identifier : &str, benchmark : Benchmark, bencher : Option<&mut ext::test::Bencher>, transcript_backend : Option<&dyn TranscriptBackend>, output : Option<&mut dyn io::Write>, verbosity : Option<TestVerbosity>) -> (Outcome<()>)
		where Benchmark : Fn (&str, &mut ext::test::Bencher, &dyn TranscriptBackend, &mut dyn io::Write, TestVerbosity) -> (Outcome<()>)
{
	
	let transcript_backend = if let Some (transcript_backend) = transcript_backend { transcript_backend } else { transcript.backend () };
	let verbosity = if let Some (verbosity) = verbosity { verbosity } else {
		let verbosity = env::var ("VONUVOLI_SCHEME_BENCHMARKS_DEBUG") .unwrap_or_else (|_| string::String::from ("false"));
		let verbosity = if verbosity == "true" { TestVerbosity::Debug } else { TestVerbosity::Quiet };
		verbosity
	};
	
	let (output, output_backend) = if let Some (output) = output {
		(Some (output), None)
	} else {
		let output = env::var ("VONUVOLI_SCHEME_BENCHMARKS_OUTPUT") .ok ();
		let output = if let Some (output) = output { Some (output.replace ("{IDENTIFIER}", identifier)) } else { None };
		if let Some (output) = output {
			let output = try_or_fail! (fs::File::create (output), 0x25b456ed);
			(None, Some (output))
		} else {
			(None, None)
		}
	};
	let mut output_backend = output_backend;
	let output : Option<&mut dyn io::Write> = if let Some (ref mut output) = output_backend { Some (output) } else { output };
	
	let (bencher, mut bencher_backend) = if let Some (bencher) = bencher {
		(Some (bencher), None)
	} else {
		let bencher = unsafe { mem::zeroed::<ext::test::Bencher> () };
		(None, Some (bencher))
	};
	let bencher = match (bencher, &mut bencher_backend) {
		(_, &mut Some (ref mut bencher)) =>
			bencher,
		(Some (bencher), _) =>
			bencher,
		_ =>
			fail_panic! (0x5e76028c, github_issue_new),
	};
	
	let mut output_buffer = StdVec::with_capacity (1024);
	r#try! (benchmark (identifier, bencher, transcript_backend, &mut output_buffer, verbosity));
	
	let output_buffer = try_or_fail! (StdString::from_utf8 (output_buffer), 0x48004fa7);
	
	if ! output_buffer.is_empty () {
		
		trace_information! (transcript, 0x1643f2b2, message = &output_buffer, backend = transcript_backend);
		
		if let Some (output) = output {
			try_or_fail! (output.write_all (output_buffer.as_bytes ()), 0x41e00f08);
		}
	}
	
	succeed! (());
}

