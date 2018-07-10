

<a id='definition__r7rs__vector'></a>

# `vector` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|vector-empty|))`
   * inputs: none;
   * output: a value of type [`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
 * `((|any| |...|) |->| (|vector-not-empty|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


#### Referenced types

[`vector-empty`](../../r7rs/types/vector-empty.md#type__r7rs__vector-empty);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`vector-not-empty`](../../r7rs/types/vector-not-empty.md#type__r7rs__vector-not-empty);


#### Description

> ````
> (vector obj ...)
> ````
> 
> 
> Returns a newly allocated vector whose elements contain the given
> arguments.  It is analogous to `list`.
> 
> ````
> (vector 'a 'b 'c)               ===>  #(a b c)
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

