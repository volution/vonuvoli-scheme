
use super::constants::exports::*;
use super::errors::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::mem;
use std::ptr;




pub mod exports {
	pub use super::{Context};
	pub use super::{Registers, RegisterTemplate};
	pub use super::{Binding, BindingTemplate};
}




#[ derive (Clone) ]
pub struct Context ( StdRc<StdRefCell<ContextInternals>> );


#[ derive (Clone, Debug) ]
struct ContextInternals {
	bindings : StdMap<StdString, Binding>,
	parent : Option<Context>,
	immutable : bool,
	handle : Handle,
}


impl Context {
	
	
	#[ inline (always) ]
	pub fn new (parent : Option<&Context>) -> (Context) {
		let internals = ContextInternals {
				bindings : StdMap::new (),
				parent : option_map! (parent, parent.clone ()),
				immutable : false,
				handle : context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	#[ inline (always) ]
	pub fn fork (&self) -> (Context) {
		return Context::new (Some (self));
	}
	
	
	#[ inline (always) ]
	pub fn resolve (&self, identifier : &Symbol) -> (Outcome<Option<Binding>>) {
		let self_0 = self.internals_ref ();
		match self_0.bindings.get (identifier.string_ref ()) {
			Some (binding) =>
				succeed! (Some (binding.clone ())),
			None =>
				if let Some (ref parent) = self_0.parent {
					return parent.resolve (identifier);
				} else {
					succeed! (None);
				},
		}
	}
	
	
	#[ inline (always) ]
	pub fn define (&self, template : &BindingTemplate) -> (Outcome<Binding>) {
		return self.define_with_prefix (template, None);
	}
	
	#[ inline (always) ]
	pub fn define_with_prefix (&self, template : &BindingTemplate, prefix : Option<&str>) -> (Outcome<Binding>) {
		use std::collections::hash_map::Entry;
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			fail! (0x4814c74f);
		}
		let template_identifier = try_some_ref! (template.identifier, 0x2cd13fb0) .string_as_str ();
		let identifier = if let Some (prefix) = prefix {
			let mut identifier = StdString::with_capacity (template_identifier.len () + prefix.len ());
			identifier.push_str (prefix);
			identifier.push_str (template_identifier);
			identifier
		} else {
			StdString::from (template_identifier)
		};
		let bindings_entry = self_0.bindings.entry (identifier);
		match bindings_entry {
			Entry::Occupied (_) =>
				fail! (0x5b8e8d57),
			Entry::Vacant (_) => {
				let binding = try! (self.new_binding (template));
				bindings_entry.or_insert (binding.clone ());
				succeed! (binding);
			},
		}
	}
	
	
	#[ inline (always) ]
	pub fn define_all (&self, templates : &[BindingTemplate]) -> (Outcome<()>) {
		return self.define_all_with_prefix (templates, None);
	}
	
	#[ inline (always) ]
	pub fn define_all_with_prefix (&self, templates : &[BindingTemplate], prefix : Option<&str>) -> (Outcome<()>) {
		{
			let mut self_0 = self.internals_ref_mut ();
			if self_0.immutable {
				fail! (0x36b1eddd);
			}
			self_0.bindings.reserve (templates.len ());
		}
		{
			for template in templates {
				try! (self.define_with_prefix (template, prefix));
			}
			succeed! (());
		}
	}
	
	
	#[ inline (always) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ inline (always) ]
	fn internals_ref (&self) -> (StdRef<ContextInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	fn internals_ref_mut (&self) -> (StdRefMut<ContextInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle;
	}
	
	
	#[ inline (always) ]
	fn new_binding (&self, template : &BindingTemplate) -> (Outcome<Binding>) {
		let binding = Binding::new (
				template.identifier.clone (),
				template.value.clone (),
				template.immutable,
			);
		succeed! (binding);
	}
	
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Context) -> (bool) {
		return ptr::eq (self.0.as_ref (), other.0.as_ref ());
	}
}


impl cmp::Eq for Context {}

impl cmp::PartialEq for Context {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::eq (&self_0.handle, &other_0.handle);
	}
}


impl cmp::Ord for Context {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::cmp (&self_0.handle, &other_0.handle);
	}
}

