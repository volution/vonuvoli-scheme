

use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Parameters, ParametersInternals};
	pub use super::{Parameter, ParameterInternals};
}




#[ derive (Clone) ]
pub struct Parameters ( StdRc<StdRefCell<ParametersInternals>> );


#[ derive (Clone, Debug) ]
pub struct ParametersInternals {
	pub bindings : StdMap<Unique, (Binding, ParameterConversion)>,
	pub parent : Option<Parameters>,
	pub immutable : bool,
	pub handle : Handle,
}


impl Parameters {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (parent : Option<&Parameters>) -> (Parameters) {
		let internals = ParametersInternals {
				bindings : StdMap::new (),
				parent : option_map! (parent, parent.clone ()),
				immutable : false,
				handle : context_handles_next (), // FIXME: Replace this!
			};
		return Parameters (StdRc::new (StdRefCell::new (internals)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork (&self) -> (Parameters) {
		return Parameters::new (Some (self));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve (&self, parameter : &Parameter, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
		let key = parameter.unique_ref ();
		loop {
			{
				let self_0 = self.internals_ref ();
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
						succeed! (value);
					},
					None =>
						(),
				}
			}
			try! (self.resolve_or_cache (key, parameter, evaluator));
		}
	}
	
	
	#[ inline (never) ]
	pub fn resolve_or_cache (&self, key : &Unique, parameter : &Parameter, evaluator : &mut EvaluatorContext) -> (Outcome<(Value, ParameterConversion)>) {
		let mut self_0 = self.internals_ref_mut ();
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
				let binding = parameter.new_binding ();
				try! (binding.initialize (value.clone ()));
				entry.insert ((binding, conversion.clone ()));
				succeed! ((value, conversion));
			},
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configure (&self, parameter : &Parameter, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
		let key = parameter.unique_ref ();
		let mut self_0 = self.internals_ref_mut ();
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
				let binding = parameter.new_binding ();
				try! (binding.initialize (value));
				entry.insert ((binding, conversion));
			},
		}
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (StdRef<ParametersInternals>) {
		// FIXME:  Use `try_borrow`!
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (StdRefMut<ParametersInternals>) {
		// FIXME:  Use `try_borrow`!
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle;
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Parameters) -> (bool) {
		return ptr::eq (self.0.as_ref (), other.0.as_ref ());
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
		let global = Binding::new (identifier.clone (), global, immutable);
		let handle = bindings_handles_next (); // FIXME: Replace this!
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
	pub fn new_binding (&self) -> (Binding) {
		let self_0 = self.internals_ref ();
		let binding = Binding::new (self_0.identifier.clone (), None, self_0.immutable);
		return binding
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_conversion (&self, value : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<(Value, ParameterConversion)>) {
		let self_0 = self.internals_ref ();
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
	pub fn is_immutable (&self) -> (bool) {
		return self.internals_ref () .immutable;
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ParameterInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle;
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Parameter) -> (bool) {
		return ptr::eq (self.0.as_ref (), other.0.as_ref ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn unique_ref (&self) -> (&Unique) {
		let self_0 = self.internals_ref ();
		return &self_0.unique;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn unique (&self) -> (Unique) {
		let self_0 = self.internals_ref ();
		return self_0.unique.clone ();
	}
}

