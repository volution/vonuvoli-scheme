

<a id='definition__r7rs__boolean_3d_3f'></a>

# `boolean=?` -- `r7rs` Definition


<a id='definition__r7rs__boolean_3d_3f__kind'></a>

#### Kind

`comparator`;


<a id='definition__r7rs__boolean_3d_3f__extended-by'></a>

#### Extended by

 * [`boolean=?`](../../vonuvoli/definitions/boolean_3d_3f.md#definition__vonuvoli__boolean_3d_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__boolean_3d_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((boolean |2...|) -> (boolean))`
   * inputs:
     * a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
     * `...` -- at least 2 times;
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

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

