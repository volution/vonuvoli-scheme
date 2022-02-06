

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Parameters, ParametersInternals};
	pub use super::{Parameter, ParameterInternals, ParameterConversion};
	pub use super::{parameter_resolve_value};
}




#[ derive ( Clone ) ] // OK
pub struct Parameters ( StdRc<StdRefCell<ParametersInternals>> );


pub struct ParametersInternals {
	pub bindings : StdMap<UniqueFingerprint, Option<(Binding, ParameterConversion)>>,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub stdin : Option<Port>,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub stdout : Option<Port>,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub stderr : Option<Port>,
	pub process_arguments : Option<StdRc<StdBox<[StdBox<ffi::OsStr>]>>>,
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::type_complexity) ) ]
	pub process_environment : Option<StdRc<StdBox<[(StdBox<ffi::OsStr>, StdBox<ffi::OsStr>)]>>>,
	#[ cfg ( feature = "blake2-rfc" ) ]
	pub process_environment_fingerprint : Option<StdRc<StdBox<[u8]>>>,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	pub transcript : StdRc<TranscriptForScript>,
	pub parent : Option<Parameters>,
	pub immutable : bool,
	pub handle : Handle,
}


impl Parameters {
	
	
	pub fn new_empty () -> (Outcome<Parameters>) {
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdin : None,
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdout : None,
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stderr : None,
				process_arguments : None,
				process_environment : None,
				#[ cfg ( feature = "blake2-rfc" ) ]
				process_environment_fingerprint : None,
				#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
				transcript : r#try! (transcript_for_script ()),
				parent : None,
				immutable : false,
				handle : parameters_handles_next (),
			};
		succeed! (Parameters (StdRc::new (StdRefCell::new (internals))));
	}
	
	pub fn new_standard <ProcessArguments, ProcessEnvironment> (process_arguments : ProcessArguments, process_environment : ProcessEnvironment) -> (Outcome<Parameters>)
			where
				ProcessArguments : iter::IntoIterator<Item = ffi::OsString>,
				ProcessEnvironment : iter::IntoIterator<Item = (ffi::OsString, ffi::OsString)>,
	{
		let mut process_environment = StdVec::from_iter (process_environment);
		process_environment.sort ();
		
		#[ cfg ( feature = "blake2-rfc" ) ]
		let process_environment_fingerprint = {
			let mut hasher = ext::blake2_rfc::blake2b::Blake2b::new (PROCESS_ENVIRONMENT_FINGERPRINT_SIZE);
			for &(ref name, ref value) in &process_environment {
				hasher.update (& unsafe { mem::transmute::<_, [u8; 8]> (name.len ()) });
				hasher.update (name.as_bytes ());
				hasher.update (& unsafe { mem::transmute::<_, [u8; 8]> (value.len ()) });
				hasher.update (value.as_bytes ());
			}
			let hash = hasher.finalize ();
			let hash = StdVec::from (hash.as_bytes ());
			let hash = hash.into_boxed_slice ();
			hash
		};
		
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdin : Some (r#try! (Port::new_stdin ())),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdout : Some (r#try! (Port::new_stdout ())),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stderr : Some (r#try! (Port::new_stderr ())),
				process_arguments : Some (StdRc::new (vec_map_into! (process_arguments, value, value.into_boxed_os_str ()) .into_boxed_slice ())),
				process_environment : Some (StdRc::new (vec_map_into! (process_environment, (name, value), (name.into_boxed_os_str (), value.into_boxed_os_str ())) .into_boxed_slice ())),
				#[ cfg ( feature = "blake2-rfc" ) ]
				process_environment_fingerprint : Some (StdRc::new (process_environment_fingerprint)),
				#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
				transcript : r#try! (transcript_for_script ()),
				parent : None,
				immutable : false,
				handle : parameters_handles_next (),
			};
		succeed! (Parameters (StdRc::new (StdRefCell::new (internals))));
	}
	
	pub fn fork (&self) -> (Outcome<Parameters>) {
		let self_0 = r#try! (self.internals_ref ());
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdin : option_ref_map! (self_0.stdin, port, port.clone ()),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stdout : option_ref_map! (self_0.stdout, port, port.clone ()),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				stderr : option_ref_map! (self_0.stderr, port, port.clone ()),
				process_arguments : option_ref_map! (self_0.process_arguments, rc, StdRc::clone (rc)),
				process_environment : option_ref_map! (self_0.process_environment, rc, StdRc::clone (rc)),
				#[ cfg ( feature = "blake2-rfc" ) ]
				process_environment_fingerprint : option_ref_map! (self_0.process_environment_fingerprint, rc, StdRc::clone (rc)),
				#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
				transcript : StdRc::clone (&self_0.transcript),
				parent : Some (self.clone ()),
				immutable : false,
				handle : parameters_handles_next (),
			};
		succeed! (Parameters (StdRc::new (StdRefCell::new (internals))));
	}
	
	
	pub fn resolve (&self, parameter : &Parameter, default : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
		let key = & r#try! (parameter.unique_ref ()) .fingerprint ();
		match r#try! (self.resolve_0 (key, Some (parameter), evaluator)) {
			Some (value) =>
				succeed! (value),
			None =>
				if let Some (default) = default {
					succeed! (default.clone ());
				} else {
					succeed! (UNDEFINED_VALUE);
				},
		}
	}
	
	pub fn resolve_for_builtin (&self, parameter : &UniqueData, evaluator : &mut EvaluatorContext) -> (Outcome<Option<Value>>) {
		if parameter.kind != UniqueKind::ParameterIdentity {
			fail! (0x293d378f);
		}
		let key = &parameter.fingerprint;
		return self.resolve_0 (key, None, evaluator);
	}
	
	fn resolve_0 (&self, key : &UniqueFingerprint, parameter : Option<&Parameter>, evaluator : &mut EvaluatorContext) -> (Outcome<Option<Value>>) {
		loop {
			match r#try! (self.resolve_self (key, evaluator)) {
				Some (value) => {
					succeed! (value);
				},
				None => {
					r#try! (self.resolve_or_cache (key, parameter, evaluator));
				},
			}
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
	fn resolve_self (&self, key : &UniqueFingerprint, evaluator : &mut EvaluatorContext) -> (Outcome<Option<Option<Value>>>) {
		let self_0 = r#try! (self.internals_ref ());
		match self_0.bindings.get (key) {
			Some (& Some ((ref binding, ref conversion))) => {
				let value = r#try! (binding.get ());
				let value = match *conversion {
					ParameterConversion::None =>
						value,
					ParameterConversion::OnConfigure (_) =>
						fail_panic! (0x0f94930a, github_issue_new),
					ParameterConversion::OnResolveOnce (_) =>
						fail_panic! (0x19d8c728, github_issue_new),
					ParameterConversion::OnResolveAlways (ref converter) =>
						r#try! (evaluator.evaluate_procedure_call_1 (converter, &value)),
				};
				succeed! (Some (Some (value)));
			},
			Some (& None) =>
				succeed! (Some (None)),
			None =>
				succeed! (None),
		}
	}
	
	fn resolve_or_cache (&self, key : &UniqueFingerprint, parameter : Option<&Parameter>, evaluator : &mut EvaluatorContext) -> (Outcome<Option<(Value, ParameterConversion)>>) {
		let mut self_0 = r#try! (self.internals_ref_mut ());
		match self_0.bindings.get (key) {
			Some (& Some ((ref binding, ref conversion))) => {
				let value = r#try! (binding.get ());
				succeed! (Some ((value, conversion.clone ())));
			},
			Some (& None) =>
				succeed! (None),
			None =>
				(),
		}
		let value_and_conversion = if let Some (ref parent) = self_0.parent {
			r#try! (parent.resolve_or_cache (key, parameter, evaluator))
		} else {
			if let Some (parameter) = parameter {
				r#try! (parameter.new_conversion (None, evaluator, true))
			} else {
				None
			}
		};
		match self_0.bindings.entry (key.clone ()) {
			StdMapEntry::Occupied (_) =>
				fail_unreachable! (0x06fa511f, github_issue_new),
			StdMapEntry::Vacant (entry) => {
				if let Some ((ref value, ref conversion)) = value_and_conversion {
					let binding = if let Some (parameter) = parameter {
						r#try! (parameter.new_binding ())
					} else {
						fail_unreachable! (0x3a0cd886, github_issue_new);
					};
					r#try! (binding.initialize (value.clone ()));
					entry.insert (Some ((binding, conversion.clone ())));
				} else {
					entry.insert (None);
				}
				succeed! (value_and_conversion);
			},
		}
	}
	
	
	pub fn configure (&self, parameter : &Parameter, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
		let key = & r#try! (parameter.unique_ref ()) .fingerprint ();
		let mut self_0 = r#try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xce261293);
		}
		match self_0.bindings.entry (key.clone ()) {
			StdMapEntry::Occupied (entry) => {
				let entry = entry.into_mut ();
				if let Some ((value, conversion_1)) = r#try! (parameter.new_conversion (Some (value), evaluator, false)) {
					match *entry {
						Some ((ref binding, ref mut conversion)) => {
							r#try! (binding.set (value));
							*conversion = conversion_1;
						},
						None => {
							let binding = r#try! (parameter.new_binding ());
							r#try! (binding.initialize (value));
							*entry = Some ((binding, conversion_1));
						}
					}
				} else {
					*entry = None;
				}
			},
			StdMapEntry::Vacant (entry) => {
				if let Some ((value, conversion)) = r#try! (parameter.new_conversion (Some (value), evaluator, true)) {
					let binding = r#try! (parameter.new_binding ());
					r#try! (binding.initialize (value));
					entry.insert (Some ((binding, conversion)));
				} else {
					entry.insert (None);
				}
			},
		}
		succeed! (());
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdin (&self) -> (Outcome<Port>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stdin, 0x158c7282) .clone ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdout (&self) -> (Outcome<Port>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stdout, 0x8133bc6b) .clone ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stderr (&self) -> (Outcome<Port>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stderr, 0xb4037a1a) .clone ());
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn configure_stdin (&self, port : &Port) -> (Outcome<()>) {
		TODO! ("find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports");
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0x27f16faa);
		//}
		let mut self_0 = r#try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0x2521a116);
		}
		self_0.stdin = Some (port.clone ());
		succeed! (());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn configure_stdout (&self, port : &Port) -> (Outcome<()>) {
		TODO! ("find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports");
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0xf78642ab);
		//}
		let mut self_0 = r#try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xa5dd23e6);
		}
		self_0.stdout = Some (port.clone ());
		succeed! (());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn configure_stderr (&self, port : &Port) -> (Outcome<()>) {
		TODO! ("find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports");
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0x5443078f);
		//}
		let mut self_0 = r#try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0x93818ef2);
		}
		self_0.stderr = Some (port.clone ());
		succeed! (());
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdin_value (&self) -> (Outcome<Value>) {
		succeed! (r#try! (self.resolve_stdin ()) .clone () .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdout_value (&self) -> (Outcome<Value>) {
		succeed! (r#try! (self.resolve_stdout ()) .clone () .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stderr_value (&self) -> (Outcome<Value>) {
		succeed! (r#try! (self.resolve_stderr ()) .clone () .into ());
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdin_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = r#try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stdin, default);
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stdout_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = r#try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stdout, default);
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub fn resolve_stderr_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = r#try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stderr, default);
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	fn resolve_port_value_or (port : &Option<Port>, default : Option<&Value>) -> (Outcome<Value>) {
		if let Some (ref port) = *port {
			succeed! (port.clone () .into ());
		} else if let Some (default) = default {
			succeed! (default.clone ());
		} else {
			succeed! (UNDEFINED_VALUE);
		}
	}
	
	
	pub fn resolve_process_arguments (&self) -> (Outcome<StdRc<StdBox<[StdBox<ffi::OsStr>]>>>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (StdRc::clone (try_some! (self_0.process_arguments.as_ref (), 0x3dcd4501)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::type_complexity) ) ]
	pub fn resolve_process_environment (&self) -> (Outcome<StdRc<StdBox<[(StdBox<ffi::OsStr>, StdBox<ffi::OsStr>)]>>>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (StdRc::clone (try_some! (self_0.process_environment.as_ref (), 0xa4f5a1a9)));
	}
	
	#[ cfg ( feature = "blake2-rfc" ) ]
	pub fn resolve_process_environment_fingerprint (&self) -> (Outcome<StdRc<StdBox<[u8]>>>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (StdRc::clone (try_some! (self_0.process_environment_fingerprint.as_ref (), 0xfd5e1226)));
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	pub fn resolve_transcript (&self) -> (Outcome<StdRc<TranscriptForScript>>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (StdRc::clone (&self_0.transcript));
	}
	
	
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = r#try! (self.internals_ref_mut ());
		self_0.immutable = true;
		succeed! (());
	}
	
	
	pub fn internals_ref (&self) -> (Outcome<StdRef<ParametersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x01545833));
	}
	
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<ParametersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0x37409eea));
	}
	
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	pub fn is_self (&self, other : &Parameters) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
}




