

<a id='definition__r7rs__raise'></a>

# `raise` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|any|) |->| (|exception|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`exception`](../../r7rs/types/exception.md#type__r7rs__exception);


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`exception`](../../r7rs/types/exception.md#type__r7rs__exception);


#### Description

> ````
> (raise obj)
> ````
> 
> 
> Raises an exception by invoking the current exception
> handler on `obj`. The handler is called with the same
> dynamic environment as that of the call to `raise`, except that
> the current exception handler is the one that was in place when the
> handler being called was installed.  If the handler returns, a secondary
> exception is raised in the same dynamic environment as the handler.
> The relationship between `obj` and the object raised by
> the secondary exception is unspecified.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:errors`](../../r7rs/categories/vs_3a_errors.md#category__r7rs__vs_3a_errors);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

