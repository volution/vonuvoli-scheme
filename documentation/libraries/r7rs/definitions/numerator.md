

<a id='definition__r7rs__numerator'></a>

# `numerator` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|integer|) |->| (|integer|))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((|rational|) |->| (|integer|))`
   * input: a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);


#### Referenced types

[`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


#### Description

> ````
> (numerator q)
> (denominator q)
> ````
> 
> 
> These procedures return the numerator or denominator of their
> argument; the result is computed as if the argument was represented as
> a fraction in lowest terms.  The denominator is always positive.  The
> denominator of `0` is defined to be `1`.
> 
> ````
> (numerator (/ 6 4))    ===>  3
> (denominator (/ 6 4))  ===>  2
> (denominator
>   (inexact (/ 6 4)))   ===>  2.0
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

