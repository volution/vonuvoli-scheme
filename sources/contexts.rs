
use super::constants::exports::*;
use super::errors::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Context, ContextInternals};
	pub use super::{Registers, RegistersInternals, Register, RegisterTemplate};
	pub use super::{Binding, BindingInternals, BindingTemplate};
}




#[ derive (Clone) ]
pub struct Context ( StdRc<StdRefCell<ContextInternals>> );


#[ derive (Clone, Debug) ]
pub struct ContextInternals {
	pub bindings : StdMap<StdString, Binding>,
	pub parent : Option<Context>,
	pub immutable : bool,
	pub handle : Handle,
}


impl Context {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (parent : Option<&Context>) -> (Context) {
		let internals = ContextInternals {
				bindings : StdMap::new (),
				parent : option_map! (parent, parent.clone ()),
				immutable : false,
				handle : context_handles_next (),
			};
		return Context (StdRc::new (StdRefCell::new (internals)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork (&self) -> (Context) {
		return Context::new (Some (self));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve (&self, identifier : &Symbol) -> (Outcome<Option<Binding>>) {
		let self_0 = try! (self.internals_ref ());
		match self_0.bindings.get (identifier.string_as_str ()) {
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define (&self, template : &BindingTemplate) -> (Outcome<Binding>) {
		return self.define_with_prefix (template, None);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define_with_prefix (&self, template : &BindingTemplate, prefix : Option<&str>) -> (Outcome<Binding>) {
		let mut self_0 = try! (self.internals_ref_mut ());
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
			StdMapEntry::Occupied (_) =>
				fail! (0x5b8e8d57),
			StdMapEntry::Vacant (_) => {
				let binding = try! (self.new_binding (template));
				bindings_entry.or_insert (binding.clone ());
				succeed! (binding);
			},
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define_all (&self, templates : &[BindingTemplate]) -> (Outcome<()>) {
		return self.define_all_with_prefix (templates, None);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define_all_with_prefix (&self, templates : &[BindingTemplate], prefix : Option<&str>) -> (Outcome<()>) {
		{
			let mut self_0 = try! (self.internals_ref_mut ());
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<ContextInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x6313d9e9));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<ContextInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0x2899cc14));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn new_binding (&self, template : &BindingTemplate) -> (Outcome<Binding>) {
		succeed! (Binding::new_from_template (template));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Context) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
}




#[ derive (Clone) ]
pub struct Registers ( StdRc<StdRefCell<RegistersInternals>> );


#[ derive (Clone, Debug) ]
pub struct RegistersInternals {
	pub registers : StdVec<Register>,
	pub count : usize,
	pub immutable : bool,
	pub handle : Handle,
}


#[ derive (Clone) ]
#[ allow (dead_code) ]
pub enum Register {
	Binding (Binding),
	Value (Value, bool),
	Uninitialized (bool),
	Undefined,
}


#[ derive (Clone, Debug) ]
pub enum RegisterTemplate {
	Borrow (usize),
	LocalBinding (BindingTemplate),
	LocalValue (Option<Value>, bool),
}




impl Registers {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new () -> (Registers) {
		let internals = RegistersInternals {
				registers : StdVec::new (),
				count : 0,
				immutable : false,
				handle : registers_handles_next (),
			};
		return Registers (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_and_define (templates : &[RegisterTemplate], borrow : Option<&Registers>) -> (Outcome<Registers>) {
		let registers = Registers::new ();
		try! (registers.define_all (templates, borrow));
		succeed! (registers);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_value (&self, index : usize) -> (Outcome<Value>) {
		let self_0 = try! (self.internals_ref_mut ());
		let register = try_some! (self_0.registers.get (index), 0x89e68eab);
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_binding_option (&self, index : usize) -> (Outcome<Option<Binding>>) {
		let self_0 = try! (self.internals_ref ());
		let register = try_some! (self_0.registers.get (index), 0x371fc84b);
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_binding_create (&self, index : usize) -> (Outcome<Binding>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		let register = try_some! (self_0.registers.get_mut (index), 0x79873ff6);
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn initialize_value (&self, index : usize, value : Value) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		let register = try_some! (self_0.registers.get_mut (index), 0x7dabdbe0);
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn update_value (&self, index : usize, value : Value) -> (Outcome<Value>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xf97e0269);
		}
		let register = &mut self_0.registers[index];
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define (&self, template : &RegisterTemplate, borrow : Option<&Registers>) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		if self_0.immutable {
			fail! (0xd7cbcdd8);
		}
		let register = try! (Self::new_register (template, borrow));
		let index = self_0.count;
		self_0.registers.push (register);
		self_0.count += 1;
		succeed! (index);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn define_all (&self, templates : &[RegisterTemplate], borrow : Option<&Registers>) -> (Outcome<()>) {
		{
			let mut self_0 = try! (self.internals_ref_mut ());
			if self_0.immutable {
				fail! (0x74189c0f);
			}
			self_0.registers.reserve (templates.len ());
		}
		{
			for template in templates {
				try! (self.define (template, borrow));
			}
			succeed! (());
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn new_register (template : &RegisterTemplate, borrow : Option<&Registers>) -> (Outcome<Register>) {
		match *template {
			RegisterTemplate::Borrow (index) => {
				let borrow = try_some! (borrow, 0x2ac76d05);
				let binding = try! (borrow.resolve_binding_option (index));
				let binding = try_some! (binding, 0x2f543c30);
				let register = Register::Binding (binding);
				succeed! (register);
			},
			RegisterTemplate::LocalBinding (ref template) => {
				let binding = Binding::new_from_template (template);
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<RegistersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x53ff1eaa));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<RegistersInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0xadb3b247));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Registers) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
}




#[ derive (Clone) ]
pub struct Binding ( StdRc<StdRefCell<BindingInternals>> );


#[ derive (Clone, Debug) ]
pub struct BindingInternals {
	pub identifier : Option<Symbol>,
	pub value : Value,
	pub initialized : bool,
	pub immutable : bool,
	pub handle : Handle,
}


#[ derive (Clone, Debug) ]
pub struct BindingTemplate {
	pub identifier : Option<Symbol>,
	pub value : Option<Value>,
	pub immutable : bool,
}


impl Binding {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
				handle : binding_handles_next (),
			};
		return Binding (StdRc::new (StdRefCell::new (internals)));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_template (template : &BindingTemplate) -> (Binding) {
		return Binding::new (
				template.identifier.clone (),
				template.value.clone (),
				template.immutable,
			);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn get (&self) -> (Outcome<Value>) {
		let self_0 = try! (self.internals_ref ());
		if ! self_0.initialized {
			fail! (0x3e185e26);
		}
		succeed! (self_0.value.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn get_option (&self) -> (Outcome<Option<Value>>) {
		let self_0 = try! (self.internals_ref ());
		if self_0.initialized {
			succeed! (Some (self_0.value.clone ()));
		} else {
			succeed! (None);
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn initialize (&self, value : Value) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		// FIXME:  This breaks bencmarks!
		//if self_0.initialized {
		//	fail! (0x10d54f09);
		//}
		self_0.value = value;
		self_0.initialized = true;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set (&self, value : Value) -> (Outcome<Value>) {
		let mut self_0 = try! (self.internals_ref_mut ());
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_initialized (&self) -> (Outcome<bool>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.initialized);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_immutable (&self) -> (Outcome<bool>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.immutable);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_immutable (&self) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut ());
		self_0.immutable = true;
		succeed! (());
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<BindingInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x4140ef1c));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<BindingInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0x9adf65ce));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Binding) -> (bool) {
		return StdRc::ptr_eq (&self.0, &other.0);
	}
}

