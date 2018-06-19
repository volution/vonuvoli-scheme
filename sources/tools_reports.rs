

use super::errors::exports::*;
use super::libraries::exports::*;
use super::primitives::exports::*;
use super::tools::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::main;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	if ! inputs.tool_arguments.is_empty () {
		fail! (0x9c2cd673);
	}
	if ! inputs.rest_arguments.is_empty () {
		fail! (0x265b1100);
	}
	
	match vec_map! (inputs.tool_commands.iter (), command, command.as_str ()) .as_slice () {
		&["r7rs", "definitions"] =>
			return main_r7rs_definitions (&mut stream),
		&["libraries", "definitions"] =>
			return main_libraries_definitions (&mut stream),
		&["primitives", "variants"] =>
			return main_primitives_variants (&mut stream),
		_ =>
			fail! (0xb4206e56),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_primitives_variants (stream : &mut io::Write) -> (Outcome<u32>) {
	
	{
		let primitives = StdVec::from (syntax_primitive_variants::<SyntaxPrimitive> ());
		let mut primitives = vec_map_into! (primitives, primitive, primitive.identifier ());
		
		primitives.sort ();
		
		for primitive in primitives.into_iter () {
			try_writeln! (stream, "{}", primitive);
		}
	}
	
	{
		let primitives = StdVec::from (procedure_primitive_variants::<ProcedurePrimitive> ());
		let mut primitives = vec_map_into! (primitives, primitive,
				match primitive.is_negated () {
					Some (false) | None =>
						borrow::Cow::from (primitive.identifier ()),
					Some (true) =>
						borrow::Cow::from (format! ("{}Negated", primitive.identifier ())),
				});
		
		primitives.sort ();
		
		for primitive in primitives.into_iter () {
			try_writeln! (stream, "{}", primitive);
		}
	}
	
	succeed! (0);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_libraries_definitions (stream : &mut io::Write) -> (Outcome<u32>) {
	
	let definitions_r7rs = try! (library_r7rs_generate_definitions ());
	let definitions_builtins = try! (library_builtins_generate_definitions ());
	
	let mut definitions = StdVec::with_capacity (definitions_r7rs.len () + definitions_builtins.len ());
	for (_, _, symbol, value) in definitions_r7rs.into_iter () {
		definitions.push (StdRc::new (("r7rs", symbol, value)));
	}
	for (symbol, value) in  definitions_builtins.into_iter () {
		definitions.push (StdRc::new (("builtins", symbol, value)));
	}
	
	let mut definitions_by_symbol = StdMap::with_capacity (definitions.len ());
	let mut definitions_by_value = StdMap::with_capacity (definitions.len ());
	
	for definition in definitions.into_iter () {
		let &(_, ref symbol, ref value) = definition.deref ();
		if let Some (existing) = definitions_by_symbol.insert (symbol.clone (), definition.clone ()) {
			let &(_, _, ref existing) = existing.deref ();
			if ! Value::is_self (value, existing) {
				fail! (0x335403e9);
			}
		} else {
			let aliases = definitions_by_value.entry (value.clone ()) .or_insert_with (StdVec::new);
			aliases.push (definition.clone ());
			aliases.sort_by (|left, right| cmp::Ord::cmp (&(&left.1, &left.0), &(&right.1, &right.0)));
		}
	}
	
	let mut possible_values = StdVec::with_capacity (definitions_by_value.len ());
	for value in definitions_by_value.keys () {
		possible_values.push (value.clone ());
	}
	for value in StdVec::from (syntax_primitive_variants ()) .into_iter () {
		possible_values.push (value);
	}
	for value in StdVec::from (procedure_primitive_variants ()) .into_iter () {
		possible_values.push (value);
	}
	possible_values.sort ();
	possible_values.dedup ();
	
	let mut exported_values = StdVec::with_capacity (possible_values.len ());
	let mut reachable_values = StdSet::with_capacity (possible_values.len ());
	let mut values_alternatives = StdMap::new ();
	for value in possible_values.into_iter () {
		let (order, unavailable, alternatives) = match value.kind_match_as_ref () {
			
			ValueKindMatchAsRef::SyntaxPrimitive (primitive) => {
				let unavailable = match primitive {
					SyntaxPrimitive::Unimplemented | SyntaxPrimitive::Unsupported | SyntaxPrimitive::Reserved =>
						true,
					_ =>
						false,
				};
				if ! unavailable {
					((11, 0, None), unavailable, None)
				} else {
					((89, 1, None), unavailable, None)
				}
			},
			ValueKindMatchAsRef::SyntaxNative (_) =>
				((12, 0, None), false, None),
			ValueKindMatchAsRef::SyntaxExtended (_) =>
				((13, 0, None), false, None),
			ValueKindMatchAsRef::SyntaxLambda (_) =>
				((14, 0, None), false, None),
			
			ValueKindMatchAsRef::ProcedurePrimitive (primitive) => {
				let primitive_class = primitive.class ();
				let (unavailable, alternatives) = match primitive {
					ProcedurePrimitive::Unimplemented | ProcedurePrimitive::Unsupported | ProcedurePrimitive::Reserved =>
						(true, None),
					ProcedurePrimitive::PrimitiveV (primitive) =>
						(false, Some (primitive.alternatives_all_into::<Value> ())),
					_ =>
						(false, None),
				};
				if ! unavailable {
					((21, primitive_class as u64, None), unavailable, alternatives)
				} else {
					((89, 2, None), unavailable, alternatives)
				}
			},
			ValueKindMatchAsRef::ProcedureNative (procedure) => {
				let symbol = borrow::Cow::from (procedure.symbol () .resolve_name ());
				let alternatives = procedure.alternatives_all_into::<Value> ();
				((22, 0, Some (symbol)), false, alternatives)
			},
			ValueKindMatchAsRef::ProcedureExtended (_) =>
				((23, 0, None), false, None),
			ValueKindMatchAsRef::ProcedureLambda (_) =>
				((24, 0, None), false, None),
			
			ValueKindMatchAsRef::Parameter (parameter) => {
				let identifier = try! (parameter.identifier ());
				let identifier = option_map! (identifier, borrow::Cow::from (StdString::from (identifier.string_as_str ())));
				((71, 0, identifier), false, None)
			},
			
			_ =>
				((99, 0, None), false, None),
			
		};
		exported_values.push ((value.clone (), order, unavailable));
		if let Some (_definitions) = definitions_by_value.get (&value) {
			reachable_values.insert (value.clone ());
		}
		if let Some (alternatives) = alternatives {
			for alternative in alternatives.iter () {
				reachable_values.insert (alternative.clone ());
			}
			values_alternatives.insert (value.clone (), alternatives);
		}
	}
	exported_values.sort_by (|left, right| cmp::Ord::cmp (&(&left.1, &left.0), &(&right.1, &right.0)));
	
	try_writeln! (stream, "| {:^8} | {:^5} |  {:^64}  |  {:<16}  |  {:}", "Library", "Flags", "Symbol", "Display", "Debug");
	try_writeln! (stream, "| {:^8} | {:^5} |  {:^64}  |  {:<16}  |  {:}", ":---:", ":---:", ":---:", ":---", ":---");
	for &(ref value, ref _order, unavailable) in exported_values.iter () {
		let alternatives = values_alternatives.get (value);
		if let Some (definitions) = definitions_by_value.get (value) {
			for definition in definitions.iter () {
				let &(source, ref symbol, ref value) = definition.deref ();
				let aliases_flag = if unavailable { "!" } else if definitions.len () > 1 { "~" } else { "" };
				if let Some (alternatives) = alternatives {
					try_writeln! (stream, "| {:^8} |  {:>2} {:1} | `{:<64}` | `{:}` | `{:?}`", source, alternatives.len (), aliases_flag, symbol.string_as_str (), value, value);
				} else {
					try_writeln! (stream, "| {:^8} |  {:>2} {:1} | `{:<64}` | `{:}` | `{:?}`", source, "", aliases_flag, symbol.string_as_str (), value, value);
				}
			}
		} else {
			assert_0! (! unavailable, 0x1b8fb3c5);
			if ! reachable_values.contains (value) {
				if let Some (alternatives) = alternatives {
					try_writeln! (stream, "| {:^8} |  {:>2} {:1} | `{:<64}` | `{:}` | `{:?}`", "!!!!", alternatives.len (), "", "!!!! not-exported !!!!", value, value);
				} else {
					try_writeln! (stream, "| {:^8} |  {:>2} {:1} | `{:<64}` | `{:}` | `{:?}`", "!!!!", "", "", "!!!! not-exported !!!!", value, value);
				}
			} else {
				assert_0! (alternatives.is_none (), 0xc287f350);
			}
		}
		if let Some (alternatives) = alternatives {
			assert_0! (! unavailable, 0x3a195236);
			assert_0! (! alternatives.is_empty (), 0xc0452104);
			for alternative in alternatives.iter () {
				try_writeln! (stream, "| {:^8} |  {:>2} {:1} |  {:<64}  | `{:}` | `{:?}`", "", "*", "", "", alternative, alternative);
			}
		}
	}
	
	succeed! (0);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_r7rs_definitions (stream : &mut io::Write) -> (Outcome<u32>) {
	
	let print_all_forced = true;
	let print_all_missing = true;
	
	let print_headers = true;
	let print_definitions_table = true;
	let print_definitions_symbols = true;
	let print_definitions_symbols_list = print_definitions_symbols && print_all_missing;
	
	let print_procedures = print_all_forced || print_all_missing;
	let print_procedure_alternatives = print_all_forced || print_all_missing;
	let print_syntaxes = print_all_forced || print_all_missing;
	let print_values = print_all_forced || print_all_missing;
	
	let print_library_base = print_all_forced || print_all_missing;
	let print_library_ports = print_all_forced || print_all_missing;
	let print_library_miscellaneous = print_all_forced || print_all_missing;
	
	let print_implemented = print_all_forced;
	let print_unimplemented = print_all_forced || print_all_missing;
	let print_unsupported = print_all_forced || print_all_missing;
	let print_reserved = print_all_forced;
	
	let print_implemented_symbols = false;
	let print_reserved_symbols = false;
	
	
	
	
	let definitions = try! (library_r7rs_generate_definitions ());
	
	try! (library_r7rs_verify_definitions (&definitions));
	
	
	
	
	let mut implemented_symbols = StdSet::new ();
	let mut unimplemented_symbols = StdSet::new ();
	let mut unsupported_symbols = StdSet::new ();
	let mut reserved_symbols = StdSet::new ();
	
	
	
	
	if print_definitions_table && print_headers {
		try_writeln! (stream);
		try_writeln! (stream, "## Scheme R7RS definitions");
		try_writeln! (stream);
	}
	
	macro_rules! print_definition {
		($library : expr, $category : expr, $type : expr, $identifier : expr, $value : expr) => (
			if print_definitions_table {
				#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
				try_writeln! (stream, "|  {:^16}  |  {:^12}  |  {:^16}  | `{:<32}` | `{:?}`", $library.string_as_str (), $category.string_as_str (), $type, $identifier.string_as_str (), $value);
				#[ cfg ( not ( feature = "vonuvoli_fmt_debug" ) ) ]
				try_writeln! (stream, "|  {:^16}  |  {:^12}  |  {:^16}  | `{:<32}` |", $library.string_as_str (), $category.string_as_str (), $type, $identifier.string_as_str ());
			}
		);
	}
	
	if print_definitions_table {
		try_writeln! (stream, "|  {:^16}  |  {:^12}  |  {:^16}  |  {:<32}  |  {:}", "Library", "Category", "Type", "Scheme identifier", "Rust value");
		try_writeln! (stream, "|  {:^16}  |  {:^12}  |  {:^16}  |  {:<32}  |  {:}", ":---:", ":---:", ":---:", ":---", ":---");
	}
	
	for (library, category, identifier, value) in definitions.into_iter () {
		
		let library_is_ports = category.string_eq ("ports");
		let library_is_base = library.string_eq ("base") && !library_is_ports;
		let library_is_miscellaneous = !library_is_base && !library_is_ports;
		if !(
				(print_library_base && library_is_base) ||
				(print_library_ports && library_is_ports) ||
				(print_library_miscellaneous && library_is_miscellaneous)
		) {
			continue;
		}
		
		match value.kind_match_as_ref () {
			
			ValueKindMatchAsRef::ProcedurePrimitive (primitive) => {
				let primitive = *primitive;
				match primitive {
					
					ProcedurePrimitive::Primitive0 (_) |
					ProcedurePrimitive::Primitive1 (_) |
					ProcedurePrimitive::Primitive2 (_) |
					ProcedurePrimitive::Primitive3 (_) |
					ProcedurePrimitive::Primitive4 (_) |
					ProcedurePrimitive::Primitive5 (_) |
					ProcedurePrimitive::PrimitiveN (_) => {
						let arity = match primitive {
							ProcedurePrimitive::Primitive0 (_) => '0',
							ProcedurePrimitive::Primitive1 (_) => '1',
							ProcedurePrimitive::Primitive2 (_) => '2',
							ProcedurePrimitive::Primitive3 (_) => '3',
							ProcedurePrimitive::Primitive4 (_) => '4',
							ProcedurePrimitive::Primitive5 (_) => '5',
							ProcedurePrimitive::PrimitiveN (_) => 'n',
							_ => panic_0! (0x7e5d3d15),
						};
						if print_procedures && print_implemented {
							print_definition! (library, category, format! ("procedure-{}", arity), identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::PrimitiveV (primitive_v) => {
						if print_procedures && print_implemented && print_procedure_alternatives {
							let mut has_alternatives = false;
							if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive_v) {
								print_definition! (library, category, "procedure-0*", identifier, ProcedurePrimitive::Primitive0 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive_v) {
								print_definition! (library, category, "procedure-1*", identifier, ProcedurePrimitive::Primitive1 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive_v) {
								print_definition! (library, category, "procedure-2*", identifier, ProcedurePrimitive::Primitive2 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive_v) {
								print_definition! (library, category, "procedure-3*", identifier, ProcedurePrimitive::Primitive3 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive_v) {
								print_definition! (library, category, "procedure-4*", identifier, ProcedurePrimitive::Primitive4 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive_v) {
								print_definition! (library, category, "procedure-5*", identifier, ProcedurePrimitive::Primitive5 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_n (primitive_v) {
								print_definition! (library, category, "procedure-n*", identifier, ProcedurePrimitive::PrimitiveN (primitive));
								has_alternatives = true;
							}
							if has_alternatives {
								print_definition! (library, category, "procedure-v*", identifier, primitive);
							} else {
								panic_0! (0xed9c74b7);
							}
						} else {
							if print_procedures && print_implemented {
								print_definition! (library, category, "procedure-v", identifier, primitive);
							}
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
			
			ValueKindMatchAsRef::SyntaxPrimitive (primitive) => {
				let primitive = *primitive;
				match primitive {
					
					SyntaxPrimitive::PrimitiveV (_) => {
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
	
	if print_definitions_table && print_headers {
		try_writeln! (stream);
		try_writeln! (stream, "****");
		try_writeln! (stream);
	}
	
	
	
	
	if print_definitions_symbols && print_headers {
		try_writeln! (stream);
		try_writeln! (stream, "## Scheme R7RS definitions -- summary");
		try_writeln! (stream);
	}
	
	macro_rules! print_symbols {
		($label : expr, $symbols : expr, $print : expr, $total_available_symbols : expr, $total_specified_symbols : expr) => (
			if print_definitions_symbols {
				let symbols_count = $symbols.len ();
				let symbols_ratio_vs_available = (symbols_count as f64) / ($total_available_symbols as f64);
				let symbols_ratio_vs_specified = (symbols_count as f64) / ($total_specified_symbols as f64);
				try_writeln! (stream, "* {:16} {:4} ({:05.2}% / {:05.2}%)", $label, symbols_count, symbols_ratio_vs_available * 100.0, symbols_ratio_vs_specified * 100.0);
				if $print && print_definitions_symbols_list {
					let mut symbols = $symbols.into_iter () .collect::<StdVec<_>> ();
					symbols.sort ();
					if ! symbols.is_empty () {
						try_writeln! (stream, "  ```");
						for symbol in symbols.into_iter () {
							try_writeln! (stream, "    {}", symbol.string_as_str ());
						}
						try_writeln! (stream, "  ```");
					}
				}
				try_writeln! (stream);
			}
		);
	}
	
	if print_definitions_symbols {
		let total_available_symbols = implemented_symbols.len () + unimplemented_symbols.len ();
		let total_specified_symbols = total_available_symbols + unsupported_symbols.len ();
		print_symbols! ("implemented", implemented_symbols, print_implemented && print_implemented_symbols, total_available_symbols, total_specified_symbols);
		print_symbols! ("unimplemented", unimplemented_symbols, print_unimplemented, total_available_symbols, total_specified_symbols);
		print_symbols! ("unsupported", unsupported_symbols, print_unsupported, total_available_symbols, total_specified_symbols);
		print_symbols! ("reserved", reserved_symbols, print_reserved && print_reserved_symbols, total_available_symbols, total_specified_symbols);
	}
	
	if print_definitions_symbols && print_headers {
		try_writeln! (stream, "****");
		try_writeln! (stream);
	}
	
	
	succeed! (0);
}

