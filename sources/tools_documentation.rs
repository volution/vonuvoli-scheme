

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
			dump_html,
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
	
	let tool_commands = vec_map! (inputs.tool_commands.iter (), command, command.as_str ());
	let tool_commands = tool_commands.as_slice ();
	let dump_function = match tool_commands {
		["dump-html"] =>
			dump_html,
		["dump-cmark"] =>
			dump_cmark,
		["dump-json"] =>
			dump_json,
		_ =>
			fail! (0x3b57eb47),
	};
	let dump_buffered = match tool_commands {
		["dump-html"] =>
			false,
		["dump-cmark"] =>
			true,
		["dump-json"] =>
			true,
		_ =>
			fail! (0xb603b11c),
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
	
	if dump_buffered {
		let mut buffer = StdVec::with_capacity (BUFFER_SIZE);
		try! (dump_function (&libraries, &mut buffer));
		try_or_fail! (stream.write_all (&buffer), 0xa74a1b0d);
	} else {
		try! (dump_function (&libraries, &mut stream));
	}
	
	succeed! (0);
}




const BUFFER_SIZE : usize = 8 * 1024 * 1024;




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
			
			"super_categories" : dump_json_identifiers_perhaps_for_entities (category.parents ()),
			"super_categories_recursive" : dump_json_identifiers_perhaps_for_entities (category.parents_recursive ()),
			"sub_categories" : dump_json_identifiers_perhaps_for_entities (category.children ()),
			"sub_categories_recursive" : dump_json_identifiers_perhaps_for_entities (category.children_recursive ()),
			
			"types" : dump_json_identifiers_perhaps_for_entities (category.value_kinds ()),
			"types_recursive" : dump_json_identifiers_perhaps_for_entities (category.value_kinds_recursive ()),
			
			"definitions" : dump_json_identifiers_perhaps_for_entities (category.definitions ()),
			"definitions_recursive" : dump_json_identifiers_perhaps_for_entities (category.definitions_recursive ()),
			
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
			
			"super_types" : dump_json_identifiers_perhaps_for_entities (value_kind.parents ()),
			"super_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.parents_recursive ()),
			"sub_types" : dump_json_identifiers_perhaps_for_entities (value_kind.children ()),
			"sub_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.children_recursive ()),
			
			"categories" : dump_json_identifiers_perhaps_for_entities (value_kind.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.categories_recursive ()),
			
			"covariant_types" : dump_json_identifiers_perhaps_for_entities (value_kind.covariants ()),
			"covariant_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.covariants_recursive ()),
			"contravariant_types" : dump_json_identifiers_perhaps_for_entities (value_kind.contravariants ()),
			"contravariant_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.contravariants_recursive ()),
			
			"definitions_input" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input ()),
			"definitions_input_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input_recursive ()),
			"definitions_input_contravariant" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input_contravariant_recursive ()),
			"definitions_output" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output ()),
			"definitions_output_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output_recursive ()),
			"definitions_output_covariant" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output_covariant_recursive ()),
			
			"description" : if let Some (description) = value_kind.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = value_kind.links () { dump_json_links (links) } else { json::Value::Null },
			
			"predicate" : if let Some (predicate) = value_kind.predicate () { dump_json_value (&predicate.format ()) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_definition (definition : &Definition) -> (json::Value) {
	json! ({
			
			"identifier" : definition.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (definition.aliases ()),
			"features" : if let Some (features) = definition.features () { dump_json_features (features) } else { json::Value::Null },
			
			"categories" : dump_json_identifiers_perhaps_for_entities (definition.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_entities (definition.categories_recursive ()),
			
			"kind" : json::Value::String (StdString::from (definition.kind () .identifier ())),
			
			"description" : if let Some (description) = definition.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = definition.links () { dump_json_links (links) } else { json::Value::Null },
			
			"procedure_signature" : if let Some (procedure_signature) = definition.procedure_signature () { dump_json_procedure_signature (procedure_signature) } else { json::Value::Null },
			"syntax_signature" : if let Some (syntax_signature) = definition.syntax_signature () { dump_json_syntax_signature (syntax_signature) } else { json::Value::Null },
			
			"referenced_types" : dump_json_identifiers_perhaps_for_entities (definition.referenced_value_kinds ()),
			
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
pub fn dump_html (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	let mut cmark_buffer = StdVec::with_capacity (BUFFER_SIZE);
	try! (dump_cmark_0 (libraries, &mut cmark_buffer, true));
	
	let cmark_buffer = try_or_fail! (StdString::from_utf8 (cmark_buffer), 0xb06a2a9f);
	let parser = ext::pulldown_cmark::Parser::new (&cmark_buffer);
	
	let mut html_buffer = StdString::with_capacity (BUFFER_SIZE);
	html_buffer.push_str (DUMP_HTML_PREFIX);
	html_buffer.push_str ("<style type='text/css'>\n");
	html_buffer.push_str (DUMP_HTML_CSS);
	html_buffer.push_str ("</style>\n");
	
	ext::pulldown_cmark::html::push_html (&mut html_buffer, parser);
	
	html_buffer.push_str (DUMP_HTML_SUFFIX);
	
	try_or_fail! (stream.write_all (html_buffer.as_bytes ()), 0x4aed615a);
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
pub fn dump_cmark (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	return dump_cmark_0 (libraries, stream, false);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_0 (libraries : &Libraries, stream : &mut dyn io::Write, use_html : bool) -> (Outcome<()>) {
	
	
	const ALL : bool = false;
	const DEBUG : bool = false;
	
	const NO_SUPER : bool = false;
	const NO_SUB : bool = false;
	const NO_TREE : bool = false;
	const NO_LIBRARIES : bool = false;
	const NO_CATEGORIES : bool = false;
	const NO_VALUE_KINDS : bool = false;
	const NO_VARIANTS : bool = true;
	const NO_DEFINITIONS : bool = false;
	const NO_APPENDICES : bool = false;
	const NO_RECURSIVE : bool = false;
	const NO_DETAILS : bool = false;
	const NO_TOC : bool = false;
	const NO_NOTES : bool = false;
	const NO_FIXME : bool = true;
	
	const COMPACT : bool = true;
	const NAVIGATOR : bool = true;
	const ANCHORS : bool = true;
	const FIXME : bool = ALL || !NO_FIXME;
	const LINTS : bool = DEBUG && !NO_FIXME;
	
	const RECURSIVE_TOC_COMPLETE : bool = true;
	const RECURSIVE_TOC_DEPTH : usize = 2;
	const RECURSIVE_TREE_COMPLETE : bool = true;
	const RECURSIVE_TREE_DEPTH : usize = 2;
	const RECURSIVE_COMPLETE : bool = false;
	
	
	const LIBRARIES : bool = ALL || !NO_LIBRARIES;
	const LIBRARIES_TOC : bool = ALL || !NO_TOC;
	
	const CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const CATEGORIES_TOC : bool = ALL || !NO_TOC;
	const CATEGORIES_SUPER : bool = ALL || !NO_SUPER;
	const CATEGORIES_SUPER_RECURSIVE : bool = CATEGORIES_SUPER && (ALL || !NO_RECURSIVE);
	const CATEGORIES_SUB : bool = ALL || !NO_SUB;
	const CATEGORIES_SUB_RECURSIVE : bool = CATEGORIES_SUB && (ALL || !NO_RECURSIVE);
	const CATEGORIES_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const CATEGORIES_VALUE_KINDS_RECURSIVE : bool = CATEGORIES_VALUE_KINDS && (ALL || !NO_RECURSIVE);
	const CATEGORIES_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const CATEGORIES_DEFINITIONS_RECURSIVE : bool = CATEGORIES_DEFINITIONS && (ALL || !NO_RECURSIVE);
	
	const VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const VALUE_KINDS_TOC : bool = ALL || !NO_TOC;
	const VALUE_KINDS_TREE : bool = ALL || !NO_TREE;
	const VALUE_KINDS_SUPER : bool = ALL || !NO_SUPER;
	const VALUE_KINDS_SUPER_RECURSIVE : bool = VALUE_KINDS_SUPER && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_SUB : bool = ALL || !NO_SUB;
	const VALUE_KINDS_SUB_RECURSIVE : bool = VALUE_KINDS_SUB && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_VARIANTS : bool = ALL || !NO_VARIANTS;
	const VALUE_KINDS_VARIANTS_RECURSIVE : bool = VALUE_KINDS_VARIANTS && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_COVARIANTS : bool = VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_COVARIANTS_RECURSIVE : bool = VALUE_KINDS_COVARIANTS && VALUE_KINDS_VARIANTS_RECURSIVE;
	const VALUE_KINDS_CONTRAVARIANTS : bool = VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_CONTRAVARIANTS_RECURSIVE : bool = VALUE_KINDS_CONTRAVARIANTS && VALUE_KINDS_VARIANTS_RECURSIVE;
	const VALUE_KINDS_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_INPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_INPUT && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT : bool = VALUE_KINDS_DEFINITIONS_INPUT && VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_DEFINITIONS_OUTPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_PREDICATE : bool = ALL || !NO_DETAILS;
	const VALUE_KINDS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	
	const DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const DEFINITIONS_TOC : bool = ALL || !NO_TOC;
	const DEFINITIONS_KIND : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_SIGNATURE : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const DEFINITIONS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	
	const APPENDICES : bool = ALL || !NO_APPENDICES;
	const APPENDICES_TOC : bool = ALL || !NO_TOC;
	
	const ALIASES : bool = ALL || !NO_DETAILS;
	const FEATURES : bool = ALL || !NO_DETAILS;
	const DESCRIPTIONS : bool = ALL || !NO_DETAILS;
	const LINKS : bool = ALL || !NO_DETAILS;
	const NOTES : bool = ALL || !NO_NOTES;
	
	
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
					"value_kind" => "type",
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
	fn write_anchor (prefix : Option<&str>, library : Option<&str>, identifier : Option<&str>, stream : &mut dyn io::Write, use_html : bool) -> (Outcome<()>) {
		if ANCHORS && use_html {
			let anchor = try! (generate_anchor (prefix, library, identifier));
			try_writeln! (stream, "<div class='anchor'><a id='{}'></a></div>\n", anchor);
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
		if ! DESCRIPTIONS {
			succeed! (());
		}
		let description = if let Some (description) = description {
			description
		} else {
			succeed! (());
		};
		if !FIXME {
			let mut lines = description.lines ();
			if let Some (first) = lines.next () {
				if (first == "**FIXME!**") || (first == "FIXME!") {
					if lines.next () .is_none () {
						succeed! (());
					}
				}
			} else {
				succeed! (());
			}
		}
		let lines_empty = description.lines () .is_empty ();
		if lines_empty && !LINTS {
			succeed! (());
		}
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Description");
		try_writeln! (stream);
		if lines_empty {
			try_writeln! (stream, "> **FIXME!**");
			succeed! (());
		}
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
			let line = DUMP_CMARK_APPENDIX_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
						let identifier = try_some_or_panic! (captures.get (1), 0x42082eb8);
						let identifier = identifier.as_str ();
						if let Some (appendix) = library.appendix_resolve (identifier) {
							let appendix_anchor = try_or_panic_0! (generate_anchor (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ())), 0x10a5c400);
							let appendix_label = appendix.title () .unwrap_or_else (|| appendix.identifier ());
							format! ("[\"{}\"](#{})", appendix_label, appendix_anchor)
						} else {
							if LINTS {
								format! ("[[{}] **ERROR!**](#errors)", identifier)
							} else {
								format! ("[[{}]](#errors)", identifier)
							}
						}
					});
			try_writeln! (stream, "> {}", line);
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_links (_library : &Library, links : Option<&Links>, stream : &mut dyn io::Write) -> (Outcome<()>) {
		if ! LINKS {
			succeed! (());
		}
		let links = if let Some (links) = links {
			links
		} else {
			succeed! (());
		};
		let links_empty = links.links () .is_empty ();
		if links_empty && !LINTS {
			succeed! (());
		}
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Links");
		try_writeln! (stream);
		if links_empty {
			try_writeln! (stream, "> **FIXME!**");
			succeed! (());
		}
		fail_unimplemented! (0x81cb5f76);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_type_tree <'a> (library : &Library, value_kind : &'a ValueKind, value_kinds_seen : &mut StdSet<&'a str>, stream : &mut dyn io::Write, recursive_complete : bool, recursive_depth : usize) -> (Outcome<()>) {
		let mut stack = StdVec::new ();
		stack.push ((value_kind, true, value_kind.children ()));
		while let Some ((value_kind, emit, sub_value_kinds)) = stack.pop () {
			if emit {
				let seen = if value_kinds_seen.contains (value_kind.identifier ()) {
					if recursive_complete { true } else { continue; }
				} else {
					value_kinds_seen.insert (value_kind.identifier ()); false
				};
				if recursive_complete || !seen {
					let padding = "  " .repeat (stack.len ());
					let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
					let fixes = if recursive_complete && !seen { "**" } else { "" };
					if value_kind.has_children () {
						try_writeln! (stream, "{}* {}[`{}`](#{}){}:", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
					} else {
						try_writeln! (stream, "{}* {}[`{}`](#{}){};", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
					}
					stack.push ((value_kind, false, sub_value_kinds));
				}
			} else {
				let mut sub_value_kinds = sub_value_kinds;
				if (stack.len () + 1) <= recursive_depth {
					if let Some (sub_value_kind) = sub_value_kinds.next () {
						stack.push ((value_kind, false, sub_value_kinds));
						stack.push ((sub_value_kind, true, sub_value_kind.children ()));
					}
				} else if ! sub_value_kinds.is_empty () {
					let padding = "  " .repeat (stack.len () + 1);
					try_writeln! (stream, "{}* ...", padding);
				}
			}
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write_break (library : &Library, stream : &mut dyn io::Write, use_html : bool) -> (Outcome<()>) {
		try_writeln! (stream);
		try_writeln! (stream, "----");
		if NAVIGATOR {
			try_writeln! (stream);
			if !use_html {
				try_write! (stream, "Goto:");
			} else {
				try_write! (stream, "<div class='navigator'><span class='navigator-header'>Goto:</span>");
			}
			let mut empty = true;
			if LIBRARIES {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let library_anchor = try! (generate_anchor (Some ("library"), Some (library.identifier ()), None));
				if !use_html {
					try_write! (stream, "[library](#{})", &library_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='#{}'>library</a>", &library_anchor);
				}
			}
			if CATEGORIES {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let categories_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories")));
				if !use_html {
					try_write! (stream, "[categories](#{})", &categories_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='#{}'>categories</a>", &categories_anchor);
				}
			}
			if DEFINITIONS {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let definitions_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("definitions")));
				if !use_html {
					try_write! (stream, "[definitions](#{})", &definitions_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='#{}'>definitions</a>", &definitions_anchor);
				}
			}
			if VALUE_KINDS {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let value_kinds_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds")));
				if !use_html {
					try_write! (stream, "[types](#{})", &value_kinds_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='#{}'>types</a>", &value_kinds_anchor);
				}
			}
			if APPENDICES {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let appendices_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("appendices")));
				if !use_html {
					try_write! (stream, "[appendices](#{})", &appendices_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='#{}'>appendices</a>", &appendices_anchor);
				}
			}
			if !empty {
				if !use_html {
					try_writeln! (stream, ".");
				} else {
					try_writeln! (stream, ".</div>");
				}
			} else {
				if !use_html {
					try_writeln! (stream, " (nothing).");
				} else {
					try_writeln! (stream, " (nothing).</div>");
				}
			}
			try_writeln! (stream);
			try_writeln! (stream, "----");
		}
		try_writeln! (stream);
		succeed! (());
	}
	
	
	for library in libraries.libraries () {
		
		
		if LIBRARIES {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("library"), Some (library.identifier ()), None, stream, use_html));
			
			if let Some (title) = library.title () {
				try_writeln! (stream, "# `{}` -- {}", library.identifier (), title);
			} else {
				try_writeln! (stream, "# `{}`", library.identifier ());
			}
			
			if LIBRARIES_TOC {
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "#### Contents");
				try_writeln! (stream);
				let mut empty = true;
				if CATEGORIES && library.has_categories () {
					let categories_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories")));
					try_writeln! (stream, " * [categories](#{});", &categories_anchor);
					empty = false;
				}
				if DEFINITIONS && library.has_definitions () {
					let definitions_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("definitions")));
					try_writeln! (stream, " * [definitions](#{});", &definitions_anchor);
					empty = false;
				}
				if VALUE_KINDS && library.has_value_kinds () {
					let value_kinds_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds")));
					try_writeln! (stream, " * [types](#{});", &value_kinds_anchor);
					empty = false;
				}
				if APPENDICES && library.has_appendices () {
					let appendices_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("appendices")));
					try_writeln! (stream, " * [appendices](#{});", &appendices_anchor);
					empty = false;
				}
				if empty {
					try_writeln! (stream, " * (nothing);");
				}
			}
			
			if FEATURES {
				if let Some (features) = library.features () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Features");
					try_writeln! (stream);
					try_writeln! (stream, "````");
					try_writeln! (stream, "{}", format_value (& features.format ()));
					try_writeln! (stream, "````");
				}
			}
			
			try! (write_description (library, library.description (), library.links (), stream));
			try! (write_links (library, library.links (), stream));
			
			try! (write_break (library, stream, use_html));
		}
		
		if CATEGORIES && library.has_categories () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories"), stream, use_html));
			
			if CATEGORIES_TOC {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "## Categories");
				try_writeln! (stream);
				
				for category in library.categories () {
					if category.has_parents () {
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
				
				try! (write_break (library, stream, use_html));
			}
			
			for category in library.categories () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), stream, use_html));
				
				try_writeln! (stream, "### Category `{}`", category.identifier ());
				
				if CATEGORIES_DEFINITIONS && category.has_definitions () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Definitions");
					try_writeln! (stream);
					for definition in category.definitions () {
						let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", definition.identifier (), definition_anchor);
					}
				}
				
				if CATEGORIES_VALUE_KINDS && category.has_value_kinds () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Types");
					try_writeln! (stream);
					for value_kind in category.value_kinds () {
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
					}
				}
				
				try! (write_description (library, category.description (), category.links (), stream));
				try! (write_links (library, category.links (), stream));
				
				if CATEGORIES_SUPER && category.has_parents () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Super-category");
					try_writeln! (stream);
					for category in category.parents () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
					if CATEGORIES_SUPER_RECURSIVE
							&& category.parents () .count () != category.parents_recursive () .count ()
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "##### Super-categories recursive");
						try_writeln! (stream);
						for category in category.parents_recursive () {
							let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
							if COMPACT {
								try_writeln! (stream, "[`{}`](#{});", category.identifier (), category_anchor);
							} else {
								try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
							}
						}
					}
				} else if CATEGORIES_SUPER {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Super-category");
					try_writeln! (stream);
					let categories_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("categories")));
					try_writeln! (stream, " * [(none)](#{});", &categories_anchor);
				}
				
				if CATEGORIES_SUB && category.has_children () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Sub-categories");
					try_writeln! (stream);
					for category in category.children () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
					if CATEGORIES_SUB_RECURSIVE
							&& category.children () .count () != category.children_recursive () .count ()
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "##### Sub-categories recursive");
						try_writeln! (stream);
						for category in category.children_recursive () {
							let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
							if COMPACT {
								try_writeln! (stream, "[`{}`](#{});", category.identifier (), category_anchor);
							} else {
								try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
							}
						}
					}
				}
				
				if CATEGORIES_DEFINITIONS_RECURSIVE
						&& category.definitions () .count () != category.definitions_recursive () .count ()
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Definitions recursive");
					try_writeln! (stream);
					for definition in category.definitions_recursive () {
						let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{});", definition.identifier (), definition_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", definition.identifier (), definition_anchor);
						}
					}
				}
				
				if CATEGORIES_VALUE_KINDS_RECURSIVE
						&& category.value_kinds () .count () != category.value_kinds_recursive () .count ()
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Types recursive");
					try_writeln! (stream);
					for value_kind in category.value_kinds_recursive () {
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						if COMPACT {
							try_writeln! (stream, "[`{}`](#{});", value_kind.identifier (), value_kind_anchor);
						} else {
							try_writeln! (stream, " * [`{}`](#{});", value_kind.identifier (), value_kind_anchor);
						}
					}
				}
				
				try! (write_break (library, stream, use_html));
			}
		}
		
		
		if DEFINITIONS && library.has_definitions () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("definitions"), stream, use_html));
			
			
			if DEFINITIONS_TOC {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "## Definitions");
				try_writeln! (stream);
				
				for definition in library.definitions () {
					let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
					try_writeln! (stream, "* [`{}`](#{});", definition.identifier (), definition_anchor);
				}
				
				try! (write_break (library, stream, use_html));
			}
			
			for definition in library.definitions () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), stream, use_html));
				
				try_writeln! (stream, "### Definition `{}`", definition.identifier ());
				
				if DEFINITIONS_KIND {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Kind");
					try_writeln! (stream);
					try_writeln! (stream, " * `{}`;", definition.kind () .identifier ());
				}
				
				if let Some (procedure_signature) = if DEFINITIONS_SIGNATURE { definition.procedure_signature () } else { None } {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Procedure signature");
					if ! procedure_signature.variants.is_empty () {
						try_writeln! (stream);
						try_writeln! (stream, "Procedure variants:");
						for procedure_signature_variant in procedure_signature.variants.iter () {
							#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
							fn write_procedure_signature_value (library : &Library, value : &ProcedureSignatureValue, prefix : &str, stream : &mut dyn io::Write) -> (Outcome<()>) {
								let value_kind = try_some_2! (value.kind.entity_resolve (), 0x131ac42a);
								let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
								if let Some (identifier) = value.identifier.as_ref () {
									try_writeln! (stream, "{}`{}` of type [`{}`](#{});", prefix, identifier, value_kind.identifier (), value_kind_anchor);
								} else {
									try_writeln! (stream, "{}a value of type [`{}`](#{});", prefix, value_kind.identifier (), value_kind_anchor);
								}
								succeed! (());
							}
							try_writeln! (stream, " * `{}`", format_value (& procedure_signature_variant.format ()));
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
				} else if definition.kind () .is_procedure () && LINTS && DEFINITIONS_SIGNATURE {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Procedure signature");
					try_writeln! (stream);
					try_writeln! (stream, "**FIXME!**  No procedure signature was provided!");
				}
				
				if let Some (syntax_signature) = if DEFINITIONS_SIGNATURE { definition.syntax_signature () } else { None } {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Syntax signature");
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
				} else if definition.kind () .is_syntax () && LINTS && DEFINITIONS_SIGNATURE {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Syntax signature");
					try_writeln! (stream);
					try_writeln! (stream, "**FIXME!**  No syntax signature was provided!");
				}
				
				if DEFINITIONS_VALUE_KINDS && definition.has_referenced_value_kinds () {
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
				
				if ALIASES && definition.has_aliases () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Aliases");
					try_writeln! (stream);
					for alias in definition.aliases () {
						try_writeln! (stream, " * `{}`;", alias);
					}
				}
				
				if FEATURES {
					if let Some (features) = definition.features () {
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Features");
						try_writeln! (stream);
						try_writeln! (stream, "````");
						try_writeln! (stream, "{}", format_value (& features.format ()));
						try_writeln! (stream, "````");
					}
				}
				
				if DEFINITIONS_CATEGORIES && definition.has_categories () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Categories");
					try_writeln! (stream);
					for category in definition.categories () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
				try! (write_break (library, stream, use_html));
			}
		}
		
		
		if VALUE_KINDS && library.has_value_kinds () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), stream, use_html));
			
			if VALUE_KINDS_TOC {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "## Types");
				try_writeln! (stream);
				
				let mut value_kinds_seen = StdSet::new ();
				for value_kind in library.value_kinds () {
					if value_kind.has_parents () {
						continue;
					}
					try! (write_type_tree (library, value_kind, &mut value_kinds_seen, stream, RECURSIVE_TOC_COMPLETE, RECURSIVE_TOC_DEPTH));
				}
				
				try! (write_break (library, stream, use_html));
			}
			
			for value_kind in library.value_kinds () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), stream, use_html));
				
				try_writeln! (stream, "### Type `{}`", value_kind.identifier ());
				
				if VALUE_KINDS_TREE
						&& value_kind.has_children ()
						&& value_kind.children () .count () != value_kind.children_recursive () .count ()
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Sub-types tree");
					try_writeln! (stream);
					let mut value_kinds_seen = StdSet::new ();
					for value_kind in value_kind.children () {
						try! (write_type_tree (library, value_kind, &mut value_kinds_seen, stream, RECURSIVE_TREE_COMPLETE, RECURSIVE_TREE_DEPTH));
					}
				}
				
				let mut value_kind_covariants_seen = StdSet::new ();
				let mut value_kind_contravariants_seen = StdSet::new ();
				
				if VALUE_KINDS_SUPER && value_kind.has_parents () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Super-type");
					try_writeln! (stream);
					for value_kind in value_kind.parents () {
						let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_covariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
					}
					if VALUE_KINDS_SUPER_RECURSIVE
							&& (RECURSIVE_COMPLETE || value_kind.parents () .count () != value_kind.parents_recursive () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "##### Super-types recursive");
						try_writeln! (stream);
						for value_kind in value_kind.parents_recursive () {
							let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_covariants_seen.insert (value_kind.identifier ()); false
							};
							let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
							}
						}
					}
				} else if VALUE_KINDS_SUPER {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Super-type");
					try_writeln! (stream);
					let value_kinds_anchor = try! (generate_anchor (Some ("toc"), Some (library.identifier ()), Some ("value_kinds")));
					try_writeln! (stream, " * [(none)](#{});", &value_kinds_anchor);
				}
				
				if VALUE_KINDS_SUB && value_kind.has_children () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Sub-types");
					try_writeln! (stream);
					for value_kind in value_kind.children () {
						let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_contravariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
					}
					if VALUE_KINDS_SUB_RECURSIVE
							&& (RECURSIVE_COMPLETE || value_kind.children () .count () != value_kind.children_recursive () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "##### Sub-types recursive");
						try_writeln! (stream);
						for value_kind in value_kind.children_recursive () {
							let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_contravariants_seen.insert (value_kind.identifier ()); false
							};
							let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
							}
						}
					}
				}
				
				if VALUE_KINDS_COVARIANTS
						&& value_kind.has_covariants ()
						&& (RECURSIVE_COMPLETE || value_kind.covariants () .count () != value_kind_covariants_seen.len ())
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Covariant types");
					try_writeln! (stream);
					for value_kind in value_kind.covariants () {
						let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_covariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
					}
					if NOTES {
						try_writeln! (stream);
						try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
					}
				}
				if VALUE_KINDS_COVARIANTS_RECURSIVE
						&& ! value_kind.covariants_recursive () .is_empty ()
						&& (RECURSIVE_COMPLETE || value_kind.covariants_recursive () .count () != value_kind_covariants_seen.len ())
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Covariant types recursive");
					try_writeln! (stream);
					for value_kind in value_kind.covariants_recursive () {
						let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_covariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						if COMPACT {
							try_writeln! (stream, "{}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
						} else {
							try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
						}
					}
					if NOTES {
						try_writeln! (stream);
						try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
					}
				}
				
				if VALUE_KINDS_CONTRAVARIANTS
						&& value_kind.has_contravariants ()
						&& (RECURSIVE_COMPLETE || value_kind.contravariants () .count () != value_kind_contravariants_seen.len ())
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Contravariant types");
					try_writeln! (stream);
					for value_kind in value_kind.contravariants () {
						let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_contravariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
					}
					if NOTES {
						try_writeln! (stream);
						try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
					}
				}
				if VALUE_KINDS_CONTRAVARIANTS_RECURSIVE
						&& ! value_kind.contravariants_recursive () .is_empty ()
						&& (RECURSIVE_COMPLETE || value_kind.contravariants_recursive () .count () != value_kind_contravariants_seen.len ())
				{
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Contravariant types recursive");
					try_writeln! (stream);
					for value_kind in value_kind.contravariants_recursive () {
						let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
							if RECURSIVE_COMPLETE { true } else { continue; }
						} else {
							value_kind_contravariants_seen.insert (value_kind.identifier ()); false
						};
						let value_kind_anchor = try! (generate_anchor (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ())));
						let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
						if COMPACT {
							try_writeln! (stream, "{}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
						} else {
							try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
						}
					}
					if NOTES {
						try_writeln! (stream);
						try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
					}
				}
				
				if VALUE_KINDS_DEFINITIONS {
					let mut value_kind_definitions_seen = StdSet::new ();
					if VALUE_KINDS_DEFINITIONS_INPUT
							&& value_kind.has_definitions_input ()
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as input");
						try_writeln! (stream);
						for definition in value_kind.definitions_input () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
					}
					if VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE
							&& value_kind.has_definitions_input ()
							&& (RECURSIVE_COMPLETE || value_kind.definitions_input_recursive () .count () != value_kind.definitions_input () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as input (recursive)");
						try_writeln! (stream);
						for definition in value_kind.definitions_input_recursive () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
						if NOTES {
							try_writeln! (stream);
							try_writeln! (stream, "Note:  These definitions consume an input that is a super-type.");
						}
					}
					if VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT
							&& ! value_kind.definitions_input_contravariant_recursive () .is_empty ()
							&& (RECURSIVE_COMPLETE || value_kind.definitions_input_contravariant_recursive () .count () != value_kind.definitions_input_recursive () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as input (contravariant)");
						try_writeln! (stream);
						for definition in value_kind.definitions_input_contravariant_recursive () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
						if NOTES {
							try_writeln! (stream);
							try_writeln! (stream, "Note:  These definitions consume an input that is a super-type-like (i.e. contravariant).");
						}
					}
				}
				
				if VALUE_KINDS_DEFINITIONS {
					let mut value_kind_definitions_seen = StdSet::new ();
					if VALUE_KINDS_DEFINITIONS_OUTPUT
							&& value_kind.has_definitions_output ()
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as output");
						try_writeln! (stream);
						for definition in value_kind.definitions_output () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
					}
					if VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE
							&& value_kind.has_definitions_output ()
							&& (RECURSIVE_COMPLETE || value_kind.definitions_output_recursive () .count () != value_kind.definitions_output () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as output (recursive)");
						try_writeln! (stream);
						for definition in value_kind.definitions_output_recursive () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
						if NOTES {
							try_writeln! (stream);
							try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type.");
						}
					}
					if VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT
							&& ! value_kind.definitions_output_covariant_recursive () .is_empty ()
							&& (RECURSIVE_COMPLETE || value_kind.definitions_output_covariant_recursive () .count () != value_kind.definitions_output_recursive () .count ())
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Referent definitions as output (covariant)");
						try_writeln! (stream);
						for definition in value_kind.definitions_output_covariant_recursive () {
							let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
								if RECURSIVE_COMPLETE { true } else { continue; }
							} else {
								value_kind_definitions_seen.insert (definition.identifier ()); false
							};
							let definition_anchor = try! (generate_anchor (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ())));
							let fixes = if RECURSIVE_COMPLETE && !seen { "**" } else { "" };
							if COMPACT {
								try_writeln! (stream, "{}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							} else {
								try_writeln! (stream, " * {}[`{}`](#{}){};", fixes, definition.identifier (), definition_anchor, fixes);
							}
						}
						if NOTES {
							try_writeln! (stream);
							try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type-like (i.e. covariant).");
						}
					}
				}
				
				try! (write_description (library, value_kind.description (), value_kind.links (), stream));
				try! (write_links (library, value_kind.links (), stream));
				
				if VALUE_KINDS_PREDICATE {
					if let Some (predicate) = value_kind.predicate () {
						match predicate {
							ValueKindPredicate::None =>
								(),
							ValueKindPredicate::Inherit =>
								(),
							ValueKindPredicate::Fixme =>
								if FIXME {
									try_writeln! (stream);
									try_writeln! (stream);
									try_writeln! (stream, "#### Predicate");
									try_writeln! (stream);
									try_writeln! (stream, "**FIXME!**");
								}
							ValueKindPredicate::Expression (ref value) =>
								{
									try_writeln! (stream);
									try_writeln! (stream);
									try_writeln! (stream, "#### Predicate");
									try_writeln! (stream);
									try_writeln! (stream, "```");
									try_writeln! (stream, "{}", format_value (value));
									try_writeln! (stream, "```");
								},
						}
					} else {
						if LINTS {
							try_writeln! (stream);
							try_writeln! (stream);
							try_writeln! (stream, "#### Predicate");
							try_writeln! (stream);
							try_writeln! (stream, "**FIXME!**  No predicate was provided!");
						}
					}
				}
				
				if ALIASES && value_kind.has_aliases () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Aliases");
					try_writeln! (stream);
					for alias in value_kind.aliases () {
						try_writeln! (stream, " * `{}`;", alias);
					}
				}
				
				if FEATURES {
					if let Some (features) = value_kind.features () {
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Features");
						try_writeln! (stream);
						try_writeln! (stream, "````");
						try_writeln! (stream, "{}", format_value (& features.format ()));
						try_writeln! (stream, "````");
					}
				}
				
				if VALUE_KINDS_CATEGORIES && value_kind.has_categories () {
					try_writeln! (stream);
					try_writeln! (stream);
					try_writeln! (stream, "#### Categories");
					try_writeln! (stream);
					for category in value_kind.categories () {
						let category_anchor = try! (generate_anchor (Some ("category"), Some (library.identifier ()), Some (category.identifier ())));
						try_writeln! (stream, " * [`{}`](#{});", category.identifier (), category_anchor);
					}
				}
				
				try! (write_break (library, stream, use_html));
			}
		}
		
		
		if APPENDICES && library.has_appendices () {
			
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream);
			try! (write_anchor (Some ("toc"), Some (library.identifier ()), Some ("appendices"), stream, use_html));
			
			if APPENDICES_TOC {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "## Appendices");
				try_writeln! (stream);
				
				for appendix in library.appendices () {
					let appendix_anchor = try! (generate_anchor (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ())));
					if let Some (title) = appendix.title () {
						try_writeln! (stream, "* [`{}`](#{}) -- {};", appendix.identifier (), appendix_anchor, title);
					} else {
						try_writeln! (stream, "* [`{}`](#{});", appendix.identifier (), appendix_anchor);
					}
				}
				
				try! (write_break (library, stream, use_html));
			}
			
			for appendix in library.appendices () {
				
				try_writeln! (stream);
				try_writeln! (stream);
				try! (write_anchor (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), stream, use_html));
				
				if let Some (title) = appendix.title () {
					try_writeln! (stream, "### Appendix `{}` -- {}", appendix.identifier (), title);
				} else {
					try_writeln! (stream, "### Appendix `{}`", appendix.identifier ());
				}
				
				try! (write_description (library, appendix.description (), appendix.links (), stream));
				try! (write_links (library, appendix.links (), stream));
				
				try! (write_break (library, stream, use_html));
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
	static ref DUMP_CMARK_APPENDIX_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[\[([a-zA-Z0-9_-]+)\]\]\(#appendices\)"), 0x039d98a7);
}




