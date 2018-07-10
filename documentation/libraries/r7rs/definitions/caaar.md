

<a id='definition__r7rs__caaar'></a>

# `caaar` -- `r7rs` Definitions


#### Kind

`accessor`;


#### Procedure signature

Procedure variants:
 * `((|any|) |->| (|any|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (caaar pair)
> (caadr pair)
> ...
> (cdddar pair)
> (cddddr pair)
> ````
> 
> 
> These twenty-four procedures are further compositions of `car` and `cdr`
> on the same principles.
> For example, `caddr` could be defined by:
> 
> ````
> (define caddr (lambda (x) (car (cdr (cdr x)))))
> ````
> 
> Arbitrary compositions up to four deep are provided.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:cxr`](../../r7rs/categories/r7rs_3a_cxr.md#category__r7rs__r7rs_3a_cxr);
[`vs:pairs`](../../r7rs/categories/vs_3a_pairs.md#category__r7rs__vs_3a_pairs);
[`vs:lists`](../../r7rs/categories/vs_3a_lists.md#category__r7rs__vs_3a_lists);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

