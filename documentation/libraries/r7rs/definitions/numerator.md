

<a id='definition__r7rs__numerator'></a>

# `numerator` -- `r7rs` Definition


<a id='definition__r7rs__numerator__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__numerator__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer) -> (integer))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((rational) -> (integer))`
   * input: a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);


<a id='definition__r7rs__numerator__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__numerator__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__numerator__description'></a>

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


<a id='definition__r7rs__numerator__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


<a id='definition__r7rs__numerator__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);
 * [`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);


<a id='definition__r7rs__numerator__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

