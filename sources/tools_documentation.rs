

use super::documentation::exports::*;
use super::documentation::exports::documentation_model::*;
use super::errors::exports::*;
use super::tools::exports::*;

use super::externals::serde_json as json;

use super::prelude::*;

use super::documentation::exports::documentation_model::Entity;
use super::values::exports::Value as SchemeValue;
use super::values::exports::ValueKind as SchemeValueKind;




pub mod exports {
	
	pub use super::main;
	
	pub use super::{
			dump_json,
			dump_text,
		};
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	if ! inputs.tool_arguments.is_empty () {
		fail! (0x2f6cb42b);
	}
	if ! inputs.rest_arguments.is_empty () {
		fail! (0x6b36228d);
	}
	
	let libraries = try! (parse_library_specifications_for_builtins ());
	
	match vec_map! (inputs.tool_commands.iter (), command, command.as_str ()) .as_slice () {
		&["dump-text"] =>
			try! (dump_text (libraries, &mut stream)),
		&["dump-json"] =>
			try! (dump_json (libraries, &mut stream)),
		_ =>
			fail! (0x3b57eb47),
	}
	
	succeed! (0);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json (libraries : Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	let libraries_json = json::Value::from (vec_map! (libraries.libraries (), library, dump_json_library (library)));
	
	try_or_fail! (json::to_writer_pretty (stream, &libraries_json), 0x200f6e78);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_library (library : &Library) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			
			"categories" : json::Map::from_iter (vec_map! (library.categories (), category, (category.identifier_clone (), dump_json_category (category)))),
			
			"types" : json::Map::from_iter (vec_map! (library.value_kinds (), value_kind, (value_kind.identifier_clone (), dump_json_value_kind (value_kind)))),
			"types_all" : json::Map::from_iter (vec_map! (library.value_kinds_all (), (alias, value_kind), (StdString::from (alias), json::Value::String (value_kind.identifier_clone ())))),
			
			"definitions" : json::Map::from_iter (vec_map! (library.definitions (), definition, (definition.identifier_clone (), dump_json_definition (definition)))),
			"definitions_all" : json::Map::from_iter (vec_map! (library.definitions_all (), (alias, definition), (StdString::from (alias), json::Value::String (definition.identifier_clone ())))),
			
			"title" : if let Some (title) = library.title () { json::Value::String (StdString::from (title)) } else { json::Value::Null },
			"description" : if let Some (description) = library.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = library.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_category (category : &Category) -> (json::Value) {
	json! ({
			
			"identifier" : category.identifier_clone (),
			
			"super_category" : dump_json_identifier_perhaps_for_entity (category.parent ()),
			"sub_categories" : dump_json_identifiers_perhaps_for_entities (category.children ()),
			
			"types" : dump_json_identifiers_perhaps_for_entities (category.value_kinds ()),
			"definitions" : dump_json_identifiers_perhaps_for_entities (category.definitions ()),
			
			"description" : if let Some (description) = category.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = category.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_value_kind (value_kind : &ValueKind) -> (json::Value) {
	json! ({
			
			"identifier" : value_kind.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (value_kind.aliases ()),
			
			"super_type" : dump_json_identifier_perhaps_for_entity (value_kind.parent ()),
			"sub_types" : dump_json_identifiers_perhaps_for_entities (value_kind.children ()),
			
			"categories" : dump_json_identifiers_perhaps_for_entities (value_kind.categories ()),
			"definitions" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions ()),
			
			"description" : if let Some (description) = value_kind.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = value_kind.links () { dump_json_links (links) } else { json::Value::Null },
			
			"predicate" : if let Some (predicate) = value_kind.predicate () { dump_json_value (predicate) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_definition (definition : &Definition) -> (json::Value) {
	json! ({
			
			"identifier" : definition.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (definition.aliases ()),
			
			"categories" : dump_json_identifiers_perhaps_for_entities (definition.categories ()),
			
			"kind" : json::Value::String (StdString::from (definition.kind () .identifier ())),
			
			"description" : if let Some (description) = definition.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = definition.links () { dump_json_links (links) } else { json::Value::Null },
			
			"procedure_signature" : if let Some (procedure_signature) = definition.procedure_signature () { dump_json_procedure_signature (procedure_signature) } else { json::Value::Null },
			"syntax_signature" : if let Some (syntax_signature) = definition.syntax_signature () { dump_json_syntax_signature (syntax_signature) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature (signature : &ProcedureSignature) -> (json::Value) {
	json! ({
			"variants" : json::Value::Array (vec_map! (signature.variants.iter (), variant, dump_json_procedure_signature_variant (variant))),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_variant (variant : &ProcedureSignatureVariant) -> (json::Value) {
	json! ({
			"inputs" : dump_json_procedure_signature_values (&variant.inputs),
			"outputs" : dump_json_procedure_signature_values (&variant.outputs),
			"source" : dump_json_value (& variant.format ()),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_values (values : &ProcedureSignatureValues) -> (json::Value) {
	json! ({
			"values" : json::Value::Array (vec_map! (values.values.iter (), value, dump_json_procedure_signature_value (value))),
			"variadic" : values.variadic,
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_value (value : &ProcedureSignatureValue) -> (json::Value) {
	json! ({
			"identifier" : if let Some (identifier) = &value.identifier { json::Value::String (StdString::from (identifier.deref () .deref ())) } else { json::Value::Null },
			"type" : value.kind.identifier_clone (),
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature (signature : &SyntaxSignature) -> (json::Value) {
	json! ({
			"keywords" : json::Map::from_iter (vec_map! (signature.keywords.iter (), keyword, (keyword.identifier_clone (), dump_json_syntax_signature_keyword (keyword)))),
			"variants" : json::Value::Array (vec_map! (signature.variants.iter (), variant, dump_json_syntax_signature_variant (variant))),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_keyword (keyword : &SyntaxSignatureKeyword) -> (json::Value) {
	let identifier = keyword.identifier_clone ();
	match keyword {
		SyntaxSignatureKeyword::Literal (_) =>
			json! ({
					"kind" : "literal",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Identifier (_) =>
			json! ({
					"kind" : "identifier",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Expression (_) =>
			json! ({
					"kind" : "expression",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Constant { identifier : _, value } =>
			json! ({
					"kind" : "constant",
					"identifier" : identifier,
					"value" : format! ("{}", value),
				}),
		SyntaxSignatureKeyword::Value { identifier : _, kind } =>
			json! ({
					"kind" : "value",
					"identifier" : identifier,
					"type" : dump_json_identifier_perhaps_for_entity (kind.as_ref ()),
				}),
		SyntaxSignatureKeyword::Pattern { identifier : _, patterns } =>
			json! ({
					"kind" : "pattern",
					"identifier" : identifier,
					"patterns" : json::Value::Array (vec_map! (patterns.iter (), pattern,
							json! ({
								"pattern" : dump_json_syntax_signature_pattern (pattern),
								"source" : dump_json_value (& pattern.format ()),
							}))),
				}),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_variant (variant : &SyntaxSignatureVariant) -> (json::Value) {
	json! ({
			"pattern" : dump_json_syntax_signature_pattern (&variant.pattern),
			"source" : dump_json_value (& variant.pattern.format ()),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_pattern (pattern : &SyntaxSignaturePattern) -> (json::Value) {
	match pattern {
		SyntaxSignaturePattern::List (patterns) =>
			json::Value::Array (vec_map! (patterns.iter (), pattern, dump_json_syntax_signature_pattern (pattern))),
		SyntaxSignaturePattern::Keyword (keyword) =>
			json::Value::String (StdString::from (keyword.identifier_clone ())),
		SyntaxSignaturePattern::Variadic (pattern) =>
			json! (["...", dump_json_syntax_signature_pattern (pattern)]),
		SyntaxSignaturePattern::SyntaxIdentifier =>
			json::Value::String (StdString::from ("_")),
		SyntaxSignaturePattern::SyntaxRules =>
			json::Value::String (StdString::from ("@syntax-rules")),
		SyntaxSignaturePattern::SyntaxTransformer =>
			json::Value::String (StdString::from ("@syntax-transformer")),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifier_perhaps_for_entity (entity : Option<&impl Entity>) -> (json::Value) {
	return dump_json_identifier_perhaps (entity.map (Entity::identifier));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifier_perhaps (identifier : Option<&str>) -> (json::Value) {
	if let Some (identifier) = identifier {
		return json::Value::String (StdString::from (identifier));
	} else {
		return json::Value::Null;
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifiers_perhaps_for_entities <'a, E : Entity + 'a> (entities : impl iter::Iterator<Item = &'a E>) -> (json::Value) {
	return dump_json_identifiers_perhaps (entities.map (Entity::identifier));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifiers_perhaps <'a> (identifiers : impl iter::Iterator<Item = &'a str>) -> (json::Value) {
	let identifiers = vec_map! (identifiers, identifier, json::Value::String (StdString::from (identifier)));
	if identifiers.is_empty () {
		return json::Value::Null;
	} else {
		return json::Value::Array (identifiers);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_description (description : &Description) -> (json::Value) {
	json::Value::Array (vec_map_into! (description.lines_clone (), line, json::Value::String (line)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_links (_links : &Links) -> (json::Value) {
	unimplemented_0! (0xb7740aad);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_value (value : &SchemeValue) -> (json::Value) {
	match value.kind () {
		SchemeValueKind::Null =>
			json::Value::String (StdString::from ("()")),
		_ =>
			json::Value::String (format! ("{}", value)),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_text (libraries : Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	for library in libraries.libraries () {
		
		try_writeln! (stream, "* library `{}`", library.identifier ());
		
		if library.has_categories () {
			try_writeln! (stream, "  * categories:");
			for category in library.categories () {
				try_writeln! (stream, "    * category `{}`", category.identifier ());
				if category.has_definitions () {
					try_writeln! (stream, "      * definitions:");
					for definition in category.definitions () {
						try_writeln! (stream, "        * definition `{}`", definition.identifier ());
					}
				}
				if category.has_value_kinds () {
					try_writeln! (stream, "      * types:");
					for value_kind in category.value_kinds () {
						try_writeln! (stream, "        * type `{}`", value_kind.identifier ());
					}
				}
			}
		}
		
		if library.has_definitions () {
			try_writeln! (stream, "  * definitions:");
			for definition in library.definitions () {
				try_writeln! (stream, "    * definition `{}`", definition.identifier ());
				{
					try_write! (stream, "      * types: `{}`", definition.kind () .identifier ());
					for definition_kind in definition.kind () .parents () {
						try_write! (stream, " `{}`", definition_kind.identifier ());
					}
					try_writeln! (stream);
				}
				if definition.has_categories () {
					try_write! (stream, "      * categories:");
					for category in definition.categories () {
						try_write! (stream, " `{}`", category.identifier ());
					}
					try_writeln! (stream);
				}
				if definition.has_aliases () {
					try_write! (stream, "      * aliases:");
					for alias in definition.aliases () {
						try_write! (stream, " `{}`", alias);
					}
					try_writeln! (stream);
				}
			}
		}
		
		if library.has_value_kinds () {
			try_writeln! (stream, "  * types:");
			for value_kind in library.value_kinds () {
				try_writeln! (stream, "    * type `{}`", value_kind.identifier ());
				if value_kind.has_categories () {
					try_write! (stream, "      * categories:");
					for category in value_kind.categories () {
						try_write! (stream, " `{}`", category.identifier ());
					}
					try_writeln! (stream);
				}
				if value_kind.has_definitions () {
					try_writeln! (stream, "      * definitions:");
					for definition in value_kind.definitions () {
						try_writeln! (stream, "        * definition `{}`", definition.identifier ());
					}
				}
				if value_kind.has_aliases () {
					try_write! (stream, "      * aliases:");
					for alias in value_kind.aliases () {
						try_write! (stream, " `{}`", alias);
					}
					try_writeln! (stream);
				}
				if let Some (predicate) = value_kind.predicate () {
					try_writeln! (stream, "      * predicate `{}`", predicate);
				}
			}
		}
	}
	
	succeed! (());
}