impl cmp::PartialOrd for Context {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::partial_cmp (&self_0.handle, &other_0.handle);
	}
}


impl hash::Hash for Context {
	
	#[ inline (always) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		self_0.handle.hash (hasher);
	}
}


impl fmt::Display for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<context:{:08x}>", self_0.handle.value ());
	}
}

impl fmt::Debug for Context {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return formatter
				.debug_struct ("Context")
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.field ("bindings", &self_0.bindings)
				.field ("parent", &self_0.parent)
				.finish ();
	}
}




#[ derive (Clone) ]
pub struct Registers {
	registers : StdVec<Register>,
	count : usize,
	immutable : bool,
	handle : Handle,
}


#[ derive (Clone, Hash) ]
#[ allow (dead_code) ]
enum Register {
	Binding (Binding),
	Value (Value, bool),
	Uninitialized (bool),
	Undefined,
}


#[ derive (Clone, Debug, Hash) ]
pub enum RegisterTemplate {
	Borrow (usize),
	LocalBinding (BindingTemplate),
	LocalValue (Option<Value>, bool),
}




impl Registers {
	
	
	#[ inline (always) ]
	pub fn new () -> (Registers) {
		let registers = Registers {
				registers : StdVec::new (),
				count : 0,
				immutable : false,
				handle : context_handles_next (),
			};
		return registers;
	}
	
	
	#[ inline (always) ]
	pub fn new_and_define (templates : &[RegisterTemplate], borrow : &Registers) -> (Outcome<Registers>) {
		let mut registers = Registers::new ();
		try! (registers.define_all (templates, borrow));
		succeed! (registers);
	}
	
	
	#[ inline (always) ]
	pub fn resolve_value (&mut self, index : usize) -> (Outcome<Value>) {
		let register = try_some! (self.registers.get (index), 0x89e68eab);
		match *register {
			Register::Binding (ref binding) =>
				return binding.get (),
			Register::Value (ref value, _) =>
				succeed! (value.clone ()),
			Register::Uninitialized (_) =>
				fail! (0x3b976812),
			Register::Undefined =>
				fail! (0xef51ae76),
		}
	}
	
	
	#[ inline (always) ]
	pub fn resolve_binding_option (&self, index : usize) -> (Outcome<Option<Binding>>) {
		let register = try_some! (self.registers.get (index), 0x371fc84b);
		match *register {
			Register::Binding (ref binding) =>
				succeed! (Some (binding.clone ())),
			Register::Value (_, _) =>
				succeed! (None),
			Register::Uninitialized (_) =>
				succeed! (None),
			Register::Undefined =>
				fail! (0x1a30890c),
		}
	}
	
	#[ inline (always) ]
	pub fn resolve_binding_create (&mut self, index : usize) -> (Outcome<Binding>) {
		let register = try_some! (self.registers.get_mut (index), 0x79873ff6);
		let binding = match *register {
			Register::Binding (ref binding) =>
				succeed! (binding.clone ()),
			Register::Value (ref mut value_for_register, immutable) => {
				let mut value_for_binding = UNDEFINED.into ();
				mem::swap (value_for_register, &mut value_for_binding);
				Binding::new (None, Some (value_for_binding), immutable)
			},
			Register::Uninitialized (immutable) => {
				Binding::new (None, None, immutable)
			},
			Register::Undefined =>
				fail! (0x7e2729e2),
		};
		*register = Register::Binding (binding.clone ());
		succeed! (binding);
	}
	
	
	#[ inline (always) ]
	pub fn initialize_value (&mut self, index : usize, value : Value) -> (Outcome<()>) {
		let register = try_some! (self.registers.get_mut (index), 0x7dabdbe0);
		match *register {
			Register::Binding (ref mut binding) =>
				return binding.initialize (value),
			Register::Value (_, _) =>
				fail! (0x27589b63),
			Register::Uninitialized (immutable) => {
				*register = Register::Value (value, immutable);
				succeed! (());
			}
			Register::Undefined =>
				fail! (0xce482df1),
		}
	}
	
