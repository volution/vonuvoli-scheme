

<a id='definition__r7rs__symbol_3d_3f'></a>

# `symbol=?` -- `r7rs` Definitions


#### Kind

`comparator`;


#### Procedure signature

Procedure variants:
 * `((|symbol|) |->| (|true|))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|symbol| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Referenced types

[`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (symbol=? symbol_1 symbol_2 symbol_3 ...)
> ````
> 
> 
> Returns `#t` if all the arguments are symbols and all have the same
> names in the sense of `string=?`.
> 
> **Note**:  The definition above assumes that none of the arguments
> are uninterned symbols.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:symbols`](../../r7rs/categories/vs_3a_symbols.md#category__r7rs__vs_3a_symbols);
[`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
[`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

