

<a id='definition__r7rs__symbol-_3e_string'></a>

# `symbol->string` -- `r7rs` Definition


<a id='definition__r7rs__symbol-_3e_string__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__symbol-_3e_string__implemented-by'></a>

#### Implemented by

 * [`symbol->string`](../../vonuvoli/definitions/symbol-_3e_string.md#definition__vonuvoli__symbol-_3e_string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__symbol-_3e_string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((symbol) -> (string))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`string`](../../r7rs/types/string.md#type__r7rs__string);


<a id='definition__r7rs__symbol-_3e_string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__symbol-_3e_string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


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

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

