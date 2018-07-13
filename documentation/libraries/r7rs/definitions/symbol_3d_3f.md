

<a id='definition__r7rs__symbol_3d_3f'></a>

# `symbol=?` -- `r7rs` Definitions


<a id='definition__r7rs__symbol_3d_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__symbol_3d_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((symbol) -> (true))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((symbol ...) -> (boolean))`
   * inputs:
     * a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__symbol_3d_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__symbol_3d_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__symbol_3d_3f__description'></a>

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


<a id='definition__r7rs__symbol_3d_3f__referenced-types'></a>

#### Referenced-types

 * [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__symbol_3d_3f__categories'></a>

#### Categories

 * [`vs:symbols`](../../r7rs/categories/vs_3a_symbols.md#category__r7rs__vs_3a_symbols);
 * [`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
 * [`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);


<a id='definition__r7rs__symbol_3d_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

