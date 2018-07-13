

<a id='definition__r7rs__boolean_3d_3f'></a>

# `boolean=?` -- `r7rs` Definitions


<a id='definition__r7rs__boolean_3d_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__boolean_3d_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((boolean) -> (true))`
   * input: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((boolean ...) -> (boolean))`
   * inputs:
     * a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__boolean_3d_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__boolean_3d_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__boolean_3d_3f__description'></a>

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


<a id='definition__r7rs__boolean_3d_3f__referenced-types'></a>

#### Referenced-types

 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);


<a id='definition__r7rs__boolean_3d_3f__categories'></a>

#### Categories

 * [`vs:booleans`](../../r7rs/categories/vs_3a_booleans.md#category__r7rs__vs_3a_booleans);
 * [`vs:comparisons`](../../r7rs/categories/vs_3a_comparisons.md#category__r7rs__vs_3a_comparisons);
 * [`vs:equivalence`](../../r7rs/categories/vs_3a_equivalence.md#category__r7rs__vs_3a_equivalence);


<a id='definition__r7rs__boolean_3d_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