	#[ inline (always) ]
	pub fn update_value (&mut self, index : usize, value : Value) -> (Outcome<Value>) {
		if self.immutable {
			fail! (0xf97e0269);
		}
		let register = &mut self.registers[index];
		match *register {
			Register::Binding (ref mut binding) =>
				return binding.set (value),
			Register::Value (ref mut value_for_register, immutable) => {
				if immutable {
					fail! (0x61871fc9);
				}
				let mut value = value;
				mem::swap (value_for_register, &mut value);
				succeed! (value);
			},
			Register::Uninitialized (_) =>
				fail! (0xa708a086),
			Register::Undefined =>
				fail! (0x274323f1),
		}
	}
	
	
	#[ inline (always) ]
	pub fn define (&mut self, template : &RegisterTemplate, borrow : &Registers) -> (Outcome<usize>) {
		if self.immutable {
			fail! (0xd7cbcdd8);
		}
		let register = try! (self.new_register (template, borrow));
		let index = self.count;
		self.registers.push (register);
		self.count += 1;
		succeed! (index);
	}
	
	#[ inline (always) ]
	pub fn define_all (&mut self, templates : &[RegisterTemplate], borrow : &Registers) -> (Outcome<()>) {
		{
			if self.immutable {
				fail! (0x74189c0f);
			}
			self.registers.reserve (templates.len ());
		}
		{
			for template in templates {
				try! (self.define (template, borrow));
			}
			succeed! (());
		}
	}
	
	
	#[ inline (always) ]
	pub fn set_immutable (&mut self) -> (Outcome<()>) {
		self.immutable = true;
		succeed! (());
	}
	
	
	#[ inline (always) ]
	fn new_register (&mut self, template : &RegisterTemplate, borrow : &Registers) -> (Outcome<Register>) {
		match *template {
			RegisterTemplate::Borrow (index) => {
				let binding = try! (borrow.resolve_binding_option (index));
				let binding = try_some! (binding, 0x2f543c30);
				let register = Register::Binding (binding);
				succeed! (register);
			},
			RegisterTemplate::LocalBinding (ref template) => {
				let binding = Binding::new (
						template.identifier.clone (),
						template.value.clone (),
						template.immutable
					);
				let register = Register::Binding (binding);
				succeed! (register);
			},
			RegisterTemplate::LocalValue (ref value, immutable) => {
				let register = if let Some (ref value) = *value {
					Register::Value (value.clone (), immutable)
				} else {
					Register::Uninitialized (immutable)
				};
				succeed! (register);
			},
		}
	}
	
	
	#[ inline (always) ]
	pub fn handle (&self) -> (Handle) {
		return self.handle;
	}
	
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Registers) -> (bool) {
		return ptr::eq (self, other);
	}
}


impl cmp::Eq for Registers {}

impl cmp::PartialEq for Registers {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		return Handle::eq (&self.handle, &other.handle);
	}
}


impl cmp::Ord for Registers {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		return Handle::cmp (&self.handle, &other.handle);
	}
}

impl cmp::PartialOrd for Registers {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		return Handle::partial_cmp (&self.handle, &other.handle);
	}
}


impl hash::Hash for Registers {
	
	#[ inline (always) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle.hash (hasher);
	}
}


impl fmt::Display for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "#<context:{:08x}>", self.handle.value ());
	}
}

impl fmt::Debug for Registers {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return formatter
				.debug_struct ("Registers")
				.field ("immutable", &self.immutable)
				.field ("handle", &self.handle)
				.field ("registers", &self.registers)
				.finish ();
	}
}


impl fmt::Display for Register {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "#<register>");
	}
}

impl fmt::Debug for Register {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			Register::Binding (ref binding) =>
				return formatter
						.debug_tuple ("Binding")
						.field (binding)
						.finish (),
			Register::Value (_, immutable) =>
				return formatter
						.debug_tuple ("Value")
						.field (&immutable)
						.finish (),
			Register::Uninitialized (immutable) =>
				return formatter
						.debug_tuple ("Uninitialized")
						.field (&immutable)
						.finish (),
			Register::Undefined =>
				return formatter
						.debug_tuple ("Undefined")
						.finish (),
		}
	}
}




#[ derive (Clone) ]
pub struct Binding ( StdRc<StdRefCell<BindingInternals>> );


