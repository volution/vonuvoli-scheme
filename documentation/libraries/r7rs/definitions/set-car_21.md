

<a id='definition__r7rs__set-car_21'></a>

# `set-car!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|pair| |any|) |->| (|undefined|))`
   * inputs:
     * a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|pair| |any|) |->| (|any|))`
   * inputs:
     * a value of type [`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * requires: `|vonuvoli|`


#### Referenced types

[`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


#### Description

> ````
> (set-car! pair obj)
> ````
> 
> 
> Stores `obj` in the car field of `pair`.
> ````
> (define (f) (list 'not-a-constant-list))
> (define (g) '(constant-list))
> (set-car! (f) 3)             ===>  #unspecified
> (set-car! (g) 3)             ===>  #error
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

