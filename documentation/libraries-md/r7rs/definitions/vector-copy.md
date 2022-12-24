

<a id='definition__r7rs__vector-copy'></a>

# `vector-copy` -- `r7rs` Definition


<a id='definition__r7rs__vector-copy__kind'></a>

#### Kind

`accessor`;


<a id='definition__r7rs__vector-copy__implemented-by'></a>

#### Implemented by

 * [`vector-copy`](../../vonuvoli/definitions/vector-copy.md#definition__vonuvoli__vector-copy) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__vector-copy__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((vector) -> (vector))`
   * input: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * `((vector range-start) -> (vector))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * `((vector range-start range-end) -> (vector))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


<a id='definition__r7rs__vector-copy__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-copy__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-copy__description'></a>

#### Description

> ````
> (vector-copy vector)
> (vector-copy vector start)
> (vector-copy vector start end)
> ````
> 
> 
> Returns a newly allocated copy of the elements of the given `vector`
> between `start` and `end`.
> The elements of the new vector are the same (in the sense of
> `eqv?`) as the elements of the old.
> 
> 
> ````
> (define a #(1 8 2 8)) ; a may be immutable
> (define b (vector-copy a))
> (vector-set! b 0 3)   ; b is mutable
> b  ===>  #(3 8 2 8)
> (define c (vector-copy b 1 3))
> c  ===>  #(8 2)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__vector-copy__referenced-types'></a>

#### Referenced-types

 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

