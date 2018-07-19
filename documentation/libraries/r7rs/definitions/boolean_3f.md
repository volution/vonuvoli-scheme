

<a id='definition__r7rs__boolean_3f'></a>

# `boolean?` -- `r7rs` Definition


<a id='definition__r7rs__boolean_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__boolean_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((boolean) -> (true))`
   * input: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__boolean_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__boolean_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__boolean_3f__description'></a>

#### Description

> ````
> (boolean? obj)
> ````
> 
> 
> The `boolean?` predicate returns `#t` if `obj` is either `#t` or
> `#f` and returns `#f` otherwise.
> 
> ````
> (boolean? #f)         ===>  #t
> (boolean? 0)          ===>  #f
> (boolean? '())        ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__boolean_3f__referenced-types'></a>

#### Referenced-types

 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__boolean_3f__categories'></a>

#### Categories

 * [`vs:booleans`](../../vonuvoli/categories/vs_3a_booleans.md#category__vonuvoli__vs_3a_booleans);
 * [`vs:types`](../../vonuvoli/categories/vs_3a_types.md#category__vonuvoli__vs_3a_types);


<a id='definition__r7rs__boolean_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

