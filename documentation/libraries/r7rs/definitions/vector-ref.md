

<a id='definition__r7rs__vector-ref'></a>

# `vector-ref` -- `r7rs` Definitions


<a id='definition__r7rs__vector-ref__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__vector-ref__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector range-offset) -> (any))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
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

 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__vector-ref__categories'></a>

#### Categories

 * [`vs:vectors`](../../r7rs/categories/vs_3a_vectors.md#category__r7rs__vs_3a_vectors);


<a id='definition__r7rs__vector-ref__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

