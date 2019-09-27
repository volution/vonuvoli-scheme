

<a id='definition__r7rs__square'></a>

# `square` -- `r7rs` Definition


<a id='definition__r7rs__square__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__square__implemented-by'></a>

#### Implemented by

 * [`square`](../../vonuvoli/definitions/square.md#definition__vonuvoli__square) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__square__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-nan) -> (real-positive-or-zero))`
   * input: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
   * output: a value of type [`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero);
 * `((complex-not-nan) -> (complex-not-nan))`
   * input: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * `((number) -> (number-nan))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__square__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__square__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__square__description'></a>

#### Description

> ````
> (square z)
> ````
> 
> 
> Returns the square of `z`.
> This is equivalent to `(* z z)`.
> 
> ````
> (square 42)       ===>  1764
> (square 2.0)      ===>  4.0
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__square__referenced-types'></a>

#### Referenced-types

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

