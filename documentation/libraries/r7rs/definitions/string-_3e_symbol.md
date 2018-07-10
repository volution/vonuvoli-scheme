

<a id='definition__r7rs__string-_3e_symbol'></a>

# `string->symbol` -- `r7rs` Definitions


#### Kind

`converter`;


#### Procedure signature

Procedure variants:
 * `((|string-empty|) |->| (|symbol|))`
   * input: a value of type [`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
   * output: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
 * `((|string-not-empty|) |->| (|symbol|))`
   * input: a value of type [`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);
   * output: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);


#### Referenced types

[`string-empty`](../../r7rs/types/string-empty.md#type__r7rs__string-empty);
[`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
[`string-not-empty`](../../r7rs/types/string-not-empty.md#type__r7rs__string-not-empty);


#### Description

> ````
> (string->symbol string)
> ````
> 
> 
> Returns the symbol whose name is `string`.  This procedure can
> create symbols with names containing special characters that would
> require escaping when written, but does not interpret escapes in its input.
> 
> ````
> (string->symbol "mISSISSIppi")                    ===>  mISSISSIppi
> (eqv? 'bitBlt (string->symbol "bitBlt"))          ===>  #t
> (eqv? 'LollyPop
>      (string->symbol
>        (symbol->string 'LollyPop)))               ===>  #t
> (string=? "K. Harper, M.D."
>           (symbol->string
>             (string->symbol "K. Harper, M.D.")))  ===>  #t
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

