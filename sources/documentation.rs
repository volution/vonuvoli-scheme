

use super::builtins::exports::*;
use super::errors::exports::*;
use super::parser::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub mod documentation_model {
		
		pub use super::super::{
				
				Libraries,
				Library,
				Category,
				Definition,
				DefinitionKind,
				ValueKind,
				
				Entity,
				EntityLink,
				
				Entities,
				EntitiesOwned,
				EntitiesLinked,
				
				Description,
				Links,
				
		};
		
	}
	
	pub use super::parse_library_specifications_for_builtins;
	pub use super::parse_library_specifications;
	
}




pub trait Entity {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier (&self) -> (&str) {
		return self.identifier_rc_ref () .deref () .deref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_clone (&self) -> (StdString) {
		return StdString::from (self.identifier ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		return StdRc::clone (self.identifier_rc_ref ());
	}
	
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>);
}




pub struct EntityLink <E : Entity> {
	identifier : StdRc<StdBox<str>>,
	entity : StdRefCell<Option<StdRc<E>>>,
}


impl <E : Entity> EntityLink<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (identifier : StdRc<StdBox<str>>) -> (EntityLink<E>) {
		return EntityLink {
				identifier,
				entity : StdRefCell::new (None),
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_resolved (entity : StdRc<E>) -> (EntityLink<E>) {
		let identifier = entity.identifier_rc_clone ();
		return EntityLink {
				identifier,
				entity : StdRefCell::new (Some (entity)),
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_link (&self, entity : StdRc<E>) -> (Outcome<()>) {
		let mut reference = try_or_fail! (self.entity.try_borrow_mut (), 0x8b92f9db);
		if let Some (ref current) = *reference {
			if StdRc::ptr_eq (current, &entity) {
				succeed! (());
			} else {
				fail! (0xa540c122);
			}
		}
		{
			*reference = Some (entity);
			succeed! (());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_link_from (&self, entities : &impl Entities<E>) -> (Outcome<()>) {
		if let Some (entity) = entities.entity_resolve_clone (self.identifier.deref () .deref ()) {
			return self.entity_link (entity);
		} else {
			fail! (0x31d6ae22);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_resolve (&self) -> (Outcome<Option<&E>>) {
		let reference = try_or_fail! (self.entity.try_borrow (), 0xf5fd3c1f);
		if let Some (ref reference) = reference.deref () {
			let reference : &E = reference.deref ();
			let reference = unsafe { mem::transmute ( reference ) };
			succeed! (Some (reference));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_resolve_clone (&self) -> (Outcome<Option<StdRc<E>>>) {
		let reference = try_or_fail! (self.entity.try_borrow (), 0xc69a4caa);
		succeed! (reference.clone ());
	}
}


impl <E : Entity> Entity for EntityLink<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl <E : Entity> ops::Deref for EntityLink<E> {
	
	type Target = E;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&E) {
		return try_some_or_panic! (try_or_panic! (self.entity_resolve ()), 0x6fd96e89);
	}
}




pub trait Entities <E : Entity> {
	
	// fn entities (&self) -> (impl iter::Iterator<Item = &E>);
	fn entity_resolve (&self, identifier : &str) -> (Option<&E>);
	fn entity_resolve_clone (&self, identifier : &str) -> (Option<StdRc<E>>);
	fn has_entities (&self) -> (bool);
}




pub struct EntitiesOwned <E : Entity> {
	entities : StdVec<StdRc<E>>,
	entities_index : StdMap<StdString, StdRc<E>>,
}


impl <E : Entity> Entities<E> for EntitiesOwned<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve (&self, identifier : &str) -> (Option<&E>) {
		return self.entities_index.get (identifier) .map (StdRc::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve_clone (&self, identifier : &str) -> (Option<StdRc<E>>) {
		return self.entities_index.get (identifier) .map (StdRc::clone);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn has_entities (&self) -> (bool) {
		return ! self.entities.is_empty ();
	}
}


impl <E : Entity> EntitiesOwned<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entities (&self) -> (impl iter::Iterator<Item = &E>) {
		return self.entities.iter () .map (StdRc::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include (&mut self, entity : StdRc<E>) -> (Outcome<()>) {
		if let Some (current) = self.entities_index.insert (entity.identifier_clone (), StdRc::clone (&entity)) {
			if StdRc::ptr_eq (&current, &entity) {
				succeed! (());
			} else {
				fail! (0x95ea3b1d);
			}
		}
		self.entities.push (entity);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (entities : impl iter::IntoIterator<Item = E>) -> (Outcome<EntitiesOwned<E>>) {
		let entities = entities.into_iter () .map (StdRc::new) .collect::<StdVec<_>> ();
		let mut entities_index = StdMap::with_capacity (entities.len ());
		for entity in entities.iter () {
			let identifier = entity.identifier_clone ();
			if let Some (_) = entities_index.insert (identifier, StdRc::clone (entity)) {
				fail! (0x8a2e7ff9);
			}
		}
		let entities = EntitiesOwned {
				entities,
				entities_index,
			};
		succeed! (entities);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_empty () -> (EntitiesOwned<E>) {
		return EntitiesOwned {
				entities : StdVec::new (),
				entities_index : StdMap::new (),
			};
	}
}




pub struct EntitiesLinked <E : Entity> {
	entities : StdVec<StdRc<EntityLink<E>>>,
	entities_index : StdMap<StdString, StdRc<EntityLink<E>>>,
}


impl <E : Entity> Entities<E> for EntitiesLinked<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve (&self, identifier : &str) -> (Option<&E>) {
		return self.entities_index.get (identifier) .map (StdRc::deref) .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve_clone (&self, identifier : &str) -> (Option<StdRc<E>>) {
		return self.entities_index.get (identifier) .map (StdRc::deref) .and_then (|entity| try_or_panic_0! (entity.entity_resolve_clone (), 0x67979ffd));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn has_entities (&self) -> (bool) {
		return ! self.entities.is_empty ();
	}
}


impl <E : Entity> EntitiesLinked<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entities (&self) -> (impl iter::Iterator<Item = &E>) {
		return self.entities.iter () .map (StdRc::deref) .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include (&mut self, entity : StdRc<E>) -> (Outcome<()>) {
		let entity = StdRc::new (EntityLink::new_resolved (entity));
		if let Some (current) = self.entities_index.insert (entity.identifier_clone (), StdRc::clone (&entity)) {
			let current = try_or_fail! (current.entity.try_borrow (), 0x0d7ace7b);
			let entity = try_or_fail! (entity.entity.try_borrow (), 0x3323ece9);
			let current = try_some! (current.as_ref (), 0xff248553);
			let entity = try_some! (entity.as_ref (), 0xbf8d4f3e);
			if StdRc::ptr_eq (current, entity) {
				succeed! (());
			} else {
				fail! (0xcc708d06);
			}
		}
		self.entities.push (entity);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (identifiers : impl iter::IntoIterator<Item = StdRc<StdBox<str>>>) -> (Outcome<EntitiesLinked<E>>) {
		let links = identifiers.into_iter () .map (|identifier| StdRc::new (EntityLink::new (identifier))) .collect::<StdVec<StdRc<EntityLink<E>>>> ();
		let mut links_index = StdMap::with_capacity (links.len ());
		for link in links.iter () {
			let identifier = link.identifier_clone ();
			if let Some (_) = links_index.insert (identifier, StdRc::clone (link)) {
				fail! (0xe6bdf0d7);
			}
		}
		let entities = EntitiesLinked {
				entities : links,
				entities_index : links_index,
			};
		succeed! (entities);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_empty () -> (EntitiesLinked<E>) {
		return EntitiesLinked {
				entities : StdVec::new (),
				entities_index : StdMap::new (),
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entities_link_from (&self, entities : &impl Entities<E>) -> (Outcome<()>) {
		for entity in self.entities.iter () {
			try! (entity.entity_link_from (entities));
		}
		succeed! (());
	}
}




pub struct Libraries {
	libraries : EntitiesOwned<Library>,
}


impl Libraries {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn libraries (&self) -> (impl iter::Iterator<Item = &Library>) {
		return self.libraries.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn library_resolve (&self, identifier : &str) -> (Option<&Library>) {
		return self.libraries.entity_resolve (identifier);
	}
}




pub struct Library {
	
	identifier : StdRc<StdBox<str>>,
	
	categories : EntitiesOwned<Category>,
	definitions : EntitiesOwned<Definition>,
	definitions_all : StdMap<StdString, StdRc<Definition>>,
	value_kinds : EntitiesOwned<ValueKind>,
	value_kinds_all : StdMap<StdString, StdRc<ValueKind>>,
	
	title : Option<StdRc<StdBox<str>>>,
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Library {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Library {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn category_resolve (&self, identifier : &str) -> (Option<&Category>) {
		return self.categories.entity_resolve (identifier);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definition_resolve (&self, identifier : &str) -> (Option<&Definition>) {
		return self.definitions.entity_resolve (identifier);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kind_resolve (&self, identifier : &str) -> (Option<&ValueKind>) {
		return self.value_kinds.entity_resolve (identifier);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_value_kinds (&self) -> (bool) {
		return self.value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (self) -> (Outcome<Library>) {
		
		let mut library = self;
		
		for category in library.categories.entities () {
			try! (category.link (&library));
		}
		for definition in library.definitions.entities () {
			try! (definition.link (&library));
		}
		for value_kind in library.value_kinds.entities () {
			try! (value_kind.link (&library));
		}
		
		let mut categories = mem::replace (&mut library.categories, EntitiesOwned::new_empty ());
		let definitions = mem::replace (&mut library.definitions, EntitiesOwned::new_empty ());
		let mut definitions_all = mem::replace (&mut library.definitions_all, StdMap::new ());
		let value_kinds = mem::replace (&mut library.value_kinds, EntitiesOwned::new_empty ());
		let mut value_kinds_all = mem::replace (&mut library.value_kinds_all, StdMap::new ());
		
		for definition in definitions.entities.iter () {
			if let Some (_) = definitions_all.insert (definition.identifier_clone (), StdRc::clone (definition)) {
				fail! (0x38d906bc);
			}
			for alias in definition.aliases.iter () {
				if let Some (_) = definitions_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (definition)) {
					fail! (0xd60c3f11);
				}
			}
			for category in definition.categories.entities.iter () {
				let category = try_some! (categories.entities_index.get_mut (category.identifier ()), 0xb9fdda59);
				let mut category : &Category = category.deref ();
				loop {
					{
						#[ allow (mutable_transmutes) ]
						let category : &mut Category = unsafe { mem::transmute (category) };
						try! (category.definitions.entity_include (StdRc::clone (definition)));
					}
					if let Some (ref parent) = category.parent {
						category = try_some! (try! (parent.entity_resolve ()), 0x3c4e0c61);
					} else {
						break;
					}
				}
			}
		}
		
		for value_kind in value_kinds.entities.iter () {
			if let Some (_) = value_kinds_all.insert (value_kind.identifier_clone (), StdRc::clone (value_kind)) {
				fail! (0xde87379f);
			}
			for alias in value_kind.aliases.iter () {
				if let Some (_) = value_kinds_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (value_kind)) {
					fail! (0x42f7f808);
				}
			}
			for category in value_kind.categories.entities.iter () {
				let category = try_some! (categories.entities_index.get_mut (category.identifier ()), 0xbcc12503);
				let mut category : &Category = category.deref ();
				loop {
					{
						#[ allow (mutable_transmutes) ]
						let category : &mut Category = unsafe { mem::transmute (category) };
						try! (category.value_kinds.entity_include (StdRc::clone (value_kind)));
					}
					if let Some (ref parent) = category.parent {
						category = try_some! (try! (parent.entity_resolve ()), 0x2ab4fdd1);
					} else {
						break;
					}
				}
			}
		}
		
		library.categories = categories;
		library.definitions = definitions;
		library.definitions_all = definitions_all;
		library.value_kinds = value_kinds;
		library.value_kinds_all = value_kinds_all;
		
		succeed! (library);
	}
}




pub struct Category {
	
	identifier : StdRc<StdBox<str>>,
	
	parent : Option<EntityLink<Category>>,
	
	definitions : EntitiesLinked<Definition>,
	value_kinds : EntitiesLinked<ValueKind>,
	
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Category {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Category {
	
	pub fn parent (&self) -> (Option<&Category>) {
		return self.parent.as_ref () .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (&self) -> (bool) {
		return self.parent.is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_value_kinds (&self) -> (bool) {
		return self.value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.definitions.entities_link_from (&library.definitions));
		try! (self.value_kinds.entities_link_from (&library.value_kinds));
		if let Some (ref parent) = self.parent {
			try! (parent.entity_link_from (&library.categories));
		}
		succeed! (());
	}
}




pub struct Definition {
	
	identifier : StdRc<StdBox<str>>,
	
	kind : DefinitionKind,
	categories : EntitiesLinked<Category>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Definition {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Definition {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (&DefinitionKind) {
		return &self.kind;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn aliases (&self) -> (impl iter::Iterator<Item = &str>) {
		return self.aliases.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_aliases (&self) -> (bool) {
		return ! self.aliases.is_empty ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.categories.entities_link_from (&library.categories));
		for alias in self.aliases.iter () {
			if let Some (_) = library.definitions.entity_resolve (alias) {
				fail! (0x73f2e1e7);
			}
		}
		succeed! (());
	}
}




#[ derive ( Copy, Clone ) ] // OK
pub enum DefinitionKind {
	
	Syntax,
	SyntaxAuxiliary,
	
	Procedure,
	ProcedureWithMutation,
	
	Predicate,
	TypePredicate,
	
	Comparator,
	
	ValueConstructor,
	ValueConverter,
	ValueAccessor,
	ValueMutator,
	ValueConstant,
	
	Parameter,
	
	Functor,
	
}


impl DefinitionKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn identifier (&self) -> (&str) {
		return match *self {
			
			DefinitionKind::Syntax => &"syntax",
			DefinitionKind::SyntaxAuxiliary => &"auxiliary-syntax",
			
			DefinitionKind::Procedure => &"procedure",
			DefinitionKind::ProcedureWithMutation => &"procedure!",
			
			DefinitionKind::Predicate => &"predicate",
			DefinitionKind::TypePredicate => &"type-predicate",
			
			DefinitionKind::Comparator => &"comparator",
			
			DefinitionKind::ValueConstructor => &"constructor",
			DefinitionKind::ValueConverter => &"converter",
			DefinitionKind::ValueAccessor => &"accessor",
			DefinitionKind::ValueMutator => &"mutator!",
			DefinitionKind::ValueConstant => &"constant",
			
			DefinitionKind::Parameter => &"parameter",
			
			DefinitionKind::Functor => &"functor",
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parent (&self) -> (Option<DefinitionKind>) {
		return match *self {
			
			DefinitionKind::Syntax => None,
			DefinitionKind::SyntaxAuxiliary => None,
			
			DefinitionKind::Procedure => None,
			DefinitionKind::ProcedureWithMutation => Some (DefinitionKind::Procedure),
			
			DefinitionKind::Predicate => Some (DefinitionKind::Procedure),
			DefinitionKind::TypePredicate => Some (DefinitionKind::Predicate),
			
			DefinitionKind::Comparator => Some (DefinitionKind::Predicate),
			
			DefinitionKind::ValueConstructor => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueConverter => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueAccessor => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueMutator => Some (DefinitionKind::ProcedureWithMutation),
			DefinitionKind::ValueConstant => None,
			
			DefinitionKind::Parameter => Some (DefinitionKind::ValueConstant),
			
			DefinitionKind::Functor => Some (DefinitionKind::Procedure),
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents (&self) -> (impl iter::Iterator<Item = DefinitionKind>) {
		struct Parents (Option<DefinitionKind>);
		impl iter::Iterator for Parents {
			type Item = DefinitionKind;
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn next (&mut self) -> (Option<DefinitionKind>) {
				if let Some (current) = self.0 {
					let parent = current.parent ();
					self.0 = parent;
				}
				return self.0;
			}
		}
		return Parents (Some (*self));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (&self) -> (bool) {
		return self.parent () .is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve (identifier : &str) -> (Outcome<DefinitionKind>) {
		succeed! (match identifier {
			
			"syntax" =>
				DefinitionKind::Syntax,
			"auxiliary-syntax" =>
				DefinitionKind::SyntaxAuxiliary,
			
			"procedure" =>
				DefinitionKind::Procedure,
			"procedure!" =>
				DefinitionKind::ProcedureWithMutation,
			
			"predicate" =>
				DefinitionKind::Predicate,
			"type-predicate" =>
				DefinitionKind::TypePredicate,
			
			"comparator" |
			"comparator=" |
			"comparator<" |
			"comparator>" |
			"comparator<=" |
			"comparator>=" =>
				DefinitionKind::Comparator,
			
			"constructor" =>
				DefinitionKind::ValueConstructor,
			"converter" =>
				DefinitionKind::ValueConverter,
			"accessor" =>
				DefinitionKind::ValueAccessor,
			"mutator!" =>
				DefinitionKind::ValueMutator,
			"constant" =>
				DefinitionKind::ValueConstant,
			
			"map" |
			"for-each" |
			"fold" =>
				DefinitionKind::Functor,
			
			"parameter" =>
				DefinitionKind::Parameter,
			
			_ =>
				fail! (0x417c9e8a),
			
		});
	}
}




pub struct ValueKind {
	
	identifier : StdRc<StdBox<str>>,
	
	parent : Option<EntityLink<ValueKind>>,
	
	categories : EntitiesLinked<Category>,
	definitions : EntitiesLinked<Definition>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	predicate : Option<Value>,
	
}


impl Entity for ValueKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl ValueKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parent (&self) -> (Option<&ValueKind>) {
		return self.parent.as_ref () .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (&self) -> (bool) {
		return self.parent.is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn aliases (&self) -> (impl iter::Iterator<Item = &str>) {
		return self.aliases.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_aliases (&self) -> (bool) {
		return ! self.aliases.is_empty ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn predicate (&self) -> (Option<&Value>) {
		return self.predicate.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		if let Some (ref parent) = self.parent {
			try! (parent.entity_link_from (&library.value_kinds));
		}
		try! (self.categories.entities_link_from (&library.categories));
		try! (self.definitions.entities_link_from (&library.definitions));
		for alias in self.aliases.iter () {
			if let Some (_) = library.value_kinds.entity_resolve (alias) {
				fail! (0x12252744);
			}
		}
		succeed! (());
	}
}




pub struct Description {
	lines : StdVec<StdRc<StdBox<str>>>,
}


impl Description {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lines : StdVec<StdRc<StdBox<str>>>) -> (Description) {
		return Description {
				lines : lines,
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn lines (&self) -> (impl iter::Iterator<Item = &str>) {
		return self.lines.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
}




pub struct Links {
	links : StdVec<StdRc<StdBox<str>>>,
}


impl Links {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (links : StdVec<StdRc<StdBox<str>>>) -> (Links) {
		return Links {
				links : links,
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (impl iter::Iterator<Item = &str>) {
		return self.links.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_library_specifications (input : &str) -> (Outcome<Libraries>) {
	
	let inputs = try! (parse_values (input, None));
	
	let libraries = try_vec_map_into! (inputs, input, parse_library (input));
	let libraries = try! (EntitiesOwned::new (libraries));
	
	let libraries = Libraries {
			libraries,
		};
	
	succeed! (libraries);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_library (input : Value) -> (Outcome<Library>) {
	
	let (_, attributes) = try! (parse_object_with_attributes (input, Some ("library"), false));
	
	let mut identifier = None;
	let mut categories_input = None;
	let mut definitions_input = None;
	let mut value_kinds_input = None;
	let mut title = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes.into_iter () {
		match attribute.deref () .as_ref () {
			
			"identifier" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				identifier = Some (token.string_rc_clone ());
			},
			
			"categories" => {
				categories_input = Some (tokens);
			},
			"definitions" => {
				definitions_input = Some (tokens);
			},
			"types" => {
				value_kinds_input = Some (tokens);
			},
			
			"title" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_string_immutable! (token);
				title = Some (token.string_rc_clone ());
			},
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			_ =>
				fail! (0x9c7a1941),
			
		}
	}
	
	let identifier = try_some! (identifier, 0x70cdea2b);
	
	let categories = if let Some (inputs) = categories_input {
		try! (parse_list_of (inputs, parse_category))
	} else {
		StdVec::new ()
	};
	let categories = try! (EntitiesOwned::new (categories));
	
	let definitions = if let Some (inputs) = definitions_input {
		try! (parse_list_of (inputs, parse_definition))
	} else {
		StdVec::new ()
	};
	let definitions = try! (EntitiesOwned::new (definitions));
	
	let value_kinds = if let Some (inputs) = value_kinds_input {
		try! (parse_list_of (inputs, parse_value_kind))
	} else {
		StdVec::new ()
	};
	let value_kinds = try! (EntitiesOwned::new (value_kinds));
	
	let library = Library {
			identifier,
			categories,
			definitions,
			definitions_all : StdMap::new (),
			value_kinds,
			value_kinds_all : StdMap::new (),
			title,
			description,
			links,
		};
	
	let library = try! (library.link ());
	
	succeed! (library);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_category (input : Value) -> (Outcome<Category>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0xb2b59df4);
	
	let mut parent = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes.into_iter () {
		match attribute.deref () .as_ref () {
			
			"parent" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				parent = Some (token.string_rc_clone ());
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			_ =>
				fail! (0x06977abb),
			
		}
	}
	
	let parent = option_map! (parent, EntityLink::new (parent));
	
	let category = Category {
			identifier,
			parent,
			definitions : EntitiesLinked::new_empty (),
			value_kinds : EntitiesLinked::new_empty (),
			description,
			links,
		};
	
	succeed! (category);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_definition (input : Value) -> (Outcome<Definition>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0x5181cc5e);
	
	let mut kind = None;
	let mut categories = None;
	let mut aliases = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes.into_iter () {
		match attribute.deref () .as_ref () {
			
			"type" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				kind = Some (try! (DefinitionKind::resolve (token.string_as_str ())));
			},
			
			"category" | "categories" => {
				categories = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"alias" | "aliases" => {
				aliases = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			_ =>
				fail! (0x24ac563a),
			
		}
	}
	
	let kind = try_some! (kind, 0x74b6b39e);
	
	let categories = try_some! (categories, 0x113cac3d);
	let categories = try! (EntitiesLinked::new (categories));
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let definition = Definition {
			identifier,
			kind,
			categories,
			aliases,
			description,
			links,
		};
	
	succeed! (definition);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_value_kind (input : Value) -> (Outcome<ValueKind>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0x6ad37e55);
	
	let mut parent = None;
	let mut categories = None;
	let mut aliases = None;
	let mut description = None;
	let mut links = None;
	let mut predicate = None;
	
	for (attribute, tokens) in attributes.into_iter () {
		match attribute.deref () .as_ref () {
			
			"parent" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				parent = Some (token.string_rc_clone ());
			},
			
			"category" | "categories" => {
				categories = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"alias" | "aliases" => {
				aliases = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			"predicate" => {
				let token = try! (vec_explode_1 (tokens));
				predicate = Some (token);
			},
			
			_ =>
				fail! (0x9e7c02e8),
			
		}
	}
	
	let parent = option_map! (parent, EntityLink::new (parent));
	
	let categories = try_some! (categories, 0x113cac3d);
	let categories = try! (EntitiesLinked::new (categories));
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let value_kind = ValueKind {
			identifier,
			parent,
			categories,
			definitions : EntitiesLinked::new_empty (),
			aliases,
			description,
			links,
			predicate,
		};
	
	succeed! (value_kind);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_object_with_attributes (input : Value, keyword : Option<&str>, identifier_expected : bool) -> (Outcome<(Option<StdRc<StdBox<str>>>, StdVec<(StdRc<StdBox<str>>, StdVec<Value>)>)>) {
	
	let tokens = try! (vec_list_clone (&input));
	
	let tokens = if let Some (keyword) = keyword {
		let (head, rest) = try! (vec_explode_1n (tokens));
		let head = try_into_symbol! (head);
		if ! str::eq (head.string_as_str (), keyword) {
			fail! (0x3ec7c223);
		}
		rest
	} else {
		tokens
	};
	
	let (identifier, tokens) = if identifier_expected {
		let (head, rest) = try! (vec_explode_1n (tokens));
		let identifier = try_into_symbol! (head);
		let identifier = identifier.string_rc_clone ();
		(Some (identifier), rest)
	} else {
		(None, tokens)
	};
	
	let mut attributes = StdMap::with_capacity (tokens.len ());
	for tokens in tokens.into_iter () {
		let tokens = try! (vec_list_clone (&tokens));
		let (head, rest) = try! (vec_explode_1n (tokens));
		let identifier = try_into_symbol! (head);
		let identifier = identifier.string_rc_clone ();
		if let Some (_) = attributes.insert (identifier, rest) {
			fail! (0x9a98dec4);
		}
	}
	
	let attributes = attributes.into_iter () .collect ();
	
	succeed! ((identifier, attributes));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_description (_input : StdVec<Value>) -> (Outcome<Description>) {
	fail_unimplemented! (0x5ca7dcd4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_links (_input : StdVec<Value>) -> (Outcome<Links>) {
	fail_unimplemented! (0xd3359173);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_list_of <T> (input : StdVec<Value>, parser : impl Fn (Value) -> (Outcome<T>)) -> (Outcome<StdVec<T>>) {
	let output = try! (input.into_iter () .map (|input| parser (input)) .collect ());
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_library_specifications_for_builtins () -> (Outcome<Libraries>) {
	return parse_library_specifications (LIBRARY_SPECIFICATIONS_SOURCES);
}

static LIBRARY_SPECIFICATIONS_SOURCES : &'static str = include_str! ("../documentation/libraries.ss");

