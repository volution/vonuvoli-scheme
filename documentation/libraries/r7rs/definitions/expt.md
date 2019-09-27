

<a id='definition__r7rs__expt'></a>

# `expt` -- `r7rs` Definition


<a id='definition__r7rs__expt__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__expt__implemented-by'></a>

#### Implemented by

 * [`expt`](../../vonuvoli/definitions/expt.md#definition__vonuvoli__expt) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__expt__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-nan real-not-nan) -> (real-not-nan))`
   * inputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
   * output: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((complex-not-nan complex-not-nan) -> (complex-not-nan))`
   * inputs:
     * a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
     * a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
   * output: a value of type [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
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

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

