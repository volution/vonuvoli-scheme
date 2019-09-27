

<a id='definition__r7rs__floor'></a>

# `floor` -- `r7rs` Definition


<a id='definition__r7rs__floor__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__floor__implemented-by'></a>

#### Implemented by

 * [`floor`](../../vonuvoli/definitions/floor.md#definition__vonuvoli__floor) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__floor__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((n integer)) -> ((n integer)))`
   * input: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: `n` of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((real-not-inf-not-nan) -> (integer))`
   * input: a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((real-inf) -> (real-inf))`
   * input: a value of type [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
   * output: a value of type [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * `((real-nan) -> (real-nan))`
   * input: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
   * output: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


<a id='definition__r7rs__floor__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__floor__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__floor__description'></a>

#### Description

> ````
> (floor x)
> (ceiling x)
> (truncate x)
> (round x)
> ````
> 
> 
> These procedures return integers.
> 
> The `floor` procedure returns the largest integer not larger than `x`.
> The `ceiling` procedure returns the smallest integer not smaller than `x`,
> `truncate` returns the integer closest to `x` whose absolute
> value is not larger than the absolute value of `x`, and `round` returns the
> closest integer to `x`, rounding to even when `x` is halfway between two
> integers.
> 
> **Rationale**:  The `round` procedure rounds to even for consistency with the default rounding
> mode specified by the IEEE 754 IEEE floating-point standard.
> 
> **Note**:  If the argument to one of these procedures is inexact, then the result
> will also be inexact.  If an exact value is needed, the
> result can be passed to the `exact` procedure.
> If the argument is infinite or a `NaN`, then it is returned.
> 
> 
> ````
> (floor -4.3)          ===>  -5.0
> (ceiling -4.3)        ===>  -4.0
> (truncate -4.3)       ===>  -4.0
> (round -4.3)          ===>  -4.0
> 
> (floor 3.5)           ===>   3.0
> (ceiling 3.5)         ===>   4.0
> (truncate 3.5)        ===>   3.0
> (round 3.5)           ===>   4.0  ; inexact
> 
> (round 7/2)           ===>   4    ; exact
> (round 7)             ===>   7
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__floor__referenced-types'></a>

#### Referenced-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

