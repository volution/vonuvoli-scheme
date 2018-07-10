

<a id='definition__r7rs__make-list'></a>

# `make-list` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `((|range-length-zero|) |->| (|null|))`
   * input: a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|range-length-zero| |any|) |->| (|null|))`
   * inputs:
     * a value of type [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|range-length-not-zero|) |->| (|list-proper-not-null|))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);
 * `((|range-length-not-zero| |any|) |->| (|list-proper-not-null|))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


#### Referenced types

[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
[`list-proper-not-null`](../../r7rs/types/list-proper-not-null.md#type__r7rs__list-proper-not-null);


#### Description

> ````
> (make-list k)
> (make-list k fill)
> ````
> 
> 
> Returns a newly allocated list of `k` elements.  If a second
> argument is given, then each element is initialized to `fill`.
> Otherwise the initial contents of each element is unspecified.
> 
> ````
> (make-list 2 3)   ===>   (3 3)
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

