

<a id='definition__r7rs__vector-ref'></a>

# `vector-ref` -- `r7rs` Definition


<a id='definition__r7rs__vector-ref__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__vector-ref__implemented-by'></a>

#### Implemented by

 * [`vector-ref`](../../vonuvoli/definitions/vector-ref.md#definition__vonuvoli__vector-ref) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-ref__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector-not-empty range-offset) -> (any))`
   * inputs:
     * a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__vector-ref__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-ref__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-ref__description'></a>

#### Description

> ````
> (vector-ref vector k)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `vector`.
> 
> The `vector-ref` procedure returns the contents of element `k` of
> `vector`.
> 
> ````
> (vector-ref '#(1 1 2 3 5 8 13 21)
>             5)                          ===>  8
> (vector-ref '#(1 1 2 3 5 8 13 21)
>             (exact
>              (round (* 2 (acos -1)))))  ===>  13
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-ref__referenced-types'></a>

#### Referenced-types

 * [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

