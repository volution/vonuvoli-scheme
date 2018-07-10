

<a id='definition__r7rs__exact_3f'></a>

# `exact?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|exact-number|) |->| (|true|))`
   * input: a value of type [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|inexact-number|) |->| (|false|))`
   * input: a value of type [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number|) |->| (|false|))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|number| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (exact? z)
> (inexact? z)
> ````
> 
> 
> These numerical predicates provide tests for the exactness of a
> quantity.  For any Scheme number, precisely one of these predicates
> is true.
> 
> ````
> (exact? 3.0)           ===>  #f
> (exact? #e3.0)         ===>  #t
> (inexact? 3.)          ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