static DUMP_HTML_PREFIX : &str =
r####"<!DOCTYPE html>
<html>
<head>
	<meta charset="UTF-8">
	<title>Scheme libraries</title>
	<link rel="stylesheet" href="https://code.cdn.mozilla.net/fonts/fira.css">
</head>
<body>
"####;


static DUMP_HTML_SUFFIX : &str =
r####"</body>
</html>
"####;


static DUMP_HTML_CSS : &str =
r####"

* {
	all: initial;
	all: unset;
	box-sizing: content-box;
}
* {
	color : inherit;
	background : transparent;
	line-height : inherit;
	font-family : inherit;
	font-size : inherit;
	font-size-adjust : inherit;
	font-weight : inherit;
	font-stretch : inherit;
	font-variant : inherit;
	cursor : inherit;
}

html:root > *, html:root style {
	display : none;
}
html:root, html:root > body {
	display : block;
}
html:root {
	font-family : "Fira Sans";
	font-size : 1.00em;
	line-height : normal;
}

html:root > body {
	margin-left : auto;
	margin-right : auto;
	max-width : 100ch;
	cursor : default;
}

h1, h2, h3, h4, h5, h6,
p, blockquote,
pre {
	display : block;
	margin-top : 0.50rem;
	margin-bottom : 0.50rem;
}
ul, ol {
	display : block;
}

