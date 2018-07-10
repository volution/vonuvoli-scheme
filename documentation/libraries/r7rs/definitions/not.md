

<a id='definition__r7rs__not'></a>

# `not` -- `r7rs` Definitions


#### Kind

`predicate`;


#### Procedure signature

Procedure variants:
 * `((|true|) |->| (|false|))`
   * input: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|false|) |->| (|true|))`
   * input: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


#### Referenced types

[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (not obj)
> ````
> 
> 
> The `not` procedure returns `#t` if `obj` is false, and returns
> `#f` otherwise.
> 
> ````
> (not #t)         ===>  #f
> (not 3)          ===>  #f
> (not (list 3))   ===>  #f
> (not #f)         ===>  #t
> (not '())        ===>  #f
> (not (list))     ===>  #f
> (not 'nil)       ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

