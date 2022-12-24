

<a id='definition__r7rs__exact_3f'></a>

# `exact?` -- `r7rs` Definition


<a id='definition__r7rs__exact_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__exact_3f__extended-by'></a>

#### Extended by

 * [`exact?`](../../vonuvoli/definitions/exact_3f.md#definition__vonuvoli__exact_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__exact_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((exact-number) -> (true))`
   * input: a value of type [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((inexact-number) -> (false))`
   * input: a value of type [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((number) -> (false))`
   * input: a value of type [`number`](../../r7rs/types/number.md#type__r7rs__number);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__exact_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__exact_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__exact_3f__description'></a>

#### Description

> ````
> (exact? z)
> (inexact? z)
> ````
> 
> 
> These numerical predicates provide tests for the exactness of a
> quantity.  For any Scheme number, precisely one of these predicates
> is true.
> 
> ````
> (exact? 3.0)           ===>  #f
> (exact? #e3.0)         ===>  #t
> (inexact? 3.)          ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__exact_3f__referenced-types'></a>

#### Referenced-types

 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