h1, h2, h3 {
	margin-top : 5.00rem;
	margin-bottom : 1.00rem;
	border-color : hsl(0, 0%, 0%);
	border-bottom-style : solid;
	border-bottom-width : 0.10em;
}
h4 {
	margin-top : 2.00rem;
	margin-bottom : 1.00rem;
	border-color : hsl(0, 0%, 75%);
	border-bottom-style : solid;
	border-bottom-width : 0.10em;
}
h5, h6 {
	margin-top : 2.00rem;
	margin-bottom : 1.00rem;
}
h1, h2, h3, h4, h5, h6 {
	font-weight : bolder;
}
h1 {
	font-size : 1.75rem;
}
h2 {
	font-size : 1.50rem;
}
h2 {
	font-size : 1.40rem;
}
h3 {
	font-size : 1.30rem;
}
h4 {
	font-size : 1.20rem;
}
h5 {
	font-size : 1.10rem;
}
h6 {
	font-size : 1.00rem;
}

html:root > body > blockquote {
	padding-left : 1.00rem;
	padding-right : 1.00rem;
	border-color : hsl(0, 0%, 75%);
	border-left-style : dashed;
	border-left-width : 0.10em;
	border-right-style : dashed;
	border-right-width : 0.10em;
}
html:root > body > blockquote > * {
	padding-left : 1.00rem;
	padding-right : 1.00rem;
	margin-top : 1.00rem;
	margin-bottom : 1.00rem;
}
blockquote blockquote {
	padding-top : 0.50rem;
	padding-bottom : 0.50rem;
	border-color : hsl(0, 0%, 75%);
	border-left-style : solid;
	border-left-width : 0.20em;
	border-right-style : solid;
	border-right-width : 0.20em;
}

