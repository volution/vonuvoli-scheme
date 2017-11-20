

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
	pub use super::{Context, ContextBindingTemplate};
	pub use super::{Registers, RegistersBindingTemplate};
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


#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct ContextBindingTemplate {
	pub identifier : Symbol,
	pub value : Option<Value>,
	pub immutable : bool,
}


impl Context {
	
	
	pub fn new (parent : Option<&Context>) -> (Context) {
		let internals = ContextInternals {
				bindings : StdMap::new (),
				parent : parent.map (|parent| parent.clone ()),
				immutable : false,
				handle : globals::context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	pub fn fork (&self) -> (Context) {
		return Context::new (Some (self));
	}
	
	
	pub fn resolve_expect<SymbolFrom> (&self, identifier : &SymbolFrom) -> (Binding)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		return self.resolve (identifier) .expect ("6ab141e4") .expect ("a3e4e132");
	}
	
	pub fn resolve<SymbolFrom> (&self, identifier : &SymbolFrom) -> (Outcome<Option<Binding>>)
			where Symbol : StdFrom<SymbolFrom>, SymbolFrom : Clone
	{
		let identifier = Symbol::from (identifier.clone ());
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (&identifier) {
			Some (binding) =>
				Ok (Some (binding.clone ())),
			None =>
				if let Some (ref parent) = self_0.parent {
					parent.resolve::<Symbol> (&identifier)
				} else {
					Ok (None)
				},
		};
	}
	
	
	pub fn define (&self, template : &ContextBindingTemplate) -> (Outcome<Binding>) {
		use std::collections::hash_map::Entry;
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			return failed! (0x4814c74f);
		}
		let bindings_entry = self_0.bindings.entry (template.identifier.clone ());
		return match bindings_entry {
			Entry::Occupied (_) => failed! (0x5b8e8d57),
			Entry::Vacant (_) => {
				let binding = try! (self.new_binding (template));
				bindings_entry.or_insert (binding.clone ());
				Ok (binding)
			},
		};
	}
	
	pub fn define_all (&self, templates : &[ContextBindingTemplate]) -> (Outcome<StdVec<Binding>>) {
		{
			let mut self_0 = self.internals_ref_mut ();
			if self_0.immutable {
				return failed! (0x36b1eddd);
			}
			self_0.bindings.reserve (templates.len ());
		}
		templates.iter () .map (|ref template| self.define (template)) .collect ()
	}
	
	
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		return Ok (());
	}
	
	
	fn internals_ref (&self) -> (StdRef<ContextInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&self) -> (StdRefMut<ContextInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	
	fn new_binding (&self, template : &ContextBindingTemplate) -> (Outcome<Binding>) {
		let binding = Binding::new (
				Some (template.identifier.clone ()),
				template.value.clone (),
				template.immutable
			);
		return Ok (binding);
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
	immutable : bool,
	handle : u32,
}


#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct RegistersBindingTemplate {
	pub identifier : Option<Symbol>,
	pub borrow : Option<usize>,
	pub value : Option<Value>,
	pub immutable : bool,
}


impl Registers {
	
	
	pub fn new () -> (Registers) {
		let internals = RegistersInternals {
				bindings : StdVec::new (),
				immutable : false,
				handle : globals::context_handles_next (),
			};
		return Registers (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	pub fn new_and_define (templates : &[RegistersBindingTemplate], borrow : Option<&Registers>) -> (Outcome<Registers>) {
		let mut registers = Registers::new ();
		try! (registers.define_all (templates, borrow));
		succeed! (registers);
	}
	
	
	pub fn resolve_expect (&self, index : usize) -> (Binding) {
		return self.resolve (index) .expect ("204a835e");
	}
	
	pub fn resolve (&self, index : usize) -> (Outcome<Binding>) {
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (index) {
			Some (binding) => Ok (binding.clone ()),
			None => failed! (0x97ff34a1),
		};
	}
	
	
	pub fn define (&self, template : &RegistersBindingTemplate, borrow : Option<&Registers>) -> (Outcome<(usize, Binding)>) {
		let binding = try! (self.new_binding (template, borrow));
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			return failed! (0xd7cbcdd8);
		}
		self_0.bindings.push (binding.clone ());
		let index = self_0.bindings.len () - 1;
		succeed! ((index, binding));
	}
	
	pub fn define_all (&mut self, templates : &[RegistersBindingTemplate], borrow : Option<&Registers>) -> (Outcome<StdVec<(usize, Binding)>>) {
		{
			let mut self_0 = self.internals_ref_mut ();
			if self_0.immutable {
				return failed! (0x74189c0f);
			}
			self_0.bindings.reserve (templates.len ());
		}
		templates.iter () .map (|ref template| self.define (template, borrow)) .collect ()
	}
	
	
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		return Ok (());
	}
	
	
	fn internals_ref (&self) -> (StdRef<RegistersInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&self) -> (StdRefMut<RegistersInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	
	fn new_binding (&self, template : &RegistersBindingTemplate, borrow : Option<&Registers>) -> (Outcome<Binding>) {
		if let Some (index) = template.borrow {
			if let Some (borrow) = borrow {
				let borrow_0 = borrow.internals_ref ();
				if let Some (binding) = borrow_0.bindings.get (index) {
					succeed! (binding.clone ());
				} else {
					fail! (0x114bb1df);
				}
			} else {
				fail! (0x0ff6a3a7);
			}
		} else {
			succeed! (Binding::new (template.identifier.clone (), template.value.clone (), template.immutable));
		}
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
// FIXME:  Add support for initialized flag!
struct BindingInternals {
	identifier : Option<Symbol>,
	value : Value,
	immutable : bool,
	handle : u32,
}


impl Binding {
	
	
	pub fn new (identifier : Option<Symbol>, value : Option<Value>, immutable : bool) -> (Binding) {
		let value = value.unwrap_or (UNDEFINED);
		let internals = BindingInternals {
				identifier : identifier,
				value : value,
				immutable : immutable,
				handle : globals::bindings_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	pub fn get (&self) -> (Outcome<Value>) {
		let self_0 = self.internals_ref ();
		return Ok (self_0.value.clone ());
	}
	
	pub fn set<ValueFrom> (&self, value : ValueFrom) -> (Outcome<Value>)
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
	
	pub fn initialize<ValueFrom> (&self, value : ValueFrom) -> (Outcome<Value>)
			where Value : StdFrom<ValueFrom>
	{
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			return failed! (0x11c77731);
		}
		let value = Value::from (value);
		self_0.value = value.clone ();
		return Ok (value);
	}
	
	
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		return Ok (());
	}
	
	
	fn internals_ref (&self) -> (StdRef<BindingInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&self) -> (StdRefMut<BindingInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
}


impl cmp::Eq for Binding {}

impl cmp::PartialEq for Binding {
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return self_0.handle == other_0.handle;
	}
}


impl hash::Hash for Binding {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		hasher.write_u32 (self_0.handle);
	}
}


impl fmt::Display for Binding {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		if let Some (ref identifier) = self_0.identifier {
			return write! (formatter, "#<binding:{:08x} {} {}>", self_0.handle, identifier, self_0.value);
		} else {
			return write! (formatter, "#<binding:{:08x} {}>", self_0.handle, self_0.value);
		}
	}
}

impl fmt::Debug for Binding {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return self_0.fmt (formatter);
	}
}

