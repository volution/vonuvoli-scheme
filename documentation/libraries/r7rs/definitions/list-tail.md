

<a id='definition__r7rs__list-tail'></a>

# `list-tail` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|list| |range-offset|) |->| (|list|))`
   * inputs:
     * a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);
     * a value of type [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
   * output: a value of type [`list`](../../r7rs/types/list.md#type__r7rs__list);


#### Referenced types

[`list`](../../r7rs/types/list.md#type__r7rs__list);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);


#### Description

> ````
> (list-tail list k)
> ````
> 
> 
> **Domain**:  It is an error if `list` has fewer than `k` elements.
> 
> Returns the sublist of `list` obtained by omitting the first `k`
> elements.
> The `list-tail` procedure could be defined by
> 
> ````
> (define list-tail
>   (lambda (x k)
>     (if (zero? k)
>         x
>         (list-tail (cdr x) (- k 1)))))
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

