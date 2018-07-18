

<a id='definition__r7rs__abs'></a>

# `abs` -- `r7rs` Definition


<a id='definition__r7rs__abs__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__abs__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-zero) -> (number-zero))`
   * input: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
   * output: a value of type [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * `((number-positive) -> (number-positive))`
   * input: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
   * output: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * `((number-negative) -> (number-positive))`
   * input: a value of type [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
   * output: a value of type [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * `((number-nan) -> (number-nan))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__abs__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__abs__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__abs__description'></a>

#### Description

> ````
> (abs x)
> ````
> 
> 
> The `abs` procedure returns the absolute value of its argument.
> ````
> (abs -7)                ===>  7
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__abs__referenced-types'></a>

#### Referenced-types

 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__abs__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);


<a id='definition__r7rs__abs__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

