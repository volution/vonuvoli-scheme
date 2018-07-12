

<a id='definition__r7rs__symbol-_3e_string'></a>

# `symbol->string` -- `r7rs` Definitions


<a id='definition__r7rs__symbol-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__symbol-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((symbol) -> (string))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__symbol-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__symbol-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__symbol-_3e_string__description'></a>

#### Description

> ````
> (symbol->string symbol)
> ````
> 
> 
> Returns the name of `symbol` as a string, but without adding escapes.
> It is an error
> to apply mutation procedures like `string-set!` to strings returned
> by this procedure.
> 
> ````
> (symbol->string 'flying-fish)
>                                   ===>  "flying-fish"
> (symbol->string 'Martin)          ===>  "Martin"
> (symbol->string
>    (string->symbol "Malvina"))
>                                   ===>  "Malvina"
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__symbol-_3e_string__referenced-types'></a>

#### Referenced-types

 * [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
 * [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__symbol-_3e_string__categories'></a>

#### Categories

 * [`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
 * [`vs:symbols`](../../r7rs/categories/vs_3a_symbols.md#category__r7rs__vs_3a_symbols);
 * [`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);


<a id='definition__r7rs__symbol-_3e_string__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

