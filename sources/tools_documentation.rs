

use super::documentation::exports::*;
use super::documentation::exports::documentation_model::*;
use super::errors::exports::*;
use super::tools::exports::*;

use super::externals::serde_json as json;
use super::externals::cpio::newc as cpio;

use super::prelude::*;

use super::documentation::exports::documentation_model::Entity;
use super::values::exports::Value as SchemeValue;
use super::values::exports::ValueKind as SchemeValueKind;




pub mod exports {
	
	pub use super::main;
	
	pub use super::{
			dump_json,
			dump_json_0,
		};
	
	pub use super::{
			dump_html,
			dump_html_0,
			dump_html_cpio,
			dump_html_cpio_0,
		};
	
	pub use super::{
			dump_cmark,
			dump_cmark_0,
			dump_cmark_cpio,
			dump_cmark_cpio_0,
		};
	
	pub use super::{
			DumpCmarkAnchor,
			DumpCmarkAnchorInto,
			DumpCmarkCallbacks,
			DumpCpioWriter,
		};
	
	pub use super::{
			dump_cmark_configure,
			DumpCmarkLibrariesConfiguration,
			DumpCmarkLibraryConfiguration,
			DumpCmarkCategoriesConfiguration,
			DumpCmarkCategoryConfiguration,
			DumpCmarkExportsConfiguration,
			DumpCmarkExportConfiguration,
			DumpCmarkDefinitionsConfiguration,
			DumpCmarkDefinitionConfiguration,
			DumpCmarkValueKindsConfiguration,
			DumpCmarkValueKindConfiguration,
			DumpCmarkAppendicesConfiguration,
			DumpCmarkAppendixConfiguration,
			DumpCmarkHierarchyConfiguration,
			DumpCmarkLinkedCategoriesConfiguration,
			DumpCmarkLinkedExportsConfiguration,
			DumpCmarkLinkedDefinitionsConfiguration,
			DumpCmarkLinkedValueKindsConfiguration,
			DumpCmarkGenericConfiguration,
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
		["dump-html-cpio"] =>
			dump_html_cpio,
		["dump-cmark"] =>
			dump_cmark,
		["dump-cmark-cpio"] =>
			dump_cmark_cpio,
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




const BUFFER_SIZE_LARGE : usize = 8 * 1024 * 1024;
const BUFFER_SIZE_SMALL : usize = 128 * 1024;




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	let mut buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
	
	try! (dump_json_0 (libraries, &mut buffer));
	
	try_or_fail! (stream.write_all (&buffer), 0xa1639c14);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json_0 (libraries : &Libraries, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	
	let libraries_json = json::Map::from_iter (vec_map! (libraries.libraries (), library, (library.identifier_clone (), dump_json_library (library))));
	
	try_or_fail! (json::to_writer_pretty (stream, &libraries_json), 0x200f6e78);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_library (library : &Library) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			"features" : if let Some (features) = library.features () { dump_json_features (features) } else { json::Value::Null },
			"examples" : if let Some (examples) = library.examples () { dump_json_examples (examples) } else { json::Value::Null },
			
			"categories" : json::Map::from_iter (vec_map! (library.categories (), category, (category.identifier_clone (), dump_json_category (category)))),
			"categories_public" : json::Map::from_iter (vec_map! (library.categories_public (), (alias, entity),
					(StdString::from (alias), dump_json_identifier_perhaps_for_library_entity (Some (entity))))),
			
			"exports" : json::Map::from_iter (vec_map! (library.exports (), export, (export.identifier_clone (), dump_json_export (export)))),
			
			"definitions" : json::Map::from_iter (vec_map! (library.definitions (), definition, (definition.identifier_clone (), dump_json_definition (definition)))),
			"definitions_public" : json::Map::from_iter (vec_map! (library.definitions_public (), (alias, entity),
					(StdString::from (alias), dump_json_identifier_perhaps_for_library_entity (Some (entity))))),
			
			"types" : json::Map::from_iter (vec_map! (library.value_kinds (), value_kind, (value_kind.identifier_clone (), dump_json_value_kind (value_kind)))),
			"types_public" : json::Map::from_iter (vec_map! (library.value_kinds_public (), (alias, entity),
					(StdString::from (alias), dump_json_identifier_perhaps_for_library_entity (Some (entity))))),
			
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
			
			"super_categories" : dump_json_identifiers_perhaps_for_library_entities (category.parents ()),
			"super_categories_recursive" : dump_json_identifiers_perhaps_for_library_entities (category.parents_recursive ()),
			"sub_categories" : dump_json_identifiers_perhaps_for_library_entities (category.children ()),
			"sub_categories_recursive" : dump_json_identifiers_perhaps_for_library_entities (category.children_recursive ()),
			
			"exports" : dump_json_identifiers_perhaps_for_library_entities (category.exports ()),
			"exports_recursive" : dump_json_identifiers_perhaps_for_library_entities (category.exports_recursive ()),
			
			"types" : dump_json_identifiers_perhaps_for_library_entities (category.value_kinds ()),
			"types_recursive" : dump_json_identifiers_perhaps_for_library_entities (category.value_kinds_recursive ()),
			
			"definitions" : dump_json_identifiers_perhaps_for_library_entities (category.definitions ()),
			"definitions_recursive" : dump_json_identifiers_perhaps_for_library_entities (category.definitions_recursive ()),
			
			"description" : if let Some (description) = category.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = category.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_export (export : &Export) -> (json::Value) {
	json! ({
			
			"identifier" : export.identifier_clone (),
			"features" : if let Some (features) = export.features () { dump_json_features (features) } else { json::Value::Null },
			
			"super_exports" : dump_json_identifiers_perhaps_for_library_entities (export.parents ()),
			"super_exports_recursive" : dump_json_identifiers_perhaps_for_library_entities (export.parents_recursive ()),
			"sub_exports" : dump_json_identifiers_perhaps_for_library_entities (export.children ()),
			"sub_exports_recursive" : dump_json_identifiers_perhaps_for_library_entities (export.children_recursive ()),
			
			"categories" : dump_json_identifiers_perhaps_for_library_entities (export.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_library_entities (export.categories_recursive ()),
			
			"description" : if let Some (description) = export.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = export.links () { dump_json_links (links) } else { json::Value::Null },
			
			"definitions" : dump_json_identifiers_perhaps_for_library_entities (export.definitions ()),
			"definitions_recursive" : dump_json_identifiers_perhaps_for_library_entities (export.definitions_recursive ()),
			
			"descriptor" : dump_json_value (&export.descriptor_format ()),
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_value_kind (value_kind : &ValueKind) -> (json::Value) {
	json! ({
			
			"identifier" : value_kind.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (value_kind.aliases ()),
			"features" : if let Some (features) = value_kind.features () { dump_json_features (features) } else { json::Value::Null },
			"examples" : if let Some (examples) = value_kind.examples () { dump_json_examples (examples) } else { json::Value::Null },
			
			"super_types" : dump_json_identifiers_perhaps_for_library_entities (value_kind.parents ()),
			"super_types_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.parents_recursive ()),
			"sub_types" : dump_json_identifiers_perhaps_for_library_entities (value_kind.children ()),
			"sub_types_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.children_recursive ()),
			
			"categories" : dump_json_identifiers_perhaps_for_library_entities (value_kind.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.categories_recursive ()),
			
			// FIXME:  Conditional compilation does not work in this position!
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "covariant_types" : dump_json_identifiers_perhaps_for_library_entities (value_kind.covariants ()),
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "covariant_types_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.covariants_recursive ()),
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "contravariant_types" : dump_json_identifiers_perhaps_for_library_entities (value_kind.contravariants ()),
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "contravariant_types_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.contravariants_recursive ()),
			
			"definitions_input" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_input ()),
			"definitions_input_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_input_recursive ()),
			// FIXME:  Conditional compilation does not work in this position!
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "definitions_input_contravariant" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_input_contravariant_recursive ()),
			"definitions_output" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_output ()),
			"definitions_output_recursive" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_output_recursive ()),
			// FIXME:  Conditional compilation does not work in this position!
			// #[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			// "definitions_output_covariant" : dump_json_identifiers_perhaps_for_library_entities (value_kind.definitions_output_covariant_recursive ()),
			
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
			"examples" : if let Some (examples) = definition.examples () { dump_json_examples (examples) } else { json::Value::Null },
			
			"categories" : dump_json_identifiers_perhaps_for_library_entities (definition.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_library_entities (definition.categories_recursive ()),
			
			"exports" : dump_json_identifiers_perhaps_for_library_entities (definition.exports ()),
			"exports_recursive" : dump_json_identifiers_perhaps_for_library_entities (definition.exports_recursive ()),
			
			"kind" : json::Value::String (StdString::from (definition.kind () .identifier ())),
			
			"description" : if let Some (description) = definition.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = definition.links () { dump_json_links (links) } else { json::Value::Null },
			
			"procedure_signature" : if let Some (procedure_signature) = definition.procedure_signature () { dump_json_procedure_signature (procedure_signature) } else { json::Value::Null },
			"syntax_signature" : if let Some (syntax_signature) = definition.syntax_signature () { dump_json_syntax_signature (syntax_signature) } else { json::Value::Null },
			
			"referenced_types" : dump_json_identifiers_perhaps_for_library_entities (definition.referenced_value_kinds ()),
			
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
					"value" : dump_json_value (value),
				}),
		SyntaxSignatureKeyword::Value { kind, .. } =>
			json! ({
					"kind" : "value",
					"identifier" : identifier,
					"type" : dump_json_identifier_perhaps_for_library_entity (kind.as_ref () .map (ops::Deref::deref)),
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
	return dump_json_value (&features.format ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_examples (examples : &Examples) -> (json::Value) {
	let examples = examples.examples ();
	if ! examples.is_empty () {
		return json::Value::Array (vec_map! (examples, example, dump_json_example (example)));
	} else {
		return json::Value::Null;
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_example (example : &Example) -> (json::Value) {
	if ! example.sequence.is_empty () {
		return json::Value::Array (vec_map! (example.sequence.iter (), sequence, dump_json_example_sequence (sequence)));
	} else {
		return json::Value::Null;
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_example_sequence (sequence : &ExampleSequence) -> (json::Value) {
	match *sequence {
		ExampleSequence::CodeText (ref text) =>
			json! ({
				"type" : "code",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::CodeExpression (ref value) =>
			json! ({
				"type" : "code",
				"value" : dump_json_value (value),
			}),
		ExampleSequence::ReturnText (ref text) =>
			json! ({
				"type" : "return",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::ReturnValue (ref value) =>
			json! ({
				"type" : "return",
				"value" : dump_json_value (value),
			}),
		ExampleSequence::ErrorText (ref text) =>
			json! ({
				"type" : "error",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::ErrorValue (ref value) =>
			json! ({
				"type" : "error",
				"value" : dump_json_value (value),
			}),
		ExampleSequence::StdinLineText (ref text) =>
			json! ({
				"type" : "stdin_line",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::StdinLineValue (ref value) =>
			json! ({
				"type" : "stdin_line",
				"value" : dump_json_value (value),
			}),
		ExampleSequence::StdoutLineText (ref text) =>
			json! ({
				"type" : "stdout_line",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::StdoutLineValue (ref value) =>
			json! ({
				"type" : "stdout_line",
				"value" : dump_json_value (value),
			}),
		ExampleSequence::StderrLineText (ref text) =>
			json! ({
				"type" : "stderr_line",
				"text" : StdString::from (text.deref () .deref ()),
			}),
		ExampleSequence::StderrLineValue (ref value) =>
			json! ({
				"type" : "stderr_line",
				"value" : dump_json_value (value),
			}),
	}
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
fn dump_json_identifier_perhaps_for_library_entity (entity : Option<&impl LibraryEntity>) -> (json::Value) {
	if let Some (entity) = entity {
		return json! ({
			"library" : json::Value::String (entity.library () .identifier_clone ()),
			"identifier" : json::Value::String (entity.identifier_clone ()),
		});
	} else {
		return json::Value::Null;
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
fn dump_json_identifiers_perhaps_for_library_entities <'a, E : LibraryEntity + 'a> (entities : impl iter::Iterator<Item = &'a E>) -> (json::Value) {
	let identifiers = vec_map! (entities, entity, dump_json_identifier_perhaps_for_library_entity (Some (entity)));
	if identifiers.is_empty () {
		return json::Value::Null;
	} else {
		return json::Value::Array (identifiers);
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
		_ => {
			FIXME! ("better handle `#null` case");
			let buffer = format! ("{}", value);
			let buffer = buffer.replace ("#null", "()");
			json::Value::String (buffer)
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (true, true));
	let stream_buffer = {
		let mut stream_buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
		try! (dump_html_header_write ("Scheme Libraries", &mut stream_buffer));
		let mut callbacks = DumpCmarkCallbacksSingleFile {
				buffer : stream_buffer,
			};
		try! (dump_html_0 (libraries, &configuration, &mut callbacks));
		let mut stream_buffer = callbacks.buffer;
		try! (dump_html_trailer_write (&mut stream_buffer));
		stream_buffer
	};
	try_or_fail! (stream.write_all (&stream_buffer), 0x4aed615a);
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (true, false));
	let stream_buffer = {
		let mut stream_buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
		let mut callbacks = DumpCmarkCallbacksSingleFile {
				buffer : stream_buffer,
			};
		try! (dump_cmark_0 (libraries, &configuration, &mut callbacks));
		callbacks.buffer
	};
	try_or_fail! (stream.write_all (&stream_buffer), 0x27d8ded6);
	succeed! (());
}


struct DumpCmarkCallbacksSingleFile {
	buffer : StdVec<u8>,
}

impl DumpCmarkCallbacks for DumpCmarkCallbacksSingleFile {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return StdVec::with_capacity (BUFFER_SIZE_SMALL);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write_0 (&mut self, _anchor_self : DumpCmarkAnchor, buffer : StdVec<u8>) -> (Outcome<()>) {
		try_or_fail! (self.buffer.write_all (&buffer), 0x67f5c369);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate_0 (&mut self, anchor_target : DumpCmarkAnchor, _anchor_self : DumpCmarkAnchor) -> (Outcome<StdString>) {
		let mut anchor_full = StdString::new ();
		let anchor_hash = try! (dump_cmark_anchor_generate (anchor_target));
		anchor_full.push ('#');
		anchor_full.push_str (&anchor_hash);
		succeed! (anchor_full);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write_0 (&mut self, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_anchor_write (anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write_0 (&mut self, title : Option<&str>, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_title_write (title, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn header_write_0 (&mut self, header_depth : usize, header_caption : &str, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_header_write (header_depth, header_caption, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write_0 (&mut self, anchor_self : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_break_write (anchor_self, configuration, self, buffer);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_cpio (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (false, true));
	let mut writer = try! (DumpCpioWriter::open (stream));
	try! (dump_html_cpio_0 (libraries, &configuration, &mut writer));
	try! (writer.close ());
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_cpio (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (false, false));
	let mut writer = try! (DumpCpioWriter::open (stream));
	try! (dump_cmark_cpio_0 (libraries, &configuration, &mut writer));
	try! (writer.close ());
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_cpio_0 <'a, Writer : io::Write + 'a> (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, writer : &'a mut DumpCpioWriter<Writer>) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksCpioFile {
			writer : writer,
			path_prefix : "./",
			path_suffix : ".html",
		};
	return dump_html_0 (libraries, configuration, &mut callbacks);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_cpio_0 <'a, Writer : io::Write + 'a> (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, writer : &'a mut DumpCpioWriter<Writer>) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksCpioFile {
			writer : writer,
			path_prefix : "./",
			path_suffix : ".md",
		};
	return dump_cmark_0 (libraries, configuration, &mut callbacks);
}


struct DumpCmarkCallbacksCpioFile <'a, Writer : io::Write + 'a> {
	writer : &'a mut DumpCpioWriter<Writer>,
	path_prefix : &'a str,
	path_suffix : &'a str,
}

impl <'a, Writer : io::Write> DumpCmarkCallbacks for DumpCmarkCallbacksCpioFile<'a, Writer> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return StdVec::with_capacity (BUFFER_SIZE_SMALL);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write_0 (&mut self, anchor_self : DumpCmarkAnchor, buffer : StdVec<u8>) -> (Outcome<()>) {
		let path = try! (dump_cmark_path_generate (anchor_self, self.path_prefix, self.path_suffix));
		let path = fs_path::PathBuf::from (path);
		try! (self.writer.write (&path, &buffer));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate_0 (&mut self, anchor_target : DumpCmarkAnchor, anchor_self : DumpCmarkAnchor) -> (Outcome<StdString>) {
		let mut anchor_full = StdString::new ();
		if anchor_self.section () .is_some () {
			fail! (0xcb91adb8);
		}
		let anchor_path_prefix = match anchor_self {
			DumpCmarkAnchor::LibrariesToc (_) =>
				"./",
			DumpCmarkAnchor::LibraryToc (_, entity, _) =>
				match entity {
					LibraryEntityKind::Library =>
						fail! (0x7b7b8dbd),
					LibraryEntityKind::Category |
					LibraryEntityKind::Export |
					LibraryEntityKind::Definition |
					LibraryEntityKind::ValueKind |
					LibraryEntityKind::Appendix =>
						"../../",
				}
			DumpCmarkAnchor::Entity (entity, _) =>
				match entity.kind () {
					LibraryEntityKind::Library =>
						"../",
					LibraryEntityKind::Category |
					LibraryEntityKind::Export |
					LibraryEntityKind::Definition |
					LibraryEntityKind::ValueKind |
					LibraryEntityKind::Appendix =>
						"../../",
				}
		};
		let anchor_path = try! (dump_cmark_path_generate (anchor_target, anchor_path_prefix, self.path_suffix));
		let anchor_hash = try! (dump_cmark_anchor_generate (anchor_target));
		anchor_full.push_str (&anchor_path);
		anchor_full.push ('#');
		anchor_full.push_str (&anchor_hash);
		succeed! (anchor_full);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write_0 (&mut self, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_anchor_write (anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write_0 (&mut self, title : Option<&str>, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_title_write (title, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn header_write_0 (&mut self, header_depth : usize, header_caption : &str, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_header_write (header_depth, header_caption, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write_0 (&mut self, anchor_self : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_break_write (anchor_self, configuration, self, buffer);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_0 (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksToHtml {
			callbacks : callbacks,
			embedded : configuration.generic.embedded,
		};
	return dump_cmark_execute (libraries, configuration, &mut callbacks);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_0 (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	return dump_cmark_execute (libraries, configuration, callbacks);
}


struct DumpCmarkCallbacksToHtml <'a, Callbacks : DumpCmarkCallbacks + 'a> {
	callbacks : &'a mut Callbacks,
	embedded : bool,
}

impl <'a, Callbacks : DumpCmarkCallbacks + 'a> DumpCmarkCallbacks for DumpCmarkCallbacksToHtml<'a, Callbacks> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return self.callbacks.buffer_build ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write_0 (&mut self, anchor_self : DumpCmarkAnchor, cmark_buffer : StdVec<u8>) -> (Outcome<()>) {
		
		let cmark_buffer = try_or_fail! (StdString::from_utf8 (cmark_buffer), 0xb06a2a9f);
		
		let mut html_buffer = self.callbacks.buffer_build ();
		
		if !self.embedded {
			let title = try! (dump_cmark_title_generate (None, anchor_self));
			try! (dump_html_header_write (&title, &mut html_buffer));
		}
		
		let mut html_buffer = try_or_fail! (StdString::from_utf8 (html_buffer), 0x20a8754d);
		
		let parser = ext::pulldown_cmark::Parser::new (&cmark_buffer);
		ext::pulldown_cmark::html::push_html (&mut html_buffer, parser);
		
		let mut html_buffer = StdVec::from (html_buffer);
		
		if !self.embedded {
			try! (dump_html_trailer_write (&mut html_buffer));
		}
		
		return self.callbacks.buffer_write_0 (anchor_self, html_buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate_0 (&mut self, anchor_target : DumpCmarkAnchor, anchor_self : DumpCmarkAnchor) -> (Outcome<StdString>) {
		return self.callbacks.anchor_generate_0 (anchor_target, anchor_self);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write_0 (&mut self, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.anchor_write_0 (anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write_0 (&mut self, title : Option<&str>, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.title_write_0 (title, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn header_write_0 (&mut self, header_depth : usize, header_caption : &str, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.header_write_0 (header_depth, header_caption, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write_0 (&mut self, anchor_self : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.break_write_0 (anchor_self, configuration, buffer);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_html_header_write (title : &str, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let title = title.replace ("<", "&lt;");
	let title = title.replace (">", "&gt;");
	let title = title.replace ("&", "&amp;");
	let title = title.replace ("\"", "&quot;");
	let prefix = DUMP_HTML_PREFIX.replace ("@{title}", &title);
	try_or_fail! (stream.write_all (prefix.as_bytes ()), 0xd730a725);
	if true {
		try_or_fail! (stream.write_all (b"<style type='text/css'>\n"), 0x64138904);
		try_or_fail! (stream.write_all (DUMP_HTML_CSS.as_bytes ()), 0xe8153313);
		try_or_fail! (stream.write_all (b"</style>\n"), 0x108d0302);
	}
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_html_trailer_write (stream : &mut StdVec<u8>) -> (Outcome<()>) {
	if true {
		try_or_fail! (stream.write_all (b"<script type='text/javascript' src='https://code.jquery.com/jquery-3.3.1.slim.min.js'></script>\n"), 0xa38f09a1);
		try_or_fail! (stream.write_all (b"<script type='text/javascript'>\n"), 0xec1427db);
		try_or_fail! (stream.write_all (DUMP_HTML_JS.as_bytes ()), 0x577b234f);
		try_or_fail! (stream.write_all (b"</script>\n"), 0x0372c56d);
	}
	try_or_fail! (stream.write_all (DUMP_HTML_SUFFIX.as_bytes ()), 0x17a2e8ae);
	succeed! (());
}




#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLibrariesConfiguration {
	pub toc : bool,
	pub toc_compact : bool,
	pub enabled : bool,
	pub configuration : DumpCmarkLibraryConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLibraryConfiguration {
	pub toc : bool,
	pub toc_compact : bool,
	pub features : bool,
	pub examples : bool,
	pub description : bool,
	pub links : bool,
	pub categories : DumpCmarkCategoriesConfiguration,
	pub exports : DumpCmarkExportsConfiguration,
	pub definitions : DumpCmarkDefinitionsConfiguration,
	pub value_kinds : DumpCmarkValueKindsConfiguration,
	pub appendices : DumpCmarkAppendicesConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkCategoriesConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkCategoryConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkCategoryConfiguration {
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	pub exports : DumpCmarkLinkedExportsConfiguration,
	pub definitions : DumpCmarkLinkedDefinitionsConfiguration,
	pub value_kinds : DumpCmarkLinkedValueKindsConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkExportsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkExportConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkExportConfiguration {
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	pub definitions : DumpCmarkLinkedDefinitionsConfiguration,
	pub descriptor : bool,
	pub features : bool,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkDefinitionsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub configuration : DumpCmarkDefinitionConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkDefinitionConfiguration {
	pub kind : bool,
	pub exports : DumpCmarkLinkedExportsConfiguration,
	pub signature : bool,
	pub aliases : bool,
	pub aliases_compact : bool,
	pub features : bool,
	pub examples : bool,
	pub value_kinds : DumpCmarkLinkedValueKindsConfiguration,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkValueKindsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkValueKindConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkValueKindConfiguration {
	pub tree : bool,
	pub tree_complete : bool,
	pub tree_depth : usize,
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub covariants : DumpCmarkLinkedValueKindsConfiguration,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub contravariants : DumpCmarkLinkedValueKindsConfiguration,
	pub definitions_all : DumpCmarkLinkedDefinitionsConfiguration,
	pub definitions_all_variant : bool,
	pub definitions_all_variant_complete : bool,
	pub definitions_all_variant_compact : bool,
	pub definitions_input : DumpCmarkLinkedDefinitionsConfiguration,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_input_contravariant : bool,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_input_contravariant_complete : bool,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_input_contravariant_compact : bool,
	pub definitions_output : DumpCmarkLinkedDefinitionsConfiguration,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_output_covariant : bool,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_output_covariant_complete : bool,
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	pub definitions_output_covariant_compact : bool,
	pub predicate : bool,
	pub aliases : bool,
	pub aliases_compact : bool,
	pub features : bool,
	pub examples : bool,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkAppendicesConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub configuration : DumpCmarkAppendixConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkAppendixConfiguration {
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}




#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkGenericConfiguration {
	pub notes : bool,
	pub fixme : bool,
	pub lints : bool,
	pub navigator : bool,
	pub navigator_categories : bool,
	pub navigator_exports : bool,
	pub navigator_definitions : bool,
	pub navigator_value_kinds : bool,
	pub navigator_appendices : bool,
	pub navigator_library : bool,
	pub navigator_libraries : bool,
	pub anchors : bool,
	pub embedded : bool,
	pub html : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkHierarchyConfiguration {
	pub super_direct : bool,
	pub super_direct_complete : bool,
	pub super_direct_compact : bool,
	pub super_recursive : bool,
	pub super_recursive_complete : bool,
	pub super_recursive_compact : bool,
	pub sub_direct : bool,
	pub sub_direct_complete : bool,
	pub sub_direct_compact : bool,
	pub sub_recursive : bool,
	pub sub_recursive_complete : bool,
	pub sub_recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedCategoriesConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedExportsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedDefinitionsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedValueKindsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_configure (embedded : bool, html : bool) -> (Outcome<DumpCmarkLibrariesConfiguration>) {
	
	
	const ALL : bool = false;
	const DEBUG : bool = false;
	
	const NO_SUPER : bool = false;
	const NO_SUB : bool = false;
	const NO_TREE : bool = false;
	const NO_LIBRARIES : bool = false;
	const NO_CATEGORIES : bool = false;
	const NO_EXPORTS : bool = false;
	const NO_VALUE_KINDS : bool = false;
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const NO_VARIANCES : bool = true;
	const NO_DEFINITIONS : bool = false;
	const NO_APPENDICES : bool = false;
	const NO_RECURSIVE : bool = false;
	const NO_DETAILS : bool = false;
	const NO_TOC : bool = false;
	const NO_NOTES : bool = false;
	const NO_FIXME : bool = true;
	
	const COMPLETE : bool = false;
	const COMPACT : bool = false;
	
	const TOC_COMPLETE : bool = true;
	const TOC_DEPTH : usize = 2;
	const TREE_COMPLETE : bool = true;
	const TREE_DEPTH : usize = 2;
	
	const NAVIGATOR : bool = true;
	const ANCHORS : bool = true;
	
	
	const LIBRARIES : bool = ALL || !NO_LIBRARIES;
	const LIBRARIES_TOC : bool = ALL || !NO_TOC;
	
	const CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const CATEGORIES_TOC : bool = ALL || !NO_TOC;
	const CATEGORIES_SUPER : bool = ALL || !NO_SUPER;
	const CATEGORIES_SUPER_RECURSIVE : bool = CATEGORIES_SUPER && (ALL || !NO_RECURSIVE);
	const CATEGORIES_SUB : bool = ALL || !NO_SUB;
	const CATEGORIES_SUB_RECURSIVE : bool = CATEGORIES_SUB && (ALL || !NO_RECURSIVE);
	const CATEGORIES_EXPORTS : bool = ALL || !NO_EXPORTS;
	const CATEGORIES_EXPORTS_RECURSIVE : bool = CATEGORIES_EXPORTS && (ALL || !NO_RECURSIVE);
	const CATEGORIES_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const CATEGORIES_DEFINITIONS_RECURSIVE : bool = CATEGORIES_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const CATEGORIES_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const CATEGORIES_VALUE_KINDS_RECURSIVE : bool = CATEGORIES_VALUE_KINDS && (ALL || !NO_RECURSIVE);
	
	const EXPORTS : bool = ALL || !NO_EXPORTS;
	const EXPORTS_TOC : bool = ALL || !NO_TOC;
	const EXPORTS_DESCRIPTOR : bool = ALL || !NO_DETAILS;
	const EXPORTS_SUPER : bool = ALL || !NO_SUPER;
	const EXPORTS_SUPER_RECURSIVE : bool = EXPORTS_SUPER && (ALL || !NO_RECURSIVE);
	const EXPORTS_SUB : bool = ALL || !NO_SUB;
	const EXPORTS_SUB_RECURSIVE : bool = EXPORTS_SUB && (ALL || !NO_RECURSIVE);
	const EXPORTS_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const EXPORTS_DEFINITIONS_RECURSIVE : bool = EXPORTS_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const EXPORTS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const EXPORTS_CATEGORIES_RECURSIVE : bool = EXPORTS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const DEFINITIONS_TOC : bool = ALL || !NO_TOC;
	const DEFINITIONS_KIND : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_EXPORTS : bool = ALL || !NO_EXPORTS;
	const DEFINITIONS_EXPORTS_RECURSIVE : bool = DEFINITIONS_EXPORTS && (ALL || !NO_RECURSIVE);
	const DEFINITIONS_SIGNATURE : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const DEFINITIONS_VALUE_KINDS_RECURSIVE : bool = DEFINITIONS_SIGNATURE && (ALL || !NO_RECURSIVE);
	const DEFINITIONS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const DEFINITIONS_CATEGORIES_RECURSIVE : bool = DEFINITIONS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const VALUE_KINDS_TOC : bool = ALL || !NO_TOC;
	const VALUE_KINDS_TREE : bool = ALL || !NO_TREE;
	const VALUE_KINDS_SUPER : bool = ALL || !NO_SUPER;
	const VALUE_KINDS_SUPER_RECURSIVE : bool = VALUE_KINDS_SUPER && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_SUB : bool = ALL || !NO_SUB;
	const VALUE_KINDS_SUB_RECURSIVE : bool = VALUE_KINDS_SUB && (ALL || !NO_RECURSIVE);
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_VARIANCES : bool = ALL || !NO_VARIANCES;
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_VARIANCES_RECURSIVE : bool = VALUE_KINDS_VARIANCES && (ALL || !NO_RECURSIVE);
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_COVARIANTS : bool = VALUE_KINDS_VARIANCES;
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_COVARIANTS_RECURSIVE : bool = VALUE_KINDS_COVARIANTS && VALUE_KINDS_VARIANCES_RECURSIVE;
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_CONTRAVARIANTS : bool = VALUE_KINDS_VARIANCES;
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_CONTRAVARIANTS_RECURSIVE : bool = VALUE_KINDS_CONTRAVARIANTS && VALUE_KINDS_VARIANCES_RECURSIVE;
	const VALUE_KINDS_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_INPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_INPUT && (ALL || !NO_RECURSIVE);
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT : bool = VALUE_KINDS_DEFINITIONS_INPUT && VALUE_KINDS_VARIANCES;
	const VALUE_KINDS_DEFINITIONS_OUTPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && (ALL || !NO_RECURSIVE);
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	const VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && VALUE_KINDS_VARIANCES;
	const VALUE_KINDS_PREDICATE : bool = ALL || !NO_DETAILS;
	const VALUE_KINDS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const VALUE_KINDS_CATEGORIES_RECURSIVE : bool = VALUE_KINDS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const APPENDICES : bool = ALL || !NO_APPENDICES;
	const APPENDICES_TOC : bool = ALL || !NO_TOC;
	
	const ALIASES : bool = ALL || !NO_DETAILS;
	const FEATURES : bool = ALL || !NO_DETAILS;
	const DESCRIPTIONS : bool = ALL || !NO_DETAILS;
	const LINKS : bool = ALL || !NO_DETAILS;
	const EXAMPLES : bool = ALL || !NO_DETAILS;
	
	const NOTES : bool = ALL || !NO_NOTES;
	const FIXME : bool = ALL || !NO_FIXME;
	const LINTS : bool = DEBUG && !NO_FIXME;
	
	
	let generic = DumpCmarkGenericConfiguration {
			notes : NOTES,
			fixme : FIXME,
			lints : LINTS,
			navigator : NAVIGATOR,
			navigator_categories : CATEGORIES,
			navigator_exports : EXPORTS,
			navigator_definitions : DEFINITIONS,
			navigator_value_kinds : VALUE_KINDS,
			navigator_appendices : APPENDICES,
			navigator_library : LIBRARIES,
			navigator_libraries : LIBRARIES,
			anchors : ANCHORS,
			embedded : embedded,
			html : html,
		};
	
	
	let category = DumpCmarkCategoryConfiguration {
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : CATEGORIES_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : CATEGORIES_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : CATEGORIES_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : CATEGORIES_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			exports : DumpCmarkLinkedExportsConfiguration {
					direct : CATEGORIES_EXPORTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_EXPORTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions : DumpCmarkLinkedDefinitionsConfiguration {
					direct : CATEGORIES_DEFINITIONS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_DEFINITIONS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			value_kinds : DumpCmarkLinkedValueKindsConfiguration {
					direct : CATEGORIES_VALUE_KINDS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_VALUE_KINDS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let categories = DumpCmarkCategoriesConfiguration {
			enabled : CATEGORIES,
			toc : CATEGORIES_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : category,
			generic : generic,
		};
	
	
	let export = DumpCmarkExportConfiguration {
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : EXPORTS_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : EXPORTS_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : EXPORTS_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : EXPORTS_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			definitions : DumpCmarkLinkedDefinitionsConfiguration {
					direct : EXPORTS_DEFINITIONS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : EXPORTS_DEFINITIONS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			descriptor : EXPORTS_DESCRIPTOR,
			features : FEATURES,
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : EXPORTS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : EXPORTS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let exports = DumpCmarkExportsConfiguration {
			enabled : EXPORTS,
			toc : EXPORTS_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : export,
			generic : generic,
		};
	
	
	let definition = DumpCmarkDefinitionConfiguration {
			kind : DEFINITIONS_KIND,
			exports : DumpCmarkLinkedExportsConfiguration {
					direct : DEFINITIONS_EXPORTS,
					direct_compact : COMPACT,
					direct_complete : COMPLETE,
					recursive : DEFINITIONS_EXPORTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			signature : DEFINITIONS_SIGNATURE,
			aliases : ALIASES,
			aliases_compact : COMPACT,
			features : FEATURES,
			examples : EXAMPLES,
			value_kinds : DumpCmarkLinkedValueKindsConfiguration {
					direct : DEFINITIONS_VALUE_KINDS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : DEFINITIONS_VALUE_KINDS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : DEFINITIONS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : DEFINITIONS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let definitions = DumpCmarkDefinitionsConfiguration {
			enabled : DEFINITIONS,
			toc : DEFINITIONS_TOC,
			configuration : definition,
			generic : generic,
		};
	
	
	let value_kind = DumpCmarkValueKindConfiguration {
			tree : VALUE_KINDS_TREE,
			tree_complete : TREE_COMPLETE,
			tree_depth : TREE_DEPTH,
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : VALUE_KINDS_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : VALUE_KINDS_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : VALUE_KINDS_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : VALUE_KINDS_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			covariants : DumpCmarkLinkedValueKindsConfiguration {
					direct : VALUE_KINDS_COVARIANTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_COVARIANTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			contravariants : DumpCmarkLinkedValueKindsConfiguration {
					direct : VALUE_KINDS_CONTRAVARIANTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_CONTRAVARIANTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_all : DumpCmarkLinkedDefinitionsConfiguration {
					direct : false,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : false,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_all_variant : false,
			definitions_all_variant_complete : COMPLETE,
			definitions_all_variant_compact : COMPACT,
			definitions_input : DumpCmarkLinkedDefinitionsConfiguration {
					direct : VALUE_KINDS_DEFINITIONS_INPUT,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_input_contravariant : VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT,
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_input_contravariant_complete : COMPLETE,
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_input_contravariant_compact : COMPACT,
			definitions_output : DumpCmarkLinkedDefinitionsConfiguration {
					direct : VALUE_KINDS_DEFINITIONS_OUTPUT,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_output_covariant : VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT,
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_output_covariant_complete : COMPLETE,
			#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
			definitions_output_covariant_compact : COMPACT,
			predicate : VALUE_KINDS_PREDICATE,
			aliases : ALIASES,
			aliases_compact : COMPACT,
			features : FEATURES,
			examples : EXAMPLES,
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : VALUE_KINDS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let value_kinds = DumpCmarkValueKindsConfiguration {
			enabled : VALUE_KINDS,
			toc : VALUE_KINDS_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : value_kind,
			generic : generic,
		};
	
	
	let appendix = DumpCmarkAppendixConfiguration {
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let appendices = DumpCmarkAppendicesConfiguration {
			enabled : APPENDICES,
			toc : APPENDICES_TOC,
			configuration : appendix,
			generic : generic,
		};
	
	
	let library = DumpCmarkLibraryConfiguration {
			toc : LIBRARIES_TOC,
			toc_compact : false,
			features : FEATURES,
			examples : EXAMPLES,
			description : DESCRIPTIONS,
			links : LINKS,
			categories : categories,
			exports : exports,
			definitions : definitions,
			value_kinds : value_kinds,
			appendices : appendices,
			generic : generic,
		};
	
	let libraries = DumpCmarkLibrariesConfiguration {
			toc : LIBRARIES_TOC,
			toc_compact : false,
			enabled : LIBRARIES,
			configuration : library,
			generic : generic,
		};
	
	
	succeed! (libraries);
}




pub trait DumpCmarkCallbacks {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write <'a> (&mut self, anchor_self : impl DumpCmarkAnchorInto<'a>, buffer : StdVec<u8>) -> (Outcome<()>) {
		let anchor_self = anchor_self.anchor ();
		return self.buffer_write_0 (anchor_self, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate <'a> (&mut self, anchor_target : impl DumpCmarkAnchorInto<'a>, anchor_self : impl DumpCmarkAnchorInto<'a>) -> (Outcome<StdString>) {
		let anchor_target = anchor_target.anchor ();
		let anchor_self = anchor_self.anchor ();
		return self.anchor_generate_0 (anchor_target, anchor_self);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write <'a> (&mut self, anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		let anchor = anchor.anchor ();
		return self.anchor_write_0 (anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write <'a> (&mut self, title : Option<&str>, anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		let anchor = anchor.anchor ();
		return self.title_write_0 (title, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn header_write <'a> (&mut self, header_depth : usize, header_caption : &str, anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		let anchor = anchor.anchor ();
		return self.header_write_0 (header_depth, header_caption, anchor, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write <'a> (&mut self, anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		let anchor_self = anchor_self.anchor ();
		return self.break_write_0 (anchor_self, configuration, buffer);
	}
	
	fn buffer_build (&mut self) -> (StdVec<u8>);
	fn buffer_write_0 (&mut self, anchor_self : DumpCmarkAnchor, buffer : StdVec<u8>) -> (Outcome<()>);
	fn anchor_generate_0 (&mut self, anchor_target : DumpCmarkAnchor, anchor_self : DumpCmarkAnchor) -> (Outcome<StdString>);
	fn anchor_write_0 (&mut self, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
	fn title_write_0 (&mut self, title : Option<&str>, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
	fn header_write_0 (&mut self, header_depth : usize, header_caption : &str, anchor : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
	fn break_write_0 (&mut self, anchor_self : DumpCmarkAnchor, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
}




#[ derive ( Copy, Clone ) ] // OK
pub enum DumpCmarkAnchor <'a> {
	LibrariesToc (Option<&'a str>),
	LibraryToc (&'a Library, LibraryEntityKind, Option<&'a str>),
	Entity (&'a dyn LibraryEntity, Option<&'a str>),
}


impl <'a> DumpCmarkAnchor<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn library (self) -> (Option<&'a Library>) {
		match self {
			DumpCmarkAnchor::LibrariesToc (_) =>
				return None,
			DumpCmarkAnchor::LibraryToc (library, _, _) =>
				return Some (library),
			DumpCmarkAnchor::Entity (entity, _) =>
				return Some (entity.library ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_kind (self) -> (LibraryEntityKind) {
		match self {
			DumpCmarkAnchor::LibrariesToc (_) =>
				return LibraryEntityKind::Library,
			DumpCmarkAnchor::LibraryToc (_, kind, _) =>
				return kind,
			DumpCmarkAnchor::Entity (entity, _) =>
				return entity.kind (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn section (self) -> (Option<&'a str>) {
		match self {
			DumpCmarkAnchor::LibrariesToc (section) =>
				return section,
			DumpCmarkAnchor::LibraryToc (_, _, section) =>
				return section,
			DumpCmarkAnchor::Entity (_, section) =>
				return section,
		}
	}
}


pub trait DumpCmarkAnchorInto<'a> {
	
	fn anchor (self) -> (DumpCmarkAnchor<'a>);
}

impl <'a> DumpCmarkAnchorInto<'a> for DumpCmarkAnchor<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		return self;
	}
}

impl <'a> DumpCmarkAnchorInto<'a> for &'a DumpCmarkAnchor<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		return *self;
	}
}

impl <'a, AsStr : StdAsRef<str> + 'a + ?Sized> DumpCmarkAnchorInto<'a> for (DumpCmarkAnchor<'a>, &'a AsStr) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		let (anchor, section) = self;
		let section = section.as_ref ();
		match anchor {
			DumpCmarkAnchor::LibrariesToc (None) =>
				return DumpCmarkAnchor::LibrariesToc (Some (section)),
			DumpCmarkAnchor::LibrariesToc (Some (_anchor_section)) =>
				return DumpCmarkAnchor::LibrariesToc (Some (section)),
			DumpCmarkAnchor::LibraryToc (library, entity, None) =>
				return DumpCmarkAnchor::LibraryToc (library, entity, Some (section)),
			DumpCmarkAnchor::LibraryToc (library, entity, Some (_anchor_section)) =>
				return DumpCmarkAnchor::LibraryToc (library, entity, Some (section)),
			DumpCmarkAnchor::Entity (entity, None) =>
				return DumpCmarkAnchor::Entity (entity, Some (section)),
			DumpCmarkAnchor::Entity (entity, Some (_anchor_section)) =>
				return DumpCmarkAnchor::Entity (entity, Some (section)),
		}
	}
}

impl <'a, AsStr : StdAsRef<str> + 'a + ?Sized> DumpCmarkAnchorInto<'a> for (&'a DumpCmarkAnchor<'a>, &'a AsStr) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		let (anchor, section) = self;
		return (*anchor, section) .anchor ();
	}
}

impl <'a, E : LibraryEntity + 'a> DumpCmarkAnchorInto<'a> for &'a E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		return DumpCmarkAnchor::Entity (self, None);
	}
}

impl <'a, E : LibraryEntity + 'a, AsStr : StdAsRef<str> + 'a + ?Sized> DumpCmarkAnchorInto<'a> for (&'a E, &'a AsStr) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		let (entity, section) = self;
		let section = section.as_ref ();
		return DumpCmarkAnchor::Entity (entity, Some (section));
	}
}

impl <'a> DumpCmarkAnchorInto<'a> for (&'a Library, LibraryEntityKind) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		let (library, entity) = self;
		return DumpCmarkAnchor::LibraryToc (library, entity, None);
	}
}

impl <'a, AsStr : StdAsRef<str> + 'a + ?Sized> DumpCmarkAnchorInto<'a> for (&'a Library, LibraryEntityKind, &'a AsStr) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor (self) -> (DumpCmarkAnchor<'a>) {
		let (library, entity, section) = self;
		let section = section.as_ref ();
		return DumpCmarkAnchor::LibraryToc (library, entity, Some (section));
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_execute (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	try! (dump_cmark_libraries (libraries.libraries (), configuration, callbacks));
	
	for library in libraries.libraries () {
		
		if configuration.enabled {
			let configuration = &configuration.configuration;
			try! (dump_cmark_library (library, configuration, callbacks));
		}
		
		let configuration = &configuration.configuration;
		
		
		if configuration.categories.enabled && library.has_categories () {
			let configuration = &configuration.categories;
			try! (dump_cmark_categories (library, library.categories (), configuration, callbacks));
			
			for category in library.categories () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_category (category, configuration, callbacks));
			}
		}
		
		
		if configuration.exports.enabled && library.has_exports () {
			let configuration = &configuration.exports;
			try! (dump_cmark_exports (library, library.exports (), configuration, callbacks));
			
			for export in library.exports () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_export (export, configuration, callbacks));
			}
		}
		
		
		if configuration.definitions.enabled && library.has_definitions () {
			let configuration = &configuration.definitions;
			try! (dump_cmark_definitions (library, library.definitions (), configuration, callbacks));
			
			for definition in library.definitions () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_definition (definition, configuration, callbacks));
			}
		}
		
		
		if configuration.value_kinds.enabled && library.has_value_kinds () {
			let configuration = &configuration.value_kinds;
			try! (dump_cmark_value_kinds (library, library.value_kinds (), configuration, callbacks));
			
			for value_kind in library.value_kinds () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_value_kind (value_kind, configuration, callbacks));
			}
		}
		
		
		if configuration.appendices.enabled && library.has_appendices () {
			let configuration = &configuration.appendices;
			try! (dump_cmark_appendices (library, library.appendices (), configuration, callbacks));
			
			for appendix in library.appendices () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_appendix (appendix, configuration, callbacks));
			}
		}
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_libraries <'a> (libraries : impl iter::ExactSizeIterator<Item = &'a Library>, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibrariesToc (None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		for library in libraries {
			let library_anchor = try! (callbacks.anchor_generate (library, anchor_self));
			if let Some (title) = library.title () {
				try_writeln! (stream, "* [`{}`]({}) -- {};", library.identifier (), library_anchor, title);
			} else {
				try_writeln! (stream, "* [`{}`]({});", library.identifier (), library_anchor);
			}
		}
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_library (library : &Library, configuration : &DumpCmarkLibraryConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::Entity (library, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (library.title (), anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		let mut empty = true;
		if configuration.categories.enabled && library.has_categories () {
			let categories_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Category), anchor_self));
			if configuration.toc_compact {
				try_writeln! (stream, "[categories]({});", &categories_anchor);
			} else {
				try_writeln! (stream, " * [categories]({});", &categories_anchor);
			}
			empty = false;
		}
		if configuration.exports.enabled && library.has_exports () {
			let exports_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Export), anchor_self));
			if configuration.toc_compact {
				try_writeln! (stream, "[exports]({});", &exports_anchor);
			} else {
				try_writeln! (stream, " * [exports]({});", &exports_anchor);
			}
			empty = false;
		}
		if configuration.definitions.enabled && library.has_definitions () {
			let definitions_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Definition), anchor_self));
			if configuration.toc_compact {
				try_writeln! (stream, "[definitions]({});", &definitions_anchor);
			} else {
				try_writeln! (stream, " * [definitions]({});", &definitions_anchor);
			}
			empty = false;
		}
		if configuration.value_kinds.enabled && library.has_value_kinds () {
			let value_kinds_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::ValueKind), anchor_self));
			if configuration.toc_compact {
				try_writeln! (stream, "[types]({});", &value_kinds_anchor);
			} else {
				try_writeln! (stream, " * [types]({});", &value_kinds_anchor);
			}
			empty = false;
		}
		if configuration.appendices.enabled && library.has_appendices () {
			let appendices_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Appendix), anchor_self));
			if configuration.toc_compact {
				try_writeln! (stream, "[appendices]({});", &appendices_anchor);
			} else {
				try_writeln! (stream, " * [appendices]({});", &appendices_anchor);
			}
			empty = false;
		}
		if empty {
			if configuration.toc_compact {
				try_writeln! (stream, "(nothing);");
			} else {
				try_writeln! (stream, " * (nothing);");
			}
		}
	}
	
	if configuration.features {
		try! (dump_cmark_features_write (library.features (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, library.description (), library.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.examples {
		try! (dump_cmark_examples_write (library.examples (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_categories <'a> (library : &'a Library, categories : impl iter::ExactSizeIterator<Item = &'a Category>, configuration : &DumpCmarkCategoriesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibraryToc (library, LibraryEntityKind::Category, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		for category in categories {
			if category.has_parents () {
				continue;
			}
			let mut stack = StdVec::new ();
			stack.push ((category, true, category.children ()));
			while let Some ((category, emit, sub_categories)) = stack.pop () {
				if emit {
					let padding = "  " .repeat (stack.len ());
					let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
					if category.has_children () {
						try_writeln! (stream, "{}* [`{}`]({}):", padding, category.identifier (), category_anchor);
					} else {
						try_writeln! (stream, "{}* [`{}`]({});", padding, category.identifier (), category_anchor);
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
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_category (category : &Category, configuration : &DumpCmarkCategoryConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let library = category.library ();
	let anchor_self = DumpCmarkAnchor::Entity (category, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.description {
		try! (dump_cmark_description_write (library, category.description (), category.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (category.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	try! (dump_cmark_linked_exports_write (category.exports (), category.exports_recursive (), anchor_self, &configuration.exports, &configuration.generic, callbacks, stream));
	
	try! (dump_cmark_linked_definitions_write (category.definitions (), category.definitions_recursive (), anchor_self, &configuration.definitions, &configuration.generic, callbacks, stream));
	
	try! (dump_cmark_linked_value_kinds_write (category.value_kinds (), category.value_kinds_recursive (), anchor_self, &configuration.value_kinds, &configuration.generic, callbacks, stream));
	
	if configuration.hierarchy.super_direct && category.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-categories", (anchor_self, "super-categories"), &configuration.generic, stream));
		try_writeln! (stream);
		let mut categories_seen = StdSet::new ();
		for category in category.parents () {
			let seen = if categories_seen.contains (category.identifier ()) {
				if configuration.hierarchy.super_direct_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
			let fixes = if configuration.hierarchy.super_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
		if configuration.hierarchy.super_recursive
				&& category.parents () .len () != category.parents_recursive () .len ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Super-categories recursive", (anchor_self, "super-categories-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for category in category.parents_recursive () {
				let seen = if categories_seen.contains (category.identifier ()) {
					if configuration.hierarchy.super_recursive_complete { true } else { continue; }
				} else {
					categories_seen.insert (category.identifier ()); false
				};
				let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
				let fixes = if configuration.hierarchy.super_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-categories", (anchor_self, "super-categories"), &configuration.generic, stream));
		try_writeln! (stream);
		let categories_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Category), anchor_self));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &categories_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &categories_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && category.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Sub-categories", (anchor_self, "sub-categories"), &configuration.generic, stream));
		try_writeln! (stream);
		let mut categories_seen = StdSet::new ();
		for category in category.children () {
			let seen = if categories_seen.contains (category.identifier ()) {
				if configuration.hierarchy.sub_direct_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
			let fixes = if configuration.hierarchy.sub_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& category.children () .len () != category.children_recursive () .len ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Sub-categories recursive", (anchor_self, "sub-categories-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for category in category.children_recursive () {
				let seen = if categories_seen.contains (category.identifier ()) {
					if configuration.hierarchy.sub_recursive_complete { true } else { continue; }
				} else {
					categories_seen.insert (category.identifier ()); false
				};
				let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
				let fixes = if configuration.hierarchy.sub_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
				}
			}
		}
	}
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_exports <'a> (library : &'a Library, exports : impl iter::ExactSizeIterator<Item = &'a Export>, configuration : &DumpCmarkExportsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibraryToc (library, LibraryEntityKind::Export, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		for export in exports {
			if export.has_parents () {
				continue;
			}
			let mut stack = StdVec::new ();
			stack.push ((export, true, export.children ()));
			while let Some ((export, emit, sub_exports)) = stack.pop () {
				if emit {
					let padding = "  " .repeat (stack.len ());
					let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
					if export.has_children () {
						try_writeln! (stream, "{}* [`{}`]({}):", padding, export.identifier (), export_anchor);
					} else {
						try_writeln! (stream, "{}* [`{}`]({});", padding, export.identifier (), export_anchor);
					}
					stack.push ((export, false, sub_exports));
				} else {
					let mut sub_exports = sub_exports;
					if let Some (sub_export) = sub_exports.next () {
						stack.push ((export, false, sub_exports));
						stack.push ((sub_export, true, sub_export.children ()));
					}
				}
			}
		}
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_export (export : &Export, configuration : &DumpCmarkExportConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let library = export.library ();
	let anchor_self = DumpCmarkAnchor::Entity (export, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.descriptor {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Descriptor", (anchor_self, "descriptor"), &configuration.generic, stream));
		try_writeln! (stream);
		try! (dump_cmark_value_write (& export.descriptor_format (), stream));
	}
	
	if configuration.features {
		try! (dump_cmark_features_write (export.features (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	try! (dump_cmark_linked_definitions_write (export.definitions (), export.definitions_recursive (), anchor_self, &configuration.definitions, &configuration.generic, callbacks, stream));
	
	if configuration.description {
		try! (dump_cmark_description_write (library, export.description (), export.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (export.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	if configuration.hierarchy.super_direct && export.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-exports", (anchor_self, "super-exports"), &configuration.generic, stream));
		try_writeln! (stream);
		for export in export.parents () {
			let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
		if configuration.hierarchy.super_recursive
				&& export.parents () .len () != export.parents_recursive () .len ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Super-exports recursive", (anchor_self, "super-exports-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for export in export.parents_recursive () {
				let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-exports", (anchor_self, "super-exports"), &configuration.generic, stream));
		try_writeln! (stream);
		let exports_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Export), anchor_self));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &exports_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &exports_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && export.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Sub-exports", (anchor_self, "sub-exports"), &configuration.generic, stream));
		try_writeln! (stream);
		for export in export.children () {
			let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& export.children () .len () != export.children_recursive () .len ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Sub-exports recursive", (anchor_self, "sub-exports-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for export in export.children_recursive () {
				let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
				}
			}
		}
	}
	
	try! (dump_cmark_linked_categories_write (export.categories (), export.categories_recursive (), anchor_self, &configuration.categories, &configuration.generic, callbacks, stream));
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_definitions <'a> (library : &'a Library, definitions : impl iter::ExactSizeIterator<Item = &'a Definition>, configuration : &DumpCmarkDefinitionsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibraryToc (library, LibraryEntityKind::Definition, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		for definition in definitions {
			let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
			try_writeln! (stream, "* [`{}`]({});", definition.identifier (), definition_anchor);
		}
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_definition (definition : &Definition, configuration : &DumpCmarkDefinitionConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let library = definition.library ();
	let anchor_self = DumpCmarkAnchor::Entity (definition, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.kind {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Kind", (anchor_self, "kind"), &configuration.generic, stream));
		try_writeln! (stream);
		try_writeln! (stream, "`{}`;", definition.kind () .identifier ());
	}
	
	if let Some (procedure_signature) = if configuration.signature { definition.procedure_signature () } else { None } {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Procedure signature", (anchor_self, "procedure-signature"), &configuration.generic, stream));
		if ! procedure_signature.variants.is_empty () {
			try_writeln! (stream);
			try_writeln! (stream, "Procedure variants:");
			for procedure_signature_variant in procedure_signature.variants.iter () {
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn write_procedure_signature_value (value : &ProcedureSignatureValue, prefix : &str, anchor_self : DumpCmarkAnchor, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
					let value_kind = value.kind.deref ();
					let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
					if let Some (identifier) = value.identifier.as_ref () {
						try_writeln! (stream, "{}`{}` of type [`{}`]({});", prefix, identifier, value_kind.identifier (), value_kind_anchor);
					} else {
						try_writeln! (stream, "{}a value of type [`{}`]({});", prefix, value_kind.identifier (), value_kind_anchor);
					}
					succeed! (());
				}
				try_writeln! (stream, " * `{}`", dump_cmark_value_format (& procedure_signature_variant.format ()));
				if ! procedure_signature_variant.inputs.values.is_empty () {
					let procedure_signature_variant_inputs = &procedure_signature_variant.inputs;
					if procedure_signature_variant_inputs.values.len () > 1 || procedure_signature_variant_inputs.variadic {
						try_writeln! (stream, "   * inputs:");
						for procedure_signature_value in procedure_signature_variant_inputs.values.iter () {
							try! (write_procedure_signature_value (procedure_signature_value, "     * ", anchor_self, callbacks, stream));
						}
						if procedure_signature_variant_inputs.variadic {
							try_writeln! (stream, "     * `...` (i.e. variadic);");
						}
					} else {
						try! (write_procedure_signature_value (&procedure_signature_variant_inputs.values[0], "   * input: ", anchor_self, callbacks, stream));
					}
				} else {
					try_writeln! (stream, "   * inputs: none;");
				}
				if ! procedure_signature_variant.outputs.values.is_empty () {
					let procedure_signature_variant_outputs = &procedure_signature_variant.outputs;
					if procedure_signature_variant_outputs.values.len () > 1 || procedure_signature_variant_outputs.variadic {
						try_writeln! (stream, "   * outputs:");
						for procedure_signature_value in procedure_signature_variant_outputs.values.iter () {
							try! (write_procedure_signature_value (procedure_signature_value, "     * ", anchor_self, callbacks, stream));
						}
						if procedure_signature_variant_outputs.variadic {
							try_writeln! (stream, "     * `...` (i.e. variadic);");
						}
					} else {
						try! (write_procedure_signature_value (&procedure_signature_variant_outputs.values[0], "   * output: ", anchor_self, callbacks, stream));
					}
				} else {
					try_writeln! (stream, "   * outputs: none;");
				}
				if let Some (features) = &procedure_signature_variant.features {
					try_writeln! (stream, "   * requires: `{}`", dump_cmark_value_format (& features.format ()));
				}
			}
		}
	} else if definition.kind () .is_procedure () && configuration.generic.lints && configuration.signature {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Procedure signature", (anchor_self, "procedure-signature"), &configuration.generic, stream));
		try_writeln! (stream);
		try_writeln! (stream, "**FIXME!**  No procedure signature was provided!");
	}
	
	if let Some (syntax_signature) = if configuration.signature { definition.syntax_signature () } else { None } {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Syntax signature", (anchor_self, "syntax-signature"), &configuration.generic, stream));
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
						try_writeln! (stream, " * `{}`: constant with value `{}`;", identifier, dump_cmark_value_format (value)),
					SyntaxSignatureKeyword::Value { identifier, kind : value_kind } =>
						if let Some (value_kind) = value_kind {
							let value_kind_anchor = try! (callbacks.anchor_generate (value_kind.deref (), anchor_self));
							try_writeln! (stream, " * `{}`: value of type [{}]({});", identifier, value_kind.identifier (), value_kind_anchor);
						} else {
							try_writeln! (stream, " * `{}`: value with unspecified type;", identifier);
						},
					SyntaxSignatureKeyword::Pattern { identifier, patterns } =>
						if ! patterns.is_empty () {
							try_writeln! (stream, " * `{}`: pattern with variants:", identifier);
							for pattern in patterns.iter () {
								try_writeln! (stream, "   * `{}`;", dump_cmark_value_format (& pattern.format ()));
							}
						},
				}
			}
		}
		if ! syntax_signature.variants.is_empty () {
			try_writeln! (stream);
			try_writeln! (stream, "Syntax variants:");
			for syntax_signature_variant in syntax_signature.variants.iter () {
				try_writeln! (stream, " * `{}`", dump_cmark_value_format (& syntax_signature_variant.pattern.format ()));
			}
		}
	} else if definition.kind () .is_syntax () && configuration.generic.lints && configuration.signature {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Syntax signature", (anchor_self, "syntax-signature"), &configuration.generic, stream));
		try_writeln! (stream);
		try_writeln! (stream, "**FIXME!**  No syntax signature was provided!");
	}
	
	try! (dump_cmark_linked_exports_write (definition.exports (), definition.exports_recursive (), anchor_self, &configuration.exports, &configuration.generic, callbacks, stream));
	
	if configuration.aliases {
		try! (dump_cmark_aliases_write (definition.aliases (), anchor_self, configuration.aliases_compact, &configuration.generic, callbacks, stream));
	}
	
	if configuration.features {
		try! (dump_cmark_features_write (definition.features (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, definition.description (), definition.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (definition.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.examples {
		try! (dump_cmark_examples_write (definition.examples (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	if configuration.value_kinds.direct && definition.has_referenced_value_kinds () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Referenced-types", (anchor_self, "referenced-types"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in definition.referenced_value_kinds () {
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			if configuration.value_kinds.direct_compact {
				try_writeln! (stream, "[`{}`]({});", value_kind.identifier (), value_kind_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", value_kind.identifier (), value_kind_anchor);
			}
		}
	}
	
	try! (dump_cmark_linked_categories_write (definition.categories (), definition.categories_recursive (), anchor_self, &configuration.categories, &configuration.generic, callbacks, stream));
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_kinds <'a> (library : &Library, value_kinds : impl iter::ExactSizeIterator<Item = &'a ValueKind>, configuration : &DumpCmarkValueKindsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibraryToc (library, LibraryEntityKind::ValueKind, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (anchor_self, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		let mut value_kinds_seen = StdSet::new ();
		for value_kind in value_kinds {
			if value_kind.has_parents () {
				continue;
			}
			try! (dump_cmark_value_kind_write_tree (value_kind, &mut value_kinds_seen, anchor_self, configuration.toc_complete, configuration.toc_depth, callbacks, stream));
		}
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_value_kind (value_kind : &ValueKind, configuration : &DumpCmarkValueKindConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let library = value_kind.library ();
	let anchor_self = DumpCmarkAnchor::Entity (value_kind, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.tree
			&& value_kind.has_children ()
			&& value_kind.children () .len () != value_kind.children_recursive () .len ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Sub-types tree", (anchor_self, "sub-types-tree"), &configuration.generic, stream));
		try_writeln! (stream);
		let mut value_kinds_seen = StdSet::new ();
		for value_kind in value_kind.children () {
			try! (dump_cmark_value_kind_write_tree (value_kind, &mut value_kinds_seen, anchor_self, configuration.tree_complete, configuration.tree_depth, callbacks, stream));
		}
	}
	
	let mut value_kind_covariants_seen = StdSet::new ();
	let mut value_kind_contravariants_seen = StdSet::new ();
	
	if configuration.hierarchy.super_direct && value_kind.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-types", (anchor_self, "super-types"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.parents () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.hierarchy.super_direct_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.hierarchy.super_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.hierarchy.super_recursive
				&& (configuration.hierarchy.super_recursive_complete || value_kind.parents () .len () != value_kind.parents_recursive () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Super-types recursive", (anchor_self, "super-types-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for value_kind in value_kind.parents_recursive () {
				let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
					if configuration.hierarchy.super_recursive_complete { true } else { continue; }
				} else {
					value_kind_covariants_seen.insert (value_kind.identifier ()); false
				};
				let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
				let fixes = if configuration.hierarchy.super_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Super-types", (anchor_self, "super-types"), &configuration.generic, stream));
		try_writeln! (stream);
		let value_kinds_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::ValueKind), anchor_self));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &value_kinds_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &value_kinds_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && value_kind.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Sub-types", (anchor_self, "sub-types"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.children () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.hierarchy.sub_direct_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.hierarchy.sub_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& (configuration.hierarchy.sub_recursive_complete || value_kind.children () .len () != value_kind.children_recursive () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (5, "Sub-types recursive", (anchor_self, "sub-types-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for value_kind in value_kind.children_recursive () {
				let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
					if configuration.hierarchy.sub_recursive_complete { true } else { continue; }
				} else {
					value_kind_contravariants_seen.insert (value_kind.identifier ()); false
				};
				let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
				let fixes = if configuration.hierarchy.sub_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				}
			}
		}
	}
	
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	{ if configuration.covariants.direct
			&& value_kind.has_covariants ()
			&& (configuration.covariants.direct_complete || value_kind.covariants () .len () != value_kind_covariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Covariant types", (anchor_self, "types-covariant"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.covariants () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.covariants.direct_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.covariants.direct_complete && !seen { "**" } else { "" };
			if configuration.covariants.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	} }
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	{ if configuration.covariants.recursive
			&& ! value_kind.covariants_recursive () .is_empty ()
			&& (configuration.covariants.recursive_complete || value_kind.covariants_recursive () .len () != value_kind_covariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Covariant types recursive", (anchor_self, "types-covariant-recursive"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.covariants_recursive () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.covariants.recursive_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.covariants.recursive_complete && !seen { "**" } else { "" };
			if configuration.covariants.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	} }
	
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	{ if configuration.contravariants.direct
			&& value_kind.has_contravariants ()
			&& (configuration.contravariants.direct_complete || value_kind.contravariants () .len () != value_kind_contravariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contravariant types", (anchor_self, "types-contravariant"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.contravariants () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.contravariants.direct_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.contravariants.direct_complete && !seen { "**" } else { "" };
			if configuration.contravariants.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	} }
	#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
	{ if configuration.contravariants.recursive
			&& ! value_kind.contravariants_recursive () .is_empty ()
			&& (configuration.contravariants.recursive_complete || value_kind.contravariants_recursive () .len () != value_kind_contravariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contravariant types recursive", (anchor_self, "types-contravariant-recursive"), &configuration.generic, stream));
		try_writeln! (stream);
		for value_kind in value_kind.contravariants_recursive () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.contravariants.recursive_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if configuration.contravariants.recursive_complete && !seen { "**" } else { "" };
			if configuration.contravariants.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	} }
	
	if configuration.definitions_input.direct || configuration.definitions_input.recursive /* FIXME: || configuration.definitions_input_contravariant */ {
		let mut value_kind_definitions_seen = StdSet::new ();
		if configuration.definitions_input.direct
				&& value_kind.has_definitions_input ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as input", (anchor_self, "referent-definitions-input"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_input () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input.direct_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_input.direct_complete && !seen { "**" } else { "" };
				if configuration.definitions_input.direct_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
		}
		if configuration.definitions_input.recursive
				&& value_kind.has_definitions_input ()
				&& (configuration.definitions_input.recursive_complete || value_kind.definitions_input_recursive () .len () != value_kind.definitions_input () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as input (recursive)", (anchor_self, "referent-definitions-input-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_input_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input.recursive_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_input.recursive_complete && !seen { "**" } else { "" };
				if configuration.definitions_input.recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions consume an input that is a super-type.");
			}
		}
		#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
		{ if configuration.definitions_input_contravariant
				&& ! value_kind.definitions_input_contravariant_recursive () .is_empty ()
				&& (configuration.definitions_input_contravariant_complete || value_kind.definitions_input_contravariant_recursive () .len () != value_kind.definitions_input_recursive () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as input (contravariant)", (anchor_self, "referent-definitions-input-contravariant"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_input_contravariant_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input_contravariant_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_input_contravariant_complete && !seen { "**" } else { "" };
				if configuration.definitions_input_contravariant_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions consume an input that is a super-type-like (i.e. contravariant).");
			}
		} }
	}
	
	if configuration.definitions_output.direct || configuration.definitions_output.recursive /* FIXME: || configuration.definitions_output_covariant */ {
		let mut value_kind_definitions_seen = StdSet::new ();
		if configuration.definitions_output.direct
				&& value_kind.has_definitions_output ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as output", (anchor_self, "referent-definitions-output"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_output () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output.direct_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_output.direct_complete && !seen { "**" } else { "" };
				if configuration.definitions_output.direct_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
		}
		if configuration.definitions_output.recursive
				&& value_kind.has_definitions_output ()
				&& (configuration.definitions_output.recursive_complete || value_kind.definitions_output_recursive () .len () != value_kind.definitions_output () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as output (recursive)", (anchor_self, "referent-definitions-output-recursive"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_output_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output.recursive_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_output.recursive_complete && !seen { "**" } else { "" };
				if configuration.definitions_output.recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type.");
			}
		}
		#[ cfg ( feature = "vonuvoli_documentation_variances" ) ]
		{ if configuration.definitions_output_covariant
				&& ! value_kind.definitions_output_covariant_recursive () .is_empty ()
				&& (configuration.definitions_output_covariant_complete || value_kind.definitions_output_covariant_recursive () .len () != value_kind.definitions_output_recursive () .len ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try! (callbacks.header_write (4, "Referent definitions as output (covariant)", (anchor_self, "referent-definitions-output-covariant"), &configuration.generic, stream));
			try_writeln! (stream);
			for definition in value_kind.definitions_output_covariant_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output_covariant_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
				let fixes = if configuration.definitions_output_covariant_complete && !seen { "**" } else { "" };
				if configuration.definitions_output_covariant_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type-like (i.e. covariant).");
			}
		} }
	}
	
	if configuration.aliases {
		try! (dump_cmark_aliases_write (value_kind.aliases (), anchor_self, configuration.aliases_compact, &configuration.generic, callbacks, stream));
	}
	
	if configuration.features {
		try! (dump_cmark_features_write (value_kind.features (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	if configuration.predicate {
		if let Some (predicate) = value_kind.predicate () {
			match predicate {
				ValueKindPredicate::None =>
					(),
				ValueKindPredicate::Inherit =>
					(),
				ValueKindPredicate::Fixme =>
					if configuration.generic.fixme {
						try_writeln! (stream);
						try_writeln! (stream);
						try! (callbacks.header_write (4, "Predicate", (anchor_self, "predicate"), &configuration.generic, stream));
						try_writeln! (stream);
						try_writeln! (stream, "**FIXME!**");
					}
				ValueKindPredicate::Expression (ref value) =>
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try! (callbacks.header_write (4, "Predicate", (anchor_self, "predicate"), &configuration.generic, stream));
						try_writeln! (stream);
						try! (dump_cmark_value_write (value, stream));
					},
			}
		} else {
			if configuration.generic.lints {
				try_writeln! (stream);
				try_writeln! (stream);
				try! (callbacks.header_write (4, "Predicate", (anchor_self, "predicate"), &configuration.generic, stream));
				try_writeln! (stream);
				try_writeln! (stream, "**FIXME!**  No predicate was provided!");
			}
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, value_kind.description (), value_kind.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (value_kind.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.examples {
		try! (dump_cmark_examples_write (value_kind.examples (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	try! (dump_cmark_linked_categories_write (value_kind.categories (), value_kind.categories_recursive (), anchor_self, &configuration.categories, &configuration.generic, callbacks, stream));
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_appendices <'a> (library : &Library, appendices : impl iter::ExactSizeIterator<Item = &'a Appendix>, configuration : &DumpCmarkAppendicesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let anchor_self = DumpCmarkAnchor::LibraryToc (library, LibraryEntityKind::Appendix, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (None, anchor_self, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Contents", (library, LibraryEntityKind::Appendix, "contents"), &configuration.generic, stream));
		try_writeln! (stream);
		
		for appendix in appendices {
			let appendix_anchor = try! (callbacks.anchor_generate (appendix, anchor_self));
			if let Some (title) = appendix.title () {
				try_writeln! (stream, "* [`{}`]({}) -- {};", appendix.identifier (), appendix_anchor, title);
			} else {
				try_writeln! (stream, "* [`{}`]({});", appendix.identifier (), appendix_anchor);
			}
		}
		
		try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_appendix (appendix : &Appendix, configuration : &DumpCmarkAppendixConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let library = appendix.library ();
	let anchor_self = DumpCmarkAnchor::Entity (appendix, None);
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.title_write (appendix.title (), anchor_self, &configuration.generic, stream));
	
	if configuration.description {
		try! (dump_cmark_description_write (library, appendix.description (), appendix.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (appendix.links (), anchor_self, &configuration.generic, callbacks, stream));
	}
	
	try! (callbacks.break_write (anchor_self, &configuration.generic, stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (anchor_self, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_mangle_identifier (identifier : &str) -> (StdString) {
	let mut buffer = StdString::with_capacity (identifier.len ());
	let mut first = true;
	for character in identifier.chars () {
		match character {
			'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' =>
				buffer.push (character),
			'-' if !first =>
				buffer.push (character),
			_ => {
				let mut character_buffer = [0; 8];
				let character_bytes = character.encode_utf8 (&mut character_buffer) .as_bytes ();
				if first {
					buffer.push_str ("ZZZZ__");
				}
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
		first = false;
	}
	if let Some (buffer_last) = buffer.pop () {
		if buffer_last != '_' {
			buffer.push (buffer_last);
		}
	}
	return buffer;
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_path_generate <'a> (anchor : impl DumpCmarkAnchorInto<'a>, prefix : &str, suffix : &str) -> (Outcome<StdString>) {
	let anchor = anchor.anchor ();
	if anchor.section () .is_some () {
		fail! (0x2f345db8);
	}
	match anchor {
		DumpCmarkAnchor::LibrariesToc (_) =>
			succeed! (format! ("{}_libraries{}", prefix, suffix)),
		DumpCmarkAnchor::LibraryToc (library, entity, _) => {
			let library_identifier = dump_cmark_anchor_mangle_identifier (library.identifier ());
			match entity {
				LibraryEntityKind::Library =>
					fail! (0xc48f4847),
				LibraryEntityKind::Category =>
					succeed! (format! ("{}{}/categories/_index{}", prefix, library_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("{}{}/exports/_index{}", prefix, library_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("{}{}/definitions/_index{}", prefix, library_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("{}{}/types/_index{}", prefix, library_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					succeed! (format! ("{}{}/appendices/_index{}", prefix, library_identifier, suffix)),
			}
		},
		DumpCmarkAnchor::Entity (entity, _) => {
			let library = entity.library ();
			let library_identifier = dump_cmark_anchor_mangle_identifier (library.identifier ());
			let entity_identifier = dump_cmark_anchor_mangle_identifier (entity.identifier ());
			match entity.kind () {
				LibraryEntityKind::Library =>
					succeed! (format! ("{}{}/_index{}", prefix, library_identifier, suffix)),
				LibraryEntityKind::Category =>
					succeed! (format! ("{}{}/categories/{}{}", prefix, library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("{}{}/exports/{}{}", prefix, library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("{}{}/definitions/{}{}", prefix, library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("{}{}/types/{}{}", prefix, library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					succeed! (format! ("{}{}/appendices/{}{}", prefix, library_identifier, entity_identifier, suffix)),
			}
		},
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_generate <'a> (anchor : impl DumpCmarkAnchorInto<'a>) -> (Outcome<StdString>) {
	let anchor = anchor.anchor ();
	let suffix = if let Some (section) = anchor.section () {
		let section = dump_cmark_anchor_mangle_identifier (section);
		format! ("__{}", section)
	} else {
		StdString::new ()
	};
	match anchor {
		DumpCmarkAnchor::LibrariesToc (_) =>
			succeed! (format! ("toc__libraries{}", suffix)),
		DumpCmarkAnchor::LibraryToc (library, entity, _) => {
			let library_identifier = dump_cmark_anchor_mangle_identifier (library.identifier ());
			match entity {
				LibraryEntityKind::Library =>
					fail! (0x31791cd2),
				LibraryEntityKind::Category =>
					succeed! (format! ("toc__{}__categories{}", library_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("toc__{}__exports{}", library_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("toc__{}__definitions{}", library_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("toc__{}__types{}", library_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					succeed! (format! ("toc__{}__appendices{}", library_identifier, suffix)),
			}
		},
		DumpCmarkAnchor::Entity (entity, _) => {
			let library = entity.library ();
			let library_identifier = dump_cmark_anchor_mangle_identifier (library.identifier ());
			let entity_identifier = dump_cmark_anchor_mangle_identifier (entity.identifier ());
			match entity.kind () {
				LibraryEntityKind::Library =>
					succeed! (format! ("library__{}{}", library_identifier, suffix)),
				LibraryEntityKind::Category =>
					succeed! (format! ("category__{}__{}{}", library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("export__{}__{}{}", library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("definition__{}__{}{}", library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("type__{}__{}{}", library_identifier, entity_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					succeed! (format! ("appendix__{}__{}{}", library_identifier, entity_identifier, suffix)),
			}
		},
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_title_generate <'a> (title : Option<&str>, anchor : impl DumpCmarkAnchorInto<'a>) -> (Outcome<StdString>) {
	let anchor = anchor.anchor ();
	if anchor.section () .is_some () {
		fail! (0x2b2a680e);
	}
	let suffix = if let Some (title) = title {
		format! (" -- {}", title)
	} else {
		StdString::new ()
	};
	match anchor {
		DumpCmarkAnchor::LibrariesToc (_) =>
			succeed! (StdString::from ("Libraries")),
		DumpCmarkAnchor::LibraryToc (library, entity, _) => {
			let library_identifier = library.identifier ();
			match entity {
				LibraryEntityKind::Library =>
					fail! (0x4d1f5443),
				LibraryEntityKind::Category =>
					succeed! (format! ("`{}` Categories{}", library_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("`{}` Exports{}", library_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("`{}` Definitions{}", library_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("`{}` Types{}", library_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					succeed! (format! ("`{}` Appendices{}", library_identifier, suffix)),
			}
		},
		DumpCmarkAnchor::Entity (entity, _) => {
			let library = entity.library ();
			let library_identifier = library.identifier ();
			let entity_identifier = entity.identifier ();
			match entity.kind () {
				LibraryEntityKind::Library =>
					if let Some (title) = title {
						succeed! (StdString::from (title));
					} else {
						succeed! (format! ("`{}` -- Library{}", library_identifier, suffix));
					},
				LibraryEntityKind::Category =>
					succeed! (format! ("`{}` -- `{}` Category{}", entity_identifier, library_identifier, suffix)),
				LibraryEntityKind::Export =>
					succeed! (format! ("`{}` -- `{}` Export{}", entity_identifier, library_identifier, suffix)),
				LibraryEntityKind::Definition =>
					succeed! (format! ("`{}` -- `{}` Definition{}", entity_identifier, library_identifier, suffix)),
				LibraryEntityKind::ValueKind =>
					succeed! (format! ("`{}` -- `{}` Type{}", entity_identifier, library_identifier, suffix)),
				LibraryEntityKind::Appendix =>
					if let Some (title) = title {
						succeed! (StdString::from (title));
					} else {
						succeed! (format! ("`{}` -- `{}` Appendix{}", entity_identifier, library_identifier, suffix));
					},
			}
		},
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_write <'a> (anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor = anchor.anchor ();
	if configuration.anchors {
		let anchor = try! (dump_cmark_anchor_generate (anchor));
		if !configuration.html {
			try_writeln! (stream, "<a id='{}'></a>\n", anchor);
		} else {
			try_writeln! (stream, "<div class='anchor'><a id='{}'></a></div>\n", anchor);
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_title_write <'a> (title : Option<&str>, anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor = anchor.anchor ();
	let title = try! (dump_cmark_title_generate (title, anchor));
	return dump_cmark_header_write (1, &title, anchor, configuration, stream);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_header_write <'a> (header_depth : usize, header_caption : &str, anchor : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor = anchor.anchor ();
	let prefix = match header_depth {
		1 => "#",
		2 => "##",
		3 => "###",
		4 => "####",
		5 => "#####",
		6 => "######",
		_ => fail! (0x277d19b2),
	};
	if configuration.anchors && configuration.html {
		let anchor = try! (dump_cmark_anchor_generate (anchor));
		try_writeln! (stream, "{} {} <div class='heading-anchor'><a id='{}' href='#{}'>&sect;</a></div>", prefix, header_caption, anchor, anchor);
	} else {
		try! (dump_cmark_anchor_write (anchor, configuration, stream));
		try_writeln! (stream, "{} {}", prefix, header_caption);
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_string_write (string : &str, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let string = string.trim_matches ('\n');
	try_writeln! (stream, "````");
	try_writeln! (stream, "{}", string);
	try_writeln! (stream, "````");
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_write (value : &SchemeValue, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let text = dump_cmark_value_format (value);
	return dump_cmark_string_write (&text, stream);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_format (value : &SchemeValue) -> (StdString) {
	match value.kind () {
		SchemeValueKind::Null =>
			StdString::from ("()"),
		_ => {
			FIXME! ("better handle `#null` case");
			let buffer = format! ("{}", value);
			let buffer = buffer.replace ("#null", "()");
			buffer
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_kind_write_tree <'a> (value_kind : &'a ValueKind, value_kinds_seen : &mut StdSet<&'a str>, anchor_self : impl DumpCmarkAnchorInto<'a>, recursive_complete : bool, recursive_depth : usize, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
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
				let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
				let fixes = if recursive_complete && !seen { "**" } else { "" };
				if value_kind.has_children () {
					try_writeln! (stream, "{}* {}[`{}`]({}){}:", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, "{}* {}[`{}`]({}){};", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
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
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (trivially_copy_pass_by_ref) ) ]
fn dump_cmark_linked_categories_write <'a> (categories_direct : impl iter::ExactSizeIterator<Item = &'a Category>, categories_recursive : impl iter::ExactSizeIterator<Item = &'a Category>, anchor_self : impl DumpCmarkAnchorInto<'a>, categories_configuration : &DumpCmarkLinkedCategoriesConfiguration, generic_configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let categories_direct_count = categories_direct.len ();
	let categories_recursive_count = categories_recursive.len ();
	let mut categories_seen = StdSet::new ();
	if categories_configuration.direct && (categories_direct_count > 0) {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Categories", (anchor_self, "categories"), generic_configuration, stream));
		try_writeln! (stream);
		for category in categories_direct {
			let seen = if categories_seen.contains (category.identifier ()) {
				if categories_configuration.direct_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
			let fixes = if categories_configuration.direct_complete && !seen { "**" } else { "" };
			if categories_configuration.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
	}
	if categories_configuration.recursive
			&& (categories_configuration.recursive_complete || (categories_recursive_count != categories_direct_count))
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Categories recursive", (anchor_self, "categories-recursive"), generic_configuration, stream));
		try_writeln! (stream);
		for category in categories_recursive {
			let seen = if categories_seen.contains (category.identifier ()) {
				if categories_configuration.recursive_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (category, anchor_self));
			let fixes = if categories_configuration.recursive_complete && !seen { "**" } else { "" };
			if categories_configuration.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (trivially_copy_pass_by_ref) ) ]
fn dump_cmark_linked_exports_write <'a> (exports_direct : impl iter::ExactSizeIterator<Item = &'a Export>, exports_recursive : impl iter::ExactSizeIterator<Item = &'a Export>, anchor_self : impl DumpCmarkAnchorInto<'a>, exports_configuration : &DumpCmarkLinkedExportsConfiguration, generic_configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let exports_direct_count = exports_direct.len ();
	let exports_recursive_count = exports_recursive.len ();
	let mut exports_seen = StdSet::new ();
	if exports_configuration.direct && (exports_direct_count > 0) {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Exports", (anchor_self, "exports"), generic_configuration, stream));
		try_writeln! (stream);
		for export in exports_direct {
			let seen = if exports_seen.contains (export.identifier ()) {
				if exports_configuration.direct_complete { true } else { continue; }
			} else {
				exports_seen.insert (export.identifier ()); false
			};
			let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
			let fixes = if exports_configuration.direct_complete && !seen { "**" } else { "" };
			if exports_configuration.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, export.identifier (), export_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){} -- `{}`;", fixes, export.identifier (), export_anchor, fixes, dump_cmark_value_format (& export.descriptor_format ()));
			}
		}
	}
	if exports_configuration.recursive
			&& (exports_configuration.recursive_complete || (exports_recursive_count != exports_direct_count))
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Exports recursive", (anchor_self, "exports-recursive"), generic_configuration, stream));
		try_writeln! (stream);
		for export in exports_recursive {
			let seen = if exports_seen.contains (export.identifier ()) {
				if exports_configuration.recursive_complete { true } else { continue; }
			} else {
				exports_seen.insert (export.identifier ()); false
			};
			let export_anchor = try! (callbacks.anchor_generate (export, anchor_self));
			let fixes = if exports_configuration.recursive_complete && !seen { "**" } else { "" };
			if exports_configuration.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, export.identifier (), export_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){} -- `{}`;", fixes, export.identifier (), export_anchor, fixes, dump_cmark_value_format (& export.descriptor_format ()));
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (trivially_copy_pass_by_ref) ) ]
fn dump_cmark_linked_definitions_write <'a> (definitions_direct : impl iter::ExactSizeIterator<Item = &'a Definition>, definitions_recursive : impl iter::ExactSizeIterator<Item = &'a Definition>, anchor_self : impl DumpCmarkAnchorInto<'a>, definitions_configuration : &DumpCmarkLinkedDefinitionsConfiguration, generic_configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let definitions_direct_count = definitions_direct.len ();
	let definitions_recursive_count = definitions_recursive.len ();
	let mut definitions_seen = StdSet::new ();
	if definitions_configuration.direct && (definitions_direct_count > 0) {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Definitions", (anchor_self, "definitions"), generic_configuration, stream));
		try_writeln! (stream);
		for definition in definitions_direct {
			let seen = if definitions_seen.contains (definition.identifier ()) {
				if definitions_configuration.direct_complete { true } else { continue; }
			} else {
				definitions_seen.insert (definition.identifier ()); false
			};
			let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
			let fixes = if definitions_configuration.direct_complete && !seen { "**" } else { "" };
			if definitions_configuration.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
			}
		}
	}
	if definitions_configuration.recursive
			&& (definitions_configuration.recursive_complete || (definitions_recursive_count != definitions_direct_count))
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Definitions recursive", (anchor_self, "definitions-recursive"), generic_configuration, stream));
		try_writeln! (stream);
		for definition in definitions_recursive {
			let seen = if definitions_seen.contains (definition.identifier ()) {
				if definitions_configuration.recursive_complete { true } else { continue; }
			} else {
				definitions_seen.insert (definition.identifier ()); false
			};
			let definition_anchor = try! (callbacks.anchor_generate (definition, anchor_self));
			let fixes = if definitions_configuration.recursive_complete && !seen { "**" } else { "" };
			if definitions_configuration.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (trivially_copy_pass_by_ref) ) ]
fn dump_cmark_linked_value_kinds_write <'a> (value_kinds_direct : impl iter::ExactSizeIterator<Item = &'a ValueKind>, value_kinds_recursive : impl iter::ExactSizeIterator<Item = &'a ValueKind>, anchor_self : impl DumpCmarkAnchorInto<'a>, value_kinds_configuration : &DumpCmarkLinkedValueKindsConfiguration, generic_configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let value_kinds_direct_count = value_kinds_direct.len ();
	let value_kinds_recursive_count = value_kinds_recursive.len ();
	let mut value_kinds_seen = StdSet::new ();
	if value_kinds_configuration.direct && (value_kinds_direct_count > 0) {
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Types", (anchor_self, "types"), generic_configuration, stream));
		try_writeln! (stream);
		for value_kind in value_kinds_direct {
			let seen = if value_kinds_seen.contains (value_kind.identifier ()) {
				if value_kinds_configuration.direct_complete { true } else { continue; }
			} else {
				value_kinds_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if value_kinds_configuration.direct_complete && !seen { "**" } else { "" };
			if value_kinds_configuration.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
	}
	if value_kinds_configuration.recursive
			&& (value_kinds_configuration.recursive_complete || (value_kinds_recursive_count != value_kinds_direct_count))
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try! (callbacks.header_write (4, "Types recursive", (anchor_self, "types-recursive"), generic_configuration, stream));
		try_writeln! (stream);
		for value_kind in value_kinds_recursive {
			let seen = if value_kinds_seen.contains (value_kind.identifier ()) {
				if value_kinds_configuration.recursive_complete { true } else { continue; }
			} else {
				value_kinds_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (value_kind, anchor_self));
			let fixes = if value_kinds_configuration.recursive_complete && !seen { "**" } else { "" };
			if value_kinds_configuration.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_features_write <'a> (features : Option<&Features>, anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let features = if let Some (features) = features {
		features
	} else {
		succeed! (());
	};
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.header_write (4, "Features", (anchor_self, "features"), configuration, stream));
	try_writeln! (stream);
	try! (dump_cmark_value_write (& features.format (), stream));
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_examples_write <'a> (examples : Option<&Examples>, anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let examples = if let Some (examples) = examples {
		examples
	} else {
		succeed! (());
	};
	let examples = examples.examples ();
	if examples.is_empty () {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.header_write (4, "Examples", (anchor_self, "examples"), configuration, stream));
	for (index, example) in examples.enumerate () {
		let index = index + 1;
		try_writeln! (stream);
		try! (callbacks.header_write (5, & format! ("Examples {}", index), (anchor_self, & format! ("example-{}", index)), configuration, stream));
		try_writeln! (stream);
		for (index, sequence) in example.sequence.iter () .enumerate () {
			let index = index + 1;
			match *sequence {
				ExampleSequence::CodeText (ref text) => {
					try_writeln! (stream, " {}. evaluating:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::CodeExpression (ref value) => {
					try_writeln! (stream, " {}. evaluating:", index);
					try! (dump_cmark_value_write (value, stream));
				},
				ExampleSequence::ReturnText (ref text) => {
					try_writeln! (stream, " {}. returns:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::ReturnValue (ref value) => {
					try_writeln! (stream, " {}. returns:", index);
					try! (dump_cmark_value_write (value, stream));
				},
				ExampleSequence::ErrorText (ref text) => {
					try_writeln! (stream, " {}. raises:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::ErrorValue (ref value) => {
					try_writeln! (stream, " {}. raises:", index);
					try! (dump_cmark_value_write (value, stream));
				},
				ExampleSequence::StdinLineText (ref text) => {
					try_writeln! (stream, " {}. stdin input:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::StdinLineValue (ref value) => {
					try_writeln! (stream, " {}. stdin input:", index);
					try! (dump_cmark_value_write (value, stream));
				},
				ExampleSequence::StdoutLineText (ref text) => {
					try_writeln! (stream, " {}. stdout output:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::StdoutLineValue (ref value) => {
					try_writeln! (stream, " {}. stdout output:", index);
					try! (dump_cmark_value_write (value, stream));
				},
				ExampleSequence::StderrLineText (ref text) => {
					try_writeln! (stream, " {}. stderr output:", index);
					try! (dump_cmark_string_write (text, stream));
				},
				ExampleSequence::StderrLineValue (ref value) => {
					try_writeln! (stream, " {}. stderr output:", index);
					try! (dump_cmark_value_write (value, stream));
				},
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_aliases_write <'a> (aliases : impl iter::ExactSizeIterator<Item = &'a str>, anchor_self : impl DumpCmarkAnchorInto<'a>, aliases_compact : bool, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	if aliases.is_empty () {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.header_write (4, "Aliases", (anchor_self, "aliases"), configuration, stream));
	try_writeln! (stream);
	for alias in aliases {
		if aliases_compact {
			try_writeln! (stream, "`{}`;", alias);
		} else {
			try_writeln! (stream, " * `{}`;", alias);
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_description_write <'a> (library : &Library, description : Option<&Description>, links : Option<&Links>, anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let description = if let Some (description) = description {
		description
	} else {
		succeed! (());
	};
	if !configuration.fixme {
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
	if lines_empty && !configuration.lints {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.header_write (4, "Description", (anchor_self, "description"), configuration, stream));
	try_writeln! (stream);
	if lines_empty {
		try_writeln! (stream, "> **FIXME!**");
		succeed! (());
	}
	for line in description.lines () {
		let line = borrow::Cow::from (line);
		let line = DUMP_CMARK_CATEGORY_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xe66c9056);
					let identifier = identifier.as_str ();
					if let Some (category) = library.category_resolve (identifier) {
						let category_anchor = try_or_panic_0! (callbacks.anchor_generate (category, anchor_self), 0x4a1b437d);
						format! ("[`{}`]({})", category.identifier (), category_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_EXPORT_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xb74a418c);
					let identifier = identifier.as_str ();
					if let Some (export) = library.export_resolve (identifier) {
						let export_anchor = try_or_panic_0! (callbacks.anchor_generate (export, anchor_self), 0x038b6c15);
						format! ("[`{}`]({})", export.identifier (), export_anchor)
					} else {
						if configuration.lints {
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
						let value_kind_anchor = try_or_panic_0! (callbacks.anchor_generate (value_kind, anchor_self), 0x438c2cde);
						format! ("[`{}`]({})", value_kind.identifier (), value_kind_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_DEFINITION_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xe8c3f9f8);
					let identifier = identifier.as_str ();
					if let Some (definition) = library.definition_resolve (identifier) {
						let definition_anchor = try_or_panic_0! (callbacks.anchor_generate (definition, anchor_self), 0xf9025e58);
						format! ("[`{}`]({})", definition.identifier (), definition_anchor)
					} else {
						if configuration.lints {
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
					if let Some (_link) = link {
						//let link_anchor = try_or_panic_0! (callbacks.anchor_generate (link, anchor_self), 0x62baae72);
						//format! ("[[{}]]({})", link.identifier (), link_anchor)
						unimplemented_0! (0x8ed0afea);
					} else {
						//if configuration.lints {
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
						let appendix_anchor = try_or_panic_0! (callbacks.anchor_generate (appendix, anchor_self), 0x10a5c400);
						let appendix_label = appendix.title () .unwrap_or_else (|| appendix.identifier ());
						format! ("[\"{}\"]({})", appendix_label, appendix_anchor)
					} else {
						if configuration.lints {
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
fn dump_cmark_links_write <'a> (links : Option<&Links>, anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	let links = if let Some (links) = links {
		links
	} else {
		succeed! (());
	};
	let links_empty = links.links () .is_empty ();
	if links_empty && !configuration.lints {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.header_write (4, "Links", (anchor_self, "links"), configuration, stream));
	try_writeln! (stream);
	if links_empty {
		try_writeln! (stream, "> **FIXME!**");
		succeed! (());
	}
	fail_unimplemented! (0x81cb5f76);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_break_write <'a> (anchor_self : impl DumpCmarkAnchorInto<'a>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut StdVec<u8>) -> (Outcome<()>) {
	let anchor_self = anchor_self.anchor ();
	try_writeln! (stream);
	try_writeln! (stream, "----");
	if configuration.navigator {
		try_writeln! (stream);
		if !configuration.html {
			try_write! (stream, "Goto:");
		} else {
			try_write! (stream, "<div class='navigator'><span class='navigator-header'>Goto:</span>");
		}
		let mut empty = true;
		if let Some (library) = anchor_self.library () {
			if configuration.navigator_library {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let library_anchor = try! (callbacks.anchor_generate (library, anchor_self));
				if !configuration.html {
					try_write! (stream, "[library]({})", &library_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>library</a>", &library_anchor);
				}
			}
			if configuration.navigator_categories && library.has_categories () {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let categories_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Category), anchor_self));
				if !configuration.html {
					try_write! (stream, "[categories]({})", &categories_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>categories</a>", &categories_anchor);
				}
			}
			if configuration.navigator_exports && library.has_exports () {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let exports_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Export), anchor_self));
				if !configuration.html {
					try_write! (stream, "[exports]({})", &exports_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>exports</a>", &exports_anchor);
				}
			}
			if configuration.navigator_definitions && library.has_definitions () {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let definitions_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Definition), anchor_self));
				if !configuration.html {
					try_write! (stream, "[definitions]({})", &definitions_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>definitions</a>", &definitions_anchor);
				}
			}
			if configuration.navigator_value_kinds && library.has_value_kinds () {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let value_kinds_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::ValueKind), anchor_self));
				if !configuration.html {
					try_write! (stream, "[types]({})", &value_kinds_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>types</a>", &value_kinds_anchor);
				}
			}
			if configuration.navigator_appendices && library.has_appendices () {
				if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
				let appendices_anchor = try! (callbacks.anchor_generate ((library, LibraryEntityKind::Appendix), anchor_self));
				if !configuration.html {
					try_write! (stream, "[appendices]({})", &appendices_anchor);
				} else {
					try_write! (stream, "<a class='navigator-link' href='{}'>appendices</a>", &appendices_anchor);
				}
			}
		}
		if configuration.navigator_libraries {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", other "); }
			let libraries_anchor = try! (callbacks.anchor_generate (DumpCmarkAnchor::LibrariesToc (None), anchor_self));
			if !configuration.html {
				try_write! (stream, "[libraries]({})", &libraries_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>libraries</a>", &libraries_anchor);
			}
		}
		if !empty {
			if !configuration.html {
				try_writeln! (stream, ".");
			} else {
				try_writeln! (stream, ".</div>");
			}
		} else {
			if !configuration.html {
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




pub struct DumpCpioWriter <Writer : io::Write> {
	writer : Writer,
	written_folders : StdSet<fs_path::PathBuf>,
	written_files : StdSet<fs_path::PathBuf>,
	timestamp : u64,
}


impl <Writer : io::Write> DumpCpioWriter<Writer> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn open (writer : Writer) -> (Outcome<DumpCpioWriter<Writer>>) {
		let writer = DumpCpioWriter {
				writer : writer,
				written_folders : StdSet::new (),
				written_files : StdSet::new (),
				timestamp : try_or_fail! (time::UNIX_EPOCH.elapsed (), 0x943afc5a) .as_secs (),
			};
		succeed! (writer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn close (self) -> (Outcome<(Writer)>) {
		let writer = try_or_fail! (cpio::trailer (self.writer), 0xa81bcf20);
		succeed! (writer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write (&mut self, path : &fs_path::Path, data : &[u8]) -> (Outcome<()>) {
		
		if data.len () > (u32::max_value () as usize) {
			fail! (0xcc80443a);
		}
		
		{
			let mut parent = path;
			let mut parents = StdVec::new ();
			while let Some (entry_path) = parent.parent () {
				if fs_path::Path::eq (entry_path, fs_path::Path::new ("")) {
					fail! (0xf1cf9cac);
				}
				if fs_path::Path::eq (entry_path, fs_path::Path::new ("..")) {
					fail! (0x90ce4ede);
				}
				if fs_path::Path::eq (entry_path, fs_path::Path::new (".")) {
					break;
				}
				if ! self.written_folders.contains (entry_path) {
					parents.push (entry_path);
				}
				parent = entry_path;
			}
			parents.reverse ();
			for entry_path in parents {
				self.written_folders.insert (fs_path::PathBuf::from (entry_path));
				let entry_ino = self.written_files.len () + self.written_folders.len ();
				
				FIXME! ("solve the issue of moving `stream` in and out of the CPIO writer");
				let original_stream : &mut dyn io::Write = unsafe { mem::transmute_copy (&self.writer) };
				
				let entry_stream = cpio::Builder
						::new (try_some_or_panic! (entry_path.to_str (), 0x710bc6c6))
						.mode (0o_040_000 | 0o_000_755)
						.ino (entry_ino as u32)
						.uid (0xfffe)
						.gid (0xfffe)
						.mtime (self.timestamp as u32)
						.nlink (1)
						.write (original_stream, 0);
				
				try_or_fail! (entry_stream.finish (), 0xc63d28a0);
			}
		}
		
		{
			let entry_path = path;
			if ! self.written_files.insert (fs_path::PathBuf::from (entry_path)) {
				fail! (0x94276b39);
			}
			let entry_ino = self.written_files.len () + self.written_folders.len ();
			
			FIXME! ("solve the issue of moving `stream` in and out of the CPIO writer");
			let original_stream : &mut dyn io::Write = unsafe { mem::transmute_copy (&self.writer) };
			
			let mut entry_stream = cpio::Builder
					::new (try_some_or_panic! (entry_path.to_str (), 0x710bc6c6))
					.mode (0o_100_000 | 0o_000_644)
					.ino (entry_ino as u32)
					.uid (0xfffe)
					.gid (0xfffe)
					.mtime (self.timestamp as u32)
					.nlink (1)
					.write (original_stream, data.len () as u32);
			
			try_or_fail! (entry_stream.write_all (data), 0x52251862);
			
			try_or_fail! (entry_stream.finish (), 0xb0be881c);
		}
		
		succeed! (());
	}
}




lazy_static! {
	static ref DUMP_CMARK_CATEGORY_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#category\)"), 0x7a74ab93);
	static ref DUMP_CMARK_EXPORT_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#export\)"), 0x61f706ce);
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
	<meta name="viewport" content="width=device-width, height=device-height, initial-scale=1, minimum-scale=0.5, maximum-scale=4.0, user-scalable=yes">
	<title>@{title}</title>
</head>
<body>
"####;


static DUMP_HTML_SUFFIX : &str =
r####"</body>
</html>
"####;


static DUMP_HTML_CSS : &str = include_str! ("../documentation/libraries.css");
static DUMP_HTML_JS : &str = include_str! ("../documentation/libraries.js");

