

<a id='definition__r7rs__vector-set_21'></a>

# `vector-set!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|vector| |range-offset| |any|) |->| (|undefined|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|vector| |range-offset| |any|) |->| (|any|))`
   * inputs:
     * a value of type [`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * requires: `|vonuvoli|`


#### Referenced types

[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


#### Description

> ````
> (vector-set! vector k obj)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `vector`.
> 
> The `vector-set!` procedure stores `obj` in element `k` of `vector`.
> ````
> (let ((vec (vector 0 '(2 2 2 2) "Anna")))
>   (vector-set! vec 1 '("Sue" "Sue"))
>   vec)      ===>  #(0 ("Sue" "Sue") "Anna")
> 
> (vector-set! '#(0 1 2) 1 "doe")  ===>  #error  ; constant vector
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

