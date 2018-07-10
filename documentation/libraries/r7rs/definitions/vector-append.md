

<a id='definition__r7rs__vector-append'></a>

# `vector-append` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|vector-empty|))`
   * inputs: none;
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((|vector| |...|) |->| (|vector|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * `...` (i.e. variadic);
   * output: a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


#### Referenced types

[`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);


#### Description

> ````
> (vector-append vector ...)
> ````
> 
> 
> Returns a newly allocated vector whose elements are the concatenation
> of the elements of the given vectors.
> 
> ````
> (vector-append #(a b c) #(d e f))  ===>  #(a b c d e f)
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

