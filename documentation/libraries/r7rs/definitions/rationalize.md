

<a id='definition__r7rs__rationalize'></a>

# `rationalize` -- `r7rs` Definition


<a id='definition__r7rs__rationalize__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__rationalize__implemented-by'></a>

#### Implemented by

 * [`rationalize`](../../vonuvoli/definitions/rationalize.md#definition__vonuvoli__rationalize) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__rationalize__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-inf-not-nan real-positive-or-zero-not-inf) -> (rational))`
   * inputs:
     * a value of type [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
     * a value of type [`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
   * output: a value of type [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


<a id='definition__r7rs__rationalize__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__rationalize__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__rationalize__description'></a>

#### Description

> ````
> (rationalize x y)
> ````
> 
> 
> The `rationalize` procedure returns the __simplest__ rational number
> differing from `x` by no more than `y`.  A rational number `r_1` is
> __simpler__ (simplest rational) than another rational number
> `r_2` if `r_1 = p_1/q_1` and `r_2 = p_2/q_2` (in lowest terms) and
> `|p_1| <= |p_2|` and `|q_1| <= |q_2|`.  Thus `3/5` is simpler than `4/7`.
> Although not all rationals are comparable in this ordering (consider `2/7`
> and `3/5`), any interval contains a rational number that is simpler than
> every other rational number in that interval (the simpler `2/5` lies
> between `2/7` and `3/5`).  Note that `0 = 0/1` is the simplest rational of
> all.
> 
> ````
> (rationalize
>   (exact .3) 1/10)           ===>  1/3    ; exact
> (rationalize .3 1/10)        ===>  #i1/3  ; inexact
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__rationalize__referenced-types'></a>

#### Referenced-types

 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

