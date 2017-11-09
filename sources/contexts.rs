

use super::globals;

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::mem;




pub mod exports {
	pub use super::Context;
	pub use super::Binding;
}




#[ derive (Clone) ]
pub struct Context ( StdRc<StdRefCell<ContextInternals>> );


#[ derive (Debug) ]
struct ContextInternals {
	parent : Option<Context>,
	bindings : StdMap<Symbol, Binding>,
	handle : u32,
}


impl Context {
	
	
	#[ inline (always) ]
	pub fn new (parent : Option<Context>) -> (Context) {
		let internals = ContextInternals {
				parent : parent,
				bindings : StdMap::new (),
				handle : globals::context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn resolve_expect<SymbolRef> (&self, identifier : SymbolRef) -> (Binding)
			where SymbolRef : StdBorrow<Symbol>
	{
		return self.resolve (identifier) .expect ("d6dcf293");
	}
	
	#[ inline (always) ]
	pub fn resolve<SymbolRef> (&self, identifier : SymbolRef) -> (Outcome<Binding>)
			where SymbolRef : StdBorrow<Symbol>
	{
		let self_0 = self.internals_ref ();
		return match self_0.bindings.get (identifier.borrow ()) {
			Some (binding) => Ok (binding.clone ()),
			None => failed! (0x7fa02d50),
		};
	}
	
	
	#[ inline (always) ]
	pub fn define_expect<SymbolRef> (&mut self, identifier : SymbolRef) -> (Binding)
			where SymbolRef : StdBorrow<Symbol>
	{
		return self.define (identifier) .expect ("16ccb995");
	}
	
	#[ inline (always) ]
	pub fn define<SymbolRef> (&mut self, identifier : SymbolRef) -> (Outcome<Binding>)
			where SymbolRef : StdBorrow<Symbol>
	{
		use std::collections::hash_map::Entry::*;
		let mut self_0 = self.internals_ref_mut ();
		let bindings_entry = self_0.bindings.entry (identifier.borrow () .clone ());
		return match bindings_entry {
			Occupied (_) => failed! (0x5b8e8d57),
			Vacant (_) => Ok (bindings_entry.or_insert (Binding::new (identifier.borrow () .clone (), UNDEFINED.clone ())) .clone ()),
		};
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
pub struct Binding ( StdRc<StdRefCell<BindingInternals>> );


#[ derive (Debug) ]
struct BindingInternals {
	identifier : Symbol,
	value : Value,
	handle : u32,
}


impl Binding {
	
	
	#[ inline (always) ]
	pub fn new (identifier : Symbol, value : Value) -> (Binding) {
		let internals = BindingInternals {
				identifier : identifier.clone (),
				value : value.clone (),
				handle : globals::bindings_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn get (&self) -> (Value) {
		let self_0 = self.internals_ref ();
		return self_0.value.clone ();
	}
	
	#[ inline (always) ]
	pub fn set<ValueInto> (&mut self, value : ValueInto) -> (Value)
		where ValueInto : Into<Value>
	{
		let mut value = value.into ();
		let mut self_0 = self.internals_ref_mut ();
		mem::swap (&mut self_0.value, &mut value);
		return value;
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

impl fmt::Debug for Binding {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return self_0.fmt (formatter);
	}
}