#[ derive (Clone, Debug, Hash) ]
struct BindingInternals {
	identifier : Option<Symbol>,
	value : Value,
	initialized : bool,
	immutable : bool,
	handle : Handle,
}


#[ derive (Clone, Debug, Hash) ]
pub struct BindingTemplate {
	pub identifier : Option<Symbol>,
	pub value : Option<Value>,
	pub immutable : bool,
}


impl Binding {
	
	
	#[ inline (always) ]
	pub fn new (identifier : Option<Symbol>, value : Option<Value>, immutable : bool) -> (Binding) {
		let (value, initialized) = if let Some (value) = value {
			(value, true)
		} else {
			(UNDEFINED.into (), false)
		};
		let internals = BindingInternals {
				identifier : identifier,
				value : value,
				initialized : initialized,
				immutable : immutable,
				handle : bindings_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ inline (always) ]
	pub fn get (&self) -> (Outcome<Value>) {
		let self_0 = self.internals_ref ();
		if ! self_0.initialized {
			fail! (0x3e185e26);
		}
		succeed! (self_0.value.clone ());
	}
	
	#[ inline (always) ]
	pub fn get_option (&self) -> (Outcome<Option<Value>>) {
		let self_0 = self.internals_ref ();
		if self_0.initialized {
			succeed! (Some (self_0.value.clone ()));
		} else {
			succeed! (None);
		}
	}
	
	
	#[ inline (always) ]
	pub fn initialize (&self, value : Value) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		// FIXME:  This breaks bencmarks!
		//if self_0.initialized {
		//	fail! (0x10d54f09);
		//}
		self_0.value = value;
		self_0.initialized = true;
		succeed! (());
	}
	
	#[ inline (always) ]
	pub fn set (&self, value : Value) -> (Outcome<Value>) {
		let mut self_0 = self.internals_ref_mut ();
		if self_0.immutable {
			fail! (0x11c77731);
		}
		if ! self_0.initialized {
			fail! (0xadd67276);
		}
		let mut value = value;
		mem::swap (&mut self_0.value, &mut value);
		succeed! (value);
	}
	
	
	#[ inline (always) ]
	pub fn is_initialized (&self) -> (bool) {
		return self.internals_ref () .initialized;
	}
	
	
	#[ inline (always) ]
	pub fn is_immutable (&self) -> (bool) {
		return self.internals_ref () .immutable;
	}
	
	#[ inline (always) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = self.internals_ref_mut ();
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ inline (always) ]
	fn internals_ref (&self) -> (StdRef<BindingInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	fn internals_ref_mut (&self) -> (StdRefMut<BindingInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	#[ inline (always) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle;
	}
	
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Binding) -> (bool) {
		return ptr::eq (self.0.as_ref (), other.0.as_ref ());
	}
}


impl cmp::Eq for Binding {}

impl cmp::PartialEq for Binding {
	
	#[ inline (always) ]
	fn eq (&self, other : &Self) -> (bool) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::eq (&self_0.handle, &other_0.handle);
	}
}


impl cmp::Ord for Binding {
	
	#[ inline (always) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::cmp (&self_0.handle, &other_0.handle);
	}
}

impl cmp::PartialOrd for Binding {
	
	#[ inline (always) ]
	fn partial_cmp (&self, other : &Self) -> (Option<cmp::Ordering>) {
		let self_0 = self.internals_ref ();
		let other_0 = other.internals_ref ();
		return Handle::partial_cmp (&self_0.handle, &other_0.handle);
	}
}


impl hash::Hash for Binding {
	
	#[ inline (always) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		self_0.handle.hash (hasher);
	}
}


impl fmt::Display for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		if let Some (ref identifier) = self_0.identifier {
			return write! (formatter, "#<binding:{:08x} {} {}>", self_0.handle.value (), identifier, self_0.value);
		} else {
			return write! (formatter, "#<binding:{:08x} {}>", self_0.handle.value (), self_0.value);
		}
	}
}

impl fmt::Debug for Binding {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return formatter
				.debug_struct ("Binding")
				.field ("identifier", &self_0.identifier)
				.field ("initialized", &self_0.initialized)
				.field ("immutable", &self_0.immutable)
				.field ("handle", &self_0.handle)
				.finish ();
	}
}

