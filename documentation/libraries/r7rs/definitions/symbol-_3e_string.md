

<a id='definition__r7rs__symbol-_3e_string'></a>

# `symbol->string` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|symbol|) |->| (|string|))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


#### Referenced types

[`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
[`string`](../../r7rs/types/string.md#type__r7rs__string);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:symbols`](../../r7rs/categories/vs_3a_symbols.md#category__r7rs__vs_3a_symbols);
[`vs:conversions`](../../r7rs/categories/vs_3a_conversions.md#category__r7rs__vs_3a_conversions);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

