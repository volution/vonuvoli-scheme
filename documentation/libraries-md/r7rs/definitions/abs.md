

<a id='definition__r7rs__abs'></a>

# `abs` -- `r7rs` Definition


<a id='definition__r7rs__abs__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__abs__implemented-by'></a>

#### Implemented by

 * [`abs`](../../vonuvoli/definitions/abs.md#definition__vonuvoli__abs) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__abs__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-zero) -> (real-zero))`
   * input: a value of type [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
   * output: a value of type [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * `((real-positive) -> (real-positive))`
   * input: a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
   * output: a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * `((real-negative) -> (real-positive))`
   * input: a value of type [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
   * output: a value of type [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * `((real) -> (real-nan))`
   * input: a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
   * output: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


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

 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

