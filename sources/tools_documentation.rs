

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
			dump_cmark,
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
		&["dump-cmark"] =>
			try! (dump_cmark (libraries, &mut stream)),
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
		SyntaxSignaturePattern::List (patterns, pattern_dotted) =>
			if let Some (pattern_dotted) = pattern_dotted {
				json! ([
						".",
						json::Value::Array (vec_map! (patterns.iter (), pattern, dump_json_syntax_signature_pattern (pattern))),
						dump_json_syntax_signature_pattern (pattern_dotted),
					])
			} else {
				json::Value::Array (vec_map! (patterns.iter (), pattern, dump_json_syntax_signature_pattern (pattern)))
			},
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
pub fn dump_cmark (libraries : Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn mangle_anchor_identifier (identifier : &str) -> (StdString) {
		identifier.chars ()
			.map (|character|
					match character {
						'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' =>
							character,
						'-' | '!' | '_' =>
							character,
						_ =>
							'_',
					})
			.collect ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn generate_anchor (prefix : Option<&str>, library : Option<&str>, identifier : Option<&str>) -> (Outcome<StdString>) {
		match (prefix, library, identifier) {
			(Some ("toc"), Some (library), Some (identifier)) => {
				let identifier = match identifier {
					"libraries" => "libraries",
					"categories" => "categories",
					"value_kinds" => "types",
					"definitions" => "definitions",
					_ => fail! (0x4bef3a8f),
				};
				let library = mangle_anchor_identifier (library);
				succeed! (format! ("toc__{}__{}", library, identifier));
			},
			(Some ("library"), Some (library), None) => {
				let library = mangle_anchor_identifier (library);
				succeed! (format! ("library__{}", library));
			},
			(Some (prefix), Some (library), Some (identifier)) => {
				let prefix = match prefix {
					"library" => "library",
					"category" => "category",
					"value_kind" => "value_kind",
					"definition" => "definition",
					_ => fail! (0x69733dab),
				};
				let library = mangle_anchor_identifier (library);
				let identifier = mangle_anchor_identifier (identifier);
				succeed! (format! ("{}__{}__{}", prefix, library, identifier));
			},
			_ =>
				fail! (0x165bf432),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_anchor (prefix : Option<&str>, library : Option<&str>, identifier : Option<&str>, stream : &mut dyn io::Write) -> (Outcome<()>) {
		let anchor = try! (generate_anchor (prefix, library, identifier));
		try_writeln! (stream, "<a id='{}'>\n", anchor);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn format_value (value : &SchemeValue) -> (StdString) {
		match value.kind () {
			SchemeValueKind::Null =>
				StdString::from ("()"),
			_ =>
				format! ("{}", value),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_description (description : Option<&Description>, stream : &mut dyn io::Write) -> (Outcome<()>) {
		let description = if let Some (description) = description {
			description
		} else {
			succeed! (());
		};
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Description");
		try_writeln! (stream);
		for line in description.lines () {
			try_writeln! (stream, "> {}", line);
		}
		try_writeln! (stream);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_links (links : Option<&Links>, stream : &mut dyn io::Write) -> (Outcome<()>) {
		let _links = if let Some (links) = links {
			links
		} else {
			succeed! (());
		};
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Links");
		try_writeln! (stream);
		fail_unimplemented! (0x81cb5f76);
	}
	
	
	for library in libraries.libraries () {
		
		let library_anchor = try! (generate_anchor (Some ("library"), Some (library.identifier ()), None));
		let categories_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories")));
		let value_kinds_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds")));
		let definitions_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("definitions")));
		
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("library"), Some (library.identifier ()), None, stream));
			
			if let Some (title) = library.title () {
				try_writeln! (stream, "# `{}` -- {}", library.identifier (), title);
			} else {
				try_writeln! (stream, "# `{}`", library.identifier ());
			}
			
			try! (write_description (library.description (), stream));
			try! (write_links (library.links (), stream));
			
			try_writeln! (stream);
			try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
			try_writeln! (stream);
			try_writeln! (stream, "----");
			try_writeln! (stream);
		}
		
		if library.has_categories () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories"), stream));
			
			try_writeln! (stream, "## Categories");
			
			{
				try_writeln! (stream);
				try_writeln! (stream, "Quick list of categories:");
				for category in library.categories () {
					if category.has_parent () {
						continue;
					}
					let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
					if category.has_children () {
						try_write! (stream, "* [`{}`](#{}):", category.identifier (), category_anchor);
						let mut is_first = true;
						for sub_category in category.children () {
							let sub_category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (sub_category.identifier ())));
							if is_first {
								try_write! (stream, " [`{}`](#{})", sub_category.identifier (), sub_category_anchor);
								is_first = false;
							} else {
								try_write! (stream, ", [`{}`](#{})", sub_category.identifier (), sub_category_anchor);
							}
						}
						try_writeln! (stream, ";");
					} else {
						try_writeln! (stream, "* [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
			}
			
			for category in library.categories () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), stream));
				
				try_writeln! (stream, "### Category `{}`", category.identifier ());
				
				if let Some (super_category) = category.parent () {
					let super_category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (super_category.identifier ())));
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the super-category: [`{}`](#{}).", super_category.identifier (), super_category_anchor);
				}
				if category.has_children () {
					try_writeln! (stream);
					try_writeln! (stream, "Contains the following sub-categories:");
					for sub_category in category.children () {
						let sub_category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (sub_category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", sub_category.identifier (), sub_category_anchor);
					}
				}
				
				try! (write_description (category.description (), stream));
				try! (write_links (category.links (), stream));
				
				if category.has_value_kinds () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Types");
					try_writeln! (stream);
					for value_kind in category.value_kinds () {
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
					}
				}
				
				if category.has_definitions () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Definitions");
					try_writeln! (stream);
					for definition in category.definitions () {
						let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", definition.identifier (), definition_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
			}
		}
		
		
		if library.has_value_kinds () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), stream));
			
			try_writeln! (stream, "## Types");
			
			{
				try_writeln! (stream);
				try_writeln! (stream, "Quick list of types:");
				for value_kind in library.value_kinds () {
					if value_kind.has_parent () {
						continue;
					}
					let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
					if value_kind.has_children () {
						try_write! (stream, "* [`{}`](#{}):", value_kind.identifier (), value_kind_anchor);
						let mut is_first = true;
						for sub_value_kind in value_kind.children () {
							let sub_value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (sub_value_kind.identifier ())));
							if is_first {
								try_write! (stream, " [`{}`](#{})", sub_value_kind.identifier (), sub_value_kind_anchor);
								is_first = false;
							} else {
								try_write! (stream, ", [`{}`](#{})", sub_value_kind.identifier (), sub_value_kind_anchor);
							}
						}
						try_writeln! (stream, ";");
					} else {
						try_writeln! (stream, "* [`{}`](#{})", value_kind.identifier (), value_kind_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
			}
			
			for value_kind in library.value_kinds () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), stream));
				
				try_writeln! (stream, "### Type `{}`", value_kind.identifier ());
				
				if value_kind.has_aliases () {
					try_writeln! (stream);
					try_writeln! (stream, "With the following aliases:");
					for alias in value_kind.aliases () {
						try_writeln! (stream, " * `{}`;", alias);
					}
				}
				
				if let Some (super_value_kind) = value_kind.parent () {
					let super_value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (super_value_kind.identifier ())));
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the super-type: [`{}`](#{}).", super_value_kind.identifier (), super_value_kind_anchor);
				}
				if value_kind.has_children () {
					try_writeln! (stream);
					try_writeln! (stream, "Contains the following sub-types:");
					for sub_value_kind in value_kind.children () {
						let sub_value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (sub_value_kind.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", sub_value_kind.identifier (), sub_value_kind_anchor);
					}
				}
				
				if let Some (predicate) = value_kind.predicate () {
					try_writeln! (stream);
					try_writeln! (stream, "Verified by the folowing predicate:");
					try_writeln! (stream, "```");
					try_writeln! (stream, "{}", predicate);
					try_writeln! (stream, "```");
				}
				
				if value_kind.has_categories () {
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the following categories:");
					for category in value_kind.categories () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
				try! (write_description (value_kind.description (), stream));
				try! (write_links (value_kind.links (), stream));
				
				if value_kind.has_definitions () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Referent definitions");
					try_writeln! (stream);
					for definition in value_kind.definitions () {
						let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", definition.identifier (), definition_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
			}
		}
		
		
		if library.has_definitions () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("definitions"), stream));
			
			try_writeln! (stream, "## Definitions");
			
			{
				try_writeln! (stream);
				try_writeln! (stream, "Quick list of definitions:");
				for definition in library.definitions () {
					let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
					try_writeln! (stream, "* [`{}`](#{});", definition.identifier (), definition_anchor);
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
			}
			
			for definition in library.definitions () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), stream));
				
				try_writeln! (stream, "### Definition `{}`", definition.identifier ());
				
				{
					try_writeln! (stream);
					try_writeln! (stream, "Has the following kind: `{}`.", definition.kind () .identifier ());
				}
				
				if definition.has_aliases () {
					try_writeln! (stream);
					try_writeln! (stream, "With the following aliases:");
					for alias in definition.aliases () {
						try_writeln! (stream, " * `{}`;", alias);
					}
				}
				
				if definition.has_categories () {
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the following categories:");
					for category in definition.categories () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
				try! (write_description (definition.description (), stream));
				try! (write_links (definition.links (), stream));
				
				if let Some (procedure_signature) = definition.procedure_signature () {
					try_writeln! (stream);
					try_writeln! (stream, "#### Procedure signature");
					try_writeln! (stream);
					if ! procedure_signature.variants.is_empty () {
						try_writeln! (stream);
						try_writeln! (stream, "Procedure variants:");
						for procedure_signature_variant in procedure_signature.variants.iter () {
							try_writeln! (stream, " * `{}`", format_value (& procedure_signature_variant.format ()));
							#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
							fn write_procedure_signature_value (library : &Library, value : &ProcedureSignatureValue, stream : &mut dyn io::Write) -> (Outcome<()>) {
								let value_kind = try_some_2! (value.kind.entity_resolve (), 0x131ac42a);
								let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
								if let Some (identifier) = value.identifier.as_ref () {
									try_writeln! (stream, "     * `{}` of type [`{}`](#{});", identifier, value_kind.identifier (), value_kind_anchor);
								} else {
									try_writeln! (stream, "     * a value type [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
								}
								succeed! (());
							}
							if ! procedure_signature_variant.inputs.values.is_empty () {
								try_writeln! (stream, "   * inputs:");
								for procedure_signature_value in procedure_signature_variant.inputs.values.iter () {
									try! (write_procedure_signature_value (library, procedure_signature_value, stream));
								}
								if procedure_signature_variant.inputs.variadic {
									try_writeln! (stream, "     * `...` (i.e. variadic);");
								}
							}
							if ! procedure_signature_variant.outputs.values.is_empty () {
								try_writeln! (stream, "   * outputs:");
								for procedure_signature_value in procedure_signature_variant.outputs.values.iter () {
									try! (write_procedure_signature_value (library, procedure_signature_value, stream));
								}
								if procedure_signature_variant.outputs.variadic {
									try_writeln! (stream, "     * `...` (i.e. variadic);");
								}
							}
						}
					}
				}
				if let Some (syntax_signature) = definition.syntax_signature () {
					try_writeln! (stream);
					try_writeln! (stream, "#### Syntax signature");
					try_writeln! (stream);
					if ! syntax_signature.keywords.is_empty () {
						try_writeln! (stream);
						try_writeln! (stream, "Syntax keywords:");
						for syntax_signature_keyword in syntax_signature.keywords.iter () {
							match syntax_signature_keyword.deref () {
								SyntaxSignatureKeyword::Literal (identifier) =>
									try_writeln! (stream, " * `{}`: literal;", identifier),
								SyntaxSignatureKeyword::Identifier (identifier) =>
									try_writeln! (stream, " * `{}`: identifier;", identifier),
								SyntaxSignatureKeyword::Expression (identifier) =>
									try_writeln! (stream, " * `{}`: expression;", identifier),
								SyntaxSignatureKeyword::Constant { identifier, value } =>
									try_writeln! (stream, " * `{}`: constant with value `{}`;", identifier, format_value (value)),
								SyntaxSignatureKeyword::Value { identifier, kind : value_kind } =>
									if let Some (value_kind) = value_kind {
										let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
										try_writeln! (stream, " * `{}`: value of type [{}](#{});", identifier, value_kind.identifier (), value_kind_anchor);
									} else {
										try_writeln! (stream, " * `{}`: value with unspecified type;", identifier);
									},
								SyntaxSignatureKeyword::Pattern { identifier, patterns } =>
									if ! patterns.is_empty () {
										try_writeln! (stream, " * `{}`: pattern with variants:", identifier);
										for pattern in patterns.iter () {
											try_writeln! (stream, "   * `{}`;", format_value (& pattern.format ()));
										}
									},
							}
						}
					}
					if ! syntax_signature.variants.is_empty () {
						try_writeln! (stream);
						try_writeln! (stream, "Syntax variants:");
						for syntax_signature_variant in syntax_signature.variants.iter () {
							try_writeln! (stream, " * `{}`", format_value (& syntax_signature_variant.pattern.format ()));
						}
					}
				}
				
				if definition.has_referenced_value_kinds () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Referenced types");
					try_writeln! (stream);
					for value_kind in definition.referenced_value_kinds () {
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
				try_writeln! (stream);
				
			}
		}
	}
	
	succeed! (());
}

