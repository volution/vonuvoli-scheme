

<a id='definition__r7rs__expt'></a>

# `expt` -- `r7rs` Definition


<a id='definition__r7rs__expt__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__expt__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number-not-nan number-not-nan) -> (number-not-nan))`
   * inputs:
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
     * a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
   * output: a value of type [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * `((number number) -> (number-nan))`
   * inputs:
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
     * a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__expt__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__expt__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__expt__description'></a>

#### Description

> ````
> (expt z_1 z_2)
> ````
> 
> 
> Returns `z_1` raised to the power `z_2`.  For nonzero `z_1`, this is
> `z_1^z_2 = e^(z_2 log z_1)`
> The value of `0^z` is `1` if `(zero? z)`, `0` if `(real-part z)`
> is positive, and an error otherwise.  Similarly for `0.0^z`,
> with inexact results.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__expt__referenced-types'></a>

#### Referenced-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


<a id='definition__r7rs__expt__categories'></a>

#### Categories

 * [`vs:arithmetic`](../../vonuvoli/categories/vs_3a_arithmetic.md#category__vonuvoli__vs_3a_arithmetic);


<a id='definition__r7rs__expt__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../vonuvoli/categories/vs.md#category__vonuvoli__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

