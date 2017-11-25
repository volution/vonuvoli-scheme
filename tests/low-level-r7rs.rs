

extern crate rust_scheme;

use rust_scheme::exports::*;
use rust_scheme::runtime::exports::*;




#[ test ]
fn test () -> () {
	
	
	
	
	let print_all_forced = true;
	let print_all_missing = false;
	
	let print_definitions_table = true;
	let print_definitions_symbols = true;
	let print_definitions_symbols_list = !print_all_forced && !print_all_missing;
	
	let print_procedures = print_all_forced || print_all_missing;
	let print_syntaxes = print_all_forced || print_all_missing;
	let print_values = print_all_forced || print_all_missing;
	
	let print_library_base = print_all_forced || print_all_missing;
	let print_library_miscellaneous = print_all_forced || print_all_missing;
	
	let print_implemented = print_all_forced || !print_all_missing;
	let print_unimplemented = print_all_forced || print_all_missing;
	let print_unsupported = print_all_forced || !print_all_missing;
	let print_reserved = print_all_forced || !print_all_missing;
	
	
	
	
	let definitions = language_r7rs_generate_definitions () .expect ("3bd1d93c");
	
	language_r7rs_verify_definitions (&definitions) .expect ("11e64ae7");
	
	
	
	
	let mut implemented_symbols = StdSet::new ();
	let mut unimplemented_symbols = StdSet::new ();
	let mut unsupported_symbols = StdSet::new ();
	let mut reserved_symbols = StdSet::new ();
	
	
	
	
	if print_definitions_table {
		println! ("##");
	}
	
	macro_rules! print_definition {
		($library : expr, $category : expr, $type : expr, $identifier : expr, $value : expr) => (
			if print_definitions_table {
				println! ("|  {:^16}  |  {:^12}  |  {:^16}  |  {:<32}  |  {:?}", $library.string_as_str (), $category.string_as_str (), $type, $identifier.string_as_str (), $value);
			}
		);
	}
	
	for (library, category, identifier, value) in definitions.into_iter () {
		
		let library_is_base = library.string_eq ("base");
		if !(
				(print_library_base && library_is_base) ||
				(print_library_miscellaneous && !library_is_base)
		) {
			continue;
		}
		
		match value.class () {
			
			ValueClass::ProcedurePrimitive => {
				let primitive = ProcedurePrimitive::from (value);
				match primitive {
					
					ProcedurePrimitive::Primitive0 (_) |
					ProcedurePrimitive::Primitive1 (_) |
					ProcedurePrimitive::Primitive2 (_) |
					ProcedurePrimitive::Primitive3 (_) |
					ProcedurePrimitive::Primitive4 (_) |
					ProcedurePrimitive::PrimitiveN (_) => {
						if print_procedures && print_implemented {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					}
					
					ProcedurePrimitive::Unimplemented => {
						if print_procedures && print_unimplemented {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						unimplemented_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::Unsupported => {
						if print_procedures && print_unsupported {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						unsupported_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::Reserved => {
						if print_procedures && print_reserved {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						reserved_symbols.insert (identifier);
					}
					
				}
			},
			
			ValueClass::SyntaxPrimitive => {
				let primitive = SyntaxPrimitive::from (value);
				match primitive {
					
					SyntaxPrimitive::Primitive0 (_) |
					SyntaxPrimitive::Primitive1 (_) |
					SyntaxPrimitive::Primitive2 (_) |
					SyntaxPrimitive::Primitive3 (_) |
					SyntaxPrimitive::Primitive4 (_) |
					SyntaxPrimitive::PrimitiveN (_) => {
						if print_syntaxes && print_implemented {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Auxiliary  => {
						if print_syntaxes && print_implemented {
							print_definition! (library, category, "auxiliary-syntax", identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Unimplemented => {
						if print_syntaxes && print_unimplemented {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						unimplemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Unsupported => {
						if print_syntaxes && print_unsupported {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						unsupported_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Reserved => {
						if print_syntaxes && print_reserved {
							print_definition! (library, category, "reserved", identifier, primitive);
						}
						reserved_symbols.insert (identifier);
					}
					
				}
			},
			
			_ =>
				if print_values {
					print_definition! (library, category, "value", identifier, value);
					implemented_symbols.insert (identifier);
				},
			
		}
	}
	
	if print_definitions_table {
		println! ("##");
		println! ();
	}
	
	
	
	
	macro_rules! print_symbols {
		($label : expr, $symbols : expr, $print : expr, $total_available_symbols : expr, $total_specified_symbols : expr) => (
			if print_definitions_symbols {
				let symbols_count = $symbols.len ();
				let symbols_ratio_vs_available = (symbols_count as f64) / ($total_available_symbols as f64);
				let symbols_ratio_vs_specified = (symbols_count as f64) / ($total_specified_symbols as f64);
				println! ("== {:16} {:4} ({:05.2}% / {:05.2}%)", $label, symbols_count, symbols_ratio_vs_available * 100.0, symbols_ratio_vs_specified * 100.0);
				if $print && print_definitions_symbols_list {
					let mut symbols = $symbols.into_iter () .collect::<StdVec<_>> ();
					symbols.sort ();
					for symbol in symbols.into_iter () {
						println! ("    {}", symbol.string_as_str ());
					}
					println! ("##");
				}
			}
		);
	}
	
	if print_definitions_symbols {
		println! ("##");
		let total_available_symbols = implemented_symbols.len () + unimplemented_symbols.len ();
		let total_specified_symbols = total_available_symbols + unsupported_symbols.len ();
		print_symbols! ("implemented", implemented_symbols, print_implemented, total_available_symbols, total_specified_symbols);
		print_symbols! ("unimplemented", unimplemented_symbols, print_unimplemented, total_available_symbols, total_specified_symbols);
		print_symbols! ("unsupported", unsupported_symbols, print_unsupported, total_available_symbols, total_specified_symbols);
		print_symbols! ("reserved", reserved_symbols, print_reserved, total_available_symbols, total_specified_symbols);
		println! ("##");
		println! ();
	}
	
}
