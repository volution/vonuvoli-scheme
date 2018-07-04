

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
				
				ProcedureSignature,
				ProcedureSignatureVariant,
				ProcedureSignatureValues,
				ProcedureSignatureValue,
				
				SyntaxSignature,
				SyntaxSignatureVariant,
				SyntaxSignatureKeyword,
				SyntaxSignaturePattern,
				
				Entity,
				EntityLink,
				
				Entities,
				EntitiesOwned,
				EntitiesLinked,
				
				Appendix,
				
				Description,
				Links,
				Link,
				
				Features,
				
		};
		
	}
	
	#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
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
	pub fn new_linked (identifier : StdRc<StdBox<str>>) -> (EntityLink<E>) {
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
		for entity in &entities {
			let identifier = entity.identifier_clone ();
			if entities_index.insert (identifier, StdRc::clone (entity)) .is_some () {
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
	fn entity_include_resolved (&mut self, entity : StdRc<E>) -> (Outcome<()>) {
		let entity = StdRc::new (EntityLink::new_resolved (entity));
		return self.entity_include (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include_linked (&mut self, identifier : StdRc<StdBox<str>>) -> (Outcome<()>) {
		let entity = StdRc::new (EntityLink::new_linked (identifier));
		return self.entity_include (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include (&mut self, entity : StdRc<EntityLink<E>>) -> (Outcome<()>) {
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
		let links = identifiers.into_iter () .map (|identifier| StdRc::new (EntityLink::new_linked (identifier))) .collect::<StdVec<StdRc<EntityLink<E>>>> ();
		let mut links_index = StdMap::with_capacity (links.len ());
		for link in &links {
			let identifier = link.identifier_clone ();
			if links_index.insert (identifier, StdRc::clone (link)) .is_some () {
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
		for entity in &self.entities {
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
	
	appendices : EntitiesOwned<Appendix>,
	
	features : Option<Features>,
	
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
	pub fn definitions_all (&self) -> (impl iter::Iterator<Item = (&str, &Definition)>) {
		return self.definitions_all.iter () .map (|(alias, entity)| (alias.deref (), entity.deref ()));
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
	pub fn value_kinds_all (&self) -> (impl iter::Iterator<Item = (&str, &ValueKind)>) {
		return self.value_kinds_all.iter () .map (|(alias, entity)| (alias.deref (), entity.deref ()));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn title (&self) -> (Option<&str>) {
		return self.title.as_ref () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn appendices (&self) -> (impl iter::Iterator<Item = &Appendix>) {
		return self.appendices.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn appendix_resolve (&self, identifier : &str) -> (Option<&Appendix>) {
		return self.appendices.entity_resolve (identifier);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_appendices (&self) -> (bool) {
		return self.appendices.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
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
		
		let categories = mem::replace (&mut library.categories, EntitiesOwned::new_empty ());
		let definitions = mem::replace (&mut library.definitions, EntitiesOwned::new_empty ());
		let mut definitions_all = mem::replace (&mut library.definitions_all, StdMap::new ());
		let value_kinds = mem::replace (&mut library.value_kinds, EntitiesOwned::new_empty ());
		let mut value_kinds_all = mem::replace (&mut library.value_kinds_all, StdMap::new ());
		
		for category in categories.entities () {
			let category_rc = try_some! (categories.entities_index.get (category.identifier ()), 0x7388978c);
			let category : &Category = category_rc.deref ();
			#[ allow (mutable_transmutes) ]
			#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
			let category_mut : &mut Category = unsafe { mem::transmute (category) };
			if let Some (parent) = &category.parent {
				let parent_rc = try_some! (categories.entities_index.get (parent.identifier ()), 0x2071fca3);
				let parent : &Category = parent_rc.deref ();
				#[ allow (mutable_transmutes) ]
				#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
				let parent_mut : &mut Category = unsafe { mem::transmute (parent) };
				try! (parent_mut.children.entity_include_resolved (StdRc::clone (category_rc)));
			}
			let mut parent_super = &category.parent;
			while let Some (parent) = parent_super {
				let parent_rc = try_some! (categories.entities_index.get (parent.identifier ()), 0x50a2c779);
				let parent : &Category = parent_rc.deref ();
				#[ allow (mutable_transmutes) ]
				#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
				let parent_mut : &mut Category = unsafe { mem::transmute (parent) };
				try! (parent_mut.children_all.entity_include_resolved (StdRc::clone (category_rc)));
				try! (category_mut.parents_all.entity_include_resolved (StdRc::clone (parent_rc)));
				parent_super = &parent.parent;
			}
		}
		
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try_some! (value_kinds.entities_index.get (value_kind.identifier ()), 0x8d7fe454);
			let value_kind : &ValueKind = value_kind_rc.deref ();
			#[ allow (mutable_transmutes) ]
			#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
			let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
			if let Some (parent) = &value_kind.parent {
				let parent_rc = try_some! (value_kinds.entities_index.get (parent.identifier ()), 0x058d3b3d);
				let parent : &ValueKind = parent_rc.deref ();
				#[ allow (mutable_transmutes) ]
				#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
				let parent_mut : &mut ValueKind = unsafe { mem::transmute (parent) };
				try! (parent_mut.children.entity_include_resolved (StdRc::clone (value_kind_rc)));
			}
			let mut parent_super = &value_kind.parent;
			while let Some (parent) = parent_super {
				let parent_rc = try_some! (value_kinds.entities_index.get (parent.identifier ()), 0x058d3b3d);
				let parent : &ValueKind = parent_rc.deref ();
				#[ allow (mutable_transmutes) ]
				#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
				let parent_mut : &mut ValueKind = unsafe { mem::transmute (parent) };
				try! (parent_mut.children_all.entity_include_resolved (StdRc::clone (value_kind_rc)));
				try! (value_kind_mut.parents_all.entity_include_resolved (StdRc::clone (parent_rc)));
				parent_super = &parent.parent;
			}
		}
		
		for definition_rc in &definitions.entities {
			let definition = definition_rc.deref ();
			if definitions_all.insert (definition.identifier_clone (), StdRc::clone (definition_rc)) .is_some () {
				fail! (0x38d906bc);
			}
			for alias in &definition.aliases {
				if definitions_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (definition_rc)) .is_some () {
					fail! (0xd60c3f11);
				}
			}
			for category in &definition.categories.entities {
				{
					let category_rc = try_some! (categories.entities_index.get (category.identifier ()), 0xb9fdda59);
					let category : &Category = category_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let category_mut : &mut Category = unsafe { mem::transmute (category) };
					try! (category_mut.definitions.entity_include_resolved (StdRc::clone (definition_rc)));
					try! (category_mut.definitions_all.entity_include_resolved (StdRc::clone (definition_rc)));
				}
				let mut category_super = &category.parent;
				while let Some (category) = category_super {
					let category_rc = try_some! (categories.entities_index.get (category.identifier ()), 0xb9fdda59);
					let category : &Category = category_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let category_mut : &mut Category = unsafe { mem::transmute (category) };
					try! (category_mut.definitions_all.entity_include_resolved (StdRc::clone (definition_rc)));
					category_super = &category.parent;
				}
			}
			if let Some (procedure_signature) = &definition.procedure_signature {
				for procedure_signature_variant in procedure_signature.variants.iter () {
					for procedure_signature_value in procedure_signature_variant.inputs.values.iter () {
						let value_kind = try_some_2! (procedure_signature_value.kind.entity_resolve_clone (), 0x9d070ae8);
						{
							let value_kind : &ValueKind = value_kind.deref ();
							#[ allow (mutable_transmutes) ]
							#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
							let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
							try! (value_kind_mut.definitions_input.entity_include_resolved (StdRc::clone (definition_rc)));
							try! (value_kind_mut.definitions_input_all.entity_include_resolved (StdRc::clone (definition_rc)));
						}
						{
							#[ allow (mutable_transmutes) ]
							#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
							let definition_mut : &mut Definition = unsafe { mem::transmute (definition) };
							try! (definition_mut.referenced_value_kinds.entity_include_resolved (value_kind));
						}
					}
					for procedure_signature_value in procedure_signature_variant.outputs.values.iter () {
						let value_kind = try_some_2! (procedure_signature_value.kind.entity_resolve_clone (), 0x921afcde);
						{
							let value_kind : &ValueKind = value_kind.deref ();
							#[ allow (mutable_transmutes) ]
							#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
							let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
							try! (value_kind_mut.definitions_output.entity_include_resolved (StdRc::clone (definition_rc)));
							try! (value_kind_mut.definitions_output_all.entity_include_resolved (StdRc::clone (definition_rc)));
						}
						{
							#[ allow (mutable_transmutes) ]
							#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
							let definition_mut : &mut Definition = unsafe { mem::transmute (definition) };
							try! (definition_mut.referenced_value_kinds.entity_include_resolved (value_kind));
						}
					}
				}
			}
			if let Some (syntax_signature) = &definition.syntax_signature {
				for syntax_signature_keyword in syntax_signature.keywords.iter () {
					match syntax_signature_keyword.deref () {
						SyntaxSignatureKeyword::Value { kind : Some (value_kind), .. } => {
							let value_kind = try_some_2! (value_kind.entity_resolve_clone (), 0x5c2f1e13);
							{
								let value_kind : &ValueKind = value_kind.deref ();
								#[ allow (mutable_transmutes) ]
								#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
								let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
								try! (value_kind_mut.definitions_input.entity_include_resolved (StdRc::clone (definition_rc)));
								try! (value_kind_mut.definitions_input_all.entity_include_resolved (StdRc::clone (definition_rc)));
							}
							{
								#[ allow (mutable_transmutes) ]
								#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
								let definition_mut : &mut Definition = unsafe { mem::transmute (definition) };
								try! (definition_mut.referenced_value_kinds.entity_include_resolved (value_kind));
							}
						}
						_ =>
							(),
					}
				}
			}
		}
		
		for value_kind_rc in &value_kinds.entities {
			let value_kind = value_kind_rc.deref ();
			if value_kinds_all.insert (value_kind.identifier_clone (), StdRc::clone (value_kind_rc)) .is_some () {
				fail! (0xde87379f);
			}
			for alias in &value_kind.aliases {
				if value_kinds_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (value_kind_rc)) .is_some () {
					fail! (0x42f7f808);
				}
			}
			for category in &value_kind.categories.entities {
				{
					let category_rc = try_some! (categories.entities_index.get (category.identifier ()), 0xbcc12503);
					let category : &Category = category_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let category_mut : &mut Category = unsafe { mem::transmute (category) };
					try! (category_mut.value_kinds.entity_include_resolved (StdRc::clone (value_kind_rc)));
					try! (category_mut.value_kinds_all.entity_include_resolved (StdRc::clone (value_kind_rc)));
				}
				let mut category_super = &category.parent;
				while let Some (category) = category_super {
					let category_rc = try_some! (categories.entities_index.get (category.identifier ()), 0xbcc12503);
					let category : &Category = category_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let category_mut : &mut Category = unsafe { mem::transmute (category) };
					try! (category_mut.value_kinds_all.entity_include_resolved (StdRc::clone (value_kind_rc)));
					category_super = &category.parent;
				}
			}
			for definition in &value_kind.definitions_input.entities {
				let definition_rc = try_some! (definitions.entities_index.get (definition.identifier ()), 0xb26f82e8);
				for value_kind in &value_kind.children_all.entities {
					let value_kind_rc = try_some! (value_kinds.entities_index.get (value_kind.identifier ()), 0xb26cffca);
					let value_kind : &ValueKind = value_kind_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
					try! (value_kind_mut.definitions_input_all.entity_include_resolved (StdRc::clone (definition_rc)));
				}
			}
			for definition in &value_kind.definitions_output.entities {
				let definition_rc = try_some! (definitions.entities_index.get (definition.identifier ()), 0x4b9dd1ab);
				for value_kind in &value_kind.parents_all.entities {
					let value_kind_rc = try_some! (value_kinds.entities_index.get (value_kind.identifier ()), 0xc358af3b);
					let value_kind : &ValueKind = value_kind_rc.deref ();
					#[ allow (mutable_transmutes) ]
					#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (transmute_ptr_to_ptr) ) ]
					let value_kind_mut : &mut ValueKind = unsafe { mem::transmute (value_kind) };
					try! (value_kind_mut.definitions_output_all.entity_include_resolved (StdRc::clone (definition_rc)));
				}
			}
		}
		
		library.categories = categories;
		library.definitions = definitions;
		library.definitions_all = definitions_all;
		library.value_kinds = value_kinds;
		library.value_kinds_all = value_kinds_all;
		
		for category in library.categories.entities () {
			try! (category.link (&library));
		}
		for definition in library.definitions.entities () {
			try! (definition.link (&library));
		}
		for value_kind in library.value_kinds.entities () {
			try! (value_kind.link (&library));
		}
		
		succeed! (library);
	}
}




pub struct Category {
	
	identifier : StdRc<StdBox<str>>,
	
	parent : Option<EntityLink<Category>>,
	parents_all : EntitiesLinked<Category>,
	children : EntitiesLinked<Category>,
	children_all : EntitiesLinked<Category>,
	
	definitions : EntitiesLinked<Definition>,
	definitions_all : EntitiesLinked<Definition>,
	value_kinds : EntitiesLinked<ValueKind>,
	value_kinds_all : EntitiesLinked<ValueKind>,
	
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parent (&self) -> (Option<&Category>) {
		return self.parent.as_ref () .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.parents_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (&self) -> (bool) {
		return self.parent.is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.children.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children_recursive (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.children_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_children (&self) -> (bool) {
		return self.children.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_recursive (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions_all.entities ();
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
	pub fn value_kinds_recursive (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.value_kinds_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_value_kinds (&self) -> (bool) {
		return self.value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		if let Some (ref parent) = self.parent {
			try! (parent.entity_link_from (&library.categories));
		}
		try! (self.parents_all.entities_link_from (&library.categories));
		try! (self.children.entities_link_from (&library.categories));
		try! (self.children_all.entities_link_from (&library.categories));
		try! (self.definitions.entities_link_from (&library.definitions));
		try! (self.definitions_all.entities_link_from (&library.definitions));
		try! (self.value_kinds.entities_link_from (&library.value_kinds));
		try! (self.value_kinds_all.entities_link_from (&library.value_kinds));
		succeed! (());
	}
}




pub struct Definition {
	
	identifier : StdRc<StdBox<str>>,
	
	kind : DefinitionKind,
	categories : EntitiesLinked<Category>,
	categories_all : EntitiesLinked<Category>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	procedure_signature : Option<ProcedureSignature>,
	syntax_signature : Option<SyntaxSignature>,
	
	referenced_value_kinds : EntitiesLinked<ValueKind>,
	
	features : Option<Features>,
	
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
	pub fn categories_recursive (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories_all.entities ();
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
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn procedure_signature (&self) -> (Option<&ProcedureSignature>) {
		return self.procedure_signature.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn syntax_signature (&self) -> (Option<&SyntaxSignature>) {
		return self.syntax_signature.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn referenced_value_kinds (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.referenced_value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_referenced_value_kinds (&self) -> (bool) {
		return self.referenced_value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.categories.entities_link_from (&library.categories));
		if let Some (ref procedure_signature) = self.procedure_signature {
			try! (procedure_signature.link (&library.value_kinds));
		}
		if let Some (ref syntax_signature) = self.syntax_signature {
			try! (syntax_signature.link (&library.value_kinds));
		}
		try! (self.referenced_value_kinds.entities_link_from (&library.value_kinds));
		for alias in &self.aliases {
			if library.definitions.entity_resolve (alias) .is_some () {
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
	pub fn identifier (self) -> (&'static str) {
		return match self {
			
			DefinitionKind::Syntax => "syntax",
			DefinitionKind::SyntaxAuxiliary => "auxiliary-syntax",
			
			DefinitionKind::Procedure => "procedure",
			DefinitionKind::ProcedureWithMutation => "procedure!",
			
			DefinitionKind::Predicate => "predicate",
			DefinitionKind::TypePredicate => "type-predicate",
			
			DefinitionKind::Comparator => "comparator",
			
			DefinitionKind::ValueConstructor => "constructor",
			DefinitionKind::ValueConverter => "converter",
			DefinitionKind::ValueAccessor => "accessor",
			DefinitionKind::ValueMutator => "mutator!",
			DefinitionKind::ValueConstant => "constant",
			
			DefinitionKind::Parameter => "parameter",
			
			DefinitionKind::Functor => "functor",
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parent (self) -> (Option<DefinitionKind>) {
		return match self {
			
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
	pub fn is_procedure (self) -> (bool) {
		return match self {
			
			DefinitionKind::Syntax => false,
			DefinitionKind::SyntaxAuxiliary => false,
			
			DefinitionKind::Procedure => true,
			DefinitionKind::ProcedureWithMutation => true,
			
			DefinitionKind::Predicate => true,
			DefinitionKind::TypePredicate => true,
			
			DefinitionKind::Comparator => true,
			
			DefinitionKind::ValueConstructor => true,
			DefinitionKind::ValueConverter => true,
			DefinitionKind::ValueAccessor => true,
			DefinitionKind::ValueMutator => true,
			DefinitionKind::ValueConstant => true,
			
			DefinitionKind::Parameter => true,
			
			DefinitionKind::Functor => true,
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_syntax (self) -> (bool) {
		return match self {
			
			DefinitionKind::Syntax => true,
			DefinitionKind::SyntaxAuxiliary => false,
			
			DefinitionKind::Procedure => false,
			DefinitionKind::ProcedureWithMutation => false,
			
			DefinitionKind::Predicate => false,
			DefinitionKind::TypePredicate => false,
			
			DefinitionKind::Comparator => false,
			
			DefinitionKind::ValueConstructor => false,
			DefinitionKind::ValueConverter => false,
			DefinitionKind::ValueAccessor => false,
			DefinitionKind::ValueMutator => false,
			DefinitionKind::ValueConstant => false,
			
			DefinitionKind::Parameter => false,
			
			DefinitionKind::Functor => false,
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (self) -> (impl iter::Iterator<Item = DefinitionKind>) {
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
		return Parents (Some (self));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (self) -> (bool) {
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
	parents_all : EntitiesLinked<ValueKind>,
	children : EntitiesLinked<ValueKind>,
	children_all : EntitiesLinked<ValueKind>,
	
	categories : EntitiesLinked<Category>,
	categories_all : EntitiesLinked<Category>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	predicate : Option<Value>,
	
	features : Option<Features>,
	
	definitions_input : EntitiesLinked<Definition>,
	definitions_input_all : EntitiesLinked<Definition>,
	definitions_output : EntitiesLinked<Definition>,
	definitions_output_all : EntitiesLinked<Definition>,
	
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
	pub fn parents_recursive (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.parents_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (&self) -> (bool) {
		return self.parent.is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.children.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children_recursive (&self) -> (impl iter::Iterator<Item = &ValueKind>) {
		return self.children_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_children (&self) -> (bool) {
		return self.children.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories_recursive (&self) -> (impl iter::Iterator<Item = &Category>) {
		return self.categories_all.entities ();
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
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn predicate (&self) -> (Option<&Value>) {
		return self.predicate.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_input (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions_input.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_input_recursive (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions_input_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions_input (&self) -> (bool) {
		return self.definitions_input.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_output (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions_output.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_output_recursive (&self) -> (impl iter::Iterator<Item = &Definition>) {
		return self.definitions_output_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions_output (&self) -> (bool) {
		return self.definitions_output.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		if let Some (ref parent) = self.parent {
			try! (parent.entity_link_from (&library.value_kinds));
		}
		try! (self.parents_all.entities_link_from (&library.value_kinds));
		try! (self.children.entities_link_from (&library.value_kinds));
		try! (self.children_all.entities_link_from (&library.value_kinds));
		try! (self.categories.entities_link_from (&library.categories));
		try! (self.categories_all.entities_link_from (&library.categories));
		try! (self.definitions_input.entities_link_from (&library.definitions));
		try! (self.definitions_input_all.entities_link_from (&library.definitions));
		try! (self.definitions_output.entities_link_from (&library.definitions));
		try! (self.definitions_output_all.entities_link_from (&library.definitions));
		for alias in &self.aliases {
			if library.value_kinds.entity_resolve (alias) .is_some () {
				fail! (0x12252744);
			}
		}
		succeed! (());
	}
}




pub struct ProcedureSignature {
	pub variants : StdBox<[ProcedureSignatureVariant]>,
}

pub struct ProcedureSignatureVariant {
	pub inputs : ProcedureSignatureValues,
	pub outputs : ProcedureSignatureValues,
	pub features : Option<Features>,
}

pub struct ProcedureSignatureValues {
	pub values : StdBox<[ProcedureSignatureValue]>,
	pub variadic : bool,
}

pub struct ProcedureSignatureValue {
	pub identifier : Option<StdRc<StdBox<str>>>,
	pub kind : EntityLink<ValueKind>,
}


impl ProcedureSignature {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for variant in self.variants.iter () {
			try! (variant.link (value_kinds));
		}
		succeed! (());
	}
}


impl ProcedureSignatureVariant {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		try! (self.inputs.link (value_kinds));
		try! (self.outputs.link (value_kinds));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		return list_build_3 (
				& self.inputs.format (),
				& symbol_clone_str ("->") .into (),
				& self.outputs.format (),
				None,
				Some (true));
	}
}


impl ProcedureSignatureValues {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for value in self.values.iter () {
			try! (value.link (value_kinds));
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		let mut tokens = StdVec::with_capacity (self.values.len ());
		for value in self.values.iter () {
			tokens.push (value.format ());
		}
		if self.variadic {
			tokens.push (symbol_clone_str ("...") .into ());
		}
		let tokens = list_collect (tokens, Some (true));
		return tokens;
	}
}


impl ProcedureSignatureValue {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		try! (self.kind.entity_link_from (value_kinds));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		let kind_token = symbol_from_rc (self.kind.identifier_rc_clone ());
		if let Some (identifier) = &self.identifier {
			return pair_new (
					symbol_from_rc (StdRc::clone (identifier)) .into (),
					kind_token.into (),
					Some (true));
		} else {
			return kind_token.into ();
		}
	}
}




pub struct SyntaxSignature {
	pub keywords : StdBox<[StdRc<SyntaxSignatureKeyword>]>,
	pub keywords_map : StdMap<StdString, StdRc<SyntaxSignatureKeyword>>,
	pub variants : StdBox<[SyntaxSignatureVariant]>,
}

pub enum SyntaxSignatureKeyword {
	
	Literal (StdRc<StdBox<str>>),
	Identifier (StdRc<StdBox<str>>),
	Expression (StdRc<StdBox<str>>),
	
	Constant {
		identifier : StdRc<StdBox<str>>,
		value : Value,
	},
	Value {
		identifier : StdRc<StdBox<str>>,
		kind : Option<EntityLink<ValueKind>>,
	},
	
	Pattern {
		identifier : StdRc<StdBox<str>>,
		patterns : StdBox<[SyntaxSignaturePattern]>,
	},
	
}

pub struct SyntaxSignatureVariant {
	pub pattern : SyntaxSignaturePattern,
}

pub enum SyntaxSignaturePattern {
	List (StdBox<[SyntaxSignaturePattern]>, Option<StdBox<SyntaxSignaturePattern>>),
	Keyword (StdRc<SyntaxSignatureKeyword>),
	Variadic (StdBox<SyntaxSignaturePattern>),
	SyntaxIdentifier,
	SyntaxRules,
	SyntaxTransformer,
}


impl SyntaxSignature {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for keyword in self.keywords.iter () {
			try! (keyword.link (value_kinds));
		}
		succeed! (());
	}
}


impl SyntaxSignatureKeyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		match self {
			
			SyntaxSignatureKeyword::Literal (_) =>
				succeed! (()),
			SyntaxSignatureKeyword::Identifier (_) =>
				succeed! (()),
			SyntaxSignatureKeyword::Expression (_) =>
				succeed! (()),
			
			SyntaxSignatureKeyword::Constant { .. } =>
				succeed! (()),
			SyntaxSignatureKeyword::Value { kind, .. } => {
				if let Some (kind) = kind {
					try! (kind.entity_link_from (value_kinds));
				}
				succeed! (());
			},
			
			SyntaxSignatureKeyword::Pattern { .. } =>
				succeed! (()),
		}
	}
}


impl Entity for SyntaxSignatureKeyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		match self {
			
			SyntaxSignatureKeyword::Literal (identifier) =>
				identifier,
			SyntaxSignatureKeyword::Identifier (identifier) =>
				identifier,
			SyntaxSignatureKeyword::Expression (identifier) =>
				identifier,
			
			SyntaxSignatureKeyword::Constant { identifier, .. } =>
				identifier,
			SyntaxSignatureKeyword::Value { identifier, .. } =>
				identifier,
			
			SyntaxSignatureKeyword::Pattern { identifier, .. } =>
				identifier,
			
		}
	}
}


impl SyntaxSignaturePattern {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		match self {
			SyntaxSignaturePattern::List (patterns, pattern_dotted) => {
				let mut tokens = StdVec::with_capacity (patterns.len ());
				for pattern in patterns.iter () {
					match pattern {
						SyntaxSignaturePattern::Variadic (pattern) => {
							tokens.push (pattern.format ());
							tokens.push (symbol_clone_str ("...") .into ());
						},
						_ =>
							tokens.push (pattern.format ()),
					}
				}
				let token_dotted = if let Some (pattern_dotted) = pattern_dotted {
					let token_dotted = pattern_dotted.format ();
					Some (token_dotted)
				} else {
					None
				};
				let tokens = list_collect_dotted (tokens, token_dotted, Some (true));
				return tokens;
			},
			SyntaxSignaturePattern::Variadic (pattern) => {
				//  NOTE:  This case shouldn't happen!
				let tokens = pair_new (
						symbol_clone_str ("...") .into (),
						pattern.format (),
						Some (true));
				return tokens;
			},
			SyntaxSignaturePattern::Keyword (keyword) =>
				symbol_from_rc (keyword.identifier_rc_clone ()) .into (),
			SyntaxSignaturePattern::SyntaxIdentifier =>
				symbol_clone_str ("_") .into (),
			SyntaxSignaturePattern::SyntaxRules =>
				symbol_clone_str ("@syntax-rules") .into (),
			SyntaxSignaturePattern::SyntaxTransformer =>
				symbol_clone_str ("@syntax-transformer") .into (),
		}
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn lines_clone (&self) -> (StdVec<StdString>) {
		return vec_map! (self.lines.iter (), line, StdString::from (line.deref () .deref ()));
	}
}




pub struct Links {
	links : EntitiesOwned<Link>,
}


impl Links {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (impl iter::Iterator<Item = &Link>) {
		return self.links.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn link_resolve (&self, identifier : &str) -> (Option<&Link>) {
		return self.links.entity_resolve (identifier);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_links (&self) -> (bool) {
		return self.links.has_entities ();
	}
}




pub struct Link {
	// FIXME: ...
	identifier : StdRc<StdBox<str>>,
}

impl Entity for Link {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}




pub struct Features {
	// FIXME: ...
	pub condition : Value,
}


impl Features {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		return self.condition.clone ();
	}
}




pub struct Appendix {
	
	identifier : StdRc<StdBox<str>>,
	
	title : Option<StdRc<StdBox<str>>>,
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Appendix {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Appendix {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn title (&self) -> (Option<&str>) {
		return self.title.as_ref () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
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
	let mut appendices_input = None;
	let mut title = None;
	let mut description = None;
	let mut links = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
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
			"appendices" => {
				appendices_input = Some (tokens);
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
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
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
	
	let appendices = if let Some (inputs) = appendices_input {
		try! (parse_list_of (inputs, parse_appendix))
	} else {
		StdVec::new ()
	};
	let appendices = try! (EntitiesOwned::new (appendices));
	
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
			appendices,
			features,
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
	
	for (attribute, tokens) in attributes {
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
	
	let parent = option_map! (parent, EntityLink::new_linked (parent));
	
	let category = Category {
			identifier,
			parent,
			parents_all : EntitiesLinked::new_empty (),
			children : EntitiesLinked::new_empty (),
			children_all : EntitiesLinked::new_empty (),
			definitions : EntitiesLinked::new_empty (),
			definitions_all : EntitiesLinked::new_empty (),
			value_kinds : EntitiesLinked::new_empty (),
			value_kinds_all : EntitiesLinked::new_empty (),
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
	let mut procedure_signature = None;
	let mut syntax_signature = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
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
			
			"signature" => {
				procedure_signature = Some (try! (parse_procedure_signature (tokens)));
			},
			"syntax-rules" => {
				syntax_signature = Some (try! (parse_syntax_signature (tokens)));
			},
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
			_ =>
				fail! (0x24ac563a),
			
		}
	}
	
	let kind = try_some! (kind, 0x74b6b39e);
	
	if procedure_signature.is_some () && ! kind.is_procedure () {
		fail! (0xf67ee3aa);
	}
	if syntax_signature.is_some () && ! kind.is_syntax () {
		fail! (0xb0210771);
	}
	
	let categories = try_some! (categories, 0xbf298927);
	let categories = try! (EntitiesLinked::new (categories));
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let definition = Definition {
			identifier,
			kind,
			categories,
			categories_all : EntitiesLinked::new_empty (),
			aliases,
			description,
			links,
			procedure_signature,
			syntax_signature,
			referenced_value_kinds : EntitiesLinked::new_empty (),
			features,
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
	let mut features = None;
	
	for (attribute, tokens) in attributes {
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
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
			_ =>
				fail! (0x9e7c02e8),
			
		}
	}
	
	let parent = option_map! (parent, EntityLink::new_linked (parent));
	
	let categories = try_some! (categories, 0x113cac3d);
	let categories = try! (EntitiesLinked::new (categories));
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let value_kind = ValueKind {
			identifier,
			parent,
			parents_all : EntitiesLinked::new_empty (),
			children : EntitiesLinked::new_empty (),
			children_all : EntitiesLinked::new_empty (),
			categories,
			categories_all : EntitiesLinked::new_empty (),
			aliases,
			description,
			links,
			predicate,
			features,
			definitions_input : EntitiesLinked::new_empty (),
			definitions_input_all : EntitiesLinked::new_empty (),
			definitions_output : EntitiesLinked::new_empty (),
			definitions_output_all : EntitiesLinked::new_empty (),
		};
	
	succeed! (value_kind);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature (input : StdVec<Value>) -> (Outcome<ProcedureSignature>) {
	
	let variants = try! (parse_list_of (input, parse_procedure_signature_variant)) .into_boxed_slice ();
	
	if variants.is_empty () {
		fail! (0x2281d2dd);
	}
	
	let signature = ProcedureSignature {
			variants,
		};
	
	succeed! (signature);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_procedure_signature_variant (input : Value) -> (Outcome<ProcedureSignatureVariant>) {
	
	let tokens = try! (vec_list_clone (&input));
	let (inputs, becomes, outputs, tokens) = try! (vec_explode_3n (tokens));
	{
		let becomes = try_into_symbol! (becomes);
		if becomes.string_as_str () != "->" {
			fail! (0x9aa6e666);
		}
	}
	
	let inputs = try! (parse_procedure_signature_values (inputs));
	let outputs = try! (parse_procedure_signature_values (outputs));
	
	let mut features = None;
	
	if ! tokens.is_empty () {
		
		let (_, attributes) = try! (parse_object_with_attributes_0 (tokens, Some ("::"), false));
		
		for (attribute, tokens) in attributes {
			match attribute.deref () .as_ref () {
				
				"features" => {
					features = Some (try! (parse_features (tokens)));
				}
				
				_ =>
					fail! (0xe911c007),
				
			}
		}
	}
	
	let variant = ProcedureSignatureVariant {
			inputs,
			outputs,
			features,
		};
	
	succeed! (variant);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature_values (token : Value) -> (Outcome<ProcedureSignatureValues>) {
	match token.class () {
		ValueClass::Symbol => {
			let value = try! (parse_procedure_signature_value (token));
			let values = ProcedureSignatureValues {
					values : StdBox::new ([value]),
					variadic : false,
				};
			succeed! (values);
		},
		ValueClass::Pair => {
			let tokens = try! (vec_list_clone (&token));
			let variadic = if let Some (last) = tokens.last () {
				match last.class_match_as_ref () {
					ValueClassMatchAsRef::Symbol (last) =>
						last.string_eq ("..."),
					_ =>
						false,
				}
			} else {
				false
			};
			let tokens = if variadic {
				let mut tokens = tokens;
				try_some_or_panic! (tokens.pop (), 0xcca15f6f);
				tokens
			} else {
				tokens
			};
			let values = try_vec_map_into! (tokens, token, parse_procedure_signature_value (token));
			let values = ProcedureSignatureValues {
					values : values.into_boxed_slice (),
					variadic : variadic,
				};
			succeed! (values);
		},
		ValueClass::Null => {
			let values = ProcedureSignatureValues {
					values : StdBox::new ([]),
					variadic : false,
				};
			succeed! (values);
		},
		_ =>
			fail! (0xa00d30be),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature_value (token : Value) -> (Outcome<ProcedureSignatureValue>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (kind) => {
			if kind.string_eq ("...") {
				fail! (0x0bbd4e95);
			}
			let kind = EntityLink::new_linked (kind.string_rc_clone ());
			let value = ProcedureSignatureValue {
					identifier : None,
					kind : kind,
				};
			succeed! (value);
		}
		ValueClassMatchInto::Pair (tokens) => {
			let tokens = try! (vec_list_clone (& tokens.value ()));
			let (identifier, kind) = try! (vec_explode_2 (tokens));
			let identifier = try_into_symbol! (identifier);
			let kind = try_into_symbol! (kind);
			if identifier.string_eq ("...") || kind.string_eq ("...") {
				fail! (0xd3afa44f);
			}
			let identifier = if ! identifier.string_eq ("_") {
				Some (identifier.string_rc_clone ())
			} else {
				None
			};
			let kind = EntityLink::new_linked (kind.string_rc_clone ());
			let value = ProcedureSignatureValue {
					identifier : identifier,
					kind : kind,
				};
			succeed! (value);
		},
		_ =>
			fail! (0x4045ae98),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature (input : StdVec<Value>) -> (Outcome<SyntaxSignature>) {
	
	let (keywords, variants) = try! (vec_explode_1n (input));
	
	let keywords = try! (vec_list_clone (&keywords));
	let (keywords, keywords_map) = try! (parse_syntax_signature_keywords (keywords));
	
	let variants = try_vec_map_into! (variants, variant, parse_syntax_signature_variant (variant, &keywords_map));
	
	let signature = SyntaxSignature {
			keywords : keywords.into_boxed_slice (),
			keywords_map : keywords_map,
			variants : variants.into_boxed_slice (),
		};
	
	succeed! (signature);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (type_complexity) ) ]
fn parse_syntax_signature_keywords (tokens : StdVec<Value>) -> (Outcome<(StdVec<StdRc<SyntaxSignatureKeyword>>, StdMap<StdString, StdRc<SyntaxSignatureKeyword>>)>) {
	
	let mut keywords = StdVec::with_capacity (tokens.len ());
	let mut keywords_map = StdMap::with_capacity (tokens.len ());
	
	for token in tokens {
		let keyword = try! (parse_syntax_signature_keyword (token, &keywords_map));
		let keyword = StdRc::new (keyword);
		keywords.push (StdRc::clone (&keyword));
		if keywords_map.insert (keyword.identifier_clone (), StdRc::clone (&keyword)) .is_some () {
			fail! (0xc4cf1b8f);
		}
	}
	
	succeed! ((keywords, keywords_map));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_keyword (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignatureKeyword>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (literal) => {
			let keyword = SyntaxSignatureKeyword::Literal (literal.string_rc_clone ());
			succeed! (keyword);
		},
		ValueClassMatchInto::Pair (tokens) => {
			let tokens = try! (vec_list_clone (& tokens.value ()));
			let (identifier, kind, tokens) = try! (vec_explode_2n (tokens));
			let identifier = try_into_symbol! (identifier);
			let kind = try_into_symbol! (kind);
			match kind.string_as_str () {
				"literal" => {
					if ! tokens.is_empty () {
						fail! (0x76b1463b);
					}
					let keyword = SyntaxSignatureKeyword::Literal (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"identifier" => {
					if ! tokens.is_empty () {
						fail! (0x5df8e23a);
					}
					let keyword = SyntaxSignatureKeyword::Identifier (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"expression" => {
					if ! tokens.is_empty () {
						fail! (0x1ec8b264);
					}
					let keyword = SyntaxSignatureKeyword::Expression (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"constant" => {
					let value = try! (vec_explode_1 (tokens));
					let keyword = SyntaxSignatureKeyword::Constant {
							identifier : identifier.string_rc_clone (),
							value : value,
						};
					succeed! (keyword);
				},
				"value" => {
					let kind = try! (vec_explode_1 (tokens));
					let kind = try_into_symbol! (kind);
					let kind = EntityLink::new_linked (kind.string_rc_clone ());
					let keyword = SyntaxSignatureKeyword::Value {
							identifier : identifier.string_rc_clone (),
							kind : Some (kind),
						};
					succeed! (keyword);
				},
				"pattern" => {
					let patterns = try_vec_map_into! (tokens, token, parse_syntax_signature_pattern (token, keywords));
					let keyword = SyntaxSignatureKeyword::Pattern {
							identifier : identifier.string_rc_clone (),
							patterns : patterns.into_boxed_slice (),
						};
					succeed! (keyword);
				},
				_ =>
					fail! (0x7e5640f4),
			}
		}
		_ =>
			fail! (0x5b273bdf),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_syntax_signature_variant (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignatureVariant>) {
	let (tokens, token_dotted) = try! (vec_list_clone_dotted (&token));
	{
		let head = try_some! (tokens.first (), 0x6cbf707b);
		let head = try_as_symbol_ref! (head);
		if ! head.string_eq ("_") {
			fail! (0x867a2057);
		}
	}
	let pattern = try! (parse_syntax_signature_patterns (tokens, token_dotted, keywords));
	let variant = SyntaxSignatureVariant {
			pattern,
		};
	succeed! (variant);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_patterns (tokens : StdVec<Value>, token_dotted : Option<Value>, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignaturePattern>) {
	let mut patterns = StdVec::with_capacity (tokens.len ());
	let mut end_expected = false;
	for token in tokens {
		if end_expected {
			fail! (0xfbe4c0da);
		}
		match token.class_match_into () {
			ValueClassMatchInto::Symbol (token) => {
				if token.string_eq ("...") {
					if let Some (last) = patterns.pop () {
						let pattern = SyntaxSignaturePattern::Variadic (StdBox::new (last));
						patterns.push (pattern);
						end_expected = true;
					} else {
						fail! (0x6ef5ca55);
					}
				} else {
					let pattern = try! (parse_syntax_signature_pattern (token.into (), keywords));
					patterns.push (pattern);
				}
			},
			ValueClassMatchInto::Pair (list) => {
				let pattern = try! (parse_syntax_signature_pattern (list.value (), keywords));
				patterns.push (pattern);
			},
			_ =>
				fail! (0x60d8e7c6),
		}
	}
	let pattern_dotted = if let Some (token_dotted) = token_dotted {
		let pattern_dotted = try! (parse_syntax_signature_pattern (token_dotted, keywords));
		Some (StdBox::new (pattern_dotted))
	} else {
		None
	};
	let pattern = SyntaxSignaturePattern::List (patterns.into_boxed_slice (), pattern_dotted);
	succeed! (pattern);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_pattern (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignaturePattern>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (keyword) => {
			if keyword.string_eq ("...") {
				fail! (0xaaefecfb);
			} else if keyword.string_eq ("_") {
				succeed! (SyntaxSignaturePattern::SyntaxIdentifier);
			} else if keyword.string_eq ("@syntax-rules") {
				succeed! (SyntaxSignaturePattern::SyntaxRules);
			} else if keyword.string_eq ("@syntax-transformer") {
				succeed! (SyntaxSignaturePattern::SyntaxTransformer);
			} else {
				let keyword = try_some! (keywords.get (keyword.string_as_str ()), 0x97ac4521);
				let keyword = StdRc::clone (keyword);
				let pattern = SyntaxSignaturePattern::Keyword (keyword);
				succeed! (pattern);
			}
		},
		ValueClassMatchInto::Pair (list) => {
			let (tokens, token_dotted) = try! (vec_list_clone_dotted (& list.value ()));
			return parse_syntax_signature_patterns (tokens, token_dotted, keywords);
		},
		ValueClassMatchInto::Null =>
			succeed! (SyntaxSignaturePattern::List (StdBox::new ([]), None)),
		_ =>
			fail! (0x2274e06a),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_appendix (input : Value) -> (Outcome<Appendix>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0xb9669009);
	
	let mut title = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
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
				fail! (0x9e7c02e8),
			
		}
	}
	
	let appendix = Appendix {
			identifier,
			title,
			description,
			links,
		};
	
	succeed! (appendix);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value, type_complexity) ) ]
fn parse_object_with_attributes (input : Value, keyword : Option<&str>, identifier_expected : bool) -> (Outcome<(Option<StdRc<StdBox<str>>>, StdVec<(StdRc<StdBox<str>>, StdVec<Value>)>)>) {
	
	let tokens = try! (vec_list_clone (&input));
	
	return parse_object_with_attributes_0 (tokens, keyword, identifier_expected);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (type_complexity) ) ]
fn parse_object_with_attributes_0 (tokens : StdVec<Value>, keyword : Option<&str>, identifier_expected : bool) -> (Outcome<(Option<StdRc<StdBox<str>>>, StdVec<(StdRc<StdBox<str>>, StdVec<Value>)>)>) {
	
	let tokens = if let Some (keyword) = keyword {
		let (head, rest) = try! (vec_explode_1n (tokens));
		let head = try_into_symbol! (head);
		if ! head.string_eq (keyword) {
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
	for tokens in tokens {
		let tokens = try! (vec_list_clone (&tokens));
		let (head, rest) = try! (vec_explode_1n (tokens));
		let identifier = try_into_symbol! (head);
		let identifier = identifier.string_rc_clone ();
		if attributes.insert (identifier, rest) .is_some () {
			fail! (0x9a98dec4);
		}
	}
	
	let attributes = attributes.into_iter () .collect ();
	
	succeed! ((identifier, attributes));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_description (input : StdVec<Value>) -> (Outcome<Description>) {
	
	let input = try! (vec_explode_1 (input));
	let input = try_as_string_ref! (&input);
	
	let mut lines = vec_map! (input.string_as_str () .lines (), line, StdRc::new (StdString::from (line.trim_right ()) .into_boxed_str ()));
	
	for _ in 0..2 {
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (while_let_loop) ) ]
		loop {
			let pop = if let Some (line) = lines.last () {
				line.trim_left () .is_empty ()
			} else {
				break;
			};
			if pop {
				lines.pop ();
			} else {
				break;
			}
		}
		lines.reverse ();
	}
	
	let description = Description {
			lines,
		};
	
	succeed! (description);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_links (_input : StdVec<Value>) -> (Outcome<Links>) {
	fail_unimplemented! (0xd3359173);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_features (input : StdVec<Value>) -> (Outcome<Features>) {
	
	let input = try! (vec_explode_1 (input));
	
	let features = Features {
			condition : input,
		};
	
	succeed! (features);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_list_of <T> (input : StdVec<Value>, parser : impl Fn (Value) -> (Outcome<T>)) -> (Outcome<StdVec<T>>) {
	let output = try! (input.into_iter () .map (parser) .collect ());
	succeed! (output);
}




#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_library_specifications_for_builtins () -> (Outcome<Libraries>) {
	return parse_library_specifications (LIBRARY_SPECIFICATIONS_SOURCES);
}

#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
static LIBRARY_SPECIFICATIONS_SOURCES : &'static str = include_str! ("../documentation/libraries.ss");

