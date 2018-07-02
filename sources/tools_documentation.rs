

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
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	if ! inputs.tool_arguments.is_empty () {
		fail! (0x2f6cb42b);
	}
	
	let dump_function = match vec_map! (inputs.tool_commands.iter (), command, command.as_str ()) .as_slice () {
		["dump-cmark"] =>
			dump_cmark,
		["dump-json"] =>
			dump_json,
		_ =>
			fail! (0x3b57eb47),
	};
	
	let source = match inputs.rest_arguments.as_slice () {
		[] =>
			None,
		[ref source] =>
			Some (fs_path::Path::new (source)),
		_ =>
			fail! (0xf4938c6d),
	};
	
	let libraries = if let Some (source_path) = source {
		let mut source_buffer = StdString::new ();
		let mut source_stream = try_or_fail! (fs::File::open (source_path), 0x463c39f9);
		try_or_fail! (source_stream.read_to_string (&mut source_buffer), 0x4025f07b);
		try! (parse_library_specifications (&source_buffer))
	} else {
		#[ cfg ( not ( feature = "vonuvoli_documentation_sources" ) ) ]
		fail! (0x834807b9);
		#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
		try! (parse_library_specifications_for_builtins ())
	};
	
	try! (dump_function (&libraries, &mut stream));
	
	succeed! (0);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	let libraries_json = json::Map::from_iter (vec_map! (libraries.libraries (), library, (library.identifier_clone (), dump_json_library (library))));
	
	try_or_fail! (json::to_writer_pretty (stream, &libraries_json), 0x200f6e78);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_library (library : &Library) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			"features" : if let Some (features) = library.features () { dump_json_features (features) } else { json::Value::Null },
			
			"categories" : json::Map::from_iter (vec_map! (library.categories (), category, (category.identifier_clone (), dump_json_category (category)))),
			
			"types" : json::Map::from_iter (vec_map! (library.value_kinds (), value_kind, (value_kind.identifier_clone (), dump_json_value_kind (value_kind)))),
			"types_all" : json::Map::from_iter (vec_map! (library.value_kinds_all (), (alias, value_kind), (StdString::from (alias), json::Value::String (value_kind.identifier_clone ())))),
			
			"definitions" : json::Map::from_iter (vec_map! (library.definitions (), definition, (definition.identifier_clone (), dump_json_definition (definition)))),
			"definitions_all" : json::Map::from_iter (vec_map! (library.definitions_all (), (alias, definition), (StdString::from (alias), json::Value::String (definition.identifier_clone ())))),
			
			"title" : if let Some (title) = library.title () { json::Value::String (StdString::from (title)) } else { json::Value::Null },
			"description" : if let Some (description) = library.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = library.links () { dump_json_links (links) } else { json::Value::Null },
			
			"appendices" : json::Map::from_iter (vec_map! (library.appendices (), appendix, (appendix.identifier_clone (), dump_json_appendix (appendix)))),
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_category (category : &Category) -> (json::Value) {
	json! ({
			
			"identifier" : category.identifier_clone (),
			
			"super_category" : dump_json_identifier_perhaps_for_entity (category.parent ()),
			"sub_categories" : dump_json_identifiers_perhaps_for_entities (category.children ()),
			"sub_categories_recursive" : dump_json_identifiers_perhaps_for_entities (category.children_recursive ()),
			
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
			"features" : if let Some (features) = value_kind.features () { dump_json_features (features) } else { json::Value::Null },
			
			"super_type" : dump_json_identifier_perhaps_for_entity (value_kind.parent ()),
			"sub_types" : dump_json_identifiers_perhaps_for_entities (value_kind.children ()),
			"sub_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.children_recursive ()),
			
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
			"features" : if let Some (features) = definition.features () { dump_json_features (features) } else { json::Value::Null },
			
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
			"features" : if let Some (features) = &variant.features { dump_json_features (features) } else { json::Value::Null },
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
		SyntaxSignatureKeyword::Constant { value, .. } =>
			json! ({
					"kind" : "constant",
					"identifier" : identifier,
					"value" : format! ("{}", value),
				}),
		SyntaxSignatureKeyword::Value { kind, .. } =>
			json! ({
					"kind" : "value",
					"identifier" : identifier,
					"type" : dump_json_identifier_perhaps_for_entity (kind.as_ref ()),
				}),
		SyntaxSignatureKeyword::Pattern { patterns, .. } =>
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
			json::Value::String (keyword.identifier_clone ()),
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
fn dump_json_features (features : &Features) -> (json::Value) {
	return dump_json_value (&features.condition);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_appendix (library : &Appendix) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			
			"title" : if let Some (title) = library.title () { json::Value::String (StdString::from (title)) } else { json::Value::Null },
			"description" : if let Some (description) = library.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = library.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
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
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
pub fn dump_cmark (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	
	const COMPACT : bool = true;
	const NAVIGATOR : bool = true;
	const ANCHORS : bool = true;
	const LINTS : bool = true;
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn mangle_anchor_identifier (identifier : &str) -> (StdString) {
		let mut buffer = StdString::with_capacity (identifier.len ());
		for character in identifier.chars () {
			match character {
				'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' =>
					buffer.push (character),
				'-' =>
					buffer.push (character),
				_ => {
					let mut character_buffer = [0; 8];
					let character_bytes = character.encode_utf8 (&mut character_buffer) .as_bytes ();
					if let Some (buffer_last) = buffer.as_bytes () .last () .cloned () {
						if buffer_last != b'_' {
							buffer.push ('_');
						}
					}
					for character_byte in character_bytes {
						buffer.push_str (& format! ("{:02x}", character_byte));
					}
					buffer.push ('_');
				}
			}
		}
		if let Some (buffer_last) = buffer.pop () {
			if buffer_last != '_' {
				buffer.push (buffer_last);
			}
		}
		return buffer;
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
					"appendices" => "appendices",
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
					"appendix" => "appendix",
					"link" => "link",
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
		if ANCHORS {
			let anchor = try! (generate_anchor (prefix, library, identifier));
			try_writeln! (stream, "<a id='{}'>\n", anchor);
		}
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
	fn write_description (library : &Library, description : Option<&Description>, links : Option<&Links>, stream : &mut dyn io::Write) -> (Outcome<()>) {
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
			let line = DUMP_CMARK_CATEGORY_HREF_REGEX.replace_all (line, |captures : &ext::regex::Captures| {
						let identifier = try_some_or_panic! (captures.get (1), 0x017ef686);
						let identifier = identifier.as_str ();
						if let Some (category) = library.category_resolve (identifier) {
							let category_anchor = try_or_panic_0! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())), 0x438c2cde);
							format! ("[`{}`](#{})", category.identifier (), category_anchor)
						} else {
							if LINTS {
								format! ("[`{}` **ERROR!**](#errors)", identifier)
							} else {
								format! ("[`{}`](#errors)", identifier)
							}
						}
					});
			let line = DUMP_CMARK_VALUE_KIND_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
						let identifier = try_some_or_panic! (captures.get (1), 0x017ef686);
						let identifier = identifier.as_str ();
						if let Some (value_kind) = library.value_kind_resolve (identifier) {
							let value_kind_anchor = try_or_panic_0! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())), 0x438c2cde);
							format! ("[`{}`](#{})", value_kind.identifier (), value_kind_anchor)
						} else {
							if LINTS {
								format! ("[`{}` **ERROR!**](#errors)", identifier)
							} else {
								format! ("[`{}`](#errors)", identifier)
							}
						}
					});
			let line = DUMP_CMARK_DEFINITION_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
						let identifier = try_some_or_panic! (captures.get (1), 0x18c49361);
						let identifier = identifier.as_str ();
						if let Some (definition) = library.definition_resolve (identifier) {
							let definition_anchor = try_or_panic_0! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())), 0xf9025e58);
							format! ("[`{}`](#{})", definition.identifier (), definition_anchor)
						} else {
							if LINTS {
								format! ("[`{}` **ERROR!**](#errors)", identifier)
							} else {
								format! ("[`{}`](#errors)", identifier)
							}
						}
					});
			let line = DUMP_CMARK_LINK_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
						let identifier = try_some_or_panic! (captures.get (1), 0x18c49361);
						let identifier = identifier.as_str ();
						let mut link = None;
						if link.is_none () {
							if let Some (links) = links {
								link = links.link_resolve (identifier);
							}
						}
						if link.is_none () {
							if let Some (links) = library.links () {
								link = links.link_resolve (identifier);
							}
						}
						if let Some (link) = link {
							let link_anchor = try_or_panic_0! (generate_anchor (Some ("link"), Some (library.identifier ()), Some (link.identifier ())), 0x62baae72);
							format! ("[[{}]](#{})", link.identifier (), link_anchor)
						} else {
							//if LINTS {
							//	format! ("[[{}] **ERROR!**](#errors)", identifier)
							//	format! ("[[{}]](#errors)", identifier)
							//} else {
								format! ("[[{}]](#errors)", identifier)
							//}
						}
					});
			try_writeln! (stream, "> {}", line);
		}
		try_writeln! (stream);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_links (_library : &Library, links : Option<&Links>, stream : &mut dyn io::Write) -> (Outcome<()>) {
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
		let appendices_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("appendices")));
		
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
			
			if let Some (features) = library.features () {
				try_writeln! (stream);
				try_writeln! (stream, "Requires the following features: `{}`.", format_value (& features.format ()));
			}
			
			try! (write_description (library, library.description (), library.links (), stream));
			try! (write_links (library, library.links (), stream));
			
			try_writeln! (stream);
			try_writeln! (stream, "----");
			if NAVIGATOR {
				try_writeln! (stream);
				try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
				try_writeln! (stream);
				try_writeln! (stream, "----");
			}
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
				for category in library.categories () {
					if category.has_parent () {
						continue;
					}
					let mut stack = StdVec::new ();
					stack.push ((category, true, category.children ()));
					while let Some ((category, emit, sub_categories)) = stack.pop () {
						if emit {
							let padding = "  " .repeat (stack.len ());
							let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
							if category.has_children () {
								try_writeln! (stream, "{}* [`{}`](#{}):", padding, category.identifier (), category_anchor);
							} else {
								try_writeln! (stream, "{}* [`{}`](#{});", padding, category.identifier (), category_anchor);
							}
							stack.push ((category, false, sub_categories));
						} else {
							let mut sub_categories = sub_categories;
							if let Some (sub_category) = sub_categories.next () {
								stack.push ((category, false, sub_categories));
								stack.push ((sub_category, true, sub_category.children ()));
							}
						}
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
			}
			
			for category in library.categories () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), stream));
				
				try_writeln! (stream, "### Category `{}`", category.identifier ());
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "#### Details");
				
				if let Some (super_category) = category.parent () {
					let super_category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (super_category.identifier ())));
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the super-category: [`{}`](#{}).", super_category.identifier (), super_category_anchor);
				}
				if category.has_children () {
					try_writeln! (stream);
					try_writeln! (stream, "Contains the following sub-categories:");
					for sub_category in category.children_recursive () {
						let sub_category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (sub_category.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{})", sub_category.identifier (), sub_category_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", sub_category.identifier (), sub_category_anchor);
						}
					}
				}
				
				if category.has_value_kinds () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Types");
					try_writeln! (stream);
					for value_kind in category.value_kinds () {
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{})", value_kind.identifier (), value_kind_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
						}
					}
				}
				
				if category.has_definitions () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Definitions");
					try_writeln! (stream);
					for definition in category.definitions () {
						let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{})", definition.identifier (), definition_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", definition.identifier (), definition_anchor);
						}
					}
				}
				
				try! (write_description (library, category.description (), category.links (), stream));
				try! (write_links (library, category.links (), stream));
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
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
				for value_kind in library.value_kinds () {
					if value_kind.has_parent () {
						continue;
					}
					let mut stack = StdVec::new ();
					stack.push ((value_kind, true, value_kind.children ()));
					while let Some ((value_kind, emit, sub_value_kinds)) = stack.pop () {
						if emit {
							let padding = "  " .repeat (stack.len ());
							let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
							if value_kind.has_children () {
								try_writeln! (stream, "{}* [`{}`](#{}):", padding, value_kind.identifier (), value_kind_anchor);
							} else {
								try_writeln! (stream, "{}* [`{}`](#{});", padding, value_kind.identifier (), value_kind_anchor);
							}
							stack.push ((value_kind, false, sub_value_kinds));
						} else {
							let mut sub_value_kinds = sub_value_kinds;
							if let Some (sub_value_kind) = sub_value_kinds.next () {
								stack.push ((value_kind, false, sub_value_kinds));
								stack.push ((sub_value_kind, true, sub_value_kind.children ()));
							}
						}
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
			}
			
			for value_kind in library.value_kinds () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), stream));
				
				try_writeln! (stream, "### Type `{}`", value_kind.identifier ());
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "#### Details");
				
				if value_kind.has_aliases () {
					try_writeln! (stream);
					try_writeln! (stream, "With the following aliases:");
					for alias in value_kind.aliases () {
						try_writeln! (stream, " * `{}`;", alias);
					}
				}
				
				if let Some (features) = library.features () {
					try_writeln! (stream);
					try_writeln! (stream, "Requires the following features: `{}`.", format_value (& features.format ()));
				}
				
				if let Some (super_value_kind) = value_kind.parent () {
					let super_value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (super_value_kind.identifier ())));
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the super-type: [`{}`](#{}).", super_value_kind.identifier (), super_value_kind_anchor);
				}
				if value_kind.has_children () {
					try_writeln! (stream);
					try_writeln! (stream, "Contains the following sub-types:");
					for sub_value_kind in value_kind.children_recursive () {
						let sub_value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (sub_value_kind.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{})", sub_value_kind.identifier (), sub_value_kind_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", sub_value_kind.identifier (), sub_value_kind_anchor);
						}
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
				
				try! (write_description (library, value_kind.description (), value_kind.links (), stream));
				try! (write_links (library, value_kind.links (), stream));
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
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
				for definition in library.definitions () {
					let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
					try_writeln! (stream, "* [`{}`](#{});", definition.identifier (), definition_anchor);
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
			}
			
			for definition in library.definitions () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), stream));
				
				try_writeln! (stream, "### Definition `{}`", definition.identifier ());
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "#### Details");
				
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
				
				if let Some (features) = library.features () {
					try_writeln! (stream);
					try_writeln! (stream, "Requires the following features: `{}`.", format_value (& features.format ()));
				}
				
				if definition.has_categories () {
					try_writeln! (stream);
					try_writeln! (stream, "Belongs to the following categories:");
					for category in definition.categories () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
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
							fn write_procedure_signature_value (library : &Library, value : &ProcedureSignatureValue, prefix : &str, stream : &mut dyn io::Write) -> (Outcome<()>) {
								let value_kind = try_some_2! (value.kind.entity_resolve (), 0x131ac42a);
								let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
								if let Some (identifier) = value.identifier.as_ref () {
									try_writeln! (stream, "{}`{}` of type [`{}`](#{});", prefix, identifier, value_kind.identifier (), value_kind_anchor);
								} else {
									try_writeln! (stream, "{}a value type [`{}`](#{});", prefix, value_kind.identifier (), value_kind_anchor);
								}
								succeed! (());
							}
							if ! procedure_signature_variant.inputs.values.is_empty () {
								let procedure_signature_variant_inputs = &procedure_signature_variant.inputs;
								if procedure_signature_variant_inputs.values.len () > 1 || procedure_signature_variant_inputs.variadic {
									try_writeln! (stream, "   * inputs:");
									for procedure_signature_value in procedure_signature_variant_inputs.values.iter () {
										try! (write_procedure_signature_value (library, procedure_signature_value, "     * ", stream));
									}
									if procedure_signature_variant_inputs.variadic {
										try_writeln! (stream, "     * `...` (i.e. variadic);");
									}
								} else {
									try! (write_procedure_signature_value (library, &procedure_signature_variant_inputs.values[0], "   * input: ", stream));
								}
							} else {
								try_writeln! (stream, "   * inputs: none;");
							}
							if ! procedure_signature_variant.outputs.values.is_empty () {
								let procedure_signature_variant_outputs = &procedure_signature_variant.outputs;
								if procedure_signature_variant_outputs.values.len () > 1 || procedure_signature_variant_outputs.variadic {
									try_writeln! (stream, "   * outputs:");
									for procedure_signature_value in procedure_signature_variant_outputs.values.iter () {
										try! (write_procedure_signature_value (library, procedure_signature_value, "     * ", stream));
									}
									if procedure_signature_variant_outputs.variadic {
										try_writeln! (stream, "     * `...` (i.e. variadic);");
									}
								} else {
									try! (write_procedure_signature_value (library, &procedure_signature_variant_outputs.values[0], "   * output: ", stream));
								}
							} else {
								try_writeln! (stream, "   * outputs: none;");
							}
							if let Some (features) = &procedure_signature_variant.features {
								try_writeln! (stream, "   * requires: `{}`", format_value (& features.format ()));
							}
						}
					}
				} else if definition.kind () .is_procedure () && LINTS {
					try_writeln! (stream);
					try_writeln! (stream, "#### Procedure signature");
					try_writeln! (stream);
					try_writeln! (stream, "**FIXME!**  No procedure signature was provided!");
					try_writeln! (stream);
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
				} else if definition.kind () .is_syntax () && LINTS {
					try_writeln! (stream);
					try_writeln! (stream, "#### Syntax signature");
					try_writeln! (stream);
					try_writeln! (stream, "**FIXME!**  No syntax signature was provided!");
					try_writeln! (stream);
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
				
				try! (write_description (library, definition.description (), definition.links (), stream));
				try! (write_links (library, definition.links (), stream));
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
				
			}
		}
		
		
		if library.has_appendices () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("appendices"), stream));
			
			try_writeln! (stream, "## Appendices");
			
			{
				try_writeln! (stream);
				for appendix in library.appendices () {
					let appendix_anchor = try! (generate_anchor (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ())));
					if let Some (title) = appendix.title () {
						try_writeln! (stream, "* [`{}`](#{}) -- {};", appendix.identifier (), appendix_anchor, title);
					} else {
						try_writeln! (stream, "* [`{}`](#{});", appendix.identifier (), appendix_anchor);
					}
				}
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
			}
			
			for appendix in library.appendices () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), stream));
				
				if let Some (title) = appendix.title () {
					try_writeln! (stream, "### Appendix `{}` -- {}", appendix.identifier (), title);
				} else {
					try_writeln! (stream, "### Appendix `{}`", appendix.identifier ());
				}
				
				try! (write_description (library, appendix.description (), appendix.links (), stream));
				try! (write_links (library, appendix.links (), stream));
				
				try_writeln! (stream);
				try_writeln! (stream, "----");
				if NAVIGATOR {
					try_writeln! (stream);
					try_writeln! (stream, "Goto: [library](#{}), [categories](#{}), [types](#{}), [definitions](#{}), [appendices](#{}).", &library_anchor, &categories_anchor, &value_kinds_anchor, &definitions_anchor, &appendices_anchor);
					try_writeln! (stream);
					try_writeln! (stream, "----");
				}
				try_writeln! (stream);
				
			}
		}
	}
	
	succeed! (());
}


lazy_static! {
	static ref DUMP_CMARK_CATEGORY_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#category\)"), 0x7a74ab93);
	static ref DUMP_CMARK_VALUE_KIND_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#types\)"), 0x93297fed);
	static ref DUMP_CMARK_DEFINITION_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#definitions\)"), 0x0e6d98c5);
	static ref DUMP_CMARK_LINK_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[\[([a-zA-Z0-9_-]+)\]\]\(#links\)"), 0xe10a7e4c);
}

