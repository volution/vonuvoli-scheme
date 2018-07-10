

<a id='definition__r7rs__vector-length'></a>

# `vector-length` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|vector-empty|) |->| (|range-length-zero|))`
   * input: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
   * output: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * `((|vector-not-empty|) |->| (|range-length-not-zero|))`
   * input: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
   * output: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Referenced types

[`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Description

> ````
> (vector-length vector)
> ````
> 
> 
> Returns the number of elements in `vector` as an exact integer.
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

