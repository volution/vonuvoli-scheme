

<a id='definition__r7rs__vector-fill_21'></a>

# `vector-fill!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|vector| |any|) |->| (|void|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|vector| |any| |range-start|) |->| (|void|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((|vector| |any| |range-start| |range-end|) |->| (|void|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * a value of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * a value of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


#### Referenced types

[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);


#### Description

> ````
> (vector-fill! vector fill)
> (vector-fill! vector fill start)
> (vector-fill! vector fill start end)
> ````
> 
> 
> The `vector-fill!` procedure stores `fill`
> in the elements of `vector`
> between `start` and `end`.
> 
> ````
> (define a (vector 1 2 3 4 5))
> (vector-fill! a 'smash 2 4)
> a  ===>  #(1 2 smash smash 5)
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

