

<a id='definition__r7rs__vector-copy_21'></a>

# `vector-copy!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `(((|source| . |vector|) (|source-start| . |range-start|) (|destination| . |vector|)) |->| (|void|))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((|source| . |vector|) (|source-start| . |range-start|) (|destination| . |vector|) (|destination-start| . |range-start|)) |->| (|void|))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((|source| . |vector|) (|source-start| . |range-start|) (|destination| . |vector|) (|destination-start| . |range-start|) (|destination-end| . |range-end|)) |->| (|void|))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination-end` of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (vector-copy! to at from)
> (vector-copy! to at from start)
> (vector-copy! to at from start end)
> ````
> 
> 
> **Domain**:  It is an error if `at` is less than zero or greater than the length of `to`.
> It is also an error if `(- (vector-length to) at)`
> is less than `(- end start)`.
> 
> Copies the elements of vector `from` between `start` and `end`
> to vector `to`, starting at `at`.  The order in which elements are
> copied is unspecified, except that if the source and destination overlap,
> copying takes place as if the source is first copied into a temporary
> vector and then into the destination.  This can be achieved without
> allocating storage by making sure to copy in the correct direction in
> such circumstances.
> 
> ````
> (define a (vector 1 2 3 4 5))
> (define b (vector 10 20 30 40 50))
> (vector-copy! b 1 a 0 2)
> b  ===>  #(10 1 2 40 50)
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

