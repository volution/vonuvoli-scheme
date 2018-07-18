

<a id='definition__r7rs__vector-copy_21'></a>

# `vector-copy!` -- `r7rs` Definition


<a id='definition__r7rs__vector-copy_21__kind'></a>

#### Kind

`mutator!`;


<a id='definition__r7rs__vector-copy_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((source . vector) (source-start . range-start) (destination . vector)) -> (void))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((source . vector) (source-start . range-start) (destination . vector) (destination-start . range-start)) -> (void))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `(((source . vector) (source-start . range-start) (destination . vector) (destination-start . range-start) (destination-end . range-end)) -> (void))`
   * inputs:
     * `source` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination-end` of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


<a id='definition__r7rs__vector-copy_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__vector-copy_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__vector-copy_21__description'></a>

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


<a id='definition__r7rs__vector-copy_21__referenced-types'></a>

#### Referenced-types

 * [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


<a id='definition__r7rs__vector-copy_21__categories'></a>

#### Categories

 * [`vs:vectors`](../../r7rs/categories/vs_3a_vectors.md#category__r7rs__vs_3a_vectors);


<a id='definition__r7rs__vector-copy_21__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

