

<a id='definition__r7rs__make-promise'></a>

# `make-promise` -- `r7rs` Definitions


#### Kind

`constructor`;


#### Procedure signature

Procedure variants:
 * `((|any|) |->| (|promise|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`promise`](../../r7rs/types/promise.md#type__r7rs__promise);


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`promise`](../../r7rs/types/promise.md#type__r7rs__promise);


#### Description

> ````
> (make-promise obj)
> ````
> 
> 
> The `make-promise` procedure returns a promise which, when forced, will return
> `obj`.  It is similar to `delay`, but does not delay
> its argument: it is a procedure rather than syntax.
> If `obj` is already a promise, it is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:lazy`](../../r7rs/categories/r7rs_3a_lazy.md#category__r7rs__r7rs_3a_lazy);
[`vs:promises`](../../r7rs/categories/vs_3a_promises.md#category__r7rs__vs_3a_promises);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

