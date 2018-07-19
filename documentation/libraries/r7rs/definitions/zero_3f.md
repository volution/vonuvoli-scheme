

<a id='definition__r7rs__zero_3f'></a>

# `zero?` -- `r7rs` Definition


<a id='definition__r7rs__zero_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__zero_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-zero) -> (true))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((number) -> (false))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number ...) -> (boolean))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__zero_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__zero_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__zero_3f__description'></a>

#### Description

> ````
> (zero? z)
> (positive? x)
> (negative? x)
> (odd? n)
> (even? n)
> ````
> 
> 
> These numerical predicates test a number for a particular property,
> returning `#t` or `#f`.  See note above.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__zero_3f__referenced-types'></a>

#### Referenced-types

 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__zero_3f__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);


<a id='definition__r7rs__zero_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

