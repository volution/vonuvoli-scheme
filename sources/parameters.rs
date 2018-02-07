

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Parameters, ParametersInternals};
	pub use super::{Parameter, ParameterInternals, ParameterConversion};
}




#[ derive (Clone) ]
pub struct Parameters ( StdRc<StdRefCell<ParametersInternals>> );


#[ derive (Clone, Debug) ]
pub struct ParametersInternals {
	pub bindings : StdMap<Unique, (Binding, ParameterConversion)>,
	pub stdin : Option<Port>,
	pub stdout : Option<Port>,
	pub stderr : Option<Port>,
	pub parent : Option<Parameters>,
	pub immutable : bool,
	pub handle : Handle,
}


impl Parameters {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_empty () -> (Parameters) {
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				stdin : None,
				stdout : None,
				stderr : None,
				parent : None,
				immutable : false,
				handle : parameters_handles_next (),
			};
		return Parameters (StdRc::new (StdRefCell::new (internals)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_standard () -> (Outcome<Parameters>) {
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				stdin : Some (try! (Port::new_stdin ())),
				stdout : Some (try! (Port::new_stdout ())),
				stderr : Some (try! (Port::new_stderr ())),
				parent : None,
				immutable : false,
				handle : parameters_handles_next (),
			};
		succeed! (Parameters (StdRc::new (StdRefCell::new (internals))));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork (&self) -> (Outcome<Parameters>) {
		let self_0 = try! (self.internals_ref ());
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				stdin : option_ref_map! (self_0.stdin, port, port.clone ()),
				stdout : option_ref_map! (self_0.stdout, port, port.clone ()),
				stderr : option_ref_map! (self_0.stderr, port, port.clone ()),
				parent : Some (self.clone ()),
				immutable : false,
				handle : parameters_handles_next (),
			};
		succeed! (Parameters (StdRc::new (StdRefCell::new (internals))));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve (&self, parameter : &Parameter, default : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
		let key = try! (parameter.unique_ref ());
		if let Some (default) = default {
			return self.resolve_or_default (key, parameter, default, evaluator);
		}
		loop {
			if let Some (value) = try! (self.resolve_self (key, evaluator)) {
				succeed! (value);
			} else {
				try! (self.resolve_or_cache (key, parameter, evaluator));
			}
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn resolve_self (&self, key : &Unique, evaluator : &mut EvaluatorContext) -> (Outcome<Option<Value>>) {
		let self_0 = try! (self.internals_ref ());
		match self_0.bindings.get (key) {
			Some (&(ref binding, ref conversion)) => {
				let value = try! (binding.get ());
				let value = match *conversion {
					ParameterConversion::None =>
						value,
					ParameterConversion::OnConfigure (_) =>
						fail_panic! (0x0f94930a),
					ParameterConversion::OnResolveOnce (_) =>
						fail_panic! (0x19d8c728),
					ParameterConversion::OnResolveAlways (ref converter) =>
						try! (evaluator.evaluate_procedure_call_1 (converter, &value)),
				};
				succeed! (Some (value));
			},
			None =>
				succeed! (None),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn resolve_or_default (&self, key : &Unique, parameter : &Parameter, default : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
		if let Some (value) = try! (self.resolve_self (key, evaluator)) {
			succeed! (value);
		}
		let self_0 = try! (self.internals_ref ());
		if let Some (ref parent) = self_0.parent {
			return parent.resolve_or_default (key, parameter, default, evaluator);
		} else {
			succeed! (default.clone ());
		};
	}
	
	#[ inline (never) ]
	fn resolve_or_cache (&self, key : &Unique, parameter : &Parameter, evaluator : &mut EvaluatorContext) -> (Outcome<(Value, ParameterConversion)>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		match self_0.bindings.get (key) {
			Some (&(ref binding, ref conversion)) => {
				let value = try! (binding.get ());
				succeed! ((value, conversion.clone ()));
			},
			None =>
				(),
		}
		let (value, conversion) = if let Some (ref parent) = self_0.parent {
			try! (parent.resolve_or_cache (key, parameter, evaluator))
		} else {
			try! (parameter.new_conversion (None, evaluator))
		};
		match self_0.bindings.entry (key.clone ()) {
			StdMapEntry::Occupied (_) =>
				fail_unreachable! (0x06fa511f),
			StdMapEntry::Vacant (entry) => {
				let binding = try! (parameter.new_binding ());
				try! (binding.initialize (value.clone ()));
				entry.insert ((binding, conversion.clone ()));
				succeed! ((value, conversion));
			},
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configure (&self, parameter : &Parameter, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
		let key = try! (parameter.unique_ref ());
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xce261293);
		}
		match self_0.bindings.entry (key.clone ()) {
			StdMapEntry::Occupied (entry) => {
				let &mut (ref binding, ref mut conversion) = entry.into_mut ();
				let (value, conversion_1) = try! (parameter.new_conversion (Some (value), evaluator));
				try! (binding.set (value));
				*conversion = conversion_1;
			},
			StdMapEntry::Vacant (entry) => {
				let (value, conversion) = try! (parameter.new_conversion (Some (value), evaluator));
				let binding = try! (parameter.new_binding ());
				try! (binding.initialize (value));
				entry.insert ((binding, conversion));
			},
		}
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdin (&self) -> (Outcome<Port>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stdin, 0x158c7282) .clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdout (&self) -> (Outcome<Port>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stdout, 0x8133bc6b) .clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stderr (&self) -> (Outcome<Port>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (try_some_ref! (self_0.stderr, 0xb4037a1a) .clone ());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configure_stdin (&self, port : &Port) -> (Outcome<()>) {
		// TODO:  Find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports!
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0x27f16faa);
		//}
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0x2521a116);
		}
		self_0.stdin = Some (port.clone ());
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configure_stdout (&self, port : &Port) -> (Outcome<()>) {
		// TODO:  Find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports!
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0xf78642ab);
		//}
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xa5dd23e6);
		}
		self_0.stdout = Some (port.clone ());
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configure_stderr (&self, port : &Port) -> (Outcome<()>) {
		// TODO:  Find a way to check if the parameters was forked, in which case we shouldn't be able to re-configure the standard ports!
		//if StdRc::strong_count (&self.0) > 1 {
		//	fail! (0x5443078f);
		//}
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0x93818ef2);
		}
		self_0.stderr = Some (port.clone ());
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdin_value (&self) -> (Outcome<Value>) {
		succeed! (try! (self.resolve_stdin ()) .clone () .into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdout_value (&self) -> (Outcome<Value>) {
		succeed! (try! (self.resolve_stdout ()) .clone () .into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stderr_value (&self) -> (Outcome<Value>) {
		succeed! (try! (self.resolve_stderr ()) .clone () .into ());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdin_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stdin, default);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stdout_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stdout, default);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_stderr_value_or (&self, default : Option<&Value>) -> (Outcome<Value>) {
		let self_0 = try! (self.internals_ref ());
		return Self::resolve_port_value_or (&self_0.stderr, default);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn resolve_port_value_or (port : &Option<Port>, default : Option<&Value>) -> (Outcome<Value>) {
		if let Some (ref port) = *port {
			succeed! (port.clone () .into ());
		} else if let Some (default) = default {
			succeed! (default.clone ());
		} else {
			succeed! (UNDEFINED_VALUE);
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<ParametersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x01545833));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<ParametersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0x37409eea));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Parameters) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
}




#[ derive (Clone) ]
pub struct Parameter ( StdRc<ParameterInternals> );


#[ derive (Clone, Debug, Hash) ]
pub struct ParameterInternals {
	pub identifier : Option<Symbol>,
	pub global : Binding,
	pub conversion : ParameterConversion,
	pub immutable : bool,
	pub handle : Handle,
	pub unique : Unique,
}


#[ derive (Clone, Debug, Hash) ]
pub enum ParameterConversion {
	None,
	OnConfigure (Value),
	OnResolveOnce (Value),
	OnResolveAlways (Value),
}


impl Parameter {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (identifier : Option<Symbol>, global : Option<Value>, conversion : ParameterConversion, immutable : bool) -> (Parameter) {
		let global = Binding::new (identifier.clone (), global, true);
		let handle = parameter_handles_next ();
		let internals = ParameterInternals {
				identifier : identifier,
				global : global,
				conversion : conversion,
				immutable : immutable,
				handle : handle,
				unique : Unique::for_parameter (handle),
			};
		return Parameter (StdRc::new (internals));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_binding (&self) -> (Outcome<Binding>) {
		let self_0 = try! (self.internals_ref ());
		let binding = Binding::new (self_0.identifier.clone (), None, self_0.immutable);
		succeed! (binding);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_conversion (&self, value : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<(Value, ParameterConversion)>) {
		let self_0 = try! (self.internals_ref ());
		let (is_configuration, value) = if let Some (value) = value {
			(true, value.clone ())
		} else {
			(false, try! (self_0.global.get ()))
		};
		match self_0.conversion {
			ParameterConversion::None =>
				succeed! ((value, ParameterConversion::None)),
			ParameterConversion::OnConfigure (ref converter) =>
				if is_configuration {
					let value = try! (evaluator.evaluate_procedure_call_1 (converter, &value));
					succeed! ((value, ParameterConversion::None));
				} else {
					succeed! ((value, ParameterConversion::None));
				},
			ParameterConversion::OnResolveOnce (ref converter) => {
				let value = try! (evaluator.evaluate_procedure_call_1 (converter, &value));
				succeed! ((value, ParameterConversion::None));
			},
			ParameterConversion::OnResolveAlways (_) =>
				succeed! ((value, self_0.conversion.clone ())),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_immutable (&self) -> (Outcome<bool>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.immutable);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<&ParameterInternals>) {
		succeed! (StdRc::as_ref (&self.0));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Parameter) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn unique_ref (&self) -> (Outcome<&Unique>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (&self_0.unique);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn unique (&self) -> (Outcome<Unique>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.unique.clone ());
	}
}

