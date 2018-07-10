

<a id='definition__r7rs__list'></a>

# `list` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|null|))`
   * inputs: none;
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|any| |...|) |->| (|list-proper|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


#### Referenced types

[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


#### Description

> ````
> (list obj ...)
> ````
> 
> 
> Returns a newly allocated list of its arguments.
> 
> ````
> (list 'a (+ 3 4) 'c)            ===>  (a 7 c)
> (list)                          ===>  ()
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