code, pre {
	font-family : "Fira Mono";
	background : hsla(0, 0%, 50%, 0.1);
}
code {
	display : inline-block;
	padding : 0.20rem;
}
h1 > code, h2 > code, h3 > code, h4 > code, h5 > code, h6 > code,
a > code {
	color : inherit;
	padding : 0px;
	background : transparent;
}

pre {
	padding : 1.00rem;
	white-space : pre;
	overflow : auto;
}
pre > code {
	display : block;
	background : transparent;
	padding : 0px;
}

ul > li, ol > li {
	display : list-item;
	margin-left : 2.00rem;
}
ul > li {
	list-style-type: square;
}
ol > li {
	list-style-type: decimal;
}

a {
	color : hsl(210, 100%, 40%);
	cursor : pointer;
}
a:hover {
	background : hsla(210, 100%, 50%, 0.05);
	border-color : hsla(210, 100%, 50%, 0.20);
	border-top-style : solid;
	border-top-width : 0.1rem;
	border-bottom-style : solid;
	border-bottom-width : 0.1rem;
}

em {
	font-style : italic;
}
strong {
	font-weight : bolder;
}

hr {
	display : none;
}

div.navigator {
	display : block;
	margin-top : 0.50rem;
	margin-bottom : 2.00rem;
	margin-left : auto;
	margin-right : auto;
	padding : 2.00rem;
	font-size : 1.25rem;
	text-align : right;
	opacity : 0.25;
}
div.navigator:hover {
	opacity : 1.00;
}
div.anchor {
	display : block;
}

*::-moz-selection {
	color : hsl(30, 100%, 40%);
	background : hsla(30, 100%, 50%, 0.05);
}

"####;

