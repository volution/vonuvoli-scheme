

<a id='definition__r7rs__exact-integer_3f'></a>

# `exact-integer?` -- `r7rs` Definitions


<a id='definition__r7rs__exact-integer_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__exact-integer_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((exact-integer) -> (true))`
   * input: a value of type [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((exact-number) -> (false))`
   * input: a value of type [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((inexact-number) -> (false))`
   * input: a value of type [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number) -> (false))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number ...) -> (boolean))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__exact-integer_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__exact-integer_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__exact-integer_3f__description'></a>

#### Description

> ````
> (exact-integer? z)
> ````
> 
> 
> Returns `#t` if `z` is both __exact__ and an __integer__;
> otherwise returns `#f`.
> 
> ````
> (exact-integer? 32)    ===>  #t
> (exact-integer? 32.0)  ===>  #f
> (exact-integer? 32/5)  ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__exact-integer_3f__referenced-types'></a>

#### Referenced-types

 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__exact-integer_3f__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
 * [`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);


<a id='definition__r7rs__exact-integer_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

