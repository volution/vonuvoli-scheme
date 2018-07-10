

<a id='definition__r7rs__list-set_21'></a>

# `list-set!` -- `r7rs` Definitions


#### Kind

`mutator!`;


#### Procedure signature

Procedure variants:
 * `((|list| |range-offset|) |->| (|undefined|))`
   * inputs:
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|list| |range-offset|) |->| (|any|))`
   * inputs:
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * requires: `|vonuvoli|`


#### Referenced types

[`list`](../../r7rs/types/list.md#type__r7rs__list);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (list-set! list k obj)
> ````
> 
> 
> **Domain**:  It is an error if `k` is not a valid index of `list`.
> 
> The `list-set!` procedure stores `obj` in element `k` of `list`.
> 
> ````
> (let ((ls (list 'one 'two 'five!)))
>   (list-set! ls 2 'three)
>   ls)      ===>  (one two three)
> 
> (list-set! '(0 1 2) 1 "oops")  ===>  #error  ; constant list
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

