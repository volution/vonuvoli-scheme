

<a id='type__r7rs__symbol'></a>

# `symbol` -- `r7rs` Type


<a id='type__r7rs__symbol__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__symbol__referent-definitions-input'></a>

#### Referent definitions as input

 * [`symbol?`](../../r7rs/definitions/symbol_3f.md#definition__r7rs__symbol_3f);
 * [`symbol=?`](../../r7rs/definitions/symbol_3d_3f.md#definition__r7rs__symbol_3d_3f);
 * [`symbol->string`](../../r7rs/definitions/symbol-_3e_string.md#definition__r7rs__symbol-_3e_string);


<a id='type__r7rs__symbol__referent-definitions-output'></a>

#### Referent definitions as output

 * [`string->symbol`](../../r7rs/definitions/string-_3e_symbol.md#definition__r7rs__string-_3e_symbol);


<a id='type__r7rs__symbol__predicate'></a>

#### Predicate

````
symbol?
````


<a id='type__r7rs__symbol__description'></a>

#### Description

> Symbols are objects whose usefulness rests on the fact that two
> symbols are identical (in the sense of `eqv?`) if and only if their
> names are spelled the same way.  For instance, they can be used
> the way enumerated values are used in other languages.
> 
> The rules for writing a symbol are exactly the same as the rules for
> writing an identifier; see sections on identifiers
> and symbols.
> 
> It is guaranteed that any symbol that has been returned as part of
> a literal expression, or read using the `read` procedure, and
> subsequently written out using the `write` procedure, will read back
> in as the identical symbol (in the sense of `eqv?`).
> 
> **Note**:  Some implementations have values known as __uninterned symbols__,
> which defeat write/read invariance, and also violate the rule that two
> symbols are the same if and only if their names are spelled the same.
> This report does not specify the behavior of
> implementation-dependent extensions.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__symbol__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);


<a id='type__r7rs__symbol__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

