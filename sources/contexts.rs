

use std::cmp;
use std::fmt;
use std::hash;
use std::mem;

use super::errors::exports::*;
use super::globals;
use super::runtime::std::*;
use super::values::exports::*;


pub mod exports {
	pub use super::Context;
	pub use super::Binding;
}




#[ derive (Clone) ]
pub struct Context ( StdRc<StdRefCell<ContextInternals>> );


struct ContextInternals {
	parent : Option<Context>,
	bindings : StdMap<Symbol, Binding>,
	handle : u32,
}


impl Context {
	
	
	pub fn new (parent : Option<Context>) -> (Context) {
		let internals = ContextInternals {
				parent : parent,
				bindings : StdMap::new (),
				handle : globals::context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	pub fn resolve_expect<SymbolRef> (&self, identifier : SymbolRef) -> (Binding)
			where SymbolRef : StdBorrow<Symbol>
	{
		return self.resolve (identifier) .expect ("d6dcf293");
	}
	
	pub fn resolve<SymbolRef> (&self, identifier : SymbolRef) -> (Outcome<Binding>)
			where SymbolRef : StdBorrow<Symbol>
	{
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (identifier.borrow ()) {
			Some (binding) => Ok (binding.clone ()),
			None => Err (error_generic (0x7fa02d50)),
		};
	}
	
	
	pub fn define_expect<SymbolRef, ValueInto> (&mut self, identifier : SymbolRef, value : ValueInto) -> (Binding)
			where SymbolRef : StdBorrow<Symbol>, ValueInto : StdInto<Value>
	{
		return self.define (identifier, value) .expect ("16ccb995");
	}
	
	pub fn define<SymbolRef, ValueInto> (&mut self, identifier : SymbolRef, value : ValueInto) -> (Outcome<Binding>)
			where SymbolRef : StdBorrow<Symbol>, ValueInto : StdInto<Value>
	{
		use std::collections::hash_map::Entry::*;
		let mut self_0 = self.internals_ref_mut ();
		let bindings_entry = self_0.bindings.entry (identifier.borrow () .clone ());
		return match bindings_entry {
			Occupied (_) => Err (error_generic (0x5b8e8d57)),
			Vacant (_) => Ok (bindings_entry.or_insert (Binding::new (identifier.borrow () .clone (), value.into ())) .clone ()),
		};
	}
	
	
	fn internals_ref (&self) -> (StdRef<ContextInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&mut self) -> (StdRefMut<ContextInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
}


impl fmt::Display for Context {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<context:{:08x}>", self_0.handle);
	}
}




#[ derive (Clone) ]
pub struct Binding ( StdRc<StdRefCell<BindingInternals>> );


struct BindingInternals {
	identifier : Symbol,
	value : Value,
	handle : u32,
}


impl Binding {
	
	
	pub fn new (identifier : Symbol, value : Value) -> (Binding) {
		let internals = BindingInternals {
				identifier : identifier.clone (),
				value : value.clone (),
				handle : globals::bindings_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	pub fn get (&self) -> (Value) {
		let self_0 = self.internals_ref ();
		return self_0.value.clone ();
	}
	
	pub fn set<ValueInto> (&mut self, value : ValueInto) -> (Value)
		where ValueInto : Into<Value>
	{
		let mut value = value.into ();
		let mut self_0 = self.internals_ref_mut ();
		mem::swap (&mut self_0.value, &mut value);
		return value;
	}
	
	
	fn internals_ref (&self) -> (StdRef<BindingInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&mut self) -> (StdRefMut<BindingInternals>) {
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
		return write! (formatter, "#<binding:{:08x} {} {}>", self_0.handle, self_0.identifier, self_0.value);
	}
}

