

<a id='definition__r7rs__vector-copy'></a>

# `vector-copy` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|vector|) |->| (|vector|))`
   * input: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * `((|vector| |range-start|) |->| (|vector|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * `((|vector| |range-start| |range-end|) |->| (|vector|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


#### Referenced types

[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:vectors`](../../r7rs/categories/vs_3a_vectors.md#category__r7rs__vs_3a_vectors);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

