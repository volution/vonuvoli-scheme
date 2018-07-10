

<a id='definition__r7rs__floor'></a>

# `floor` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|number-not-inf-not-nan|) |->| (|integer|))`
   * input: a value of type [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
   * output: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * `((|number-inf|) |->| (|number-inf|))`
   * input: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
   * output: a value of type [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * `((|number-nan|) |->| (|number-nan|))`
   * input: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
   * output: a value of type [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


#### Referenced types

[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
[`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:arithmetic`](../../r7rs/categories/vs_3a_arithmetic.md#category__r7rs__vs_3a_arithmetic);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