#[ derive ( Clone ) ] // OK
pub struct Parameter ( StdRc<ParameterInternals> );


pub struct ParameterInternals {
	pub identifier : Option<Symbol>,
	pub global : Option<Binding>,
	pub conversion : ParameterConversion,
	pub immutable : bool,
	pub handle : Handle,
	pub unique : Unique,
}


#[ derive ( Clone ) ] // OK ??
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub enum ParameterConversion {
	None,
	OnConfigure (Value),
	OnResolveOnce (Value),
	OnResolveAlways (Value),
}


impl Parameter {
	
	
	pub fn new (identifier : Option<Symbol>, global : Option<Value>, conversion : ParameterConversion, immutable : bool) -> (Parameter) {
		let global = option_map! (global, Binding::new (identifier.clone (), Some (global), true));
		let handle = parameter_handles_next ();
		let internals = ParameterInternals {
				identifier : identifier,
				global : global,
				conversion : conversion,
				immutable : immutable,
				handle : handle,
				unique : UniqueData::for_parameter (handle) .into (),
			};
		return Parameter (StdRc::new (internals));
	}
	
	
	pub fn for_builtin (identifier : Symbol, handle : u32, immutable : bool) -> (Parameter) {
		let handle = Handle::for_builtin (handle);
		let internals = ParameterInternals {
				identifier : Some (identifier),
				global : None,
				conversion : ParameterConversion::None,
				immutable : immutable,
				handle : handle,
				unique : UniqueData::for_parameter (handle) .into (),
			};
		return Parameter (StdRc::new (internals));
	}
	
	
	pub fn new_binding (&self) -> (Outcome<Binding>) {
		let self_0 = r#try! (self.internals_ref ());
		let binding = Binding::new (self_0.identifier.clone (), None, self_0.immutable);
		succeed! (binding);
	}
	
	
	pub fn new_conversion (&self, value : Option<&Value>, evaluator : &mut EvaluatorContext, is_initialization : bool) -> (Outcome<Option<(Value, ParameterConversion)>>) {
		let self_0 = r#try! (self.internals_ref ());
		if ! is_initialization && self_0.immutable {
			fail! (0x4419c0cc);
		}
		let (is_configuration, value) = if let Some (value) = value {
			(true, value.clone ())
		} else if let Some (ref global) = self_0.global {
			(false, r#try! (global.get ()))
		} else {
			succeed! (None);
		};
		let value_and_conversion = match self_0.conversion {
			ParameterConversion::None =>
				(value, ParameterConversion::None),
			ParameterConversion::OnConfigure (ref converter) =>
				if is_configuration {
					let value = r#try! (evaluator.evaluate_procedure_call_1 (converter, &value));
					(value, ParameterConversion::None)
				} else {
					(value, ParameterConversion::None)
				},
			ParameterConversion::OnResolveOnce (ref converter) => {
				let value = r#try! (evaluator.evaluate_procedure_call_1 (converter, &value));
				(value, ParameterConversion::None)
			},
			ParameterConversion::OnResolveAlways (_) =>
				(value, self_0.conversion.clone ()),
		};
		succeed! (Some (value_and_conversion));
	}
	
	
	pub fn identifier (&self) -> (Outcome<Option<Symbol>>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (self_0.identifier.clone ());
	}
	
	
	pub fn is_immutable (&self) -> (Outcome<bool>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (self_0.immutable);
	}
	
	
	pub fn internals_ref (&self) -> (Outcome<&ParameterInternals>) {
		succeed! (StdRc::as_ref (&self.0));
	}
	
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	pub fn is_self (&self, other : &Parameter) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
	
	pub fn unique_ref (&self) -> (Outcome<&Unique>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (&self_0.unique);
	}
	
	pub fn unique (&self) -> (Outcome<Unique>) {
		let self_0 = r#try! (self.internals_ref ());
		succeed! (self_0.unique.clone ());
	}
}




TODO! ("rename and move this");
pub fn parameter_resolve_value (option : Option<Value>, parameter : &UniqueData, evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<Option<Value>>) {
	if let Some (option) = option {
		succeed! (Some (option))
	} else if let Some (ref mut evaluator) = *evaluator {
		evaluator.parameter_resolve_for_builtin (parameter)
	} else {
		succeed! (None)
	}
}




const PROCESS_ENVIRONMENT_FINGERPRINT_SIZE : usize = 256 / 8;

