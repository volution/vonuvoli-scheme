

<a id='definition__r7rs__min'></a>

# `min` -- `r7rs` Definition


<a id='definition__r7rs__min__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__min__implemented-by'></a>

#### Implemented by

 * [`min`](../../vonuvoli/definitions/min.md#definition__vonuvoli__min) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__min__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((x real-not-nan)) -> ((x real-not-nan)))`
   * input: `x` of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
   * output: `x` of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real-not-nan |2...|) -> (real-not-nan))`
   * inputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * `...` -- at least 2 times;
   * output: a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real |1...|) -> (real-nan))`
   * inputs:
     * a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
     * `...` -- at least one time;
   * output: a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


<a id='definition__r7rs__min__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__min__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__min__description'></a>

#### Description

> ````
> (max x_1 x_2 ...)
> (min x_1 x_2 ...)
> ````
> 
> 
> These procedures return the maximum or minimum of their arguments.
> 
> ````
> (max 3 4)              ===>  4    ; exact
> (max 3.9 4)            ===>  4.0  ; inexact
> ````
> 
> **Note**:  If any argument is inexact, then the result will also be inexact (unless
> the procedure can prove that the inaccuracy is not large enough to affect the
> result, which is possible only in unusual implementations).  If `min` or
> `max` is used to compare numbers of mixed exactness, and the numerical
> value of the result cannot be represented as an inexact number without loss of
> accuracy, then the procedure may report a violation of an implementation
> restriction.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__min__referenced-types'></a>

#### Referenced-types

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

