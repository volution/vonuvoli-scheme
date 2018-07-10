

<a id='definition__r7rs__boolean_3d_3f'></a>

# `boolean=?` -- `r7rs` Definitions


#### Kind

`comparator`;


#### Procedure signature

Procedure variants:
 * `((|boolean|) |->| (|true|))`
   * input: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|boolean| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Referenced types

[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
[`true`](../../r7rs/types/true.md#type__r7rs__true);


#### Description

> ````
> (boolean=? boolean_1 boolean_2 boolean_3 ...)
> ````
> 
> 
> Returns `#t` if all the arguments are booleans and all
> are `#t` or all are `#f`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:booleans`](../../r7rs/categories/vs_3a_booleans.md#category__r7rs__vs_3a_booleans);
[`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
[`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

