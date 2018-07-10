

<a id='definition__r7rs__car'></a>

# `car` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|pair|) |->| (|any|))`
   * input: a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (car pair)
> ````
> 
> 
> Returns the contents of the car field of `pair`.  Note that it is an
> error to take the car of the __empty list__.
> 
> ````
> (car '(a b c))          ===>  a
> (car '((a) b c d))      ===>  (a)
> (car '(1 . 2))          ===>  1
> (car '())               ===>  #error
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:pairs`](../../r7rs/categories/vs_3a_pairs.md#category__r7rs__vs_3a_pairs);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

