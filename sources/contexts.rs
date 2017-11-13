

use super::globals;

use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::mem;




pub mod exports {
	pub use super::Context;
	pub use super::Registers;
	pub use super::Binding;
}




#[ derive (Clone) ]
pub struct Context ( StdRc<StdRefCell<ContextInternals>> );


#[ derive (Debug) ]
struct ContextInternals {
	bindings : StdMap<Symbol, Binding>,
	parent : Option<Context>,
	immutable : bool,
	handle : u32,
}


impl Context {
	
	
	#[ inline (always) ]
	pub fn new (parent : Option<Context>) -> (Context) {
		let internals = ContextInternals {
				bindings : StdMap::new (),
				parent : parent,
				immutable : false,
				handle : globals::context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn resolve_expect<SymbolFrom> (&self, identifier : &SymbolFrom) -> (Binding)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		return self.resolve (identifier) .unwrap ();
	}
	
	#[ inline (always) ]
	pub fn resolve<SymbolFrom> (&self, identifier : &SymbolFrom) -> (Outcome<Binding>)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		let identifier = Symbol::from (identifier.clone ());
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (&identifier) {
			Some (binding) => Ok (binding.clone ()),
			None => failed! (0x7fa02d50),
		};
	}
	
	
	#[ inline (always) ]
	pub fn define_expect<SymbolFrom> (&mut self, identifier : &SymbolFrom) -> (Binding)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		return self.define (identifier) .unwrap ();
	}
	
	#[ inline (always) ]
	pub fn define<SymbolFrom> (&mut self, identifier : &SymbolFrom) -> (Outcome<Binding>)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		use std::collections::hash_map::Entry;
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			return failed! (0x4814c74f);
		}
		let identifier = Symbol::from (identifier.clone ());
		let bindings_entry = self_0.bindings.entry (identifier.clone ());
		return match bindings_entry {
			Entry::Occupied (_) => failed! (0x5b8e8d57),
			Entry::Vacant (_) => {
				let binding = Binding::new (identifier.clone (), UNDEFINED, false);
				bindings_entry.or_insert (binding.clone ());
				Ok (binding)
			},
		};
	}
	
	
	#[ inline (always) ]
	pub fn set_immutable (&mut self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		return Ok (());
	}
	
	
	#[ inline (always) ]
	fn internals_ref (&self) -> (StdRef<ContextInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	fn internals_ref_mut (&mut self) -> (StdRefMut<ContextInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
}


impl cmp::Eq for Context {}

impl cmp::PartialEq for Context {
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return self_0.handle == other_0.handle;
	}
}


impl hash::Hash for Context {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		hasher.write_u32 (self_0.handle);
	}
}


impl fmt::Display for Context {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<context:{:08x}>", self_0.handle);
	}
}

impl fmt::Debug for Context {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return self_0.fmt (formatter);
	}
}




#[ derive (Clone) ]
pub struct Registers ( StdRc<StdRefCell<RegistersInternals>> );


#[ derive (Debug) ]
struct RegistersInternals {
	bindings : StdVec<Binding>,
	handle : u32,
}


impl Registers {
	
	
	#[ inline (always) ]
	pub fn new (count : usize) -> (Registers) {
		let internals = RegistersInternals {
				bindings : StdVec::with_capacity (count),
				handle : globals::context_handles_next (),
			};
		return Registers (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn resolve_expect (&self, index : usize) -> (Binding) {
		return self.resolve (index) .unwrap ();
	}
	
	#[ inline (always) ]
	pub fn resolve (&self, index : usize) -> (Outcome<Binding>) {
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (index) {
			Some (binding) => Ok (binding.clone ()),
			None => failed! (0x97ff34a1),
		};
	}
	
	
	#[ inline (always) ]
	fn internals_ref (&self) -> (StdRef<RegistersInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ allow (dead_code) ]
	#[ inline (always) ]
	fn internals_ref_mut (&mut self) -> (StdRefMut<RegistersInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
}


impl fmt::Display for Registers {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<context:{:08x}>", self_0.handle);
	}
}

impl fmt::Debug for Registers {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return self_0.fmt (formatter);
	}
}




#[ derive (Clone) ]
pub struct Binding ( StdRc<StdRefCell<BindingInternals>> );


#[ derive (Debug) ]
struct BindingInternals {
	identifier : Symbol,
	value : Value,
	immutable : bool,
	handle : u32,
}


impl Binding {
	
	
	#[ inline (always) ]
	pub fn new (identifier : Symbol, value : Value, immutable : bool) -> (Binding) {
		let internals = BindingInternals {
				identifier : identifier,
				value : value,
				immutable : immutable,
				handle : globals::bindings_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn get (&self) -> (Outcome<Value>) {
		let self_0 = self.internals_ref ();
		return Ok (self_0.value.clone ());
	}
	
	#[ inline (always) ]
	pub fn set<ValueFrom> (&mut self, value : ValueFrom) -> (Outcome<Value>)
			where Value : StdFrom<ValueFrom>
	{
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			return failed! (0x11c77731);
		}
		let mut value = Value::from (value);
		mem::swap (&mut self_0.value, &mut value);
		return Ok (value);
	}
	
	
	#[ inline (always) ]
	pub fn set_immutable (&mut self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		return Ok (());
	}
	
	
	#[ inline (always) ]
	fn internals_ref (&self) -> (StdRef<BindingInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	fn internals_ref_mut (&mut self) -> (StdRefMut<BindingInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
}


impl cmp::Eq for Binding {}

impl cmp::PartialEq for Binding {
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return self_0.handle == other_0.handle;
	}
}


impl hash::Hash for Binding {
	#[ inline (always) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		hasher.write_u32 (self_0.handle);
	}
}


impl fmt::Display for Binding {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<binding:{:08x} {} {}>", self_0.handle, self_0.identifier, self_0.value);
	}
}

impl fmt::Debug for Binding {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return self_0.fmt (formatter);
	}
}

