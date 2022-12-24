

<a id='definition__r7rs__inexact'></a>

# `inexact` -- `r7rs` Definition


<a id='definition__r7rs__inexact__kind'></a>

#### Kind

`converter`;


<a id='definition__r7rs__inexact__implemented-by'></a>

#### Implemented by

 * [`inexact`](../../vonuvoli/definitions/inexact.md#definition__vonuvoli__inexact) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__inexact__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((number) -> (inexact-number))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);


<a id='definition__r7rs__inexact__exports'></a>

#### Exports

 * [`scheme:complex`](../../r7rs/exports/scheme_3a_complex.md#export__r7rs__scheme_3a_complex) -- `(scheme complex)`;


<a id='definition__r7rs__inexact__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__inexact__description'></a>

#### Description

> ````
> (inexact z)
> (exact z)
> ````
> 
> 
> The procedure `inexact` returns an __inexact__ representation of `z`.
> The value returned is the
> __inexact__ number that is numerically closest to the argument.
> For inexact arguments, the result is the same as the argument. For exact
> complex numbers, the result is a complex number whose real and imaginary
> parts are the result of applying `inexact` to the real
> and imaginary parts of the argument, respectively.
> If an __exact__ argument has no reasonably close __inexact__ equivalent
> (in the sense of `=`),
> then a violation of an implementation restriction may be reported.
> 
> The procedure `exact` returns an __exact__ representation of
> `z`.  The value returned is the __exact__ number that is numerically
> closest to the argument.
> For exact arguments, the result is the same as the argument. For inexact
> non-integral real arguments, the implementation may return a rational
> approximation, or may report an implementation violation. For inexact
> complex arguments, the result is a complex number whose real and
> imaginary parts are the result of applying `exact` to the
> real and imaginary parts of the argument, respectively.
> If an __inexact__ argument has no reasonably close __exact__ equivalent,
> (in the sense of `=`),
> then a violation of an implementation restriction may be reported.
> 
> These procedures implement the natural one-to-one correspondence between
> __exact__ and __inexact__ integers throughout an
> implementation-dependent range.  See section on restrictions.
> 
> **Note**:  These procedures were known in __R5RS__ as `exact->inexact` and
> `inexact->exact`, respectively, but they have always accepted
> arguments of any exactness.  The new names are clearer and shorter,
> as well as being compatible with __R6RS__.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__inexact__referenced-types'></a>

#### Referenced-types

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

