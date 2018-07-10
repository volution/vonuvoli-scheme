

<a id='definition__r7rs__list-copy'></a>

# `list-copy` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|null|) |->| (|null|))`
   * input: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
   * output: a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * `((|list-not-circular|) |->| (|list-not-circular|))`
   * input: a value of type [`list-not-circular`](../../r7rs/types/list-not-circular.md#type__r7rs__list-not-circular);
   * output: a value of type [`list-not-circular`](../../r7rs/types/list-not-circular.md#type__r7rs__list-not-circular);
 * `((|list-circular|) |->| (|exception|))`
   * input: a value of type [`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
   * output: a value of type [`exception`](../../r7rs/types/exception.md#type__r7rs__exception);
 * `((|any|) |->| (|any|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`list-not-circular`](../../r7rs/types/list-not-circular.md#type__r7rs__list-not-circular);
[`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
[`exception`](../../r7rs/types/exception.md#type__r7rs__exception);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (list-copy obj)
> ````
> 
> 
> Returns a newly allocated copy of the given `obj` if it is a list.
> Only the pairs themselves are copied; the cars of the result are
> the same (in the sense of `eqv?`) as the cars of `list`.
> If `obj` is an improper list, so is the result, and the final
> cdrs are the same in the sense of `eqv?`.
> An `obj` which is not a list is returned unchanged.
> It is an error if `obj` is a circular list.
> 
> ````
> (define a '(1 8 2 8))     ; a may be immutable
> (define b (list-copy a))
> (set-car! b 3)            ; b is mutable
> b  ===>  (3 8 2 8)
> a  ===>  (1 8 2 8)
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

